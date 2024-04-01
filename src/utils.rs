use cfg_if::cfg_if;
use leptos::*;

use crate::auth::User;
pub type UserContextType =
    Resource<(usize, usize, usize, usize), Result<Option<User>, ServerFnError>>;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::auth::AuthSession;
        use sqlx::PgPool;



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
