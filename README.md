This repository is base on the [Axum starter template for Leptos](https://github.com/leptos-rs/start-axum), created with `cargo leptos new --git leptos-rs/start-axum` (with `cargo-leptos` version `0.1.11`).

The bug seems to be linked to the use of a signal in a `ProtectedRoute` condition function.


What has been added?
====================

The only file edition is the `src/app.rs` file (and the `README.md` file):

1. Added a `UserAuthentication` enum.
2. Added `AuthenticatedPage` and `UnauthenticatedPage` components.
3. In the `App` component, created a signal which store the authentication state (which is a `UserAuthentication` value).
4. Still in the `App` component, created two closures guards named `is_authenticated` and `is_unauthanticated` that check the current status of the signal.
5. In the `Routes`, added two `ProtectedRoute`, each using one of the closure guard and redirecting to the other when the guard fails.


How to reproduce?
=================

Run `cargo leptos watch` in this repository, wait for compilation and try to got to `http://localhost:3000/authenticated` or `http://localhost:3000/not-authenticated`.

The page won't load but you'll see that `cargo leptos watch` logged a panic (but is still running).
