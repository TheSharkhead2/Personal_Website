use stylist::css;
use yew::prelude::*;

use crate::basic_components::UsernameText;

/// Props object for the about command
#[derive(Properties, PartialEq)]
pub struct UserCommandAboutProps {
    pub command_text: String,
}

/// function component for the content of the about command
#[function_component(UserCommandAbout)]
pub fn user_command_about(props: &UserCommandAboutProps) -> Html {
    let text_color = css!("color: var(--text-color-main); margin: 0px; margin-left: 2px;");
    let paragraph_break = css!("margin-top: 1rem;");

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
            // <p>
            //     {"Hello, my name is Theo. I am currently studying at Harvey Mudd College (class of 2026) and I am pursing degrees in both Computer Science and Math."}
            // </p>
            // <p class={paragraph_break.clone()}>
            //     {"I love to explore the world in order to find patterns and understanding. My passions for Computer Science and Math allow me to explore the world in this way."}
            // </p>
            // <p class={paragraph_break.clone()}>
            //     {"Math is the language of discovery. I find it one of the greatest feelings when I finish the string of logic that completes a proof, and this drives my passion for math.
            //     It it amazing to me how math can so perfectly describe patterns and connections, and how it can silmultaneously outline the tools through which you can utilize those patterns."}
            // </p>
            // <p class={paragraph_break.clone()}>
            //     {"Computer science allows me to harness the power of math and build logic into anything I want. I can answer questions with simulations or data analysis and I can
            //     solve problems by automating tasks or building tools. It is this power to see a problem and have the ability to pursue a solution to it that draws me to computer science."}
            // </p>
            // <p class={paragraph_break.clone()}>
            //     {"While I spend a lot of my time thinking about and playing around with math and computer science, I bet I am sounding a bit to academic (I am sorry, but I can't help it).
            //     I also love to play tennis and try to get on the ski hill for at least a few days every year. I quite enjoy the art of magic (the sleight of hand variety) and I used to
            //     perform at parties and family gatherings. Other things I am interested in are puzzles, Star Wars (Disney movies don't count), and sharks."}
            // </p>
            // <p class={paragraph_break}>
            //     {"If I have somehow interested you feel free to reach out! I would love to talk about anything here or just learn about what you love. It is amazing to talk with people about their passions and I will never pass up that opportunity."}
            // </p>

            <p>
                {"Hello, my name is Theo and I am currently studying at Harvey Mudd College (class of 2026). I am pursing degrees in both computer science and math. I love to explore the world of academics largely, but when I am writing a proof or building an efficient algorithm, I feel the most at home. This is so much so that I spend a lot of my time outside of school working on programming projects relating to topics I find interesting (see 'projects'). I was also a performing magician a few years ago and I love puzzles, skiing, and tennis (plus I play a few video games here and there)."}
            </p>
            <p>
                {"Feel free to look around at what I have here and if you have any questions, or just want to talk about something interesting, contact me! (Here is an email: theorodester@gmail.com)"}
            </p>
        </div>
    }
}
