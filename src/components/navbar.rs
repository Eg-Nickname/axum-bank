// use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;

use crate::auth::User;

#[component]
pub fn NavBar() -> impl IntoView {
    let user = use_context::<Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>>().expect("User resource shoud have been provided.");
    view! {
        <nav>
            <ul class="nav-list">
                <li><A href="/" class="nav-link">
                    <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 576 512"><path d="M575.8 255.5c0 18-15 32.1-32 32.1h-32l.7 160.2c0 2.7-.2 5.4-.5 8.1V472c0 22.1-17.9 40-40 40H456c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1H416 392c-22.1 0-40-17.9-40-40V448 384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"/></svg>
                    "Strona główna"
                </A></li>

                <li><A href="/" class="nav-link">
                    "Placeholder"
                </A></li>

                <li><A href="/" class="nav-link">
                    "Placeholder"
                </A></li>

                <li><A href="/" class="nav-link">
                    "Placeholder"
                </A></li>

                <Transition
                    fallback=move || view! {<span>"Loading..."</span>}
                >
                    {move || {
                        user.get().map(|user| match user {
                            Err(_e) => view! {
                                <li>
                                    <A href="/login" class="login-link">"Zaloguj się"</A>
                                    <A href="/signup" class="signup-link">"Zarejestruj się"</A>
                                </li>
                            }.into_view(),
                            Ok(None) => view! {
                                <li>
                                    <A href="/login" class="login-link">"Zaloguj się"</A>

                                    <A href="/signup" class="signup-link">"Zarejestruj się"</A>
                                </li>
                            }.into_view(),

                            // User logged in view
                            Ok(Some(user)) => view! {
                                <li class="account">
                                    <svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 448 512"><path d="M224 256A128 128 0 1 0 224 0a128 128 0 1 0 0 256zm-45.7 48C79.8 304 0 383.8 0 482.3C0 498.7 13.3 512 29.7 512H418.3c16.4 0 29.7-13.3 29.7-29.7C448 383.8 368.2 304 269.7 304H178.3z"/></svg>
                                    // <A href="/settings">"Ustawienia"</A>
                                    <span>{format!("{}", user.username)}</span>
                                    <svg class="chevron" xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 512 512"><path d="M233.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L256 338.7 86.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"/></svg>
                                    <div class="settings-dropdown">
                                        <ul>
                                            <li>"Panel użytkownika"</li>
                                            <li>"Ustawienia"</li>
                                            <li>"Inne"</li>
                                            <li>
                                                <a rel="external" href="/api/logout">"Wyloguj się"</a>
                                            </li>
                                        </ul>
                                        <div class="settings-dropdown-background"></div>
                                    </div>
                                </li>
                            }.into_view()
                        })
                    }}
                </Transition>
            </ul>
        </nav>
    }
}