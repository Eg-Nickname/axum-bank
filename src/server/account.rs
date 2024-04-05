use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::pool;
        use crate::auth::get_user;
        use crate::auth::User;
        use axum_session_auth::HasPermission;
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

#[server(GenerateApiToken, "/api")]
pub async fn generate_api_token() -> Result<(), ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            // Check if user has api permission
            if user.has("api_token_gen", &None).await {
                let pool = pool()?;

                // Generate new Token
                use rand::{distributions::Alphanumeric, Rng};
                let api_token: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect();

                // Update token to Db
                if sqlx::query!(
                    "UPDATE users SET api_token=$1 WHERE id=$2",
                    api_token,
                    user.id
                )
                .execute(&pool)
                .await
                .is_err()
                {
                    return Err(ServerFnError::ServerError(
                        "Error during updating api_key in DB".to_string(),
                    ));
                };
            } else {
                // Return error if user doesn't have api key permission
                return Err(ServerFnError::ServerError(
                    "User can't generate API token.".to_string(),
                ));
            }

            leptos_axum::redirect("/account");
            Ok(())
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to generate API token.".to_string(),
        )),
    }
}

struct ApiToken {
    api_token: Option<String>,
}
#[server(GetApiToken, "/api")]
pub async fn get_api_token() -> Result<Option<String>, ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            let pool = pool()?;

            let token =
                match sqlx::query_as!(ApiToken, "SELECT api_token FROM users WHERE id=$1", user.id)
                    .fetch_one(&pool)
                    .await
                {
                    Err(_) => {
                        return Err(ServerFnError::ServerError(
                            "Couldn't get api token.".to_string(),
                        ))
                    }
                    Ok(api_token) => api_token.api_token,
                };

            Ok(token)
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to get API token.".to_string(),
        )),
    }
}

#[server(AddUserRequest, "/api")]
pub async fn add_user_request() -> Result<(), ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            // TODO add request adding handler function
            Ok(())
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to create user request.".to_string(),
        )),
    }
}
