use leptos::*;
use leptos_router::*;
use crate::auth::Signup;

#[component]
pub fn SignupPage(
    action: Action<Signup, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <div class="register-container">
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
                <div>
                    <input type="submit" class="solid-purple-button" value="Rejestruj" name="transaction" />
                </div>
            </ActionForm>
        </div>
        <div class="additional-info-container">
            <h3>"Masz już konto?"</h3>
            <p style="margin-bottom: 30px;">
                "Zaloguj się klikając w link poniżej i ciesz się możliwościami!"
            </p>
            <A  class="transparent-purple-button" href="/login/">"Zaloguj sie"</A>  
        </div>
    }
}