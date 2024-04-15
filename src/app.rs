use crate::auth::*;
use crate::components::navbar::NavBar;
use crate::components::require_login::RequireLoginWithRedirect;
use crate::pages::fallback_page::FallbackPage;
use crate::pages::homepage::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::signup_page::SignupPage;
use crate::server::account::ChangePassword;
use crate::server::currency_exchange::CreateExchangeListing;
use crate::server::currency_exchange::DeleteExchangeListing;
use crate::server::currency_exchange::UserExchangeCurrencies;
use crate::server::transaction_orders::DeleteTransactionOrder;
use crate::server::transaction_orders::NewUserTransactionOrder;
use crate::server::transactions::{NewUserTransaction, WithdrawOrder};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::transactions::TransactionsPage;
use crate::pages::transactions::{NewTransactionPopUp, WithrawOrderPopUp};

use crate::pages::currency_exchange::CreateExchangeListingPopUp;
use crate::pages::currency_exchange::CurrencyExchangePage;
use crate::pages::currency_exchange::DeleteExchangeListingPopUp;
use crate::pages::currency_exchange::InspectExchangeListingPopUp;

use crate::pages::transaction_orders::DeleteTransactionOrderPopup;
use crate::pages::transaction_orders::NewTransactionOrderPopup;
use crate::pages::transaction_orders::TransactionOrdersPage;

use crate::pages::account::AccountPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // User actions
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();
    let change_password = create_server_action::<ChangePassword>();

    use crate::utils::UserContextType;
    let user: UserContextType = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
                change_password.version().get(),
            )
        },
        move |_| get_user(),
    );
    provide_context(user);

    // Transactions actions
    let new_transaction_action = create_server_action::<NewUserTransaction>();
    let withdraw_order_action = create_server_action::<WithdrawOrder>();

    // Transactions and Exchange orders action
    let new_exchange_order_action = create_server_action::<CreateExchangeListing>();

    let transactions_reload = create_memo(move |_| {
        (
            new_transaction_action.version().get(),
            withdraw_order_action.version().get(),
            new_exchange_order_action.version().get(),
        )
    });
    use crate::utils::TransactionsReload;
    provide_context(TransactionsReload(transactions_reload));

    // Exchange orders actions
    let exchange_currecy_action = create_server_action::<UserExchangeCurrencies>();
    let delete_exchange_order_action = create_server_action::<DeleteExchangeListing>();

    let exchange_offers_reload = create_memo(move |_| {
        (
            new_exchange_order_action.version().get(),
            exchange_currecy_action.version().get(),
            delete_exchange_order_action.version().get(),
        )
    });
    use crate::utils::ExchangeOffersReload;
    provide_context(ExchangeOffersReload(exchange_offers_reload));

    // Transaction orders actions
    let new_transaction_order_action = create_server_action::<NewUserTransactionOrder>();
    let delete_transaction_order_action = create_server_action::<DeleteTransactionOrder>();

    let transaction_orders_reload = create_memo(move |_| {
        (
            new_transaction_order_action.version().get(),
            delete_transaction_order_action.version().get(),
        )
    });
    use crate::utils::TransactionOrderssReload;
    provide_context(TransactionOrderssReload(transaction_orders_reload));

    view! {
        <Stylesheet id="leptos" href="/pkg/axum-bank.css"/>
        <Stylesheet id="fontawsome" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css" />
        <Title text="Axum bank"/>
        <Router fallback=|| {
            view! { <FallbackPage />}.into_view()
        }>
            <NavBar />
            <Routes>
                    <Route path="" view=|| view! {<HomePage/> }/>

                    <Route path="/transactions/" view=|| view! {
                        <RequireLoginWithRedirect>
                            <Outlet />
                            <TransactionsPage />
                        </RequireLoginWithRedirect>
                    }>
                        <Route path="/" view=|| view! {} />
                        <Route path="new_transaction/" view=move || view! { <NewTransactionPopUp new_transaction_action=new_transaction_action /> } />
                        <Route path="withdraw/" view=move || view! { <WithrawOrderPopUp withdraw_order_action=withdraw_order_action /> } />
                    </Route>

                    <Route path="/currency_exchange/" view=|| view! {
                        <RequireLoginWithRedirect>
                            <Outlet />
                            <CurrencyExchangePage />
                        </RequireLoginWithRedirect>
                    }>
                        <Route path="/" view=|| view! {} />
                        <Route path="new_exchange_order/" view=move || view! { <CreateExchangeListingPopUp new_exchange_order_action=new_exchange_order_action /> } />
                        <Route path="exchange/:id" view=move || view! { <InspectExchangeListingPopUp exchange_action=exchange_currecy_action />  } />
                        <Route path="delete/:id" view=move || view! { <DeleteExchangeListingPopUp delete_action=delete_exchange_order_action /> } />
                    </Route>

                    <Route path="/transaction_orders/" view=|| view! {
                        <RequireLoginWithRedirect>
                            <Outlet />
                            <TransactionOrdersPage />
                        </RequireLoginWithRedirect>
                    }>
                        <Route path="/" view=|| view! {} />
                        <Route path="new_transaction_order/" view=move || view! { <NewTransactionOrderPopup new_order_action=new_transaction_order_action /> } />
                        <Route path="delete/:id" view=move || view! { <DeleteTransactionOrderPopup delete_action=delete_transaction_order_action /> } />
                    </Route>

                    <Route path="account/" view=move || view! {
                        <RequireLoginWithRedirect>
                            <AccountPage change_password_action=change_password />
                        </RequireLoginWithRedirect>
                    }/>

                    <Route path="signup/" view=move || view! {
                        <SignupPage action=signup/>
                    }/>
                    <Route path="login/" view=move || view! {
                        <LoginPage action=login />
                    }/>
                </Routes>
        </Router>
    }
}
