use leptos::*;
use leptos_router::*;
use crate::auth::Login;

#[component]
pub fn LoginPage(
    action: Action<Login, Result<(), ServerFnError>>,
) -> impl IntoView {

    view! {
    <div class="container">
        <div class="form-box">
            <a href="../">
                <div class="home">
                <i class="fas fa-house"></i>
                </div>
            </a>

            <div class="company-logo">
                <img src="images/logo_bank.png" alt="" />
            </div>

            <div class="sign-in">
                <ActionForm action=action>
                <h2 class="title">"Zaloguj się"</h2>
                    <div class="input-box">
                        <label>
                            <i class="fas fa-user"></i>
                            <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
                        </label>
                    </div>
                    <br/>
                    <label>
                        <i class="fas fa-lock"></i>
                        <input type="password" placeholder="Password" name="password" class="auth-input" />
                    </label>
                    <br/>
                    <label>
                        "Remember me?"
                        <input type="checkbox" name="remember" class="auth-input" />
                    </label>
                    <br/>
                    <input type="submit" class="buton" value="Zaloguj sie" />
                </ActionForm>
            </div>
        </div>


        <div class="panels-box">
            <div class="spacer">
                <svg id="visual" viewBox="0 0 960 300" width="960" height="300" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1"><rect x="0" y="0" width="960" height="300" fill="#FFF"></rect><path d="M0 95L17.8 99.2C35.7 103.3 71.3 111.7 106.8 118.7C142.3 125.7 177.7 131.3 213.2 130.2C248.7 129 284.3 121 320 115C355.7 109 391.3 105 426.8 101.2C462.3 97.3 497.7 93.7 533.2 99.8C568.7 106 604.3 122 640 122.5C675.7 123 711.3 108 746.8 107.5C782.3 107 817.7 121 853.2 129.8C888.7 138.7 924.3 142.3 942.2 144.2L960 146L960 301L942.2 301C924.3 301 888.7 301 853.2 301C817.7 301 782.3 301 746.8 301C711.3 301 675.7 301 640 301C604.3 301 568.7 301 533.2 301C497.7 301 462.3 301 426.8 301C391.3 301 355.7 301 320 301C284.3 301 248.7 301 213.2 301C177.7 301 142.3 301 106.8 301C71.3 301 35.7 301 17.8 301L0 301Z" fill="#d6cef4"></path><path d="M0 140L17.8 147.8C35.7 155.7 71.3 171.3 106.8 168C142.3 164.7 177.7 142.3 213.2 135.8C248.7 129.3 284.3 138.7 320 149.3C355.7 160 391.3 172 426.8 169C462.3 166 497.7 148 533.2 145.5C568.7 143 604.3 156 640 159.5C675.7 163 711.3 157 746.8 149.5C782.3 142 817.7 133 853.2 135.7C888.7 138.3 924.3 152.7 942.2 159.8L960 167L960 301L942.2 301C924.3 301 888.7 301 853.2 301C817.7 301 782.3 301 746.8 301C711.3 301 675.7 301 640 301C604.3 301 568.7 301 533.2 301C497.7 301 462.3 301 426.8 301C391.3 301 355.7 301 320 301C284.3 301 248.7 301 213.2 301C177.7 301 142.3 301 106.8 301C71.3 301 35.7 301 17.8 301L0 301Z" fill="#bcaef0"></path><path d="M0 182L17.8 176.8C35.7 171.7 71.3 161.3 106.8 155.2C142.3 149 177.7 147 213.2 149.5C248.7 152 284.3 159 320 161.5C355.7 164 391.3 162 426.8 158.8C462.3 155.7 497.7 151.3 533.2 152.2C568.7 153 604.3 159 640 161.3C675.7 163.7 711.3 162.3 746.8 164.2C782.3 166 817.7 171 853.2 178C888.7 185 924.3 194 942.2 198.5L960 203L960 301L942.2 301C924.3 301 888.7 301 853.2 301C817.7 301 782.3 301 746.8 301C711.3 301 675.7 301 640 301C604.3 301 568.7 301 533.2 301C497.7 301 462.3 301 426.8 301C391.3 301 355.7 301 320 301C284.3 301 248.7 301 213.2 301C177.7 301 142.3 301 106.8 301C71.3 301 35.7 301 17.8 301L0 301Z" fill="#a28eea"></path><path d="M0 195L17.8 204.2C35.7 213.3 71.3 231.7 106.8 233C142.3 234.3 177.7 218.7 213.2 216.2C248.7 213.7 284.3 224.3 320 228.3C355.7 232.3 391.3 229.7 426.8 228.5C462.3 227.3 497.7 227.7 533.2 231C568.7 234.3 604.3 240.7 640 235.2C675.7 229.7 711.3 212.3 746.8 205.3C782.3 198.3 817.7 201.7 853.2 207.7C888.7 213.7 924.3 222.3 942.2 226.7L960 231L960 301L942.2 301C924.3 301 888.7 301 853.2 301C817.7 301 782.3 301 746.8 301C711.3 301 675.7 301 640 301C604.3 301 568.7 301 533.2 301C497.7 301 462.3 301 426.8 301C391.3 301 355.7 301 320 301C284.3 301 248.7 301 213.2 301C177.7 301 142.3 301 106.8 301C71.3 301 35.7 301 17.8 301L0 301Z" fill="#876ee4"></path><path d="M0 233L17.8 234C35.7 235 71.3 237 106.8 237.5C142.3 238 177.7 237 213.2 242C248.7 247 284.3 258 320 259.5C355.7 261 391.3 253 426.8 252.7C462.3 252.3 497.7 259.7 533.2 264.8C568.7 270 604.3 273 640 272.7C675.7 272.3 711.3 268.7 746.8 262C782.3 255.3 817.7 245.7 853.2 244.5C888.7 243.3 924.3 250.7 942.2 254.3L960 258L960 301L942.2 301C924.3 301 888.7 301 853.2 301C817.7 301 782.3 301 746.8 301C711.3 301 675.7 301 640 301C604.3 301 568.7 301 533.2 301C497.7 301 462.3 301 426.8 301C391.3 301 355.7 301 320 301C284.3 301 248.7 301 213.2 301C177.7 301 142.3 301 106.8 301C71.3 301 35.7 301 17.8 301L0 301Z" fill="#6a4edc"></path></svg>
            </div>
            <div class="panel panel-left">
                <div class="content">
                    <h3>"Jestes tu nowy?"</h3>
                        <p>
                            "Zarejestruj się już teraz klikając poniższy przycisk. Odniesiesz wiele korzyści zaczynając od możliwości sprawdzaenia swojego salda, až po dostęp do wymiany swoich pieniędzy na inne waluty czy trade'owanie się z innymi użytkownikami!"
                        </p>
                        <a class="button" id="rejestracja-button" href="/register">"Zarejestruj sie"</a>  

                </div>
                <img src="images/login.svg" alt="" class="image" />
            </div>
        </div>
    </div>
    }
}
