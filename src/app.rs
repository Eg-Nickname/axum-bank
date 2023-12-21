use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::*;
use crate::components::navbar::NavBar;
use crate::pages::homepage::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::signup_page::SignupPage;
use crate::pages::fallback_page::FallbackPage;

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
        <Title text="Axum exchange"/>
        <Router fallback=|| {
            view! {<FallbackPage />}.into_view()
        }>
        

            <main>
                <NavBar />
                <Routes>
                    <Route path="" view=|| view! {<HomePage/> }/>
                    <Route path="signup" view=move || view! {
                        <SignupPage action=signup/>
                    }/>
                    <Route path="login" view=move || view! {
                        <LoginPage action=login />
                    }/>
                </Routes>
            </main>
        </Router>
    }
}
