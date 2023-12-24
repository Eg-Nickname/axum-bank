use leptos::*;
use leptos_router::*;

// use leptos_router::*;
#[component]
pub fn NewTransactionPopUp() -> impl IntoView{
    view! {
        <div class="popup active" id="popup-1">
        <A href="/transactions"><div class="overlay"></div></A>
        
        <div class="content-popup">
            <form method="post">
            <h1>"Nowy Przelew"</h1>
            // TODO add new transaction form
            // {% for field in transaction_form %} 
            //     <div class="input-box">
            //         <i>{{field.label}}:</i>
            //         <br>
            //         <br>
            //         {{field}}
            //     </div>

            //     {% for error in transaction_form.field.errors %}
            //         <p class="login-error">{{error}}</p>
            //     {% endfor %}
            // {% endfor %}


            //     {% if transaction_form.non_field_errors %}
            //         <div class="">
            //             <a>{{transaction_form.non_field_errors}}</a>
            //         </div>
            //     {% endif %}
            <input type="submit" class="buton-przelew" value="Wyslij przelew" name="transaction" />
            </form>

            // <ActionForm action=action>
            // <h1>"Nowy Przelew"</h1>
            // // TYTUŁ PRZELEWU
            // // NAZWA ODBIORCY
            // // KWOTA
            // // WALUTA
            // <div class="input-box">
            //     <i>{{field.label}}:</i>
            //     <br/>
            //     <br/>
            //     {{field}}
            // </div>


            //     <input type="submit" class="buton-przelew" value="Wyslij przelew" name="transaction" />
            // </ActionForm>

            <A href="/transactions"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
        </div>
    </div>
    }
} 

#[component]
pub fn WithrawOrderPopUp() -> impl IntoView{
    view! {
        <div class="popup active" id="popup-2">
            <A href="/transactions"><div class="overlay"></div></A>
            <div class="content-popup">

            <form method="post">
            <h1>Wypłać Pieniądze</h1>
        //     <!-- Form -->
        //     {% for field in withdraw_form %}
                    
        //     <div class="input-box">
        //         <i>{{field.label}}:</i>
        //         <br>
        //         <br>
        //         {{field}}
        //         {% if field.help_text %}
        //             <div class="help-text">{{field.help_text}}</div>
        //         {% endif %}
        //     </div>

        //     {% for error in withdraw_form.field.errors %}
        //         <p class="login-error">{{error}}</p>
        //     {% endfor %}
        // {% endfor %}


        //     {% if withdraw_form.non_field_errors %}
        //         <div class="">
        //             <a>{{withdraw_form.non_field_errors}}</a>
        //         </div>
        //     {% endif %}

            <input type="submit" class="buton-przelew" value="Wyplac pieniądze" name="withdraw" />
            </form>

            <A href="/transactions"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
            </div>
        </div>
    }
}

#[component]
fn AccountBalance() -> impl IntoView{
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
                    // <button><i class="fas fa-plus"></i>"Nowy Przelew"</button>
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
    let (filter_status, set_filter_status) = create_signal(false);
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
    
            // TODO ADD querrying of transactions from db
//                 {% for transaction in sent_transactions.qs %}
                // TODO ADD LINK WITH MORE INFO
                <div class="more-info"><a href="% transaction.transaction_id %">"Więcej Informacji"</a></div>


//                 {% if transaction.typ == 0 %}
//                     {% if transaction.sender_id is not None %}
//                         <div class="sender">{{transaction.sender_id}}</div>
//                     {% else %}
//                         <div class="sender">Użytkownik usunięty</div>
//                     {% endif %}
            

//                     {% if transaction.reciver_id is not None %}
//                         <div class="reciver">{{transaction.reciver_id}}</div>
//                     {% else %}
//                         <div class="reciver">Użytkownik usunięty</div>
//                     {% endif %}
//                 {% elif transaction.typ == 1 %}
//                     <div class="sender">Wpłata na konto</div><div class="reciver"></div>
//                 {% elif transaction.typ == 2 %}
//                     <div class="sender">Wypłata z konta </bold></div><div class="reciver"></div>
//                 {% elif transaction.typ == 3 %}
//                     <div class="sender">Wymiana Walut</div><div class="reciver"></div>         
//                 {% endif %}
        

//                 {% if transaction.sender_id == request.user %}
//                     <div><a style="color: red;">-{{transaction.kwota}}</a></div>
//                 {% else %}
//                     <div><a style="color: rgb(0, 173, 0);">+{{transaction.kwota}}</a></div>
//                 {% endif %}


//                 <div class="currency"><a>{{transaction.waluta}}</a></div>
    

//                 {% if transaction.typ == 0 or 2 %}
//                     <div class="title"><a>{{transaction.title}}</a></div>
//                 {% else %}
//                     <div></div>
//                 {% endif %}
//                     <div class="date"><a>{{transaction.data_transakcji}}</a></div>
//                 {% endfor %}

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