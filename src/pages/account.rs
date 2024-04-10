use leptos::*;
use leptos_router::*;

use crate::components::require_perm::RequirePerm;
use crate::server::account::ChangePassword;

#[component]
pub fn AccountPage(
    change_password_action: Action<ChangePassword, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <div class="account-container">
            <ActionForm action=change_password_action>
                <h1>"Zmień hasło"</h1>
                <div class="input-box1">
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
        </div>
        <RequirePerm permission="api".to_string()>
            <h1>"To jest tylko widoczne dla użytkownika z permisją api"</h1>
        </RequirePerm>

        <RequirePerm permission="admin".to_string()>
            <h1>"To jest tylko widoczne dla użytkownika z permisją admin"</h1>
        </RequirePerm>

        <div class="api-container">
            // Send request for generation permission
            // If granted add api token generator

            // <form action=change_password_action>
            //     <h1>"Dodaj token API"</h1>
            //     <div class="input-box1">
            //         <i>"Stare hasło"</i>
            //         <input type="password" placeholder="Stare hasło" name="old_password" class="text-input" />
            //     </div>

            //     <div class="transacition-input-box">
            //         <input type="submit" class="solid-purple-button" value="Wygeneruj token API" name="transaction" />
            //     </div>
            // </form>


            // If token present

            // Delete button
            // Warning of problems with token
            // Regenerate Token button
            // Api documentation
        </div>
    }
}
