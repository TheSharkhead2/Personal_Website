use stylist::css;
use stylist::yew::use_style;
use yew::prelude::*;

/// Component for the user information and whatnot before the command input section
#[function_component(UsernameText)]
pub fn username_text() -> Html {
    // define basic color styles for use here
    let style_main = use_style!("color: var(--text-color-fifth); margin: 0px;");
    let style1 = use_style!("color: var(--text-color-third);");
    let style2 = use_style!("color: var(--text-color-secondary);");
    let style3 = use_style!("color: var(--text-color-quad);");

    html! {
        <p class={style_main}>
            <span class={style1}>{"guest"}</span> // possibly temp, with user login this would change
            {"@"}
            <span class={style2}>{"theorode.com"}</span>
            {":"}
            <span class={style3}>{"~"}</span> // temp, change to dir later
            {"$"}
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct UserCommandWrapperProps {
    pub command_text: String,
    pub command_result: String,
    pub invalid_input: bool, // whether or not to display text as invalid
}

/// Component which turns user entered command into object to distplay in command history/log
#[function_component(UserCommandWrapper)]
pub fn user_command_wrapper(props: &UserCommandWrapperProps) -> Html {
    let invalid_text_color =
        css!("color: var(--text-color-incorrect); margin: 0px; margin-left: 2px;");
    let valid_text_color = css!("color: var(--text-color-main); margin: 0px; margin-left: 2px;");

    html! {
        <>
            <div class="user-command">
                <div class="command-single-line">
                    <UsernameText/>
                    <div class = "command-input">
                        if props.invalid_input {
                            <p class={invalid_text_color}>
                                {props.command_text.clone()}
                            </p>
                        } else {
                            <p class={valid_text_color}>
                                {props.command_text.clone()}
                            </p>
                        }
                    </div>
                </div>
                <p>
                    {props.command_result.clone()}
                </p>
            </div>
        </>
    }
}
