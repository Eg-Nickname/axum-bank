use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::utils::pool;
        use sqlx::query;
        use crate::auth::get_user;
        use crate::auth::User;
        use crate::server::utils::*;
        use crate::server::transactions::{TransactionType, TransactionStatus};


        fn nwd(x: i64, y: i64) -> i64{
            let mut a = x;
            let mut b = y;

            while a != b{
                if a>b{ a=a-b }
                else{ b=b-a }
            }
            a
        }

        async fn get_protected_exchange_listing(db_transcation: &mut sqlx::Transaction<'_, sqlx::Postgres>, listing_id: i64) -> Result<RawExchangeListing, ServerFnError>{
            match sqlx::query_as!(
                RawExchangeListing,
                r#"SELECT
                id,
                listing_creator as creator_id, 
                currency_from_id, 
                currency_to_id, 
                amount_from, 
                amount_to, 
                ratio_from, 
                ratio_to,
                is_fixed 
        
                FROM currency_exchange_listings as cel 

                WHERE id = $1
                "#,
                listing_id
            ).fetch_one(&mut **db_transcation)
            .await{
                Err(_) =>{
                    return Err(ServerFnError::ServerError("Can't get listing with provided id {listing_id}".to_string()));
                },
                Ok(listing) => return Ok(listing),
            }
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

        async fn use_exchange_listing_fn(listing_id: i64, exchanger_id: i64, amount_exchange_to: i64, amount_exchange_from: i64) -> Result<(), ServerFnError>{
            let pool = pool()?;

            // Begin db transaction to ensure that user balance won't change during transaction creation
            let mut db_transaction = match pool.begin().await{
                Err(_) => {
                    return Err(ServerFnError::ServerError("Cannot begin transaction".to_string()));
                },
                Ok(transaction) => transaction
            };

            // GET LISTING
            // TODO move to separate function
            let listing = get_protected_exchange_listing(&mut db_transaction, listing_id).await?;

            // Check if ammounts from and to are correct
            if amount_exchange_to%listing.ratio_to != 0 || amount_exchange_from%listing.ratio_from != 0 || amount_exchange_to/listing.ratio_to != amount_exchange_from/listing.ratio_from{
                return Err(ServerFnError::ServerError("Amount to/from parameters not correct.".to_string()));
            }

            // Check if exchanging user have enough balance
            let exchanger_balance = get_balance(&mut db_transaction, exchanger_id, listing.currency_to_id).await?;
            if exchanger_balance.balance - amount_exchange_to <0{
                return Err(ServerFnError::ServerError("User has not enough balance to exchange".to_string()));
            }

            // Check if offer has sufficient ammounts 1. if yes just substract and update it 2. Just trade maximum ammount and delete offer
            if listing.is_fixed{
                // Do nothing
            }else if listing.amount_to - amount_exchange_to > 0 {
                // Update exchange offer ammounts to exchange
                match query!(
                    "UPDATE currency_exchange_listings SET amount_to = amount_to - $1, amount_from = amount_from - $2 WHERE id=$3",
                    amount_exchange_to,
                    amount_exchange_from,
                    listing.id,
                ).execute(&mut *db_transaction)
                .await{
                    Err(_) =>{
                        return Err(ServerFnError::ServerError("Internal error during updating listing".to_string()));
                    },
                    Ok(_) => (),
                }
            }else{
                // Delete exchange offer due to fullfilment
                match query!(
                    "DELETE FROM currency_exchange_listings WHERE id=$1",
                    listing.id,
                ).execute(&mut *db_transaction)
                .await{
                    Err(_) =>{
                        return Err(ServerFnError::ServerError("Internal error during deleting listing".to_string()));
                    },
                    Ok(_) => (),
                }
            }
            // Selects amount provided by user or maximum amount of that offer if it is less
            let exchanged_amount_to = amount_exchange_to.min(listing.amount_to);
            let exchanged_amount_from = amount_exchange_from.min(listing.amount_from);

            // Uptade exchanger balance
            // Updates user balance in currency send
            match query!(
                "UPDATE account_balance SET balance = balance - $1 WHERE user_id=$2 AND currency_id=$3",
                exchanged_amount_to,
                exchanger_id,
                listing.currency_to_id
            ).execute(&mut *db_transaction)
            .await{
                Err(_) =>{
                    return Err(ServerFnError::ServerError("Internal error during updating listing".to_string()));
                },
                Ok(_) => (),
            }
            // Updates user balance in currency recived
            match query!(
                "UPDATE account_balance SET balance = balance + $1 WHERE user_id=$2 AND currency_id=$3",
                exchanged_amount_from,
                exchanger_id,
                listing.currency_from_id
            ).execute(&mut *db_transaction)
            .await{
                Err(_) =>{
                    return Err(ServerFnError::ServerError("Internal error during updating listing".to_string()));
                },
                Ok(_) => (),
            }

            // Updates offer creator balance if exists
            if let Some(creator_id) = listing.creator_id{
                match query!(
                    "UPDATE account_balance SET balance = balance + $1 WHERE user_id=$2 AND currency_id=$3",
                    exchanged_amount_to,
                    creator_id,
                    listing.currency_to_id
                ).execute(&mut *db_transaction)
                .await{
                    Err(_) =>{
                        return Err(ServerFnError::ServerError("Internal error during updating listing".to_string()));
                    },
                    Ok(_) => (),
                }
            }

            // Add transaction from exchanger to offer creator
            match query!(
                "INSERT INTO transactions(sender_id, reciver_id, currency_id, ammount, status, type, title) VALUES ($1, $2, $3, $4, $5, $6, $7);",
                exchanger_id,
                listing.creator_id,
                listing.currency_to_id,
                exchanged_amount_to,
                TransactionStatus::Sent as TransactionStatus,
                TransactionType::CurrencyExchange as TransactionType,
                "Wymiana"
            ).execute(&mut *db_transaction)
            .await{
                Err(_) =>{
                    return Err(ServerFnError::ServerError("Internal error during inserting exchange transaction.".to_string()));
                },
                Ok(_) => (),
            }

            // Add transaction from offer creator to exchanger
            match query!(
                "INSERT INTO transactions(sender_id, reciver_id, currency_id, ammount, status, type, title) VALUES ($1, $2, $3, $4, $5, $6, $7);",
                listing.creator_id,
                exchanger_id,
                listing.currency_from_id,
                exchanged_amount_from,
                TransactionStatus::Sent as TransactionStatus,
                TransactionType::CurrencyExchange as TransactionType,
                "Wymiana"
            ).execute(&mut *db_transaction)
            .await{
                Err(_) =>{
                    return Err(ServerFnError::ServerError("Internal error during inserting exchange transaction.".to_string()));
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

        async fn delete_exchange_listing_fn(listing_id: i64, user_id: i64) -> Result<(), ServerFnError>{
            let pool = pool()?;

            let user = match User::get(user_id, &pool).await{
                None => { return Err(ServerFnError::ServerError("User with id: {user_id} does not exist.".to_string())); },
                Some(sender) => sender
            };

            // Begin db transaction to ensure that user balance won't change during transaction creation
            let mut db_transaction = match pool.begin().await{
                Err(_) => {
                    return Err(ServerFnError::ServerError("Cannot begin transaction".to_string()));
                },
                Ok(transaction) => transaction
            };

            // GET LISTING
            let listing = get_protected_exchange_listing(&mut db_transaction, listing_id).await?;

            // Check if logged user is creator of the listing+
            if Some(user.id) != listing.creator_id{
                return Err(ServerFnError::ServerError("User: {user.username} is not the creator of the listing.".to_string()));
            }

            match query!(
                "UPDATE account_balance SET balance = balance + $1 WHERE user_id = $2 AND currency_id = $3;",
                listing.amount_from,
                user.id,
                listing.currency_from_id,
            ).execute(&mut *db_transaction)
            .await{
                Err(_) =>{
                    return Err(ServerFnError::ServerError("Internal error during updating user balance".to_string()));
                },
                Ok(_) => (),
            }

            match query!(
                "DELETE FROM currency_exchange_listings WHERE id=$1 AND listing_creator=$2",
                listing.id,
                user.id,
            ).execute(&mut *db_transaction)
            .await{
                Err(_) =>{
                    return Err(ServerFnError::ServerError("Internal error during deleting listing".to_string()));
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

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct ValidatedExchangeListingsQueryData{
            currency_code_from: String,
            currency_code_to: String,
            min_amount_from: i64,
            min_amount_to: i64
        }
        impl ExchangeListingsQueryData{
            async fn validate(self) -> Result<ValidatedExchangeListingsQueryData, ServerFnError>{
                let pool = pool()?;

                let currency_from_code = match sqlx::query_as!(CurrencyCode, "SELECT code FROM currencies WHERE code= $1", self.currency_code_from).fetch_one(&pool).await {
                    Err(_) => "%".to_string(),
                    Ok(currency) => currency.code
                };

                let currency_to_code = match sqlx::query_as!(CurrencyCode, "SELECT code FROM currencies WHERE code= $1", self.currency_code_to).fetch_one(&pool).await {
                    Err(_) => "%".to_string(),
                    Ok(currency) => currency.code
                };

                Ok(ValidatedExchangeListingsQueryData{
                    currency_code_from: currency_from_code,
                    currency_code_to: currency_to_code,
                    min_amount_from: self.min_amount_from,
                    min_amount_to: self.min_amount_to
                })
            }
        }

    }
}

#[server(CreateExchangeListing, "/api")]
pub async fn create_exchange_listing(
    currency_code_from: String,
    currency_code_to: String,
    amount_from: i64,
    amount_to: i64,
) -> Result<(), ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            leptos_axum::redirect("/currency_exchange");
            create_new_exchange_listing(
                user.username,
                currency_code_from,
                currency_code_to,
                amount_from,
                amount_to,
                false,
            )
            .await
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to create withdraw order.".to_string(),
        )),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct RawExchangeListing {
    pub id: i64,
    pub creator_id: Option<i64>,
    pub currency_from_id: i64,
    pub currency_to_id: i64,
    pub amount_from: i64,
    pub amount_to: i64,
    pub ratio_from: i64,
    pub ratio_to: i64,
    pub is_fixed: bool,
}

#[server(DeleteExchangeListing, "/api")]
pub async fn delete_exchange_listing(listing_id: i64) -> Result<(), ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            leptos_axum::redirect("/currency_exchange");
            delete_exchange_listing_fn(listing_id, user.id).await
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to create withdraw order.".to_string(),
        )),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeListingsQueryData {
    pub currency_code_from: String,
    pub currency_code_to: String,
    pub min_amount_from: i64,
    pub min_amount_to: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ExchangeListing {
    pub id: i64,
    pub listing_creator_username: String,
    pub listing_creator_id: i64,
    pub currency_from_code: String,
    pub currency_to_code: String,
    pub amount_from: i64,
    pub amount_to: i64,
    pub ratio_from: i64,
    pub ratio_to: i64,
}
#[server(GetExchangeListing, "/api")]
pub async fn get_exchange_listing(listing_id: i64) -> Result<ExchangeListing, ServerFnError> {
    let pool = pool()?;

    match sqlx::query_as!(
        ExchangeListing,
        r#"SELECT 
        cel.id as id,
        users.id as listing_creator_id, 
        users.username as listing_creator_username, 
        cur_from.code as currency_from_code, 
        cur_to.code as currency_to_code, 
        cel.amount_from, 
        cel.amount_to, 
        cel.ratio_from, 
        cel.ratio_to
        
        FROM currency_exchange_listings as cel 
        JOIN users ON cel.listing_creator = users.id
        JOIN currencies as cur_from ON cel.currency_from_id = cur_from.id
        JOIN currencies as cur_to ON cel.currency_to_id = cur_to.id

        WHERE cel.id = $1
        "#,
        listing_id
    )
    .fetch_one(&pool)
    .await
    {
        Ok(listing) => Ok(listing),
        Err(_) => Err(ServerFnError::ServerError(
            "Can't get listing with id: {listing_id}.".to_string(),
        )),
    }
}

#[server(GetExchangeListings, "/api")]
pub async fn get_exchange_listings(
    querry_data: ExchangeListingsQueryData,
) -> Result<Vec<ExchangeListing>, ServerFnError> {
    let pool = pool()?;

    let validated_querry_data = querry_data.validate().await?;

    let mut exchange_listings = Vec::new();
    let mut rows = sqlx::query_as!(
        ExchangeListing,
        r#"SELECT 
        cel.id as id,
        users.id as listing_creator_id, 
        users.username as listing_creator_username, 
        cur_from.code as currency_from_code, 
        cur_to.code as currency_to_code, 
        cel.amount_from, 
        cel.amount_to, 
        cel.ratio_from, 
        cel.ratio_to
        
        FROM currency_exchange_listings as cel 
        JOIN users ON cel.listing_creator = users.id
        JOIN currencies as cur_from ON cel.currency_from_id = cur_from.id
        JOIN currencies as cur_to ON cel.currency_to_id = cur_to.id

        WHERE cel.amount_from >= $1
        AND cel.amount_to >= $2
        AND cur_from.code LIKE $3
        AND cur_to.code LIKE $4
        ORDER BY CAST(cel.ratio_to AS DOUBLE PRECISION)/cel.ratio_from DESC NULLS LAST
        "#,
        validated_querry_data.min_amount_from,
        validated_querry_data.min_amount_to,
        validated_querry_data.currency_code_from,
        validated_querry_data.currency_code_to,
    )
    .fetch(&pool);

    use futures::TryStreamExt;
    while let Some(row) = rows.try_next().await? {
        exchange_listings.push(row);
    }
    Ok(exchange_listings)
}

#[server(UserExchangeCurrencies, "/api")]
pub async fn use_exchange_listing(
    listing_id: i64,
    amount_exchange_to: i64,
    amount_exchange_from: i64,
) -> Result<(), ServerFnError> {
    match get_user().await {
        Ok(Some(user)) => {
            leptos_axum::redirect("/currency_exchange");
            use_exchange_listing_fn(
                listing_id,
                user.id,
                amount_exchange_to,
                amount_exchange_from,
            )
            .await
        }
        _ => Err(ServerFnError::ServerError(
            "Can't get user to exchange_currencies.".to_string(),
        )),
    }
}
