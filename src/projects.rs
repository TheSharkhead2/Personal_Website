use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Properties object for the projects command component
#[derive(Properties, PartialEq)]
pub struct UserCommandProjectsProps {
    pub command_text: String,
    pub command_args: Vec<String>,
}

/// Function component for the content of the projects command
#[function_component(UserCommandProjects)]
pub fn user_command_projects(props: &UserCommandProjectsProps) -> Html {
    let text_color = css!("color: var(--text-color-main); margin: 0px; margin-left: 2px;");
    let new_line_below = css!("padding-bottom: 1rem;");

    let mut content_html = html! {
        <>
            <p class={new_line_below}> {"Here are some of my favorite projects! If you would like to learn more about a specific project you can run 'projects [project_name]'. For example: 'projects the game of gradients'."}</p>
            <ProjectsAll/>
        </>
    }; // blank html to change into what the command will display

    if props.command_args.len() > 1 {
        // otherwise try to find specific project to display
        let project_arg_name = props.command_args[1..].join(" "); // join all args into a single string
        let project_arg_name_lowercase = project_arg_name.to_lowercase();

        if [
            "spotify analytics dashboard",
            "spotify analytics",
            "spotify dashboard",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<SpotifyAnalyticsDashboard/>};
        } else if [
            "spoitfy api wrapper",
            "spotify api",
            "rust spotify api wrapper",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<SpotifyApiWrapper/>};
        } else if ["the game of gradients", "game of gradients"]
            .contains(&&project_arg_name_lowercase[..])
        {
            // if the project name supplied by user can be inferred to be the game of gradients
            content_html = html! {<TheGameOfGradients/>};
        } else if [
            "shark attack data analysis",
            "shark attack data",
            "shark attack",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<SharkAttackDataAnalysis/>};
        } else if [
            "math curriculum",
            "math teaching",
            "writing and teaching math curriculum",
            "writing math curriculum",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<MathCurriculum/>};
        } else if ["discord bot", "finnish"].contains(&&project_arg_name_lowercase[..]) {
            content_html = html! {<DiscordBot />};
        } else if ["primality", "prime testing", "primality testing"]
            .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<PrimalityTesting />};
        } else if ["quantum checkers"].contains(&&project_arg_name_lowercase[..]) {
            content_html = html! { <QuantumCheckers/>};
        } else if [
            "knapsack problem",
            "dynamic programming exploration",
            "knapsack problem exploration",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<KnapsackProblemExploration/>};
        } else if [
            "pagerank",
            "page rank",
            "page rank implementation",
            "pagerank implementation",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<PageRankImplementation/>};
        } else if ["various sorting algorithms", "sorting algorithms"]
            .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<VariousSortingAlgorithms/>};
        } else if [
            "image processing",
            "image processing algorithms",
            "canny edge detection",
            "seam carving",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<ImageProcessingAlgorithms />};
        } else if ["sha256", "sha256 implementation", "sha implementation"]
            .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<Sha256Implementation />};
        } else if ["rsa", "rsa encryption", "rsa encryption implementation"]
            .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<RsaEncryption />};
        } else if [
            "three body",
            "three body problem",
            "three body simulation",
            "third body simulation",
            "three body problem simulation",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<ThreeBodyProblemSimulation />};
        } else if [
            "spotify recommendations",
            "spotify recommendation engine",
            "spotify recommendation",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<SpotifyRecommendationEngine />};
        } else if [
            "covid data analysis",
            "covid19 data analysis",
            "covid-19 data analysis",
            "covid analysis",
            "covid modeling",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<CovidDataAnalysis />};
        } else if [
            "text based rpg",
            "text based role playing game",
            "text-based rpg",
            "text-based role playing game",
        ]
        .contains(&&project_arg_name_lowercase[..])
        {
            content_html = html! {<TextBasedRolePlayingGame />};
        } else {
            // if no project is recognized, simply display an error message
            content_html = html! {
                <>
                    <p> {"The project '"} {project_arg_name} {"' is not recognised. Try one of the following projects:"} </p>
                    <p> {"Spotify Analytics Dashboard, Spotify API Wrapper, The Game of Gradients"} </p>
                </>
            };
        }
    }

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
            {content_html}
        </div>
    }
}

