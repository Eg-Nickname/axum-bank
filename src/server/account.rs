use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::pool;
        use crate::auth::get_user;
        use crate::auth::User;
        use sqlx::query;
        use bcrypt::{hash, verify, DEFAULT_COST};
        use tracing::warn;

        struct PasswordHash{
            password: String
        }
        // TODO FIX
        async fn change_user_password(user_id: i64, old_password: String, new_password: String, new_password_repeat: String) -> Result<(), ServerFnError>{
            let pool = pool()?;
            if new_password != new_password_repeat{
                return Err(ServerFnError::ServerError("New passwords did not match.".to_string()));
            }
            let old_password_hash = match sqlx::query_as!(
                PasswordHash,
                "SELECT password FROM users WHERE id=$1",
                user_id
            )
            .fetch_one(&pool)
            .await{
                Err(_) => return Err(ServerFnError::ServerError("Couldn't get password hash.".to_string())),
                Ok(hash) => hash.password,
            };

            match verify(old_password, &old_password_hash){
                Ok(true) => {},
                _ => {
                    return Err(ServerFnError::ServerError("Old password isn't correct.".to_string()))
                }
            }

            let new_password_hash = hash(new_password, DEFAULT_COST).unwrap();
            if sqlx::query!(
                "UPDATE users SET password=$1 WHERE id=$2",
                &new_password_hash,
                user_id
            )
            .execute(&pool)
            .await.is_err(){
                return Err(ServerFnError::ServerError("Error during updating password in DB".to_string()))
            };

            Ok(())
        }
    }
}

#[server(ChangePassword, "/api")]
pub async fn change_password(
    old_password: String,
    new_password: String,
    new_password_repeat: String,
) -> Result<(), ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            change_user_password(user.id, old_password, new_password, new_password_repeat).await?;
            use crate::utils::auth;
            let auth = auth()?;

            auth.logout_user();
            leptos_axum::redirect("/");
            Ok(())
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to change password.".to_string(),
        )),
    }
}
