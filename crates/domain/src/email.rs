//! Email address value object.

use std::fmt;

use thiserror::Error;

/// Email validation failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum EmailError {
    /// The address is empty after trimming.
    #[error("email address is empty")]
    Empty,
    /// The address does not contain exactly one `@` with local and domain parts.
    #[error("email address is not valid")]
    Invalid,
    /// The domain has no dot-separated suffix.
    #[error("email domain is not valid")]
    InvalidDomain,
    /// The address is longer than the maximum supported length.
    #[error("email address is too long")]
    TooLong,
}

/// A validated email address that preserves the user-visible form and a normalized form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email {
    display: String,
    normalized: String,
}

impl Email {
    /// Parses and validates an email address.
    ///
    /// # Errors
    ///
    /// Returns the specific [`EmailError`] when the address is empty, malformed, or too long.
    pub fn parse(input: &str) -> Result<Self, EmailError> {
        let display = input.trim().to_string();
        if display.is_empty() {
            return Err(EmailError::Empty);
        }
        if display.len() > 254 {
            return Err(EmailError::TooLong);
        }
        if display.chars().any(char::is_whitespace) {
            return Err(EmailError::Invalid);
        }

        let mut parts = display.split('@');
        let (Some(local), Some(domain), None) = (parts.next(), parts.next(), parts.next()) else {
            return Err(EmailError::Invalid);
        };
        if local.is_empty() || domain.is_empty() || !domain.contains('.') {
            return Err(EmailError::InvalidDomain);
        }
        if domain.starts_with('.') || domain.ends_with('.') || domain.contains("..") {
            return Err(EmailError::InvalidDomain);
        }
        if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
            return Err(EmailError::Invalid);
        }
        if !display
            .chars()
            .all(|c| c.is_ascii() && !c.is_control() && !c.is_whitespace())
        {
            return Err(EmailError::Invalid);
        }

        let normalized = display.to_lowercase();
        Ok(Self {
            display,
            normalized,
        })
    }

    /// Returns the user-visible form of the address.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.display
    }

    /// Returns the normalized (lowercased, trimmed) form used for lookup and uniqueness.
    #[must_use]
    pub fn normalized(&self) -> &str {
        &self.normalized
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.display)
    }
}

impl std::str::FromStr for Email {
    type Err = EmailError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_and_normalizes_valid_addresses() {
        let email = Email::parse("  Person@Example.COM ").expect("valid");
        assert_eq!(email.as_str(), "Person@Example.COM");
        assert_eq!(email.normalized(), "person@example.com");
    }

    #[test]
    fn rejects_invalid_addresses() {
        assert_eq!(Email::parse(""), Err(EmailError::Empty));
        assert_eq!(Email::parse("   "), Err(EmailError::Empty));
        assert_eq!(Email::parse("no-at-sign"), Err(EmailError::Invalid));
        assert_eq!(Email::parse("a@b"), Err(EmailError::InvalidDomain));
        assert_eq!(Email::parse("a@@b.com"), Err(EmailError::Invalid));
        assert_eq!(Email::parse("a b@example.com"), Err(EmailError::Invalid));
        assert_eq!(Email::parse(".a@example.com"), Err(EmailError::Invalid));
    }

    #[test]
    fn normalization_is_lookup_stable() {
        let first = Email::parse("User@Example.com").expect("valid");
        let second = Email::parse("user@example.com").expect("valid");
        assert_eq!(first.normalized(), second.normalized());
    }
}
