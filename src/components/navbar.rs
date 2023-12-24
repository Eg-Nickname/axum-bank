// use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;
// use leptos_router::A;

use crate::auth::User;

#[component]
pub fn NavBar() -> impl IntoView {
    let user = use_context::<Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>>().expect("User resource shoud have been provided.");
    view! {
        <nav class="menu">
            <div class="flexBlock ul-wrapper">
                <div class="hamburger">
                    <A href="/"><img src="main/images/logo_bank.png" alt="My image"/></A>
                    <div class="burger-bars">
                        <div class="bar1"></div>
                        <div class="bar2"></div>
                        <div class="bar3"></div>
                    </div>
                </div>

                <ul class="flexBlock" id="top-ul">
                    <li class="logo"><A href="/"><img src="/images/logo_bank.png" alt="My image"/></A></li>
                    <li class="li-nav"><A class="link" href="/">"Strona główna"</A></li>
                    <li class="li-nav"><A class="link" href="/transactions/">"Saldo"</A></li>
                    <li class="li-nav"><A class="link" href="/currency_exchange/">"Wymiana walut"</A></li>
                    <li class="li-nav"><A class="link" href="/transaction_orders/">"Zlecenia transakcji"</A></li>
                </ul>

                <ul class="flexBlock logowanie" id="bottom-ul">
                    <Transition
                        fallback=move || view! {<span>"Loading..."</span>}
                    >
                        {move || {
                            user.get().map(|user| match user {
                                Ok(Some(_user)) => view! {
                                    <li class="login"><A href="/account/">"Ustawienia"</A></li>
                                    <li class="login li-responsive"><a rel="external" href="/api/logout">"Wyloguj się"</a></li>
                                    // TODO ADD ADMIN DASHBOARD LINK
                                    // {% if user.is_superuser %}
                                    // <li class="li-nav"><A class="link" href="/admindashboard/">"Admin Panel"</A></li>
                                    // {% endif %}
                                }.into_view(),
                                _ => view! {
                                    warn!("mogus")
                                    <li>
                                        <li class="login"><A href="/login/">"Zaloguj się"</A></li>
                                        <li class="login"><A href="/signup/">"Zarejestruj się"</A></li>
                                    </li>
                                }.into_view(),
                            })
                        }}
                    </Transition>
                </ul>
            </div>
        </nav>
    }
}