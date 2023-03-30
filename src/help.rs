use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

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
    let command_info: Vec<String> = vec![
        help_command_formatter(
            "head",
            "Displays simple heading information about the website.",
        ),
        help_command_formatter(
            "help",
            "Displays available commands and breif descriptions.",
        ),
        help_command_formatter("about", "More information about me! (If you're curious)"),
        help_command_formatter(
            "projects",
            "A list of all the projects I have worked on with descriptions.",
        ),
    ];

    command_info.join("\n")
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
