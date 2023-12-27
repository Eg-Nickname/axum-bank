use cfg_if::cfg_if;
use leptos::{*};
use serde::{Deserialize, Serialize};


cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::pool;
        use crate::auth::get_user;
        use crate::auth::User;
        use sqlx::query;

        #[derive(sqlx::Type)]
        #[repr(i32)]
        pub enum TransactionType{
            Other = 0,
            Transfer = 1,
            Withdraw = 2,
            Payment = 3,
            CurrencyExchange = 4
        }
        
        #[derive(sqlx::Type)]
        #[repr(i32)]
        pub enum TransactionStatus{
            Other = 0,
            Pending = 1,
            Sent = 2
        }
        

    struct CurrencyId{
        id: i64,
    }

    struct UserCurrencyBalance{
        #[allow(dead_code)]
        user_id: i64,
        #[allow(dead_code)]

        currency_id: i64,
        balance: i64,
    }

    // TODO properly log error on server side
    async fn get_balance(db_transcation: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: i64, currency_id: i64) -> Result<UserCurrencyBalance, ServerFnError>{
        match sqlx::query_as!(UserCurrencyBalance, "SELECT * FROM account_balance WHERE user_id=$1 AND currency_id=$2", user_id, currency_id).fetch_one(&mut **db_transcation).await{
            Err(_) => return Err(ServerFnError::ServerError("Can't get currency balance of sender: {sender_username}".to_string())),
            Ok(balance) => {
                return Ok(balance);
            }
        }
    }

    // TODO properly log all possible errors on server side
    async fn create_new_transaction(sender_username: String, reciver_username: String, currency_code: String, amount: i64, title: String, transcation_type: TransactionType, transaction_status: TransactionStatus) -> Result<(), ServerFnError>{
        let pool = pool()?;
        
        // Validates if fuction input is correct
        // Check if user with sender_username exists
        let sender = match User::get_from_username(sender_username, &pool).await{
            None => { return Err(ServerFnError::ServerError("Sender: {sender_username} does not exist.".to_string())); },
            Some(sender) => sender
        };
        // Check if user with reciver_username exists
        let reciver = match User::get_from_username(reciver_username, &pool).await{
            None => { return Err(ServerFnError::ServerError("Reciver: {reciver_username} does not exist.".to_string())); },
            Some(reciver) => reciver
        };
        // Check if currency with currency_code exists
        let currency_id = match sqlx::query_as!(CurrencyId, "SELECT id FROM currencies WHERE code= $1", currency_code).fetch_one(&pool).await {
            Err(_) => { return Err(ServerFnError::ServerError("Currency: {currency_code} does not exist.".to_string())) },
            Ok(currency) => currency.id 
        };

        
        // Begin db transaction to ensure that user balances won't change during transaction creation
        let mut db_transaction = match pool.begin().await{
            Err(_) => {
                return Err(ServerFnError::ServerError("Cannot begin transaction".to_string()));
            },
            Ok(transaction) => transaction
        };
        
        // Get sender balances
        let sender_balance = get_balance(&mut db_transaction, sender.id, currency_id).await?;

        // Check if sender has enough balance to make transaction
        if sender_balance.balance - amount < 0{
            return Err(ServerFnError::ServerError("Not enough balance to create transaction.".to_string()));
        }
        // TODO Add transaction to db and update sender & reciver balance
        // Update sender balance
        match query!(
            "UPDATE account_balance SET balance = balance - $1 WHERE user_id = $2 AND currency_id = $3;",
            amount,
            sender.id,
            currency_id
        ).execute(&mut *db_transaction)
        .await{
            Err(_) =>{
                return Err(ServerFnError::ServerError("Internal error during creating transaction".to_string()));
            },
            Ok(_) => (),
        }

        // Update reciver balance
        match query!(
            "UPDATE account_balance SET balance = balance + $1 WHERE user_id = $2 AND currency_id = $3;",
            amount,
            reciver.id,
            currency_id
        ).execute(&mut *db_transaction)
        .await{
            Err(_) =>{
                return Err(ServerFnError::ServerError("Internal error during creating transaction".to_string()));
            },
            Ok(_) => (),
        }

        // Create transaction record
        match query!(
            "INSERT INTO transactions(sender_id, reciver_id, currency_id, ammount, status, type, title) VALUES ($1, $2, $3, $4, $5, $6, $7);",
            sender.id,
            reciver.id,
            currency_id,
            amount,
            transaction_status as TransactionStatus,
            transcation_type as TransactionType,
            title
        ).execute(&mut *db_transaction)
        .await{
            Err(_) =>{
                return Err(ServerFnError::ServerError("Internal error during creating transaction".to_string()));
            },
            Ok(_) => (),
        }

        // Commit db transaction
        match db_transaction.commit().await{
            Err(_) =>{
                return Err(ServerFnError::ServerError("Internal error during creating transaction".to_string()));
            },
            Ok(_) => (),
        }
        Ok(())
    }
}}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Balance {
    pub currency_code: String,
    pub currency_name: String,
    pub balance: i64,
}

#[server(GetUserBalances, "/api")]
pub async fn get_user_balances() -> Result<Vec<Balance>, ServerFnError> {
    let pool = pool()?;
    let maby_user = get_user().await;

    match maby_user {
        Ok(Some(user)) => {  
            let mut balances = Vec::new();
            let mut rows = sqlx::query_as!(
                Balance,
                "SELECT currencies.code as currency_code, currencies.name as currency_name, account_balance.balance FROM account_balance JOIN currencies ON account_balance.currency_id = currencies.id WHERE account_balance.user_id = $1",
                user.id
            ).fetch(&pool);

            use futures::TryStreamExt;
            while let Some(row) = rows.try_next().await? {
                balances.push(row);
            }
            Ok(balances)
        },
        _ => {
            Err(ServerFnError::ServerError("User not logged in.".to_string()))
        },
    }
}

#[server(NewUserTransaction, "/api")]
pub async fn new_user_transaction(reciver_username: String, currency_code: String, amount: i64, title: String) -> Result<(), ServerFnError>{
    match get_user().await {
        Ok(Some(user)) => {
            leptos_axum::redirect("/transactions");
            create_new_transaction(user.username, reciver_username, currency_code, amount, title, TransactionType::Transfer, TransactionStatus::Sent).await },
        _ => {
            Err(ServerFnError::ServerError("Can't get user to create transaction".to_string()))
        },
    }
}