use crate::command_parsing::SUPPORTED_COMMANDS; // import all commands implemented currently

/// Function for the auto complete
pub fn auto_complete(current: String) -> Option<String> {
    let command_args: Vec<String> = current.split_whitespace().map(|s| s.to_owned()).collect();

    if command_args.len() > 1 {
        // pass to individual command helper functions if looking for argument completion
        if SUPPORTED_COMMANDS.contains(&&command_args[0][..]) {
            // if the command is recognized

            // match command to auto complete helper function
            let auto_complete_command = match &command_args[0][..] {
                "about" => about_auto_complete,
                "clear" => clear_auto_complete,
                "echo" => echo_auto_complete,
                "education" => education_auto_complete,
                "github" => github_auto_complete,
                "head" => head_auto_complete,
                "help" => help_auto_complete,
                "linkedin" => linkedin_auto_complete,
                "projects" => projects_auto_complete,
                "resume" => resume_auto_complete,
                _ => return None, // shouldn't run, but backstop
            };

            // run and return auto complete results
            return auto_complete_command(current, command_args);
        } else {
            return None; // no recognized command, no results
        }
    }

    // loop through all possible commands and look for match
    for possible_command in SUPPORTED_COMMANDS {
        if possible_command.contains(&&current[..]) {
            // if the first part matches, then return command complete
            if possible_command == &&current[..] {
                // return None on exact match
                return None;
            }
            return Some(String::from(*possible_command));
        }
    }

    // no match, return no auto complete
    None
}

/// auto complete for the about function
fn about_auto_complete(_current: String, _current_args: Vec<String>) -> Option<String> {
    None
}

/// auto complete for the clear function
fn clear_auto_complete(_current: String, _current_args: Vec<String>) -> Option<String> {
    None
}

/// auto complete for the echo function
fn echo_auto_complete(_current: String, _current_args: Vec<String>) -> Option<String> {
    None
}

/// auto complete for the education function
fn education_auto_complete(current: String, current_args: Vec<String>) -> Option<String> {
    // let current_arg_name = current_args[1..].join(" "); // join all args into a single string
    let current_arg_name = String::from(&current[(current_args[0].len() + 1)..]); // take everything but the command
    let current_arg_name_lowercase = current_arg_name.to_lowercase();

    // match to hmc
    if "harvey mudd college".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Harvey Mudd College"));
    } else if "hmc".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("HMC"));
    }

    // match to nueva
    if "nueva high school".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Nueva High School"));
    }

    None
}

/// auto complete for github function
fn github_auto_complete(_current: String, _current_args: Vec<String>) -> Option<String> {
    None
}

/// auto complete for the head function
fn head_auto_complete(_current: String, _current_args: Vec<String>) -> Option<String> {
    None
}

/// auto complete for the help function
fn help_auto_complete(_current: String, _current_args: Vec<String>) -> Option<String> {
    None
}

/// auto complete for the linkedin function
fn linkedin_auto_complete(_current: String, _current_args: Vec<String>) -> Option<String> {
    None
}

/// auto complete for the projects function
fn projects_auto_complete(current: String, current_args: Vec<String>) -> Option<String> {
    // let current_arg_name = current_args[1..].join(" "); // join all args into a single string
    let current_arg_name = String::from(&current[(current_args[0].len() + 1)..]); // take everything but the command
    let current_arg_name_lowercase = current_arg_name.to_lowercase();

    // match StudentLive
    if "studentlive".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("StudentLive"));
    } else if "student live".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Student Live"));
    }

    // match spotify analytics dashboard
    if "spotify analytics dashboard".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Spotify Analytics Dashboard"));
    }

    // match spotify api
    if "spotify api wrapper".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Spotify API Wrapper"));
    }

    // match the game of gradients
    if "the game of gradients".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("The Game of Gradients"));
    } else if "game of gradients".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Game of Gradients"));
    }

    // match shark attack data analysis
    if "shark attack data analysis".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Shark Attack Data Analysis"));
    }

    // match math curriculum
    if "writing and teaching math curriculum".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Writing and Teaching Math Curriculum"));
    } else if "math curriculum".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Math Curriculum"));
    }

    // match discord bot
    if "discord bot".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Discord Bot"));
    } else if "finnish".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Finnish"));
    }

    // match quantum nlp
    if "quantum natural language processing".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Quantum Natural Language Processing"));
    } else if "quantum nlp".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Quantum NLP"));
    }

    // match primality testing exploration
    if "primaility testing exploration".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Primality Testing Exploration"));
    }

    // match quantum checkers
    if "quantum checkers".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Quantum Checkers"));
    }

    // match knapsack problem exploration
    if "knapsack problem exploration".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Knapsack Problem Exploration"));
    }

    // match pagerank implementation
    if "pagerank implementation".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("PageRank Implementation"));
    } else if "page rank implementation".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Page Rank Implementation"));
    }

    // match various sorting algorithms
    if "various sorting algorithm implementations".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Various Sorting Algorithm Implementations"));
    } else if "sorting algorithms".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Sorting Algorithms"));
    }

    // match image processing algorithms
    if "image processing algorithms".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Image Processing Algorithms"));
    }

    // match SHA256 implementation
    if "sha256 implementation".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("SHA256 Implementation"));
    }

    // match RSA encryption implementation
    if "rsa encryption implementation".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("RSA Encryption Implementation"));
    }

    // match three body problem simulation
    if "three body problem simulation".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Three Body Problem Simulation"));
    }

    // match spotify recommendation engine
    if "spotify recommendation engine".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Spotify Recommendation Engine"));
    }

    // match covid-19 data analysis
    if "covid-19 data analysis".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("COVID-19 Data Analysis"));
    } else if "covid19 data analysis".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("COVID19 Data Analysis"));
    }

    // match text based role playing game
    if "text based role playing game".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Text Based Role Playing Game"));
    } else if "text based rpg".starts_with(&current_arg_name_lowercase[..]) {
        return Some(String::from("Text Based RPG"));
    }

    None
}

/// auto complete for the resume function
fn resume_auto_complete(_current: String, _current_args: Vec<String>) -> Option<String> {
    None
}
