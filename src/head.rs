use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Props component for the UserCommandHead component
#[derive(Properties, PartialEq)]
pub struct UserCommandHeadProps {
    pub command_text: String,
}

/// Function component for representing the contnet from running the head command
#[function_component(UserCommandHead)]
pub fn user_command_head(props: &UserCommandHeadProps) -> Html {
    let text_color = css!("color: var(--text-color-main); margin: 0px; margin-left: 2px;");
    let pre_format = css!("margin:0px;");
    let description_format = css!("margin-top:1rem;");

    html! {
        <>
            <div class="user-command">
                <div class="command-single-line">
                    <UsernameText/>
                    <div class="command-input">
                        <p class={text_color}>
                            {props.command_text.clone()}
                        </p>
                    </div>
                </div>
                // ascii art is from: https://patorjk.com/software/taag/#p=display&h=1&f=Larry%203D&t=Hello!%0A
                <pre class={pre_format.clone()}>
                    {r" __  __           ___    ___             __     "}
                </pre>
                <pre class={pre_format.clone()}>
                    {r"/\ \/\ \         /\_ \  /\_ \           /\ \    "}
                </pre>
                <pre class={pre_format.clone()}>
                    {r"\ \ \_\ \      __\//\ \ \//\ \      ___ \ \ \   "}
                </pre>
                <pre class={pre_format.clone()}>
                    {r" \ \  _  \   /'__`\\ \ \  \ \ \    / __`\\ \ \  "}
                </pre>
                <pre class={pre_format.clone()}>
                    {r"  \ \ \ \ \ /\  __/ \_\ \_ \_\ \_ /\ \L\ \\ \_\ "}
                </pre>
                <pre class={pre_format.clone()}>
                    {r"   \ \_\ \_\\ \____\/\____\/\____\\ \____/ \/\_\"}
                </pre>
                <pre class={pre_format.clone()}>
                    {r"    \/_/\/_/ \/____/\/____/\/____/ \/___/   \/_/"}
                </pre>
                <p class={description_format}>
                    {"Welcome to my website! My name is Theo and I am currently a student. Please feel free to explore this website and reach out with questions or if you want to share something, I love to talk about cool things with people!
                    This website is meant to showcase some of the stuff I do. It is also modeled after a Linux terminal,
                    so to interact you can simply type commands and hit 'enter'. A list of commands can be found by running the 'help' command."}
                </p>
            </div>
        </>
    }
}
