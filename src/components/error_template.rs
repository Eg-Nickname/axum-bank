// use cfg_if::cfg_if;
// use http::status::StatusCode;
use leptos::{Errors, *};
// #[cfg(feature = "ssr")]
// use leptos_axum::ResponseOptions;
// use thiserror::Error;


// #[derive(Clone, Debug, Error)]
// pub enum AppError {
//     #[error("Not Found")]
//     NotFound,
// }

// impl AppError {
//     pub fn status_code(&self) -> StatusCode {
//         match self {
//             AppError::NotFound => StatusCode::NOT_FOUND,
//         }
//     }
// }

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] _outside_errors: Option<Errors>,
    #[prop(optional)] _errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    // let errors = match outside_errors {
    //     Some(e) => create_rw_signal(e),
    //     None => match errors {
    //         Some(e) => e,
    //         None => panic!("No Errors found and we expected errors!"),
    //     },
    // };
    // // Get Errors from Signal
    // let errors = errors.get();

    // // Downcast lets us take a type that implements `std::error::Error`
    // let errors: Vec<AppError> = errors
    //     .into_iter()
    //     .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
    //     .collect();
    // // println!("Errors: {errors:#?}");

    // // Only the response code for the first error is actually sent from the server
    // // this may be customized by the specific application
    // cfg_if! { if #[cfg(feature="ssr")] {
    //     let response = use_context::<ResponseOptions>();
    //     if let Some(response) = response {
    //         response.set_status(errors[0].status_code());
    //     }
    // }}

    view! {
        // <h1>{if errors.len() > 1 {"Errors"} else {"Error"}}</h1>
        <p>"Error occured"</p>
    }
}
