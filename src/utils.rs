use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::auth::AuthSession;
        use sqlx::PgPool;
        use leptos::*;

        pub fn pool() -> Result<PgPool, ServerFnError> {
            use_context::<PgPool>()
                .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))

        }
    }
}