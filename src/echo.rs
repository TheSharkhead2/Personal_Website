use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Properties object for the Echo Command component
#[derive(Properties, PartialEq)]
pub struct EchoCommandProps {
    pub command_text: String,
    pub command_args: Vec<String>,
}

/// Echo command component
#[function_component(EchoCommand)]
pub fn echo_command(props: &EchoCommandProps) -> Html {
    let text_color = css!("color: var(--text-color-main); margin: 0px; margin-left: 2px;");

    let command_body = props.command_args[1..].join(" ");

    html! {
        <div class="user-command">
            <div class="command-single-line">
                <UsernameText/>
                <div class = "command-input">
                    <p class={text_color}>
                        {props.command_text.clone()}
                    </p>
                </div>
            </div>
            <p>
                {command_body}
            </p>
        </div>
    }
}
