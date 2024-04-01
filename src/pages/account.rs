use leptos::*;
use leptos_router::*;

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
    }
}
