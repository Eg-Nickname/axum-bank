use cfg_if::cfg_if;
use leptos::{*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[repr(i32)]
pub enum TransactionType{
    Other = 0,
    Transfer = 1,
    Withdraw = 2,
    Payment = 3,
    CurrencyExchange = 4
}

impl From<i32> for TransactionType{
    fn from(number: i32) -> Self{
        match number {
            4 => TransactionType::CurrencyExchange,
            3 => TransactionType::Payment,
            2 => TransactionType::Withdraw,
            1 => TransactionType::Transfer,
            _ => TransactionType::Other,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[repr(i32)]
pub enum TransactionStatus{
    Other = 0,
    Pending = 1,
    Sent = 2
}

impl From<i32> for TransactionStatus{
    fn from(number: i32) -> Self{
        match number {
            2 => TransactionStatus::Sent,
            1 => TransactionStatus::Pending,
            _ => TransactionStatus::Other,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::pool;
        use crate::auth::get_user;
        use crate::auth::User;
        use sqlx::query;
        use crate::server::utils::*;



    // TODO properly log all possible errors on server side
    async fn create_new_transaction(sender_username: String, reciver_username: String, currency_code: String, amount: i64, title: String, transcation_type: TransactionType, transaction_status: TransactionStatus) -> Result<(), ServerFnError>{
        let pool = pool()?;
        
        // Validates if fuction input is correct
        if amount < 0{
            return Err(ServerFnError::ServerError("Transaction ammount can't be negative.".to_string())); 
        }
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
            Err(ServerFnError::ServerError("Can't get user to create transaction.".to_string()))
        },
    }
}

#[server(WithdrawOrder, "/api")]
pub async fn withdraw_order(currency_code: String, amount: i64, withdrawer_nick: String) -> Result<(), ServerFnError>{
    match get_user().await {
        Ok(Some(user)) => {
            leptos_axum::redirect("/transactions");
            // TODO change reciver account from admun
            create_new_transaction(user.username, "Admun".to_string(), currency_code, amount, withdrawer_nick, TransactionType::Withdraw, TransactionStatus::Pending).await },
        _ => {
            Err(ServerFnError::ServerError("Can't get user to create withdraw order.".to_string()))
        },
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Transaction {
    pub id: i64,
    pub sender_id: Option<i64>,
    pub sender_username: Option<String>,
    pub reciver_username: Option<String>,
    pub amount: i64,
    pub currency_code: String,
    pub title: String,
    pub transaction_type: TransactionType,
    pub transaction_status: TransactionStatus,
    // TODO add serde serlalize/deserialize on date
    // pub date: time::OffsetDateTime,
}

// TODO add pagination to transactions
#[server(GetUserTransactions, "/api")]
pub async fn get_user_transactions() -> Result<Vec<Transaction>, ServerFnError> {
    let pool = pool()?;
    let maby_user = get_user().await;

    match maby_user {
        Ok(Some(user)) => {  
            let mut transactions = Vec::new();
            let mut rows = sqlx::query_as!(
                Transaction,
                r#"
                SELECT
                transactions.id as id,
                transactions.sender_id as sender_id,
                transactions.ammount as amount,
                transactions.title as title,
                transactions.status as transaction_status,
                transactions.type as transaction_type,
                currencies.code as currency_code,
                (SELECT username FROM users WHERE id=sender_id) as sender_username, 
                (SELECT username FROM users WHERE id=reciver_id) as reciver_username 
                FROM transactions JOIN currencies ON transactions.currency_id=currencies.id
                WHERE transactions.sender_id= $1 OR transactions.reciver_id = $1
                ORDER BY transactions.id DESC"#,
                user.id
            ).fetch(&pool);

            use futures::TryStreamExt;
            while let Some(row) = rows.try_next().await? {
                transactions.push(row);
            }
            Ok(transactions)
        },
        _ => {
            Err(ServerFnError::ServerError("User not logged in.".to_string()))
        },
    }
}