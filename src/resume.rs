use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Props for the resume object
#[derive(Properties, PartialEq)]
pub struct ResumeCommandProps {
    pub command_text: String,
}

/// Function component for github command
#[function_component(ResumeCommand)]
pub fn resume_command(props: &ResumeCommandProps) -> Html {
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
                {"Check out my resume here: "} <a href="https://github.com/TheSharkhead2/Resume/blob/main/resume.pdf" target="_blank"> {"Resume!"} </a>
            </p>
        </div>
    }
}
