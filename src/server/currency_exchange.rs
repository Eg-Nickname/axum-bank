use cfg_if::cfg_if;
use leptos::{*};
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::pool;
        use sqlx::query;
        use crate::auth::get_user;
        use crate::auth::User;
        use crate::server::utils::*;

        fn nwd(x: i64, y: i64) -> i64{
            let mut a = x;
            let mut b = y;

            while a!=b{
                if a>b{ a=a-b }
                else{ b=b-a }
            }
            a
        }

        // TODO properly log error on server side
        // TODO fix this shit
        async fn create_new_exchange_listing(listing_creator_username: String, currency_code_from: String, currency_code_to: String, amount_from: i64, amount_to: i64, is_fixed: bool) -> Result<(), ServerFnError>{
            let pool = pool()?;

            // Check if user with listing_creator_username exists
            let listing_creator = match User::get_from_username(listing_creator_username, &pool).await{
                None => { return Err(ServerFnError::ServerError("Listing creator: {listing_creator_username} does not exist.".to_string())); },
                Some(sender) => sender
            };
            // Check if currency with currency_code_from exists
            let currency_from_id = match sqlx::query_as!(CurrencyId, "SELECT id FROM currencies WHERE code= $1", currency_code_from).fetch_one(&pool).await {
                Err(_) => { return Err(ServerFnError::ServerError("Currency: {currency_code} does not exist.".to_string())) },
                Ok(currency) => currency.id 
            };
            // Check if currency with currency_code_to exists
            let currency_to_id = match sqlx::query_as!(CurrencyId, "SELECT id FROM currencies WHERE code= $1", currency_code_to).fetch_one(&pool).await {
                Err(_) => { return Err(ServerFnError::ServerError("Currency: {currency_code} does not exist.".to_string())) },
                Ok(currency) => currency.id 
            };
            if amount_from < 0{
                return Err(ServerFnError::ServerError("Ammount from can't be negative.".to_string()))
            }
            if amount_to < 0{
                return Err(ServerFnError::ServerError("Ammount to can't be negative.".to_string()))
            }

            // Begin db transaction to ensure that user balance won't change during transaction creation
            let mut db_transaction = match pool.begin().await{
                Err(_) => {
                    return Err(ServerFnError::ServerError("Cannot begin transaction".to_string()));
                },
                Ok(transaction) => transaction
            };

            let listing_creator_balacne = get_balance(&mut db_transaction, listing_creator.id, currency_from_id).await?;
            
            if listing_creator_balacne.balance - amount_from < 0{
                return Err(ServerFnError::ServerError("Not enough balance to create exchange listing.".to_string()));
            }
            // Update sender balance
            match query!(
                "UPDATE account_balance SET balance = balance - $1 WHERE user_id = $2 AND currency_id = $3;",
                amount_from,
                listing_creator.id,
                currency_from_id
            ).execute(&mut *db_transaction)
            .await{
                Err(_) =>{
                    return Err(ServerFnError::ServerError("Internal error during creating exchange listing".to_string()));
                },
                Ok(_) => (),
            }
            
            let nwd = nwd(amount_from, amount_to);
            let ratio_from = amount_from/nwd;
            let ratio_to = amount_to/nwd;

            // Create new exchange listing record
            match query!(
                "INSERT INTO currency_exchange_listings(listing_creator, currency_from_id, amount_from, currency_to_id, amount_to, ratio_from, ratio_to, is_fixed) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
                listing_creator.id,
                currency_from_id,
                amount_from,
                currency_to_id,
                amount_to,
                ratio_from,
                ratio_to,
                is_fixed
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
    }
}

#[server(CreateExchangeListing, "/api")]
pub async fn create_exchange_listing(currency_code_from: String, currency_code_to: String, amount_from: i64, amount_to: i64) -> Result<(), ServerFnError>{
    match get_user().await {
        Ok(Some(user)) => {
            leptos_axum::redirect("/currency_exchange");
            create_new_exchange_listing(user.username, currency_code_from, currency_code_to, amount_from, amount_to, false).await
        },
        _ => {
            Err(ServerFnError::ServerError("Can't get user to create withdraw order.".to_string()))
        },
    }
}