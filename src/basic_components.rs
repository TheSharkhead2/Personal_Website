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
    let invalid_text_color = css!("color: var(--text-color-incorrect); margin: 0px;");
    let valid_text_color = css!("color: var(--text-color-main); margin: 0px;");

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

/// Helper function which takes in a description and parses it into strings which are less than the specified size, but as long as possible
fn help_command_description_parser(description: &str, length: usize) -> Vec<String> {
    let mut description_chunks: Vec<String> = Vec::new(); // blank vector to push chunks to

    let words: Vec<&str> = description.split(' ').collect(); // split into words

    let mut temp_chunk = String::new(); // temp variable for the current chunk the loop is parsing

    // loop through all words to create chunks
    for word in words.iter() {
        // check to see if chunk will exceed size with next word
        if temp_chunk.len() + 1 + word.len() > length {
            description_chunks.push(temp_chunk); // save previous words as chunk

            // reset temp variables
            temp_chunk = format!("{} ", word);
        } else {
            temp_chunk.push_str(word); // push word to chunk
            temp_chunk.push(' '); // add space
        }
    }

    // finally save last temp chunk
    description_chunks.push(temp_chunk);

    description_chunks
}

/// Helper function for formatting commands for the help function
fn help_command_formatter(command: &str, description: &str) -> String {
    let description_chunks = help_command_description_parser(description, 100); // split description into chunks

    let help_text = format!("  {:15}{}", command, description_chunks[0]); // take first chunk of description and make initial string in help text

    // take remaining description chunks and align them
    let remaining_description =
        (description_chunks[1..]).join(&("\n".to_owned() + &" ".repeat(17))[..]);

    help_text + &remaining_description
}

/// Helper function which generates help text for help command
fn generate_help_text() -> String {
    format!(
        "{}\n{}",
        help_command_formatter(
            "head",
            "Displays simple heading information about the website.",
        ),
        help_command_formatter(
            "help",
            "Displays available commands and breif descriptions."
        )
    )
}

/// Props object for the help command component
#[derive(Properties, PartialEq)]
pub struct UserCommandHelpProps {
    pub command_text: String,
}

/// Function component for the help command content
#[function_component(UserCommandHelp)]
pub fn user_command_help(props: &UserCommandHelpProps) -> Html {
    let text_color = css!("color: var(--text-color-main); margin: 0px; margin-left: 2px;");
    let pre_format = css!("margin:0px; font-size: 1rem;");

    let help_text = generate_help_text();

    html! {
        <div class="user-command">
            <div class="command-single-line">
                <UsernameText/>
                <div class="command-input">
                    <p class={text_color}>
                        {props.command_text.clone()}
                    </p>
                </div>
            </div>
            <p>
                {"Available commands:"}
            </p>
            <pre class={pre_format}>
                {help_text}
            </pre>
        </div>
    }
}