/// Properties object for the project breif component
#[derive(Properties, PartialEq)]
struct ProjectBriefProps {
    pub name: String,                               // name of project
    pub project_date: String,                       // Date range for project
    pub description: String,                        // description of the project
    pub tech: Vec<String>,                          // the tech used in the project
    pub links: Option<Vec<(String, &'static str)>>, // Links for project (link text, actual link)
}

/// Function component for the breif project descriptions
#[function_component(ProjectBrief)]
fn project_brief(props: &ProjectBriefProps) -> Html {
    let project_css = css!("padding-bottom: 1rem;");
    let project_name = css!("font-weight: bold; font-size: 1rem; margin: 0px;");
    let project_date = css!("color: var(--text-color-main-transparent); padding-bottom: 0.3rem;");
    let accent_color = css!("color: var(--text-color-secondary);");
    let additional_info = css!("padding-top: 0.3rem;");
    let additional_info_text = css!("color: var(--text-color-main-transparent);");

    let tech_string = props.tech.join(" • "); // join together techs with bullets

    let mut links_html: Vec<Html> = Vec::new(); // blank vec to put into this scope

    if let Some(links) = &props.links {
        links_html = links[..(links.len()-1)].iter().map(
            |(text, link)| html! { <> <a href={link.to_owned()} target={"_blank"}>{text.clone()}</a> <span> {" • "} </span> </> },
        ).collect();
        links_html.push(html! { <a href={links[links.len()-1].1} target="_blank"> {links[links.len()-1].0.clone()} </a> });
    }

    html! {
        <div class={project_css.clone()}>
            <h1 class={project_name.clone()}>
                {props.name.clone()}
            </h1>
            <p class={project_date.clone()}>
                {props.project_date.clone()}
            </p>
            <p>
                {props.description.clone()}
            </p>
            <div class={additional_info.clone()}>
                <p class={additional_info_text.clone()}>
                    <span class={accent_color.clone()}>
                        {"Tech: "}
                    </span>
                    {tech_string}
                </p>
                if props.links.is_some() {
                    <p class={additional_info_text.clone()}>
                        <span class={accent_color.clone()}>
                            {"Links: "}
                        </span>
                        {for links_html}
                    </p>
                }
            </div>
        </div>
    }
}

/// Function component for the content of the projects command when no project args are passed
#[function_component(ProjectsAll)]
fn projects_all() -> Html {
    html! {
        <>
            <ProjectBrief
                name={"The Game of Gradients"}
                project_date={"Nov 2022 - Jan 2023"}
                description={"I led a small team in designing and creating a web game to teach the intuition behind gradient fields in vector calculus. My high school math teacher even replaced the last day of his MVC class with the game!"}
                tech={vec![String::from("Rust"), String::from("Wasm"), String::from("Bevy")]}
                links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/The_Game_of_Gradients"), (String::from("The Game!"), r"https://thesharkhead2.github.io/The_Game_of_Gradients/")]}
            />
            <ProjectBrief
                name={"Spotify Analytics Dashboard"}
                project_date={"Oct 2022 - Present"}
                description={"The goal of this project is to make an all purpose analytics app for your Spotify listening history.
                    There is a particular emphasis on your listening trends over time instead of just averages or cumulative data."}
                tech={vec![String::from("TypeScript"), String::from("Rust"), String::from("Svelte"), String::from("Tauri"), String::from("D3.js")]}
                links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/Spotify_Analytics_Dashboard")]}
            />
            <ProjectBrief
                name={"Three Body Problem Simulation"}
                project_date={"Nov 2021 - Dec 2021"}
                description={"I developed an interactive simulation of the three body problem in order to demonstrate the intricacies of this chaotic physics problem. The simulation allows you to view the path of the third body using arbitrary starting conditions and at the precise Lagrange points."}
                tech={vec![String::from("Python"), String::from("PyGame")]}
                links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/third_body_simulation")]}
            />
            // <ProjectBreif
            //     name={"Rust Spotify API Wrapper"}
            //     project_date={"Aug 2022 - Present"}
            //     description={"This project is being developed side by side with my Spotify Analytics Dashboard. It is designed to be a simple wrapper for interacting with the Spotify API that focuses on clarity and reliability in the content it returns."}
            //     tech={vec![String::from("Rust")]}
            //     links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/spotify.rs"), (String::from("crates.io"), r"https://crates.io/crates/spotifyrs")]}
            // />
        </>
    }
}

/// Function component for the Spotify Analytics Dashboard project (detailed)
#[function_component(SpotifyAnalyticsDashboard)]
fn spotify_analytics_dashboard() -> Html {
    html! {
        <ProjectBrief
            name={"Spotify Analytics Dashboard"}
            project_date={"Oct 2022 - Present"}
            description={"The goal of this project is to build an all purpose analytics app for your Spotify listening history. I love to dig into analytics about data in order to find patterns and looking at data that pertains to yourself is always even more interesting.
                        I want to build this app such that you can look at trends in your music over time, how similar your music is, and other patterns that can be found in your music. The particular focus of this app is the graphing of data over time because most music 
                        analytics apps today are cumulative data rather timeseries data. The app uses a Rust backend, and my Spotify.rs API wrapper, with a Svelte frontend that is built using Tauri."}
            tech={vec![String::from("TypeScript"), String::from("Rust"), String::from("Svelte"), String::from("Tauri"), String::from("D3.js")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/Spotify_Analytics_Dashboard")]}
        />
    }
}

/// Function component for the spotify api wrapper project (detailed)
#[function_component(SpotifyApiWrapper)]
fn spotify_api_wrapper() -> Html {
    html! {
        <ProjectBrief
            name={"Spotify API Wrapper"}
            project_date={"Aug 2022 - Present"}
            description={"This project is being developed side by side with my Spotify Analytics Dashboard in order to meet the needs of that project. The goal of the project is to develop a wrapper such that the Spotify API can be interacted with in a simple and reliable way.
                        Currently, this is a package on crates.io and is available to use in any rust project!"}
            tech={vec![String::from("Rust")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/spotify.rs"), (String::from("crates.io"), r"https://crates.io/crates/spotifyrs")]}
        />
    }
}

/// Function component for the game of gradients project (detailed)
#[function_component(TheGameOfGradients)]
fn the_game_of_gradients() -> Html {
    html! {
        <ProjectBrief
            name={"The Game of Gradients"}
            project_date={"Nov 2022 - Jan 2023"}
            description={"This project was a open-ended final project for my multivariable calculus class at Harvey Mudd. The goal of this project was to create a game that could teach the player intuition behind gradient fields.
                        I led the small team of three people in designing and developing the game, which is written in pure Rust and uses the Bevy game engine. In order to create the game we wanted, we had to implement a real-time vector field 
                        visualizer that could respond to changes in our basic algebraic function composer. In order to distribute the game, it is compiled to Wasm and shipped on a basic webpage. After the game was suitable to play, I demonstrated the game to my high school MVC teacher who actually scrapped his last day plans for class
                        and played the game with his class instead! I am quite proud of where the game is and I plan to work on it more in the future. Feel free to try the game through the link(s) below!"}
            tech={vec![String::from("Rust"), String::from("Wasm"), String::from("Bevy")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/The_Game_of_Gradients"), (String::from("The Game!"), r"https://thesharkhead2.github.io/The_Game_of_Gradients/")]}
        />
    }
}

/// Function component for the shark attack data analysis project (detailed)
#[function_component(SharkAttackDataAnalysis)]
fn shark_attack_data_analysis() -> Html {
    html! {
        <ProjectBrief
            name={"Shark Attack Data Analysis"}
            project_date={"Nov 2022 - Dec 2022"}
            description={"This was an end of year, open-ended project for my CSCI042 class. I analyzed shark attack data over the last two centuries in order to look at relative dangers of different species, activites, and more. A majority of the project was formatting the poorly kept dataset as much of the data was inconsistently entered or missing. While I can't post the project online, if you are curious and would like to see it simply reach out!"}
            tech={vec![String::from("Julia"), String::from("Pluto.jl"), String::from("DataFrames.jl"), String::from("Plots.jl")]}
        />
    }
}

/// Function component for the math curriculum project (detailed)
#[function_component(MathCurriculum)]
fn math_curriculum() -> Html {
    html! {
        <ProjectBrief
            name={"Writing and Teaching Math Curriculum"}
            project_date={"Jun 2021 - Aug 2022"}
            description={"This project was a culmination of many things for me. It started when I fell in love with math and I wanted to see if there was a way to bring higher math to younger ages. This prompted me to write a week long curriculum for teaching the principles of calculus to middle schoolers, something which I piloted as a small summer camp in 2021. Watching the excitement that the kids had over the concepts of calculus made me think about ways in which math could be taught differently to bring that excitement to everyone, so I began writing a sort of textbook that was aimed at redoing the way we teach math. I wrote about half of a sixth grade cirruculum, but unfortunately got busy with many other projects. This is a project I am very passionate about, however, and plan to continue in the future with a series of video lectures."}
            tech={vec![String::from("LaTeX")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/MathV2")]}
        />
    }
}

/// Function component for the discord bot project (detailed)
#[function_component(DiscordBot)]
fn discord_bot() -> Html {
    html! {
        <ProjectBrief
            name={"Discord Bot"}
            project_date={"Jul 2020 - Jun 2022"}
            description={"Like many people in the middle of 2020, my friends and I were at a bit of a loss about what to do. We had a small Discord server and I decided to create something that could connect us a little bit more. This led to the creation of a Discord bot that could help me manage the server, give jokes and track funny things we said, give us analytics on the messages we sent, and most importantly, provide Discord-based escape rooms and puzzles!
                        I used this project as a way to get experience developing software for customers. I made sure that I had proper versioning and tracked bugs reported by my friends. The bot was also an avenue for my love of puzzles as I put a lot of work into designing custom puzzles, and even entire digital escape rooms with stories that anyone on the server could interact with. At the end I was developing full on puzzle games with save states, user accounts, 
                        and inventories. There is almost nothing better than watching your friends solve an escape room you made custom for them! After consistently maintaining the bot for about two years, I decided to stop when we all left for college as we were spending a lot less time online. This is still one of my favorite projects I have worked on, and maybe I might come back to it one day!"}
            tech={vec![String::from("Python")]}
        />
    }
}

/// Function component for the primality testing project (detailed)
#[function_component(PrimalityTesting)]
fn primality_testing() -> Html {
    html! {
        <ProjectBrief
            name={"Primality Testing Algorithms Exploration"}
            project_date={"Mar 2022 - Jun 2022"}
            description={"I ended up completing the coursework for my algorithms class in high school about 3 months early, so for the rest of the year I decided to do an in-depth analysis of a particular type of algorithm, hence this project.
                        Through this project I built up 5 primality testing algorithms from scratch in order to understand the inner workings of each algorithm. Additionally, I theoretically and experimentally analyzed the runtime of each algorithm.
                        In culmination I implemented the Miller-Rabin primality testing algorithm, which involved also implementing a fast modular exponentiation algorithm."}
            tech={vec![String::from("Julia")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/Primality_Testing")]}
        />
    }
}

/// Function component for the quantum checkers simulation (detailed)
#[function_component(QuantumCheckers)]
fn quantum_checkers() -> Html {
    html! {
        <ProjectBrief
            name={"Quantum Checkers"}
            project_date={"Mar 2022 - Apr 2022"}
            description={"This was a project for my Quantum Information and Computation class in high school. The project involved implementing a simulation to run a particular game that involved propagating qubits over finite time and space, using a solution to the Schrodinger equation, and passing the entangled state of the two qubits through various logic gates depending on the results of measuring the locations of the qubits.
                        I built the project in pure Julia using the GameZero.jl game engine. The game runs locally and is designed to be as playable as possible, though the game itself is quite confusing and nonsensical (not my fault, it was designed as an intellectual problem)."}
            tech={vec![String::from("Julia")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/Quantum_Checkers")]}
        />
    }
}

/// Function component for the knapsack problem (detailed)
#[function_component(KnapsackProblemExploration)]
fn knapsack_problem_exploration() -> Html {
    html! {
        <ProjectBrief
            name={"Knapsack Problem Exploration"}
            project_date={"Feb 2022 - Mar 2022"}
            description={"For my algorithms class in high school, I decided to explore dynamic programming by trying to implement the dymanic programming solution to the knapsack problem without looking at any implementations. I used the README on the GitHub page to document my process: the struggles, my learnings, and everything I tried in the process. In the end, I was able to implement the dynamic programming algorithm to the knapsack problem where every item can have between 0 and k occurences. I verified the solution by checking the runtime against the theoretical runtime."}
            tech={vec![String::from("Julia")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/Knapsack_Problem")]}
        />
    }
}

/// Function component for the pagerank algorithm implementation (detailed)
#[function_component(PageRankImplementation)]
fn page_rank_implementation() -> Html {
    html! {
        <ProjectBrief
            name={"PageRank Implementation"}
            project_date={"Feb 2022"}
            description={"For my high school algorithms class I implemented the PageRank algorithm in Julia. This was overall a very small project, though it was still incredibly enjoyable to implement such an important algorithm!"}
            tech={vec![String::from("Julia")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/PageRank")]}
        />
    }
}

/// Function component for the sorting algorithms (detailed)
#[function_component(VariousSortingAlgorithms)]
fn various_sorting_algorithms() -> Html {
    html! {
        <ProjectBrief
            name={"Various Sorting Algorithm Implementations"}
            project_date={"Jan 2022"}
            description={"This project is a collection of a few sorting algorithms that I implemented in my high school algorithms class. It also includes an implementation of AVL trees. As part of this project, I also did an analysis on the running time of each algorithm to insure it was implemented correctly."}
            tech={vec![String::from("Julia")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/list_sorting")]}
        />
    }
}

/// Function component for the image processing algorithms project (detailed)
#[function_component(ImageProcessingAlgorithms)]
fn image_processing_algorithms() -> Html {
    html! {
        <ProjectBrief
            name={"Image Processing Algorithms"}
            project_date={"Sep 2021 - Dec 2021"}
            description={"For my high school computer vision class I implemented numerous image processing algorithms all from scratch and all in Julia. These include: implementing a convolutional filter algorithm to build everything else upon, Canny edge detection, Harris corner detection, and finally seam carving. All algorithms were designed to be used as an external API that is simple to use."}
            tech={vec![String::from("Julia")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/imageProcessing")]}
        />
    }
}

/// Function component for my SHA256 implementation project (detailed)
#[function_component(Sha256Implementation)]
fn sha256_implementation() -> Html {
    html! {
        <ProjectBrief
            name={"SHA256 Implementation"}
            project_date={"Nov 2021 - Dec 2021"}
            description={"For this project, I implemented the SHA256 algorithm, and all its underlying logic, from scratch in Julia. This was a project I chose for my high school computer security class because I wanted to understand hashing algorithms better and I thought a good way of doing this was to implement one from the ground up.
                        I learned a lot about how hashing accomplishes what it does from this project and it was generally a really fun project to work on."}
            tech={vec![String::from("Julia")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/sha256")]}
        />
    }
}

/// Function component for my RSA encryption implementation (detailed)
#[function_component(RsaEncryption)]
fn rsa_encryption() -> Html {
    html! {
        <ProjectBrief
            name={"RSA Encryption Implementation"}
            project_date={"Nov 2021"}
            description={"This was a project I picked for my computer security class in high school and I picked it because I wanted to learn more about how encryption functions generally. RSA seemed like a reasonable choice because it is widely used today. For this project, I implemented the RSA encryption algorithm from scratch in Julia. I was able to learn a lot from digging into the math behind the algorithm and writing the entire implementation myself."}
            tech={vec![String::from("Julia")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/RSA")]}
        />
    }
}

/// Function component for my three body simulation project (detailed)
#[function_component(ThreeBodyProblemSimulation)]
fn three_body_problem_simulation() -> Html {
    html! {
        <ProjectBrief
            name={"Three Body Problem Simulation"}
            project_date={"Nov 2021 - Dec 2021"}
            description={"The three body problem is a famous problem from differential equations and physics which asks about the gravitational behavior of 3 objects. While for 2 bodies the behavior is entirely predictable, with a third body the motion becomes chaotic and it is impossible to perfectly predict the outcome in a system from the initial conditions. For my differential equations class, I implemented a simulation of the three body problem in order to demonstrate the situation and to look for patterns. In particular, my group wanted to confirm calculations like the position of Lagrange points, which I built the simulation to test and verify. The entire simulation is built off of the PyGame game engine in Python."}
            tech={vec![String::from("Python")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/third_body_simulation")]}
        />
    }
}

/// Function component for Spotify recommendation engine
#[function_component(SpotifyRecommendationEngine)]
fn spotify_recommendation_engine() -> Html {
    html! {
        <ProjectBrief
            name={"Spotify Recommendation Engine"}
            project_date={"Jan 2021 - Apr 2021"}
            description={"The goal of this project was to offer a desktop client that could determine the likelihood that you would like a particular song. I used the song data available on Spotify in conjunction with a TensorFlow machine learning algorithm in order to build a model that predicted how much a user would like a particular song. In addition to the model, this project involved creating an interface through which users could rate songs in order to give the model labeled data to act upon. If I were to continue the project, the end goal would be to use this model in order to provide recommendations in the form of playlists for people."}
            tech={vec![String::from("Python"), String::from("TensorFlow"), String::from("Pandas")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/SpotifyRecommendations")]}
        />
    }
}

/// Function component for COVID data analysis
#[function_component(CovidDataAnalysis)]
fn covid_data_analysis() -> Html {
    html! {
        <ProjectBrief
            name={"COVID-19 Data Analysis"}
            project_date={"May 2020 - Aug 2020"}
            description={"I wanted to understand how the pandemic was changing, so I began this project as a way of looking at the case and death data in a way I understood. I used various predictive models to analyze the patterns in the data with a particular emphasis on predicting whether cases were going to continue to exponentially grow or level off in the near future. I also worked to develop intuitive ways of displaying the data so that the information it contained was properly conveyed."}
            tech={vec![String::from("Python"), String::from("Pandas"), String::from("Matplotlib")]}
            links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/COVID19_Modeling")]}
        />
    }
}

/// Function component for text based role-playing game
#[function_component(TextBasedRolePlayingGame)]
fn text_based_role_playing_game() -> Html {
    html! {
        <ProjectBrief
            name={"Text Based Role Playing Game"}
            project_date={"Oct 2018 - Jan 2021"}
            description={"I have always really enjoyed the complexity of building characters in RPGs (role playing games), so I spent almost a year building a text-based RPG game in Python that focused on the complexity of building a character through gear and abilities. I built a combat system that took into account dozens of factors from a player's gear and build in the game as well as a in-depth gear system which allowed for that significant customization.
                        Following this initial game, I began working on a second version which involved creating a tile-based rendering system so that the player could interact with more than a terminal window. I never finished this second version, however."}
            tech={vec![String::from("Python")]}
            links={vec![(String::from("Version 1 GitHub"), r"https://github.com/TheSharkhead2/rpggame1"), (String::from("Version 2 GitHub"), r"https://github.com/TheSharkhead2/rpggame2")]}
        />
    }
}
