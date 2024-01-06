use leptos::*;
use leptos_router::*;
#[component]
fn InspectExchangeListing() -> impl IntoView{
    view! {
    // {%if selected_exchange_listing %}

    // <div class="popup active" id="popup-2">
    //     <div class="overlay" onclick="togglePopup1()">

    //     </div>
    //     <div class="content-popup">
    //     <!-- <h4>{{selected_exchange_listing.id}}</h4> -->
    //     <form method="post">{% csrf_token %}

    //         {% for field in exchange_money_form %}
                
    //         <div class="input-box">
    //             <i>{{field.label}}:</i>
    //             <br>
    //             <br>
    //             {{field}}
    //         </div>
        
    //         {% for error in exchange_money_form.field.errors %}
    //             <p class="login-error">{{error}}</p>
    //         {% endfor %}
    //         {% endfor %}            
    //     <input type="submit" value="Wymień Pieniądze" name="exchange-money">
    //     </form>
    //     <script>
    //         document.getElementById("id_amount_exchanged").step = Number({{selected_exchange_listing.ratio_to}})
    //         document.getElementById("id_amount_exchanged").max = Number({{selected_exchange_listing.amount_wanted}}) 
    //         document.getElementById("id_amount_exchanged").placeholder = "Kwota którą chcesz wymienić w " + "{{selected_exchange_listing.exchange_to}}"
    //         document.getElementById("id_amount_recived").placeholder = "Kwota którą dostaniesz w " + "{{selected_exchange_listing.exchange_from}}"
    //     </script>

    //     <script>
    //         document.getElementById("id_amount_exchanged").oninput = () => {
    //         document.getElementById("id_amount_recived").value = ((document.getElementById("id_amount_exchanged").value)/{{selected_exchange_listing.ratio_to}}) * {{selected_exchange_listing.ratio_from}}
    //         }
    //     </script>

    //     <div class="close-bnt" onclick="togglePopup1()">&times;</div>
    //     </div>
    // </div>
    // {%endif%}
    }
}

#[component]
pub fn CreateExchangeListingPopUp() -> impl IntoView{
    view!{
        <div class="popup active">
        <A href="/currency_exchange"><div class="overlay"></div></A>
        
            
        <div class="content-popup">
            <form method="post">
                <h1>"Wymień waluty"</h1>

                // TODO fix transaction-input-box class name
                // TODO update to select like component
                <div class="transacition-input-box">
                    <i>"Waluta Z"</i>
                    <input type="text" placeholder="CNY" name="currency_from" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    <i>"Kwota Z"</i>
                    <input type="number" placeholder="0" name="ammount_from" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    <i>"Waluta Na"</i>
                    <input type="text" placeholder="RUB" name="currency_to" class="transaction-input" />
                </div>
                <div class="transacition-input-box">
                    <i>"Kwota Na"</i>
                    <input type="number" placeholder="0" name="ammount_to" class="transaction-input" />
                </div>

                <div class="transacition-input-box">
                    <input type="submit" class="buton-przelew" value="Wymień" name="transaction" />
                </div>
            </form>
            
            <A href="/currency_exchange"><div class="close-bnt"><i class="fa-solid fa-xmark"></i></div></A>
        </div>
        </div>
    }
}

#[component]
pub fn CurrencyExchangePage() -> impl IntoView{
    view!{
        <div class="content">
    //     <div class="exchange-info-container">
    //         <h2>Wymień waluty</h2> <button onclick="togglePopup()" class="buton-przelew button-oferawymiany"><i class="fas fa-plus"></i>Nowa Oferta Wymiany Walut</button>

    //         <form method="get">
    //             {% for field in exchangelisting_filter.form %}
                
    //             <div class="input-box filter-box">
    //                 <i>{{field.label}}:</i>
    //                 <br>
    //                 {{field}}
    //             </div>
            
    //             {% for error in transaction_form.field.errors %}
    //                 <p class="login-error">{{error}}</p>
    //             {% endfor %}
    //             {% endfor %}
    //             <button type="submit" class="buton-przelew nowy-przelew">Filtruj</button>
    //         </form>
    //     </div>

    // {% for exchange_listing in exchange_listings.qs %}
    // <div class="exchange-listing-container">
    //     <h2>Wymień</h2>
    //     <h3>{{exchange_listing.amount_wanted}} {{exchange_listing.exchange_to}} Na {{exchange_listing.amount_owned}} {{exchange_listing.exchange_from}}</h3>
    //     <h3>Ratio:</h3>
    //     <h4>{{exchange_listing.ratio_to}} {{exchange_listing.exchange_to}} : {{exchange_listing.ratio_from}} {{exchange_listing.exchange_from}}</h4>
    //     <h4 style="color: gray">Oferta od: {{exchange_listing.owner}}</h4>
    //     <!-- <h4 style="color: gray">Id: {{exchange_listing.id}}</h4> -->

        
    //     {% if exchange_listing.owner == request.user %}
    //     <a href="{% url "remove-listing" exchange_listing.id %}" class="link-button button-del">Usuń ofertę wymiany</a>
    //     {% else %}
    //     <a href="{% url "exchange-money" exchange_listing.id %}" class="link-button">Wymień</a>
    //     {% endif %}
    // </div>
    // {% endfor %}


    // </div>
</div>
    }
}