use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use serde::{Deserialize, Serialize};

        pub struct CurrencyId{
            pub id: i64,
        }
        pub struct CurrencyCode{
            pub code: String,
        }
    
        pub struct UserCurrencyBalance{
            #[allow(dead_code)]
            pub user_id: i64,
            #[allow(dead_code)]
    
            pub currency_id: i64,
            pub balance: i64,
        }
    
        // TODO properly log error on server side
        pub async fn get_balance(db_transcation: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: i64, currency_id: i64) -> Result<UserCurrencyBalance, ServerFnError>{
            match sqlx::query_as!(UserCurrencyBalance, "SELECT * FROM account_balance WHERE user_id=$1 AND currency_id=$2", user_id, currency_id).fetch_one(&mut **db_transcation).await{
                Err(_) => return Err(ServerFnError::ServerError("Can't get currency balance of sender: {sender_username}".to_string())),
                Ok(balance) => {
                    return Ok(balance);
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[cfg_attr(feature = "ssr", derive(sqlx::Type))]
        #[repr(i32)]
        pub enum UserRequestType{
            Other = 0,
            GetApiPerm = 1,
            AddCurrency = 2,
        }

        impl From<i32> for UserRequestType {
            fn from(number: i32) -> Self {
                match number {
                    2 => UserRequestType::AddCurrency,
                    1 => UserRequestType::GetApiPerm,
                    _ => UserRequestType::Other,
                }
            }
        }

        pub async fn create_user_request(user_id: i64, request_mesage: String, request_type: UserRequestType) -> Result<(), ServerFnError>{
            // Add data to database
            // Verify
            todo!()
        }
    }
}