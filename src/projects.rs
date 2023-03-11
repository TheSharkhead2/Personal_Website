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
            <p class={new_line_below}> {"Here are some of my favorite projects! If you would like to learn more about a specific project you can run 'projects [project_name]'. For example: 'projects the game of gradients'."}</p>
            <ProjectsAll/>
        </div>
    }
}

/// Properties object for the project breif component
#[derive(Properties, PartialEq)]
struct ProjectBreifProps {
    pub name: String,                       // name of project
    pub project_date: String,               // Date range for project
    pub description: String,                // description of the project
    pub tech: Vec<String>,                  // the tech used in the project
    pub links: Vec<(String, &'static str)>, // Links for project (link text, actual link)
}

/// Function component for the breif project descriptions
#[function_component(ProjectBreif)]
fn project_breif(props: &ProjectBreifProps) -> Html {
    let project_css = css!("padding-bottom: 1rem;");
    let project_name = css!("font-weight: bold; font-size: 1rem; margin: 0px;");
    let project_date = css!("color: var(--text-color-main-transparent); padding-bottom: 0.3rem;");
    let accent_color = css!("color: var(--text-color-secondary);");
    let additional_info = css!("padding-top: 0.3rem;");
    let additional_info_text = css!("color: var(--text-color-main-transparent);");

    let tech_string = props.tech.join(" • "); // join together techs with bullets

    let mut links_html: Vec<Html> = props.links[..(props.links.len()-1)].iter().map(
        |(text, link)| html! { <> <a href={link.to_owned()} target={"_blank"}>{text.clone()}</a> <span> {" • "} </span> </> },
    ).collect();
    links_html.push(html! { <a href={props.links[props.links.len()-1].1} target="_blank"> {props.links[props.links.len()-1].0.clone()} </a> });

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
                <p class={additional_info_text.clone()}>
                    <span class={accent_color.clone()}>
                        {"Links: "}
                    </span>
                    {for links_html}
                </p>
            </div>
        </div>
    }
}

/// Function component for the content of the projects command when no project args are passed
#[function_component(ProjectsAll)]
fn projects_all() -> Html {
    html! {
        <>
            <ProjectBreif
                name={"The Game of Gradients"}
                project_date={"Nov 2022 - Jan 2023"}
                description={"I led a small team in designing and creating a web game to teach the intuition behind gradient fields in vector calculus. My high school math teacher even replaced the last day of his MVC class with the game!"}
                tech={vec![String::from("Rust"), String::from("Wasm"), String::from("Bevy")]}
                links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/The_Game_of_Gradients"), (String::from("The Game!"), r"https://thesharkhead2.github.io/The_Game_of_Gradients/")]}
            />
            <ProjectBreif
                name={"Spotify Analytics Dashboard"}
                project_date={"Oct 2022 - Present"}
                description={"The goal of this project is to make an all purpose analytics app for your Spotify listening history.
                    There is a particular emphasis on your listening trends over time instead of just averages or cumulative data."}
                tech={vec![String::from("TypeScript"), String::from("Rust"), String::from("Svelte"), String::from("Tauri"), String::from("D3.js")]}
                links={vec![(String::from("GitHub"), r"https://github.com/TheSharkhead2/Spotify_Analytics_Dashboard")]}
            />
            <ProjectBreif
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
