use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Properties object for the Linkedin Command component
#[derive(Properties, PartialEq)]
pub struct LinkedinCommandProps {
    pub command_text: String,
}

/// Linkedin command component
#[function_component(LinkedinCommand)]
pub fn linkedin_command(props: &LinkedinCommandProps) -> Html {
    let text_color = css!("color: var(--text-color-main); margin: 0px; margin-left: 2px;");

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
                {"Check out my LinkedIn here: "} <a href="https://www.linkedin.com/in/theorode/" target="_blank"> {"linkedin.com/in/theorode"} </a>
            </p>
        </div>
    }
}
