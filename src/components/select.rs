use leptos::*;

#[derive(Clone)]
struct OptionTextSetter(WriteSignal<&'static str>);

#[derive(Clone)]
struct OptionValueSetter(WriteSignal<&'static str>);

#[derive(Clone)]
struct ChildrenVisibilitySetter(WriteSignal<bool>);

#[component]
pub fn SelectInput(
    /// Optional default text if we don't have sensible default value.
    /// Will be overwritten if we use default on <SelectOption ... />
    #[prop(optional)]
    display_text: Option<&'static str>,
    /// Input name for server function
    name: &'static str,
    children: ChildrenFn,
) -> impl IntoView {
    let (selected_text, text_setter) = create_signal(display_text.unwrap_or_default());
    provide_context(OptionTextSetter(text_setter));

    let (selected_value, value_setter) = create_signal("");
    provide_context(OptionValueSetter(value_setter));

    // Show children
    let (show_children, set_show_children) = create_signal(false);
    provide_context(ChildrenVisibilitySetter(set_show_children));

    view! {
        <div class="select-input">
            <div class="select-header">{selected_text}
                <input type="hidden" value={selected_value} name=name />
                {move || {
                    if show_children(){
                        view!{
                            <a class="arrow" href="#" on:click=move |_| { set_show_children(!show_children()) }><i class="fa-solid fa-angle-down"></i></a>
                            <div class="collapse-children" on:click=move |_| { set_show_children(!show_children()) }>""</div>
                        }
                    }else{
                        view!{
                            ""
                            <a class="arrow" href="#" on:click=move |_| { set_show_children(!show_children()) }><i class="fa-solid fa-angle-up"></i></a>
                        }
                    }
                }}
            </div>

            <div class={move || {if show_children(){
                "options-wrapper options-visable"
            }else{
                "options-wrapper options-hidden"
            }}}>
            {children()}
            </div>
        </div>

    }
}

#[component]
pub fn SelectOption(
    /// Text displayed for user
    option_text: &'static str,
    /// Value setted while selected
    option_value: &'static str,
    /// Prop telling if this option should be set as default
    #[prop(optional)]
    default: Option<bool>,
) -> impl IntoView {
    let text_setter = use_context::<OptionTextSetter>().unwrap().0;
    let value_setter = use_context::<OptionValueSetter>().unwrap().0;
    let children_visbility_setter = use_context::<ChildrenVisibilitySetter>().unwrap().0;

    let on_click = move |_| {
        text_setter.update(|option| *option = option_text);
        value_setter.update(|option| *option = option_value);
        children_visbility_setter.update(|visibility| *visibility = false)
    };

    if default.unwrap_or(false) {
        text_setter.update(|option| *option = option_text);
        value_setter.update(|option| *option = option_value);
    }

    view! {
            <div class="select-option" on:click=on_click>
                {option_text}
            </div>
    }
}

/// Select option accepting text and value as single tuple
#[component]
pub fn SelectOptionTuple(
    /// Tuple containing text and value
    text_value_tuple: (&'static str, &'static str),
    /// Prop telling if this option should be set as default
    #[prop(optional)]
    default: Option<bool>,
) -> impl IntoView {
    let (option_text, option_value) = text_value_tuple;
    view! {
        <SelectOption  option_text=option_text option_value=option_value default=default.unwrap_or(false) />
    }
}

/// Select option that has same value and text display
#[component]
pub fn SelectOptionSingle(
    // Option value and displayed text
    option_value: &'static str,
    /// Prop telling if this option should be set as default
    #[prop(optional)]
    default: Option<bool>,
) -> impl IntoView {
    view! {
        <SelectOption  option_text=option_value option_value=option_value default=default.unwrap_or(false) />
    }
}
