use yew::prelude::*;

use crate::about::UserCommandAbout;
use crate::basic_components::UserCommandWrapper;
use crate::echo::EchoCommand;
use crate::education::EducationCommand;
use crate::github::GithubCommand;
use crate::head::UserCommandHead;
use crate::help::UserCommandHelp;
use crate::linkedin::LinkedinCommand;
use crate::projects::UserCommandProjects;
use crate::resume::ResumeCommand;

pub const SUPPORTED_COMMANDS: &[&str] = &[
    "help",
    "head",
    "about",
    "projects",
    "clear",
    "github",
    "linkedin",
    "resume",
    "echo",
    "education",
];

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
pub fn parse_command(command: &str, clear_command_callback: &Callback<bool>) -> Html {
    let command_args: Vec<String> = command.split_whitespace().map(|s| s.to_owned()).collect();

    // check to make sure the command is valid
    if !valid_command(command) {
        return command_not_found(command, command_args);
    }

    // help command gives information on avaliable commands
    if command_args[0] == "help" {
        return html! {<UserCommandHelp command_text={command.to_owned()}/>};
    }

    // head command gives introductory info for the website and me
    if command_args[0] == "head" {
        return html! {<UserCommandHead command_text={command.to_owned()}/>};
    }

    // about command gives more detailed information about me
    if command_args[0] == "about" {
        return html! {<UserCommandAbout command_text={command.to_owned()}/>};
    }

    if command_args[0] == "projects" {
        return html! {<UserCommandProjects command_text={command.to_owned()} command_args={command_args}/>};
    }

    if command_args[0] == "clear" {
        clear_command_callback.emit(false);
        return html! {};
    }

    if command_args[0] == "github" {
        return html! {<GithubCommand command_text={command.to_owned()} />};
    }

    if command_args[0] == "linkedin" {
        return html! {<LinkedinCommand command_text={command.to_owned()} />};
    }

    if command_args[0] == "resume" {
        return html! {<ResumeCommand command_text={command.to_owned()} />};
    }

    if command_args[0] == "echo" {
        return html! {<EchoCommand command_text={command.to_owned()} command_args={command_args} />};
    }

    if command_args[0] == "education" {
        return html! {<EducationCommand command_text={command.to_owned()} command_args={command_args} />};
    }

    // here the command is in the SUPPORTED_COMMANDS array, but for some reason hasn't been implemented yet. Return invalid command as safety
    command_not_found(command, command_args)
}

/// Returns Html element for when the command is not found
fn command_not_found(command: &str, command_args: Vec<String>) -> Html {
    // get formatted string for help test
    let text = format!(
        "Command '{}' not found.\nTry 'help' for the list of valid commands.",
        command_args[0]
    );

    html! {
        <UserCommandWrapper command_text={command.to_owned()} command_result={text} invalid_input={true}/>
    }
}

/// Props for command history
#[derive(Properties, PartialEq)]
pub struct PreviousCommandsProps {
    pub command_history: Vec<String>,
    pub clear_command_callback: Callback<bool>,
}

/// Component for all of user's previous commands
#[function_component(PreviousCommands)]
pub fn previous_commands(props: &PreviousCommandsProps) -> Html {
    html! {
        // <ul class="command-history">
            {for props.command_history.iter().map(|command| html!{<PreviousCommand command={command.clone()} clear_command_callback={&props.clear_command_callback}/>})}
        // </ul>
    }
}

/// Props for specific item in command history
#[derive(Properties, PartialEq)]
pub struct PreviousCommandProps {
    pub command: String,
    pub clear_command_callback: Callback<bool>,
}

/// Component for specific command in history
#[function_component(PreviousCommand)]
pub fn previous_command(props: &PreviousCommandProps) -> Html {
    parse_command(&props.command[..], &props.clear_command_callback) // parse command to get html representation of output
}
