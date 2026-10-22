//! Quick Find and deep search: deterministic, normalized matching across lists, entities,
//! notes, checklists, and logged content.
//!
//! Ranking is intentionally transparent: exact matches first, then prefixes, then word starts,
//! then substrings; field priority (title before checklist before notes) and recency only break
//! ties within the same confidence tier, and every match is returned so nothing is hidden.
//! Matching folds case and Latin diacritics while display text stays original.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use momentum_domain::AccountId;

use crate::areas::AreaService;
use crate::commands::CommandError;
use crate::projects::ProjectService;
use crate::tags::TagService;
use crate::todos::{TodoScope, TodoService, TodoSummary};

/// What kind of destination a hit is.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum SearchKind {
    /// A built-in or special list.
    List,
    /// A setting or settings page.
    Setting,
    /// An area.
    Area,
    /// A project.
    Project,
    /// A heading inside a project.
    Heading,
    /// A tag.
    Tag,
    /// A to-do title.
    Todo,
}

impl SearchKind {
    /// Stable identifier used by markup and tests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Setting => "setting",
            Self::Area => "area",
            Self::Project => "project",
            Self::Heading => "heading",
            Self::Tag => "tag",
            Self::Todo => "todo",
        }
    }
}

/// Which field matched the query.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum SearchField {
    /// The title matched.
    Title,
    /// A checklist item matched.
    Checklist,
    /// Notes matched.
    Notes,
}

impl SearchField {
    /// Stable identifier used by markup and tests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Checklist => "checklist",
            Self::Notes => "notes",
        }
    }
}

/// One navigable destination.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SearchHit {
    /// Display title (original, unnormalized text).
    pub title: String,
    /// In-application destination.
    pub url: String,
    /// Destination kind.
    pub kind: SearchKind,
    /// Field that matched the query.
    pub matched_in: SearchField,
}

/// A ranked hit.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ScoredHit {
    /// The destination.
    pub hit: SearchHit,
    /// Confidence tier: 0 exact, 1 prefix, 2 word start, 3 substring.
    pub score: u8,
    /// Whether the destination appears in the account's recents.
    pub recent: bool,
}

/// Stores recently visited destinations for ranking tie-breaks.
#[async_trait]
pub trait RecentsStore: Send + Sync {
    /// Lists recent destinations, most recent first.
    async fn list(&self, account_id: AccountId) -> Result<Vec<String>, CommandError>;

    /// Records a visit to a destination.
    async fn record(&self, account_id: AccountId, url: &str) -> Result<(), CommandError>;
}

/// In-process recents store (bounded, deduplicated, most recent first).
#[derive(Debug, Default)]
pub struct MemoryRecentsStore {
    entries: Mutex<Vec<(AccountId, String)>>,
}

impl MemoryRecentsStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

const RECENTS_LIMIT: usize = 20;

#[async_trait]
impl RecentsStore for MemoryRecentsStore {
    async fn list(&self, account_id: AccountId) -> Result<Vec<String>, CommandError> {
        Ok(self
            .entries
            .lock()
            .expect("recents lock")
            .iter()
            .filter(|(account, _)| *account == account_id)
            .map(|(_, url)| url.clone())
            .collect())
    }

    async fn record(&self, account_id: AccountId, url: &str) -> Result<(), CommandError> {
        let mut entries = self.entries.lock().expect("recents lock");
        entries.retain(|(account, existing)| *account != account_id || existing.as_str() != url);
        entries.insert(0, (account_id, url.to_string()));
        let mut kept = 0_usize;
        entries.retain(|(account, _)| {
            if *account == account_id {
                kept += 1;
                kept <= RECENTS_LIMIT
            } else {
                true
            }
        });
        Ok(())
    }
}

/// Searches every navigable surface for an account.
pub struct SearchService {
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
    areas: Option<Arc<AreaService>>,
    tags: Option<Arc<TagService>>,
    recents: Option<Arc<dyn RecentsStore>>,
}

impl SearchService {
    /// Creates the service.
    #[must_use]
    pub fn new(
        todos: Arc<TodoService>,
        projects: Arc<ProjectService>,
        areas: Option<Arc<AreaService>>,
        tags: Option<Arc<TagService>>,
        recents: Option<Arc<dyn RecentsStore>>,
    ) -> Self {
        Self {
            todos,
            projects,
            areas,
            tags,
            recents,
        }
    }

