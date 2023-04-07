use stylist::{css, Style};
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Props for help command command basic description
#[derive(Properties, PartialEq)]
struct HelpCommandDescriptionProps {
    pub command_name: String,
    pub command_description: String,
}

/// Function component for each command and its description
#[function_component(HelpCommandDescription)]
fn help_command_description(props: &HelpCommandDescriptionProps) -> Html {
    let command_div = css!("display: flex; flex-direction: row; align-items: flex-start;");

    // have all the divs line up with 15 characters for command name and description
    let margin = 15 - props.command_name.len();
    let command_name = Style::new(format!(
        "font-weight: 500; margin-left: 1ch; margin-right: {margin}ch; color: var(--text-color-quad);"
    ))
    .expect("failed to create style");

    html! {
        <div class={command_div}>
            <p class={command_name}> {&props.command_name} </p>
            <p> {&props.command_description} </p>
        </div>
    }
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
            <HelpCommandDescription
                command_name={String::from("about")}
                command_description={String::from("More information about me! (If you're curious)")}
            />
            <HelpCommandDescription
                command_name={String::from("clear")}
                command_description={String::from("Clears the screen.")}
            />
            <HelpCommandDescription
                command_name={String::from("echo")}
                command_description={String::from("Print any text to the screen.")}
            />
            <HelpCommandDescription
                command_name={String::from("github")}
                command_description={String::from("A link to my GitHub!")}
            />
            <HelpCommandDescription
                command_name={String::from("head")}
                command_description={String::from("Displays simple heading information about the website.")}
            />
            <HelpCommandDescription
                command_name={String::from("help")}
                command_description={String::from("Displays available commands and breif descriptions.")}
            />
            <HelpCommandDescription
                command_name={String::from("linkedin")}
                command_description={String::from("A link to my LinkedIn!")}
            />
            <HelpCommandDescription
                command_name={String::from("projects")}
                command_description={String::from("A list of all the projects I have worked on with descriptions.")}
            />
            <HelpCommandDescription
                command_name={String::from("resume")}
                command_description={String::from("A link to my resume!")}
            />
        </div>
    }
}
