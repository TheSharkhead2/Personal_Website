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
    let command_color = css!("color: var(--text-color-quad);");
    let ascii_art_div = css!("margin-top: 1rem; margin-bottom: 1rem;");

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
                <div class={ascii_art_div}>
                    // <pre class={pre_format.clone()}>
                    //     {r" __  __           ___    ___             __     "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"/\ \/\ \         /\_ \  /\_ \           /\ \    "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"\ \ \_\ \      __\//\ \ \//\ \      ___ \ \ \   "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r" \ \  _  \   /'__`\\ \ \  \ \ \    / __`\\ \ \  "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"  \ \ \ \ \ /\  __/ \_\ \_ \_\ \_ /\ \L\ \\ \_\ "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"   \ \_\ \_\\ \____\/\____\/\____\\ \____/ \/\_\"}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"    \/_/\/_/ \/____/\/____/\/____/ \/___/   \/_/"}
                    // </pre>

                    // ascii art name from: https://onlineasciitools.com/convert-text-to-ascii-art
                    <pre class={pre_format.clone()}>
                        {r"  _   _                                _      "}
                    </pre>
                    <pre class={pre_format.clone()}>
                        {r" | |_| |__   ___  ___    _ __ ___   __| | ___ "}
                    </pre>
                    <pre class={pre_format.clone()}>
                        {r" | __| '_ \ / _ \/ _ \  | '__/ _ \ / _` |/ _ \"}
                    </pre>
                    <pre class={pre_format.clone()}>
                        {r" | |_| | | |  __/ (_) | | | | (_) | (_| |  __/"}
                    </pre>
                    <pre class={pre_format.clone()}>
                        {r"  \__|_| |_|\___|\___/  |_|  \___/ \__,_|\___|"}
                    </pre>

                    // <pre class={pre_format.clone()}>
                    //     {r"   __  __                                 __   "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"  / /_/ /_  ___  ____     _________  ____/ /__ "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r" / __/ __ \/ _ \/ __ \   / ___/ __ \/ __  / _ \"}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"/ /_/ / / /  __/ /_/ /  / /  / /_/ / /_/ /  __/"}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"\__/_/ /_/\___/\____/  /_/   \____/\__,_/\___/ "}
                    // </pre>
                    // <p class={description_format}>
                    //     {"Welcome to my website! My name is Theo and I am currently a student. Please feel free to explore this website and reach out with questions or if you want to share something, I love to talk about cool things with people!
                    //     This website is meant to showcase some of the stuff I do. It is also modeled after a Linux terminal,
                    //     so to interact you can simply type commands and hit 'enter'. A list of commands can be found by running the 'help' command."}
                    // </p>

                    // <pre class={pre_format.clone()}>
                    //     {r"  _______ _                  _____           _      "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r" |__   __| |                |  __ \         | |     "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"    | |  | |__   ___  ___   | |__) |___   __| | ___ "}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"    | |  | '_ \ / _ \/ _ \  |  _  // _ \ / _` |/ _ \"}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"    | |  | | | |  __/ (_) | | | \ \ (_) | (_| |  __/"}
                    // </pre>
                    // <pre class={pre_format.clone()}>
                    //     {r"    |_|  |_| |_|\___|\___/  |_|  \_\___/ \__,_|\___|"}
                    // </pre>
                </div>

                <p class={description_format.clone()}>
                    {"Welcome to my terminal-style website! You can explore my projects and other possibly cool things."}
                </p>
                <p class={description_format.clone()}>
                    {"Type '"} <span class={command_color}> {"help"} </span> {"' for a list of commands."}
                </p>
             </div>
        </>
    }
}
