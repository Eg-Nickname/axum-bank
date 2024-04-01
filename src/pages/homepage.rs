use leptos::*;
use leptos_router::*;

use crate::utils::UserContextType;

#[component]
pub fn HomePage() -> impl IntoView {
    let user = use_context::<UserContextType>().expect("User resource shoud have been provided.");

    view! {
        <div class="info-container">
            <div>
                <p style="font-size:32px;">"Z nami transakcje staja się łatwiejsze..."</p>

                <p style="margin: 10px 0px">
                    "Chcesz, żeby twoje życie było prostsze? Chcesz pozbyć się problemów związanych ze swoimi transakcjami? Jeśli tak, to trafiłes w idealne miejsce. Na naszej stronie masz możliwość sprawdzenia swojego salda, wymiany swoich pieniędzy między czterema rodajami walut (Ruble, Peso Chilijskie, Yuan, Liry). Oferujemy również rozbudowany międzyplatformowy system wymiany walut międy innymi użytkownikami. Jeśli posiadasz już konto zaloguj się lub dołacz do nas klikając przycisk poniżej i spraw by twoje życie było łatwiejsze!!"
                </p>

                <p style="margin: 10px 0px 40px">
                    "Dołacz do nas już dziś i spraw by twoje życie nie było uciążliwe"
                </p>
                <Suspense
                        fallback=move || view! {<span>"Loading..."</span>}
                    >
                        {move || {
                            user.get().map(|user| match user {
                                // Logged in user
                                Ok(Some(_user)) => view! {
                                    <A class="transparent-purple-button" href="/transactions">"Dziękujemy za zaufanie!"<i class="fas fa-arrow-right"></i></A>
                                }.into_view(),

                                // Not logged in user
                                Ok(None) => view! {
                                    <A class="transparent-purple-button" href="/signup">"Dołacz do nas!"<i class="fas fa-arrow-right"></i></A>
                                }.into_view(),

                                // Else
                                _ => view!{}.into_view()
                            })
                        }}
                </Suspense>
            </div>
        </div>
        <div class="main-page-footer">
            <svg id="visual" viewBox="0 0 1250 300" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1"><path d="M0 74L29.8 79.2C59.7 84.3 119.3 94.7 178.8 106.5C238.3 118.3 297.7 131.7 357.2 138.8C416.7 146 476.3 147 535.8 142C595.3 137 654.7 126 714.2 111.2C773.7 96.3 833.3 77.7 892.8 81.8C952.3 86 1011.7 113 1071.2 112C1130.7 111 1190.3 82 1220.2 67.5L1250 53L1250 301L1220.2 301C1190.3 301 1130.7 301 1071.2 301C1011.7 301 952.3 301 892.8 301C833.3 301 773.7 301 714.2 301C654.7 301 595.3 301 535.8 301C476.3 301 416.7 301 357.2 301C297.7 301 238.3 301 178.8 301C119.3 301 59.7 301 29.8 301L0 301Z" fill="#d6cef4"></path><path d="M0 111L29.8 119.5C59.7 128 119.3 145 178.8 147.3C238.3 149.7 297.7 137.3 357.2 138.7C416.7 140 476.3 155 535.8 163.3C595.3 171.7 654.7 173.3 714.2 174.5C773.7 175.7 833.3 176.3 892.8 177.2C952.3 178 1011.7 179 1071.2 168.8C1130.7 158.7 1190.3 137.3 1220.2 126.7L1250 116L1250 301L1220.2 301C1190.3 301 1130.7 301 1071.2 301C1011.7 301 952.3 301 892.8 301C833.3 301 773.7 301 714.2 301C654.7 301 595.3 301 535.8 301C476.3 301 416.7 301 357.2 301C297.7 301 238.3 301 178.8 301C119.3 301 59.7 301 29.8 301L0 301Z" fill="#bcaef0"></path><path d="M0 184L29.8 188.5C59.7 193 119.3 202 178.8 198.5C238.3 195 297.7 179 357.2 168.2C416.7 157.3 476.3 151.7 535.8 157.8C595.3 164 654.7 182 714.2 191.7C773.7 201.3 833.3 202.7 892.8 193.7C952.3 184.7 1011.7 165.3 1071.2 155.5C1130.7 145.7 1190.3 145.3 1220.2 145.2L1250 145L1250 301L1220.2 301C1190.3 301 1130.7 301 1071.2 301C1011.7 301 952.3 301 892.8 301C833.3 301 773.7 301 714.2 301C654.7 301 595.3 301 535.8 301C476.3 301 416.7 301 357.2 301C297.7 301 238.3 301 178.8 301C119.3 301 59.7 301 29.8 301L0 301Z" fill="#a28eea"></path><path d="M0 226L29.8 226.3C59.7 226.7 119.3 227.3 178.8 223.2C238.3 219 297.7 210 357.2 209.5C416.7 209 476.3 217 535.8 218.8C595.3 220.7 654.7 216.3 714.2 217.2C773.7 218 833.3 224 892.8 220.3C952.3 216.7 1011.7 203.3 1071.2 199.2C1130.7 195 1190.3 200 1220.2 202.5L1250 205L1250 301L1220.2 301C1190.3 301 1130.7 301 1071.2 301C1011.7 301 952.3 301 892.8 301C833.3 301 773.7 301 714.2 301C654.7 301 595.3 301 535.8 301C476.3 301 416.7 301 357.2 301C297.7 301 238.3 301 178.8 301C119.3 301 59.7 301 29.8 301L0 301Z" fill="#876ee4"></path><path d="M0 264L29.8 259.8C59.7 255.7 119.3 247.3 178.8 244.3C238.3 241.3 297.7 243.7 357.2 246.8C416.7 250 476.3 254 535.8 257.3C595.3 260.7 654.7 263.3 714.2 258.8C773.7 254.3 833.3 242.7 892.8 242.7C952.3 242.7 1011.7 254.3 1071.2 259C1130.7 263.7 1190.3 261.3 1220.2 260.2L1250 259L1250 301L1220.2 301C1190.3 301 1130.7 301 1071.2 301C1011.7 301 952.3 301 892.8 301C833.3 301 773.7 301 714.2 301C654.7 301 595.3 301 535.8 301C476.3 301 416.7 301 357.2 301C297.7 301 238.3 301 178.8 301C119.3 301 59.7 301 29.8 301L0 301Z" fill="#6a4edc"></path></svg>
        </div>
    }
}
