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
    let command_color = css!("color: var(--text-color-quad);");

    // check for passed args
    if props.command_args.len() > 1 {
        let education_arg_name = props.command_args[1..].join(" "); // join all args into a single string
        let education_arg_name_lowercase = education_arg_name.to_lowercase();

        let mut content_html = html! {
            <p>
                {"The argument '"} {education_arg_name} {"' isn't recognized as a school I've gone to. Try: '"} <span class={command_color}> {"education Harvey Mudd College"} </span> {"'"}
            </p>
        };

        if [
            "the nueva school",
            "nueva",
            "nueva school",
            "high school",
            "nueva high school",
        ]
        .contains(&&education_arg_name_lowercase[..])
        {
            content_html = html! {<NuevaSchoolClasses />};
        } else if [
            "harvey mudd",
            "mudd",
            "harvey mudd college",
            "hmc",
            "college",
            "undergraduate",
        ]
        .contains(&&education_arg_name_lowercase[..])
        {
            content_html = html! {<HarveyMuddClasses />};
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
    let intro_text = css!("margin-bottom: 1rem;");
    let command_color = css!("color: var(--text-color-quad);");

    html! {
        <>
            <p class={intro_text}> {"I am currently a student at Harvey Mudd College, but here is my full education history. If you want to see the classes I have taken run '"} <span class={command_color}> {"education [SCHOOL_NAME]"} </span> {"' to see the classes I took (or am taking) at a specific institution."} </p>
            <SchoolHeadline
                name={String::from("Harvey Mudd College")}
                date={String::from("2022 - 2026")}
                description={String::from("I am pursing degrees in Computer Science and Mathematics. Currently, I am a Homework Hotline Tutor providing homework assistance on-call to all elementary through high school students across America. Relevant coursework includes: Discrete Math, Advanced Linear Algebra, and Principles & Practice: Computer Science.")}
                education_type={String::from("Bachelor of Science Degree")}
            />
            <SchoolHeadline
                name={String::from("The Nueva School")}
                date={String::from("2018 - 2022")}
                description={String::from("I am a current alumni volunteer helping to organize alumni events. I additionally was part of the track team and tennis team. Relevant coursework includes: Quantum Information and Computation, Algorithms, Into and Advanced Machine Learning, Computer Security, Computer Vision, Graph Theory, and Mobile App Design.")}
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

/// Props for a class component
#[derive(Properties, PartialEq)]
struct ClassNameProps {
    pub name: String,
    pub class_code: String,
}

/// Component for displaying a class and a class code
#[function_component(ClassName)]
fn class_name(props: &ClassNameProps) -> Html {
    let indent = css!("margin-left: 1rem;");
    let class_code_css = css!("color: var(--text-color-main-transparent);");
    html! {
        <p class={indent}>
            <span class={class_code_css}> {"("}{props.class_code.clone()}{")"} {" "} </span>{props.name.clone()}
        </p>
    }
}

/// Component for displaying my classes at HMC
#[function_component(HarveyMuddClasses)]
fn harvey_mudd_classes() -> Html {
    let category_text = css!("font-weight: bold; margin-top: 1rem;");

    html! {
        <>
            <p> {"I am currently attending Harvey Mudd College and pursing degrees in computer science and mathematics."} </p>
            <p> {"Below are the classes I have taken or am taking: "} </p>

            <p class={category_text.clone()}> {"Computer Science"} </p>
            <ClassName name={"Principles & Practice: Computer Science"} class_code={"CSCI042"} />

            <p class={category_text.clone()}> {"Mathematics"} </p>
            <ClassName name={"Advanced Linear Algebra"} class_code={"MATH173"} />
            <ClassName name={"Discrete Mathematics"} class_code={"MATH055"} />
            <ClassName name={"Single & Multivariable Calculus"} class_code={"MATH019"} />

            <p class={category_text.clone()}> {"HSA (Humanities, Social studies, and the Arts)"} </p>
            <ClassName name={"Critical Inquiry (Political Analysis)"} class_code={"HSA010"} />

            <p class={category_text}> {"Other"} </p>
            <ClassName name={"Mechanics & Wave Motion"} class_code={"PHYS024"} />
            <ClassName name={"Special Relativity"} class_code={"PHYS023"} />

            <ClassName name={"Chemistry in the Modern World"} class_code={"CHEM042"} />
            <ClassName name={"Chemistry Laboratory"} class_code={"CHEM024"} />

            <ClassName name={"Introduction to Biology"} class_code={"BIOL046"} />
            <ClassName name={"Biology Laboratory"} class_code={"BIOL023"} />

            <ClassName name={"Introduction to Academic Writing"} class_code={"WRIT001"} />
        </>
    }
}

/// Component for displaying my classes at Nueva
#[function_component(NuevaSchoolClasses)]
fn nueva_school_classes() -> Html {
    let category_text = css!("font-weight: bold; margin-top: 1rem;");

    html! {
        <>
            <p> {"I was incredibly fortunate to attend Nueva high school where I was able to take a wide variety of math, computer science, and physics courses, sparking my interest in all three areas."} </p>
            <p > {"Here are the classes I took at Nueva: "} </p>
            <p class={category_text.clone()}> {"Computer Science"} </p>
            <ClassName name={"Game Design Studio"} class_code={"CS390"} />
            <ClassName name={"Advanced Machine Learning"} class_code={"CS321"} />
            <ClassName name={"Intro to Machine Learning"} class_code={"CS320"} />
            <ClassName name={"Computer Vision"} class_code={"CS280"} />
            <ClassName name={"Introduction to Cryptocurrency"} class_code={"CS245"} />
            <ClassName name={"Computer Security"} class_code={"CS240"} />
            <ClassName name={"Introduction to Algorithms"} class_code={"CS223"} />
            <ClassName name={"Mobile App Design"} class_code={"CS120"} />
            <ClassName name={"Intro Computer Programming"} class_code={"CS110"} />

            <p class={category_text.clone()}> {"Mathematics"} </p>
            <ClassName name={"Quantum Information and Computation"} class_code={"MATH580"} />
            <ClassName name={"Differential Equations"} class_code={"MATH570"} />
            <ClassName name={"Advanced Probability"} class_code={"MATH565"} />
            <ClassName name={"Linear Algebra"} class_code={"MATH530"} />
            <ClassName name={"Multivariable Calculus"} class_code={"MATH520"} />
            <ClassName name={"Graph Theory"} class_code={"MATH435"} />
            <ClassName name={"Calculus"} class_code={"MATH401"} />
            <ClassName name={"Math 3 (Pre-Calculus)"} class_code={"MATH301"} />
            <ClassName name={"Math 2 (Algebra 2)"} class_code={"MATH201"} />
            <ClassName name={"Math 1A (Geometry)"} class_code={"MATH101A"} />

            <p class={category_text.clone()}> {"Physics"} </p>
            <ClassName name={"Advanced Mechanics"} class_code={"PHYS360"} />
            <ClassName name={"Modern Physics"} class_code={"PHYS250"} />
            <ClassName name={"OPTIMA"} class_code={"PHYS220"} />
            <ClassName name={"Physics"} class_code={"PHYS101"} />

            <p class={category_text.clone()}> {"Other"} </p>
            <ClassName name={"Intro to Engineering and Fabrication"} class_code={"EFD110"} />

            <ClassName name={"US History"} class_code={"HIST301"} />
            <ClassName name={"Global History: Modern World"} class_code={"HIST201"} />
            <ClassName name={"Global History: Modern Europe"} class_code={"HIST201"} />
            <ClassName name={"Global History: Ancient and Medieval Worlds"} class_code={"HIST101"} />
            <ClassName name={"History 9 - Europe, Americas"} class_code={"HIST101"} />

            <ClassName name={"Shakespeare in the World"} class_code={"ENG450"} />
            <ClassName name={"Advanced Literature Seminar (Satire)"} class_code={"ENG401"} />
            <ClassName name={"English 11"} class_code={"ENG301"} />
            <ClassName name={"English 10"} class_code={"ENG201"} />
            <ClassName name={"English 9"} class_code={"ENG101"} />

            <ClassName name={"Spanish 3"} class_code={"SPAN301"} />
            <ClassName name={"Spanish 2"} class_code={"SPAN201"} />
            <ClassName name={"Spanish 1"} class_code={"SPAN101"} />

            <ClassName name={"Biology"} class_code={"BIO101"} />
            <ClassName name={"Chemistry"} class_code={"CHEM101"} />

            <ClassName name={"Advanced Photography"} class_code={"FA270"} />
            <ClassName name={"Intro to Photography"} class_code={"FA170"} />


        </>
    }
}
