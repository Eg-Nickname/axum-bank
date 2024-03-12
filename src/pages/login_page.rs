use leptos::*;
use leptos_router::*;
use crate::auth::Login;

#[component]
pub fn LoginPage(
    action: Action<Login, Result<(), ServerFnError>>,
) -> impl IntoView {

    view! {
            <div class="login-container">
                <ActionForm action=action>
                    <h2 class="title">"Zaloguj się"</h2>
                        <div class="input-box1">
                            <i class="fas fa-user"></i>
                            " Login"
                            <input type="text" placeholder="Login" maxlength="32" name="username" class="text-input" />
                        </div>

                        <div>
                            <i class="fas fa-lock"></i>
                            " Hasło"
                            <input type="password" placeholder="Hasło" name="password" class="text-input" />
                        </div>


                        <div>
                            "Zapamiętaj mnie"
                            <input type="checkbox" name="remember" class="text-input" />
                        </div>
                        <div>
                            <input type="submit" class="solid-purple-button" value="Zaloguj sie" />
                        </div>
                </ActionForm>
            </div>

            <div class="additional-info-container">
                <h3>"Jestes tu nowy?"</h3>
                <p style="margin-bottom: 30px;">
                    "Zarejestruj się już teraz klikając poniższy przycisk. Odniesiesz wiele korzyści zaczynając od możliwości sprawdzaenia swojego salda, až po dostęp do wymiany swoich pieniędzy na inne waluty czy trade'owanie się z innymi użytkownikami!"
                </p>
                <A  class="transparent-purple-button" href="/signup">"Zarejestruj sie"</A>  
            </div>
    }
}
