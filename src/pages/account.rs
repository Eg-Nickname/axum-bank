use leptos::*;
use leptos_router::*;

use crate::components::require_perm::RequirePerm;
use crate::components::select::*;

use crate::server::account::AddUserRequest;
use crate::server::account::ChangePassword;
use crate::server::account::DeleteApiToken;
use crate::server::account::GenerateApiToken;

#[component]
fn ChangePasswordForm(
    change_password_action: Action<ChangePassword, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action=change_password_action>
            <h1>"Zmień hasło"</h1>
            <div class="transacition-input-box">
                <i>"Stare hasło"</i>
                <input type="password" placeholder="Stare hasło" name="old_password" class="text-input" />
            </div>
            <div class="transacition-input-box">
                <i>"Nowe hasło"</i>
                <input type="password" placeholder="Nowe hasło" name="new_password" class="text-input" />
            </div>
            <div class="transacition-input-box">
                <i>"Potwierdź hasło"</i>
                <input type="password" placeholder="Powtórzone nowe hasło" name="new_password_repeat" class="text-input" />
            </div>
            <div class="transacition-input-box">
                <input type="submit" class="solid-purple-button" value="Zmień hasło" name="transaction" />
            </div>
        </ActionForm>
    }
}

#[component]
fn UserRequests() -> impl IntoView {
    let add_request = create_server_action::<AddUserRequest>();
    view! {
        // Form adding user request
        <ActionForm action=add_request>
            <h1>"Skontaktuj się w sprawie"</h1>

            <div class="transacition-input-box">
                <i>"Typ sprawy"</i>
                // <input type="text" placeholder="Feature request" name="request_type" class="text-input" />

                <SelectInput display_text="Wybierz typ" name="request_type">
                    <SelectOption option_text="Dodaj walute" option_value="AddCurrency" />
                    <SelectOption option_text="Dodaj token API" option_value="GetApiPerm" />
                    <SelectOption option_text="Inna" option_value="Diffrent" />
                </SelectInput>
            </div>

            <div class="transacition-input-box">
                <i>"Treść Wiadomości"</i>
            </div>

            <div class="transacition-input-box">
                <textarea name="request_message" class="textarea-input">
                    "Treść zawierająca dokładniejsze dane twojego zapytania"
                </textarea>
            </div>

            <div class="transacition-input-box">
                <input type="submit" class="solid-purple-button" value="Wyślij zapytanie" name="transaction" />
            </div>
        </ActionForm>
    }
}

#[component]
fn ApiInfo() -> impl IntoView {
    let generate_token_action = create_server_action::<GenerateApiToken>();
    let delete_token_action = create_server_action::<DeleteApiToken>();

    let api_token = create_resource(
        move || {
            (
                generate_token_action.version().get(),
                delete_token_action.version().get(),
            )
        },
        move |_| {
            use crate::server::account::get_api_token;
            get_api_token()
        },
    );

    let (token_visibility, set_token_visibility) = create_signal(false);

    view! {
        <Suspense fallback=move || view! {<p>"Loading..."</p> }>
        {move || {
            api_token.get().map(move |res| match res {
                    Ok(api_token) => {
                        if let Some(token) = api_token {
                            view! {
                                // Show token
                                <div>
                                    {if token_visibility() {
                                        view! { <input type="text" value={token} class="text-input" /> }
                                    }else{
                                        view! { <input type="password" value={token} class="text-input" /> }
                                    }}

                                    <label class="switch">
                                        <input type="checkbox" on:click=move |_| {set_token_visibility(!token_visibility());} checked=token_visibility() />
                                        <span class="slider round"></span>
                                    </label>
                                </div>

                                // Regenerate Token button
                                <div>
                                    <ActionForm action=generate_token_action>
                                        <i>"Nowy token API zasąpi stary. Nie będziesz mógł już używać starego"</i>
                                        <input type="submit" value="Wygeneruj nowy token" class="button-del" />
                                    </ActionForm>
                                </div>

                                // Delete button
                                <div>
                                    <ActionForm action=delete_token_action>
                                        <i>"Nie będziesz posiadał aktywnego tokenu API."</i>
                                        <input type="submit" value="Usuń obecny token" class="transparent-button-del" />
                                    </ActionForm>
                                </div>
                            }.into_view()
                        }else{
                            view! {
                                // Generate token
                                <div>
                                    <ActionForm action=generate_token_action>
                                        <input type="submit" value="Wygeneruj nowy token" class="solid-purple-button" />
                                    </ActionForm>
                                </div>
                            }.into_view()
                        }
                    },
                    Err(_) => view! {
                        <div>"Niestety wysąpił problem podczas próby odczytania twojego tokenu api."</div>
                    }.into_view(),
                }).unwrap_or_default()
        }}
        </Suspense>
        // Warning of problems with token

        // Api documentation
    }
}

#[component]
pub fn AccountPage(
    change_password_action: Action<ChangePassword, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <div class="account-container">
            <ChangePasswordForm change_password_action=change_password_action />
        </div>
        <div class="requests-container">
            <UserRequests />
        </div>

        <RequirePerm permission="api_token_gen".to_string()>
            <div class="api-container">
                <ApiInfo />
            </div>
        </RequirePerm>
    }
}
