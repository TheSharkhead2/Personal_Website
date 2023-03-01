use stylist::css;
use stylist::yew::use_style;
use yew::prelude::*;

use crate::basic_components::{UserCommandHead, UserCommandWrapper, UsernameText};

pub const SUPPORTED_COMMANDS: &[&str] = &["help", "head"];

/// Checks to see if the supplied command is valid
pub fn valid_command(command: &str) -> bool {
    let command: Vec<&str> = command.split(' ').collect(); // split command at whitespace for parsing

    // if SUPPORTED_COMMANDS.contains(&command[0]) {
    //     return true;
    // } else {
    //     return false;
    // }
    SUPPORTED_COMMANDS.contains(&command[0])
}

/// Takes in user command and parses it
pub fn parse_command(command: &str) -> Html {
    // check to make sure the command is valid
    if !valid_command(command) {
        return command_not_found(command);
    }

    if command == "help" {}

    // head command gives introductory info for the website and me
    if command == "head" {
        return html! {<UserCommandHead command_text={command.to_owned()}/>};
    }

    html! {}
}

/// Returns Html element for when the command is not found
fn command_not_found(command: &str) -> Html {
    // get formatted string for help test
    let text = format!(
        "Command '{}' not found.\nAvailable commands:\n{:?}",
        command.to_owned(),
        "help"
    );

    html! {
        <UserCommandWrapper command_text={command.to_owned()} command_result={text} invalid_input={true}/>
    }
}

/// Props for command history
#[derive(Properties, PartialEq)]
pub struct PreviousCommandsProps {
    pub command_history: Vec<String>,
}

/// Component for all of user's previous commands
#[function_component(PreviousCommands)]
pub fn previous_commands(props: &PreviousCommandsProps) -> Html {
    html! {
        // <ul class="command-history">
            {for props.command_history.iter().map(|command| html!{<PreviousCommand command={command.clone()}/>})}
        // </ul>
    }
}

/// Props for specific item in command history
#[derive(Properties, PartialEq)]
pub struct PreviousCommandProps {
    pub command: String,
}

/// Component for specific command in history
#[function_component(PreviousCommand)]
pub fn previous_command(props: &PreviousCommandProps) -> Html {
    parse_command(&props.command[..]) // parse command to get html representation of output
}
