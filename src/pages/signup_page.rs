use leptos::*;
use leptos_router::*;
use crate::auth::Signup;

#[component]
pub fn SignupPage(
    action: Action<Signup, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h2>"Zarejestruj się"</h2>
            <div>
                <i class="fas fa-user"></i>
                " Login"
                <input type="text" placeholder="Login" maxlength="32" name="username" class="auth-input" />
            </div>

            <div>
                <i class="fas fa-lock"></i>
                " Hasło"
                <input type="password" placeholder="Hasło" name="password" class="auth-input" />
            </div>

            <div>
                <i class="fas fa-lock"></i>
                " Potwierdź hasło"
                <input type="password" placeholder="Hasło" name="password_confirmation" class="auth-input" />
            </div>


            <div>
            "Zapamiętaj mnie"
                <input type="checkbox" name="remember" class="auth-input" />
            </div>

            <input type="submit" class="solid-purple-button" value="Rejestruj" name="transaction" />
        </ActionForm>
    }
}