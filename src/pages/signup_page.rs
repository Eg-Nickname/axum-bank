use leptos::*;
use leptos_router::*;
use crate::auth::Signup;

#[component]
pub fn SignupPage(
    action: Action<Signup, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Sign Up"</h1>
            <label>
                "User ID:"
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                "Confirm Password:"
                <input type="password" placeholder="Password again" name="password_confirmation" class="auth-input" />
            </label>
            <br/>
            <label>
                "Remember me?"
                <input type="checkbox" name="remember" class="auth-input" />
            </label>

            <br/>
            <button type="submit">"Sign Up"</button>
        </ActionForm>
    }
}