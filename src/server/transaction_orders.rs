use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::pool;
        use crate::auth::get_user;
        use crate::auth::User;
        use sqlx::query;
        use crate::server::utils::*;

        async fn delete_transaction_order_fn(trans_order_id: i64, user_id: i64)-> Result<(), ServerFnError>{
            let pool = pool()?;
            if query!(
                "DELETE FROM transaction_orders WHERE id=$1 AND sender_id=$2",
                trans_order_id,
                user_id,
            ).execute(&pool)
            .await.is_err(){
                return Err(ServerFnError::ServerError("Internal error during deleting listing".to_string()));
            };
            Ok(())
        }

        async fn create_transaction_order_fn(sender_id: i64, reciver_username: String, currency_code: String, amount: i64, delay_days: i64, title: String) -> Result<(), ServerFnError>{
            let pool = pool()?;

            let reciver_id = if let Some(reciver) = User::get_from_username(reciver_username, &pool).await{
                reciver.id
            }else {
                return Err(ServerFnError::ServerError("Reciver with that username does not exist".to_string()));
            };

            let currency_id = match sqlx::query_as!(CurrencyId, "SELECT id FROM currencies WHERE code= $1", currency_code).fetch_one(&pool).await {
                Err(_) => { return Err(ServerFnError::ServerError("Currency: {currency_code} does not exist.".to_string())) },
                Ok(currency) => currency.id
            };

            if amount < 1{
                return Err(ServerFnError::ServerError("Amount can't be less than one.".to_string()));
            }

            if delay_days < 1{
                return Err(ServerFnError::ServerError("Delay can't be less than one.".to_string()));
            }

            // Create transaction order
            if query!(
                "INSERT INTO transaction_orders(sender_id, reciver_id, amount, currency_id, last_transaction, transaction_delay, title) VALUES ($1, $2, $3, $4, '2000-01-11 11:11', $5, $6);",
                sender_id,
                reciver_id,
                amount,
                currency_id,
                delay_days as i32,
                title
            ).execute(&pool)
            .await.is_err(){
                return Err(ServerFnError::ServerError("Internal error during creating transaction order".to_string()));
            }
            Ok(())
        }
    }
}

#[server(DeleteTransactionOrder, "/api")]
pub async fn delete_transaction_order(trans_order_id: i64) -> Result<(), ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            leptos_axum::redirect("/transaction_orders");
            delete_transaction_order_fn(trans_order_id, user.id).await
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to delete transaction order.".to_string(),
        )),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct TransactionOrder {
    pub id: i64,
    pub sender_id: Option<i64>,
    pub reciver_username: Option<String>,
    pub amount: i64,
    pub currency_code: String,
    pub title: String,
    pub transaction_delay: i32,
    pub last_transaction: Option<String>,
}

#[server(GetUserTransactionOrders, "/api")]
pub async fn get_user_transaction_orders() -> Result<Vec<TransactionOrder>, ServerFnError> {
    let pool = pool()?;
    let maby_user = get_user().await;

    match maby_user {
        Ok(Some(user)) => {
            let mut transaction_orders = Vec::new();
            let mut rows = sqlx::query_as!(
                TransactionOrder,
                r#"
                SELECT
                transaction_orders.id as id,
                transaction_orders.sender_id as sender_id,
                transaction_orders.amount as amount,
                transaction_orders.title as title,
                transaction_orders.transaction_delay as transaction_delay,
                to_char(transaction_orders.last_transaction, 'DD Mon YYYY HH:MI:SSPM') as last_transaction,

                currencies.code as currency_code,

                reciver.username as reciver_username 

                FROM transaction_orders 
                JOIN currencies ON transaction_orders.currency_id=currencies.id

                JOIN users as reciver ON transaction_orders.reciver_id = reciver.id


                WHERE transaction_orders.sender_id = $1
                ORDER BY transaction_orders.id DESC"#,
                user.id
            ).fetch(&pool);

            use futures::TryStreamExt;
            while let Some(row) = rows.try_next().await? {
                transaction_orders.push(row);
            }
            Ok(transaction_orders)
        }
        _ => Err(ServerFnError::ServerError(
            "User not logged in.".to_string(),
        )),
    }
}

#[server(NewUserTransactionOrder, "/api")]
pub async fn new_user_transaction_order(
    reciver_username: String,
    currency_code: String,
    amount: i64,
    delay_days: i64,
    title: String,
) -> Result<(), ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            leptos_axum::redirect("/transaction_orders");
            create_transaction_order_fn(
                user.id,
                reciver_username,
                currency_code,
                amount,
                delay_days,
                title,
            )
            .await
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to create withdraw order.".to_string(),
        )),
    }
}
