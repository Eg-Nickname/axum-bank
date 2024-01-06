use leptos::*;
use leptos_router::*;

use crate::server::transactions::{NewUserTransaction, WithdrawOrder};
#[component]
pub fn NewTransactionPopUp(new_transaction_action: Action<NewUserTransaction, Result<(), ServerFnError>>) -> impl IntoView{
    view! {
        <div class="popup active">
        <A href="/transactions"><div class="overlay"></div></A>
        
        <div class="content-popup">
            <ActionForm action=new_transaction_action class="new-transaction-form">
                <h1>"Nowy Przelew"</h1>
                <div class="input-box1">
                    <i>"Tytuł przelewu:"</i>
                    <input type="text" placeholder="Tytuł" name="title" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    // TODO Add username suggestion while inputing
                    <i>"Nazwa odbiorcy:"</i>
                    <input type="text" placeholder="Reciver username" maxlength="32" name="reciver_username" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    // TODO change to custom select implementation
                    <i>"Waluta"</i>
                    <input type="text" placeholder="cny" maxlength="32" name="currency_code" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    <i>"Kwota"</i>
                    <input type="number" placeholder="0" name="amount" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    <input type="submit" class="buton-przelew" value="Wyslij przelew" name="transaction" />
                </div>

            </ActionForm>
            <A href="/transactions"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
        </div>
    </div>
    }
} 

#[component]
pub fn WithrawOrderPopUp(withdraw_order_action: Action<WithdrawOrder, Result<(), ServerFnError>>) -> impl IntoView{
    view! {
        <div class="popup active">
            <A href="/transactions"><div class="overlay"></div></A>
            <div class="content-popup">

                <ActionForm action=withdraw_order_action class="new-transaction-form">
                    <h1>"Wypłać pieniądze"</h1>
                    <div class="input-box1">
                        <i>"Nick z serwera:"</i>
                        <input type="text" placeholder="Nick z serwera" name="withdrawer_nick" class="transaction-input" />
                    </div>
                    <div class="transacition-input-box">
                        // TODO change to custom select implementation
                        <i>"Waluta"</i>
                        <input type="text" placeholder="cny" maxlength="32" name="currency_code" class="transaction-input" />
                    </div>
                    <div class="transacition-input-box">
                        <i>"Kwota"</i>
                        <input type="number" placeholder="0" name="amount" class="transaction-input" />
                    </div>
                    <div class="transacition-input-box">
                        <input type="submit" class="buton-przelew" value="Wypłać pieniądze" name="transaction" />
                    </div>

                </ActionForm>
                <A href="/transactions"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
            </div>
        </div>
    }
}

