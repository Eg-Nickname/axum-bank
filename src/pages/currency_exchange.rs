use leptos::*;
use leptos_router::*;
use crate::server::currency_exchange::{CreateExchangeListing, DeleteExchangeListing};
use crate::auth::User;



#[component]
fn InspectExchangeListing() -> impl IntoView{
    view! {
    // {%if selected_exchange_listing %}

    // <div class="popup active" id="popup-2">
    //     <div class="overlay" onclick="togglePopup1()">

    //     </div>
    //     <div class="content-popup">
    //     <!-- <h4>{{selected_exchange_listing.id}}</h4> -->
    //     <form method="post">{% csrf_token %}

    //         {% for field in exchange_money_form %}
                
    //         <div class="input-box">
    //             <i>{{field.label}}:</i>
    //             <br>
    //             <br>
    //             {{field}}
    //         </div>
        
    //         {% for error in exchange_money_form.field.errors %}
    //             <p class="login-error">{{error}}</p>
    //         {% endfor %}
    //         {% endfor %}            
    //     <input type="submit" value="Wymień Pieniądze" name="exchange-money">
    //     </form>
    //     <script>
    //         document.getElementById("id_amount_exchanged").step = Number({{selected_exchange_listing.ratio_to}})
    //         document.getElementById("id_amount_exchanged").max = Number({{selected_exchange_listing.amount_wanted}}) 
    //         document.getElementById("id_amount_exchanged").placeholder = "Kwota którą chcesz wymienić w " + "{{selected_exchange_listing.exchange_to}}"
    //         document.getElementById("id_amount_recived").placeholder = "Kwota którą dostaniesz w " + "{{selected_exchange_listing.exchange_from}}"
    //     </script>

    //     <script>
    //         document.getElementById("id_amount_exchanged").oninput = () => {
    //         document.getElementById("id_amount_recived").value = ((document.getElementById("id_amount_exchanged").value)/{{selected_exchange_listing.ratio_to}}) * {{selected_exchange_listing.ratio_from}}
    //         }
    //     </script>

    //     <div class="close-bnt" onclick="togglePopup1()">&times;</div>
    //     </div>
    // </div>
    // {%endif%}
    }
}

#[derive(Params, PartialEq, Clone)]
struct ListingParams {
    id: i64
}

#[component]
pub fn DeleteExchangeListingPopUp() -> impl IntoView{
    let params = use_params::<ListingParams>();
    let listing_id = move || {
        params.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .unwrap_or(-1)
        })
    };
    let action = create_server_action::<DeleteExchangeListing>();

    view! {
        <div class="popup active">
        <A href="/currency_exchange"><div class="overlay"></div></A>

        <div class="content-popup">
        <ActionForm action=action>
            <h1>"Czy chcesz usunąć oferte wymiany?"</h1>
            <div class="delete_exchange_listing_form">
                <input type="hidden" value={listing_id} name="listing_id" />
                <input type="submit" value="Usuń" class="button-del link-button"/>
                <A href="/currency_exchange" class="link-button">"Rezygnuje"</A>
            </div> 
        </ActionForm>

        <A href="/currency_exchange"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
        </div>
        </div>
    }
}

#[component]
pub fn CreateExchangeListingPopUp(new_exchange_order_action:  Action<CreateExchangeListing, Result<(), ServerFnError>>) -> impl IntoView{
    view!{
        <div class="popup active">
        <A href="/currency_exchange"><div class="overlay"></div></A>
        
            
        <div class="content-popup">
            <ActionForm action=new_exchange_order_action>
                <h1>"Wymień waluty"</h1>

                // TODO fix transaction-input-box class name
                // TODO update to select like component
                <div class="transacition-input-box">
                    <i>"Waluta Z"</i>
                    <input type="text" placeholder="CNY" name="currency_code_from" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    <i>"Kwota Z"</i>
                    <input type="number" placeholder="0" name="amount_from" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    <i>"Waluta Na"</i>
                    <input type="text" placeholder="RUB" name="currency_code_to" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    <i>"Kwota Na"</i>
                    <input type="number" placeholder="0" name="amount_to" class="transaction-input" />
                </div>

                <div class="transacition-input-box">
                    <input type="submit" class="buton-przelew" value="Wymień" name="transaction" />
                </div>
            </ActionForm>
            
            <A href="/currency_exchange"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
        </div>
        </div>
    }
}

#[component]
pub fn CurrencyExchangePage() -> impl IntoView{
    view!{
        <div class="content">
            <ExchangeListings />
            //     <div class="exchange-info-container">
            //         <h2>Wymień waluty</h2> <button onclick="togglePopup()" class="buton-przelew button-oferawymiany"><i class="fas fa-plus"></i>Nowa Oferta Wymiany Walut</button>

            //         <form method="get">
            //             {% for field in exchangelisting_filter.form %}
                        
            //             <div class="input-box filter-box">
            //                 <i>{{field.label}}:</i>
            //                 <br>
            //                 {{field}}
            //             </div>
                    
            //             {% for error in transaction_form.field.errors %}
            //                 <p class="login-error">{{error}}</p>
            //             {% endfor %}
            //             {% endfor %}
            //             <button type="submit" class="buton-przelew nowy-przelew">Filtruj</button>
            //         </form>
            //     </div>
            // </div>
        </div>
    }
}

#[component]
fn ExchangeListings() -> impl IntoView{
    let user = use_context::<Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>>().expect("User resource shoud have been provided.");

    let exchange_listings = create_resource(|| (), move |_| {
        use crate::server::currency_exchange::get_exchange_listings;
        get_exchange_listings()
    });

    view! {
        <Transition fallback=move || view! {<p>"Loading..."</p> }>
        {move || {
            let exchange_view = {move || {
                let current_user_id: Option<i64> = user.get().map(|user| match user {
                    Ok(Some(user)) => user.id,
                    _ => -1 ,
                });

                exchange_listings.get().map(move |res| match res {
                    Ok(exchange_listings_list) => {
                        exchange_listings_list.into_iter().map(move |exchange_listing|{
                            view!{ 
                                <div class="exchange-listing-container">
                                    <h2>Wymień</h2>
                                    <h3>{exchange_listing.amount_from} {exchange_listing.currency_from_code.clone()} Na {exchange_listing.amount_to} {exchange_listing.currency_to_code.clone()}</h3>
                                    <h3>"Ratio:"</h3>
                                    <h4>{exchange_listing.ratio_from} {exchange_listing.currency_from_code} : {exchange_listing.ratio_to} {{exchange_listing.currency_to_code}}</h4>
                                    <h4 style="color: gray">"Oferta od: " {exchange_listing.listing_creator_username} </h4>

                                    {move ||{
                                        if Some(exchange_listing.listing_creator_id) == current_user_id{
                                            view! {<A href="./delete/".to_string() + &exchange_listing.id.to_string()  class="link-button button-del">"Usuń ofertę wymiany"</A>}
                                        }else{
                                            view! {<A href="./exchange/".to_string() + &exchange_listing.id.to_string() class="link-button">"Wymień"</A>}
                                        }
                                    }}
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
                {exchange_view}
            }
            }
        }
        </Transition>
    }
}