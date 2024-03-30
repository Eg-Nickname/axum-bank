use crate::server::transaction_orders::{DeleteTransactionOrder, NewUserTransactionOrder};
use leptos::*;
use leptos_router::*;

#[component]
pub fn NewTransactionOrderPopup(
    new_order_action: Action<NewUserTransactionOrder, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <div class="popup active">
        <A href="/transaction_orders"><div class="overlay"></div></A>

        <div class="content-popup">
            <ActionForm action=new_order_action class="new-transaction-form">
                <h1>"Nowe zlecenie transakcji"</h1>
                <div class="transacition-input-box">
                    <i>"Tytuł zlecenia:"</i>
                    <input type="text" placeholder="Tytuł" name="title" class="text-input" />
                </div>
                <div class="transacition-input-box">
                    // TODO Add username suggestion while inputing
                    <i>"Nazwa odbiorcy:"</i>
                    <input type="text" placeholder="Nazwa odbiorcy" maxlength="32" name="reciver_username" class="text-input" />
                </div>
                <div class="transacition-input-box">
                    // TODO change to custom select implementation
                    <i>"Waluta"</i>
                    <input type="text" placeholder="cny" maxlength="32" name="currency_code" class="text-input" />
                </div>
                <div class="transacition-input-box">
                    <i>"Kwota"</i>
                    <input type="number" placeholder="0" name="amount" class="text-input" />
                </div>

                <div class="transacition-input-box">
                    <i>"Cykliczność zlecenia (dni)"</i>
                    <input type="number" placeholder="0" name="delay_days" class="text-input" />
                </div>

                <div class="transacition-input-box">
                    <input type="submit" class="solid-purple-button" value="Wyslij przelew" name="transaction" />
                </div>

            </ActionForm>
            <A href="/transaction_orders"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
        </div>
    </div>
    }
}

#[derive(Params, PartialEq, Clone)]
struct ListingParams {
    id: i64,
}

#[component]
pub fn DeleteTransactionOrderPopup() -> impl IntoView {
    let params = use_params::<ListingParams>();
    let listing_id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or(-1));
    let action = create_server_action::<DeleteTransactionOrder>();

    view! {
        <div class="popup active">
        <A href="/transaction_orders"><div class="overlay"></div></A>


        <div class="content-popup">
        <ActionForm action=action>
            <h1>"Czy chcesz usunąć zlecenie transakcji?"</h1>
            <div class="delete_exchange_listing_form">
                <input type="hidden" value={listing_id} name="trans_order_id" />
                <input type="submit" value="Usuń" class="button-del link-button"/>
                <A href="/transaction_orders" class="solid-purple-button">"Rezygnuje"</A>
            </div>
        </ActionForm>

        <A href="/transaction_orders"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
        </div>
        </div>
    }
}

#[component]
fn TransactionOrders() -> impl IntoView {
    view! {}
}

#[component]
pub fn TransactionOrdersPage() -> impl IntoView {
    view! {
        <div class="orders-container">
            <h2>"Zlecenia transakcji"</h2>
            <div class="flexBlock">
                <A class="solid-purple-button" href="new_transaction_order"><i class="fas fa-plus"></i>"Dodaj zlecenie"</A>
            </div>
            <div class="orders">
                <TransactionOrders />
            </div>
        </div>
    }
}