    /// Returns ranked hits for `query`; an empty query returns nothing.
    ///
    /// # Errors
    ///
    /// Propagates read-store failures; a failing recents store degrades to no recency boost.
    pub async fn search(
        &self,
        account_id: AccountId,
        query: &str,
    ) -> Result<Vec<ScoredHit>, CommandError> {
        let needle = normalize(query.trim());
        if needle.is_empty() {
            return Ok(Vec::new());
        }
        let recents = match &self.recents {
            Some(store) => store.list(account_id).await.unwrap_or_default(),
            None => Vec::new(),
        };
        let mut raw: Vec<(SearchHit, u8)> = Vec::new();
        for hit in built_in_destinations() {
            if let Some(score) = score_for(&hit.title, &needle) {
                raw.push((hit, score));
            }
        }
        for todo in self.todos.list(account_id, TodoScope::AllOpen).await? {
            if let Some((hit, score)) = todo_hit(&todo, &needle) {
                raw.push((hit, score));
            }
        }
        for logged in self
            .todos
            .list(account_id, TodoScope::Logbook { logged_only: false })
            .await?
        {
            if let Some((hit, score)) = todo_hit(&logged, &needle) {
                raw.push((hit, score));
            }
        }
        for project in self.projects.list(account_id).await? {
            if let Some(score) = score_for(&project.title, &needle) {
                raw.push((
                    SearchHit {
                        title: project.title.clone(),
                        url: format!("/projects/{}", project.id),
                        kind: SearchKind::Project,
                        matched_in: SearchField::Title,
                    },
                    score,
                ));
            } else if let Some(score) = score_for(&project.notes, &needle) {
                raw.push((
                    SearchHit {
                        title: project.title.clone(),
                        url: format!("/projects/{}", project.id),
                        kind: SearchKind::Project,
                        matched_in: SearchField::Notes,
                    },
                    score,
                ));
            }
            for heading in project.headings.iter().filter(|heading| !heading.archived) {
                if let Some(score) = score_for(&heading.title, &needle) {
                    raw.push((
                        SearchHit {
                            title: heading.title.clone(),
                            url: format!("/projects/{}#{}", project.id, heading.id),
                            kind: SearchKind::Heading,
                            matched_in: SearchField::Title,
                        },
                        score,
                    ));
                }
            }
        }
        if let Some(areas) = &self.areas {
            for area in areas.list(account_id).await? {
                if let Some(score) = score_for(&area.title, &needle) {
                    raw.push((
                        SearchHit {
                            title: area.title,
                            url: format!("/areas/{}", area.id),
                            kind: SearchKind::Area,
                            matched_in: SearchField::Title,
                        },
                        score,
                    ));
                }
            }
        }
        if let Some(tags) = &self.tags {
            for tag in tags.list(account_id).await? {
                if let Some(score) = score_for(&tag.title, &needle) {
                    raw.push((
                        SearchHit {
                            title: tag.title.clone(),
                            url: tag_url(tag.id),
                            kind: SearchKind::Tag,
                            matched_in: SearchField::Title,
                        },
                        score,
                    ));
                }
            }
        }
        // One row per destination, keeping the best-matching field.
        let mut best: Vec<(SearchHit, u8)> = Vec::new();
        for (hit, score) in raw {
            let existing = best.iter_mut().find(|(known, _)| known.url == hit.url);
            match existing {
                Some((known, known_score)) => {
                    if (score, hit.matched_in) < (*known_score, known.matched_in) {
                        *known = hit;
                        *known_score = score;
                    }
                }
                None => best.push((hit, score)),
            }
        }
        let mut scored: Vec<ScoredHit> = best
            .into_iter()
            .map(|(hit, score)| {
                let recent = recents.contains(&hit.url);
                ScoredHit { hit, score, recent }
            })
            .collect();
        scored.sort_by(|left, right| {
            left.score
                .cmp(&right.score)
                .then_with(|| left.hit.matched_in.cmp(&right.hit.matched_in))
                .then_with(|| left.recent.cmp(&right.recent).reverse())
                .then_with(|| left.hit.kind.cmp(&right.hit.kind))
                .then_with(|| normalize(&left.hit.title).cmp(&normalize(&right.hit.title)))
                .then_with(|| left.hit.url.cmp(&right.hit.url))
        });
        scored.truncate(60);
        Ok(scored)
    }

    /// Records a visit for recency ranking.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn record_visit(&self, account_id: AccountId, url: &str) -> Result<(), CommandError> {
        match &self.recents {
            Some(store) => store.record(account_id, url).await,
            None => Ok(()),
        }
    }
}

