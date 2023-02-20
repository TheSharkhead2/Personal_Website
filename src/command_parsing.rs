pub const SUPPORTED_COMMANDS: &[&str] = &["help"];

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
