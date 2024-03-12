use leptos::*;
/// Component that ensures that page is not shown user if user is not logged in. If user is logged displays children components.
#[component]
pub fn require_login_with_redirect(children: ChildrenFn) -> impl IntoView{
    use leptos_router::Redirect;
    use crate::auth::User;
    let user = use_context::<Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>>().expect("User resource shoud have been provided.");

    view!{
        <Suspense
            fallback=move || view! {<span>"Loading..."</span>}
        >
        {move ||{
            user.get().map(|user| match user {
                Ok(Some(_user)) => view! {}.into_view(),
                _ => {
                    // match redirect_route{
                    //     Some(route) =>{ view!{ <Redirect path={route} /> }.into_view()},
                    //     None => { view!{ <Redirect path="/" /> }.into_view()},
                    // }
                    view!{ <Redirect path="/login" /> }.into_view()}
                
            });
        }}
        {children()}
        </Suspense>
        
    }
}

/// Comonent showing children components only to logged users.
#[component]
pub fn require_login(children: ChildrenFn) -> impl IntoView{
    use crate::auth::User;
    let user = use_context::<Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>>().expect("User resource shoud have been provided.");
    let (show_children, set_show_children) = create_signal(false);
    view!{
        <Suspense
            fallback=move || view! {<span>"Loading..."</span>}
        >
        {move ||{
            user.get().map(|user| match user {
                Ok(Some(_user)) => set_show_children(true),
                _ => ()
        })}}

        {if show_children(){
            view!{""{children()}}.into_view()
        }else{
            view! {""}.into_view()
        }}

        </Suspense>
        
    }
}