/// Folds Latin diacritics and lowercases text for matching; display text stays original.
#[must_use]
pub fn normalize(text: &str) -> String {
    text.chars()
        .map(fold_char)
        .flat_map(char::to_lowercase)
        .collect()
}

fn fold_char(character: char) -> char {
    match character {
        'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' | 'Á' | 'À' | 'Â' | 'Ä' | 'Ã' | 'Å' => 'a',
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => 'e',
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => 'i',
        'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'Ó' | 'Ò' | 'Ô' | 'Ö' | 'Õ' => 'o',
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => 'u',
        'ç' | 'Ç' => 'c',
        'ñ' | 'Ñ' => 'n',
        'ý' | 'ÿ' | 'Ý' => 'y',
        other => other,
    }
}

fn todo_hit(todo: &TodoSummary, needle: &str) -> Option<(SearchHit, u8)> {
    let url = format!("/todos/{}", todo.id);
    let mut best: Option<(SearchHit, u8, u8)> = None;
    let mut consider = |content: &str, matched_in: SearchField| {
        let Some(score) = score_for(content, needle) else {
            return;
        };
        let field_rank = match matched_in {
            SearchField::Title => 0_u8,
            SearchField::Checklist => 1,
            SearchField::Notes => 2,
        };
        let better = best.as_ref().is_none_or(|(_, best_score, best_rank)| {
            (score, field_rank) < (*best_score, *best_rank)
        });
        if better {
            best = Some((
                SearchHit {
                    title: todo.title.clone(),
                    url: url.clone(),
                    kind: SearchKind::Todo,
                    matched_in,
                },
                score,
                field_rank,
            ));
        }
    };
    consider(&todo.title, SearchField::Title);
    for item in &todo.checklist {
        consider(&item.title, SearchField::Checklist);
    }
    consider(&todo.notes, SearchField::Notes);
    best.map(|(hit, score, _)| (hit, score))
}

fn tag_url(id: momentum_domain::EntityId) -> String {
    format!("/inbox?tags={id}")
}

fn score_for(title: &str, needle: &str) -> Option<u8> {
    let lowered = normalize(title);
    if lowered == needle {
        return Some(0);
    }
    if lowered.starts_with(needle) {
        return Some(1);
    }
    if lowered
        .split(|character: char| !character.is_alphanumeric())
        .any(|word| !word.is_empty() && word.starts_with(needle))
    {
        return Some(2);
    }
    lowered.contains(needle).then_some(3)
}

fn built_in_destinations() -> Vec<SearchHit> {
    [
        ("Inbox", "/inbox", SearchKind::List),
        ("Today", "/today", SearchKind::List),
        ("Upcoming", "/upcoming", SearchKind::List),
        ("Tomorrow", "/tomorrow", SearchKind::List),
        ("Anytime", "/anytime", SearchKind::List),
        ("Someday", "/someday", SearchKind::List),
        ("Deadlines", "/deadlines", SearchKind::List),
        ("Logbook", "/logbook", SearchKind::List),
        ("Repeating", "/repeating", SearchKind::List),
        ("Projects", "/projects", SearchKind::List),
        ("Areas", "/areas", SearchKind::List),
        ("Trash", "/trash", SearchKind::List),
        ("Settings", "/settings", SearchKind::Setting),
    ]
    .into_iter()
    .map(|(title, url, kind)| SearchHit {
        title: title.to_string(),
        url: url.to_string(),
        kind,
        matched_in: SearchField::Title,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::{normalize, score_for};

    #[test]
    fn scores_match_confidence_tiers() {
        assert_eq!(score_for("Roadmap", "roadmap"), Some(0));
        assert_eq!(score_for("Roadmap planning", "roadmap"), Some(1));
        assert_eq!(score_for("Prepare roadmap", "roadmap"), Some(2));
        assert_eq!(score_for("Myroadmap", "roadmap"), Some(3));
        assert_eq!(score_for("Quarterly re-roadmap-ed", "roadmap"), Some(2));
        assert_eq!(score_for("Something else", "roadmap"), None);
        assert_eq!(score_for("INBOX", "inbox"), Some(0));
    }

    #[test]
    fn normalization_folds_case_and_diacritics_for_matching_only() {
        assert_eq!(normalize("Café RÉSUMÉ"), "cafe resume");
        assert_eq!(normalize("Señor Muñoz"), "senor munoz");
        assert_eq!(score_for("Café run", "cafe"), Some(1));
        assert_eq!(score_for("Résumé review", "resume"), Some(1));
        assert_eq!(score_for("El área norte", "area"), Some(2));
    }
}
