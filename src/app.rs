use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::*;
use crate::components::navbar::NavBar;
use crate::components::require_login::RequireLoginWithRedirect;
use crate::pages::homepage::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::signup_page::SignupPage;
use crate::pages::fallback_page::FallbackPage;
use crate::pages::transactions::TransactionsPage;

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let signup = create_server_action::<Signup>();

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

    // Provide user resource to components
    provide_context(user);

    provide_meta_context();

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

                    <Route path="/transactions" view=|| view! { <RequireLoginWithRedirect><Outlet /></RequireLoginWithRedirect> }>
                        <Route path="/" view=|| view! {<TransactionsPage/>} />
                        <Route path="new_transaction/" view=|| view! {"nowa tranzakcyja" <TransactionsPage/> } />
                        <Route path="withdraw/" view=|| view! {"wyplacamy kasiure" <TransactionsPage/> } />
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
