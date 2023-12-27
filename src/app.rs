use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::*;
use crate::server::transactions::NewUserTransaction;
use crate::components::navbar::NavBar;
use crate::components::require_login::RequireLoginWithRedirect;
use crate::pages::homepage::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::signup_page::SignupPage;
use crate::pages::fallback_page::FallbackPage;
use crate::pages::transactions::TransactionsPage;

use crate::pages::transactions::{NewTransactionPopUp, WithrawOrderPopUp};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Actions
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();

    let new_transaction_action = create_server_action::<NewUserTransaction>();

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
            view! {<FallbackPage />}.into_view()
        }>
            <NavBar />
            <div class="animation">
                <Routes>
                    <Route path="" view=|| view! {<HomePage/> }/>

                    <Route path="/transactions" view=|| view! { 
                        <RequireLoginWithRedirect>
                            <Outlet />
                            <TransactionsPage/>
                        </RequireLoginWithRedirect> 
                    }>
                        <Route path="/" view=|| view! {} />
                        <Route path="new_transaction/" view=move || view! { <NewTransactionPopUp new_transaction_action=new_transaction_action /> } />
                        <Route path="withdraw/" view=|| view! { <WithrawOrderPopUp /> } />
                    </Route>

                    <Route path="signup" view=move || view! {
                        <SignupPage action=signup/>
                    }/>
                    <Route path="login" view=move || view! {
                        <LoginPage action=login />
                    }/>
                </Routes>
            </div>
        </Router>
    }
}