#[component]
fn AccountBalance() -> impl IntoView{
    // TODO UPDATE BALANCE AFTER CREATING NEW TRANSACTION
    let balances = create_resource(|| (), move |_| {
        use crate::server::transactions::get_user_balances;
        get_user_balances()
    });

    view! {
        <div class="info-container">
            <div class="informacje">
                <h2>"Stan konta"</h2> 
                <div class="flexBlock">
                    <A class="link-button action-button" href="new_transaction"><i class="fas fa-plus"></i>"Nowy Przelew"</A>
                </div>
                <div class="flexBlock buttongroup">
                    <A class="link-button action-button" href="withdraw"><i class="fas fa-money-bill-wave"></i>"Wypłata z konta"</A>
                </div>
                <div class="currency-wrapper">
                <Transition fallback=move || view! {<p>"Loading..."</p> }>
                    {move || {
                        let balances_view = {move || {
                            balances.get().map(move |res| match res {
                                Ok(balances_list) => {
                                    balances_list.into_iter().map(move |balance|{
                                        view!{ 
                                            <div class="info-inner-container">
                                                <div class="currency-flag"><img src={"/images/flags/".to_owned() + &balance.currency_code + ".svg" } class="country-flag" /></div>
                                                <div class="currency-ammount">{balance.currency_name} ": " {balance.balance}</div>
                                            </div>
                                        }.into_view()
                                    }).collect_view()
                                },
                                Err(_) => view! { 
                                    <div></div> 
                                }.into_view(),
                            }).unwrap_or_default()
                        }};
                        view! {
                            {balances_view}
                        }
                        }
                    }
                </Transition>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Transactions() -> impl IntoView{
    use crate::server::transactions::TransactionType;
    // use crate::server::transactions::TransactionStatus;

    let (filter_status, set_filter_status) = create_signal(false);
    let balances = create_resource(|| (), move |_| {
        use crate::server::transactions::get_user_transactions;
        get_user_transactions()
    });

    view!{
        <div class="transactions-container">
        <h1>"Transakcje"</h1>
        <div>          
            <h3>"Opcje filtrowania"</h3> 
            <label class="switch">
                <input type="checkbox" on:click=move |_| {set_filter_status(!filter_status());} checked=false />
                <span class="slider round"></span>
            </label>

            {move || { if filter_status(){
                view!{
                    <div class="filter-options filter-active flexBlock" id="filter-options">
                    // TODO add querry filter and embed it into url
                    <form method="get">
                        // {% for field in transactions_filter.form %}
                        //     <div class="input-box filter-box">
                        //         <i>{{field.label}}:</i>
                        //         <br>
                        //         {{field}}
                        //     </div>
                        
                        //     {% for error in transaction_form.field.errors %}
                        //         <p class="login-error">{{error}}</p>
                        //     {% endfor %}
                        // {% endfor %}
                        <button type="submit" class="buton-przelew nowy-przelew">"Filtruj"</button>
                    </form>
                </div>
                }
            }else{
                view! { <div></div>}
            }}}

        </div>

        <div class="transactions-wrapper">
            <div class="collumn-name" id="more-info">"Informacje"</div>
    
            <div class="collumn-name" id="sender">"Nadawca"</div>

            <div class="collumn-name" id="reciver">"Odbiorca"</div>
    
            <div class="collumn-name" id="ammount">"Kwota"</div>
    
            <div class="collumn-name" id="currency">"Waluta"</div>

            <div class="collumn-name" id="title">"Tytuł przelewu"</div>
    
            <div class="collumn-name" id="date">"Data"</div>
    
            // TODO ADD LINK WITH MORE INFO
            <div class="more-info"><a href="% transaction.transaction_id %">"Więcej Informacji"</a></div>

            // TODO FIX TRANSACTION DISPLAY STYLE``
            <Transition fallback=move || view! {<p>"Loading..."</p> }>
            {move || {
                let transactions_view = {move || {
                    balances.get().map(move |res| match res {
                        Ok(transactions_list) => {
                            transactions_list.into_iter().map(move |transaction|{
                                let trans_type = transaction.transaction_type.clone();
                                view!{ 
                                    // {transaction.id}
                                    {move ||{
                                        match transaction.transaction_type.clone(){
                                            TransactionType::Transfer | TransactionType::CurrencyExchange => view! {
                                                <div class="sender">{transaction.sender_username.clone().unwrap_or("Użytkownik usunięty".to_string())}</div>
                                                <div class="reciver">{transaction.reciver_username.clone().unwrap_or("Użytkownik usunięty".to_string())}</div>
                                            }.into_view(),
                                            TransactionType::Withdraw | TransactionType::Payment | TransactionType::Other => view! {<div class="sender"></div><div class="reciver"></div>}.into_view(),
                                        }
                                    }}

                                    // TODO Check if user is sender or reciever of transaction
                                    <div><a style="color: red;">"-"{transaction.amount}</a></div>
                                    // <div><a style="color: rgb(0, 173, 0);">"+"{transaction.amount}</a></div>
                                    
                                    <div class="currency"><a>{transaction.currency_code}</a></div>

                                    {move ||{
                                        match trans_type{
                                            TransactionType::Transfer | TransactionType::Other => view! { <div class="title"><a>{transaction.title.clone()}</a></div> }.into_view(),
                                            TransactionType::CurrencyExchange => view! { <div class="title"><a>"Wymiana walut"</a></div>}.into_view(),
                                            TransactionType::Withdraw => view! { <div class="title"><a>"Wypłata z konta"</a></div>}.into_view(),
                                            TransactionType::Payment => view! { <div class="title"><a>"Wpłata na  konto"</a></div>}.into_view(),
                                        }
                                    }}

                                    <div class="date"><a>"19-10-2023 19:20:01"</a></div>
                                }.into_view()
                            }).collect_view()
                        },
                        Err(_) => view! { 
                            <div></div> 
                        }.into_view(),
                    }).unwrap_or_default()
                }};
                view! {
                    {transactions_view}
                }
                }
            }
            </Transition>
        </div>
    </div>
    }
}

#[component]
pub fn TransactionsPage() -> impl IntoView {
    view! {
        <div class="content">
            <AccountBalance />
            
            <Transactions />
        </div>
    }
}