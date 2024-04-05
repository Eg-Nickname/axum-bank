use leptos::*;
use leptos_router::Redirect;
/// Component that ensures that children aren't shown to user if user hasn't got correct permission it redirets else it displays children components.
#[component]
pub fn require_perm_with_redirect(permission: String, children: ChildrenFn) -> impl IntoView {
    let permission_check = create_blocking_resource(
        || {},
        move |_| {
            use crate::auth::check_user_permission;
            check_user_permission(permission.clone())
        },
    );

    view! {
        <Suspense
            fallback=move || view! {<span>"Loading..."</span>}
        >
        {move ||{
            match permission_check.get(){
                Some(Ok(true)) => view!{}.into_view(),
                _ => view!{ <Redirect path="/" /> }.into_view()
            }
        }}
        {children()}
        </Suspense>
    }
}

/// Comonent showing children components only to user with correct permission.
#[component]
pub fn require_perm(permission: String, children: ChildrenFn) -> impl IntoView {
    let permission_check = create_blocking_resource(
        || {},
        move |_| {
            use crate::auth::check_user_permission;
            check_user_permission(permission.clone())
        },
    );
    let (show_children, set_show_children) = create_signal(false);
    view! {
        <Suspense
            fallback=move || view! {<span>"Loading..."</span>}
        >
        {move ||{
            if let Some(Ok(true)) = permission_check.get(){
                set_show_children(true)
            };
        }};

        {if show_children(){
            view!{{children()}}.into_view()
        }else{
            view! {}.into_view()
        }}
        </Suspense>
    }
}
