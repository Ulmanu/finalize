//! Loading, not-found, unauthorized, and error boundaries.

use crate::layouts::PublicLayout;
use leptos::prelude::*;

/// Polite loading indicator shown while route content is pending.
#[component]
pub fn Loading(#[prop(optional, into)] label: String) -> impl IntoView {
    let label = if label.is_empty() {
        "Loading…".to_string()
    } else {
        label
    };
    view! {
        <div class="boundary-loading" role="status" aria-live="polite" data-testid="loading">
            <span class="spinner" aria-hidden="true"></span>
            <span>{label}</span>
        </div>
    }
}

/// Not-found boundary for unknown destinations.
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <PublicLayout>
            <section class="boundary-page" data-testid="not-found">
                <h1>"Page not found"</h1>
                <p>"The destination you followed does not exist in Momentum."</p>
                <p>
                    <a href="/today">"Go to Today"</a>
                </p>
            </section>
        </PublicLayout>
    }
}

/// Unauthorized boundary for protected destinations without an authenticated session.
#[component]
pub fn Unauthorized() -> impl IntoView {
    view! {
        <PublicLayout>
            <section class="boundary-page" data-testid="unauthorized">
                <h1>"Sign in required"</h1>
                <p>"Sign in to open this destination."</p>
                <p>
                    <a href="/login">"Sign in"</a>
                </p>
            </section>
        </PublicLayout>
    }
}

/// Recoverable error fallback rendered by [`AppBoundary`].
#[component]
pub fn ErrorView(#[prop(into, optional)] message: MaybeProp<String>) -> impl IntoView {
    view! {
        <PublicLayout>
            <section class="boundary-page" role="alert" data-testid="error-boundary">
                <h1>"Something went wrong"</h1>
                <p>"This view could not be rendered. Your data was not changed."</p>
                <p class="boundary-detail">
                    {move || message.get().unwrap_or_default()}
                </p>
                <p>
                    <a href="/today">"Back to Today"</a>
                </p>
            </section>
        </PublicLayout>
    }
}

/// Wraps route content with loading and error boundaries.
#[component]
pub fn AppBoundary(children: ChildrenFn) -> impl IntoView {
    let fallback = |errors: ArcRwSignal<Errors>| {
        let message = move || {
            errors
                .get()
                .into_iter()
                .map(|(_, error)| error.to_string())
                .collect::<Vec<_>>()
                .join("; ")
        };
        view! { <ErrorView message=MaybeProp::derive(move || Some(message()))/> }
    };
    view! {
        <ErrorBoundary fallback=fallback>
            <Suspense fallback=|| view! { <Loading/> }>
                {children()}
            </Suspense>
        </ErrorBoundary>
    }
}
