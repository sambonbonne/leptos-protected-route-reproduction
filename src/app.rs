use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

enum UserAuthentication {
    Unauthenticated,
    Authenticated,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (authentication, _set_authentication) = create_signal(UserAuthentication::Unauthenticated);

    let is_authenticated = move || authentication.with(|auth| matches!(auth, UserAuthentication::Authenticated));
    let is_unauthenticated = move || authentication.with(|auth| matches!(auth, UserAuthentication::Unauthenticated));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/repro.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                    <ProtectedRoute
                        path="/authenticated"
                        view=AuthenticatedPage
                        condition=is_authenticated
                        redirect_path="/not-authenticated" />
                    <ProtectedRoute
                        path="/not-authenticated" 
                        view=UnauthenticatedPage
                        condition=is_unauthenticated
                        redirect_path="/authenticated" />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn AuthenticatedPage() -> impl IntoView {
    view! {
        <h1>"You are " <strong>"authenticated"</strong></h1>
        <p>"Congratulations!"</p>
    }
}

#[component]
fn UnauthenticatedPage() -> impl IntoView {
    view! {
        <h1>"You are " <strong>"not authenticated"</strong></h1>
        <p>"Nevermind."</p>
    }
}
