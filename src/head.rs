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
                    {"Welcome to my website! My name is Theo and I am currently a freshmen studying Computer Science and Math at Harvey Mudd College.
                    This website is designed to show you a little bit about me, so feel free to explore. I wanted to build a website where the interaction 
                    was similar to that of a Linux terminal, so if you want to look at the ways you can interact with the website you can run 'help' to get a list of commands."}
                </p>
            </div>
        </>
    }
}
