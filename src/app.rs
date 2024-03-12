use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::*;
use crate::server::transactions::{NewUserTransaction, WithdrawOrder};
use crate::server::currency_exchange::CreateExchangeListing;
use crate::components::navbar::NavBar;
use crate::components::require_login::RequireLoginWithRedirect;
use crate::pages::homepage::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::signup_page::SignupPage;
use crate::pages::fallback_page::FallbackPage;

use crate::pages::transactions::TransactionsPage;
use crate::pages::transactions::{NewTransactionPopUp, WithrawOrderPopUp};

use crate::pages::currency_exchange::CurrencyExchangePage;
use crate::pages::currency_exchange::CreateExchangeListingPopUp;
use crate::pages::currency_exchange::DeleteExchangeListingPopUp; 
use crate::pages::currency_exchange::InspectExchangeListingPopUp;



#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Actions
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();

    let new_transaction_action = create_server_action::<NewUserTransaction>();
    let withdraw_order_action = create_server_action::<WithdrawOrder>();

    let new_exchange_order_action = create_server_action::<CreateExchangeListing>();


    // Resources
    let user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>> = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(),
    );
    provide_context(user);

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
                    <Route path="podstrona" view=|| view! {"test of router" }/>

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
                        <Route path="exchange/:id" view=move || view! { <InspectExchangeListingPopUp />  } />
                        <Route path="delete/:id" view=move || view! { <DeleteExchangeListingPopUp /> } />
                    </Route>


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
