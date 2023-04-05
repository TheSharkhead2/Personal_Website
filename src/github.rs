// use web_sys::window;
use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Props for the github object
#[derive(Properties, PartialEq)]
pub struct GithubCommandProps {
    pub command_text: String,
}

/// Function component for github command
#[function_component(GithubCommand)]
pub fn github_command(props: &GithubCommandProps) -> Html {
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
                {"Check out my GitHub here: "} <a href="https://github.com/TheSharkhead2" target="_blank"> {"github.com/TheSharkhead2"} </a>
            </p>
        </div>
    }
}

// some weird bug means that this drops a closure that I didn't make so it crashes the website... no fix that I can see without patching web_sys

// /// component for the github command
// pub struct GithubCommand;

// impl Component for GithubCommand {
//     type Message = ();
//     type Properties = GithubCommandProps;

//     fn create(_ctx: &Context<Self>) -> Self {
//         GithubCommand
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {<UserCommandWrapper command_text={ctx.props().command_text.clone()} command_result={"Opening github.com/TheSharkhead2 in a new tab."} invalid_input={false}/>}
//     }

//     fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
//         // only open link on first render
//         if first_render {
//             let window = window();
//             // open github link in new tab
//             if let Some(window) = window {
//                 window
//                     .open_with_url_and_target("https://github.com/TheSharkhead2", "_blank")
//                     .unwrap(); // don't really care if this errors
//             }
//         }
//     }
// }
