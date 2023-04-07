use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Props object for the education componenent
#[derive(Properties, PartialEq)]
pub struct EducationCommandProps {
    pub command_text: String,
    pub command_args: Vec<String>,
}

/// Education command component
#[function_component(EducationCommand)]
pub fn eductation_command(props: &EducationCommandProps) -> Html {
    let text_color = css!("color: var(--text-color-main); margin: 0px; margin-left: 2px;");

    // check for passed args
    if props.command_args.len() > 1 {
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
            </div>
        }
    } else {
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
                <BasicEducation />
            </div>
        }
    }
}

/// Component for just my schools
#[function_component(BasicEducation)]
fn basic_education() -> Html {
    html! {
        <>
            <SchoolHeadline
                name={String::from("Harvey Mudd College")}
                date={String::from("2022 - 2026")}
                description={String::from("I am pursing degrees in Computer Science and Mathematics. Currently, I am a Homework Hotline Tutor providing homework assistance on-call to all elementary through high school students across America. Relevant coursework includes: Discrete Math, Advanced Linear Algebra, and Principles & Practice: Computer Science.")}
                education_type={String::from("Bachelor of Science Degree")}
            />
            <SchoolHeadline
                name={String::from("The Nueva School")}
                date={String::from("2018 - 2022")}
                description={String::from("I am a current alumni volunteer helping to organize alumni events. Relevant coursework includes: Quantum Information and Computation, Algorithms, Into and Advanced Machine Learning, Computer Security, Computer Vision, Graph Theory, and Mobile App Design.")}
                education_type={String::from("High School Diploma")}
            />
        </>
    }
}

/// Props component for school component
#[derive(Properties, PartialEq)]
struct SchoolHeadlineProps {
    pub name: String,
    pub date: String,
    pub description: String,
    pub education_type: String,
}

/// Component for basic school overview
#[function_component(SchoolHeadline)]
fn school_headline(props: &SchoolHeadlineProps) -> Html {
    let main_div_css = css!("padding-bottom: 1rem;");
    let project_name = css!("font-weight: bold; font-size: 1rem; margin: 0px;");
    let project_date = css!("color: var(--text-color-main-transparent); padding-bottom: 0.3rem;");

    html! {
        <div class={main_div_css}>
            <h1 class={project_name}>
                {props.name.clone()} {" "} <span class={project_date.clone()}> {"("}{props.education_type.clone()}{")"} </span>
            </h1>
            <p class={project_date}>
                {props.date.clone()}
            </p>
            <p>
                {props.description.clone()}
            </p>
        </div>
    }
}
