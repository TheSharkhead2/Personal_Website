use stylist::css;
use stylist::yew::{use_style, Global};
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod command_parsing;

use command_parsing::valid_command;

#[derive(Properties, PartialEq)]
pub struct CommandEnterLineProps {
    pub on_input: Callback<web_sys::InputEvent>,
    pub node_ref: NodeRef,
    pub on_key_enter: Callback<KeyboardEvent>,
}

/// Function component for text extry line where user puts in commands
#[function_component(CommandEnterLine)]
fn command_enter_line(props: &CommandEnterLineProps) -> Html {
    let input_node_ref = props.node_ref.clone();

    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let oninput = props.on_input.reform(move |e| e);
    let onkeyenter = props.on_key_enter.reform(move |e| e);

    let style_main = use_style!("color: var(--text-color-fifth); margin: 0px;");
    let style1 = use_style!("color: var(--text-color-third);");
    let style2 = use_style!("color: var(--text-color-secondary);");
    let style3 = use_style!("color: var(--text-color-quad);");

    let input_node_ref_clone = input_node_ref.clone();
    use_effect(move || {
        if let Some(input) = input_node_ref_clone.cast::<HtmlInputElement>() {
            input.focus();
        }
    });

    html!(
        <div class="command-single-line">
            <p class={style_main}>
                <span class={style1}>{"guest"}</span> // possibly temp, with user login this would change
                {"@"}
                <span class={style2}>{"theorode.com"}</span>
                {":"}
                <span class={style3}>{"~"}</span> // temp, change to dir later
                {"$"}
            </p>
            <div class="command-input">
                <label for="command-input">
                    <input ref={input_node_ref}
                        oninput={oninput}
                        onkeyup={onkeyenter}
                        id="command-input"
                        type="text"
                        class="command-input"
                        maxlength="100"
                    />
                </label>
            </div>
        </div>
    )
}

#[function_component(App)]
fn app() -> Html {
    let last_command_handle = use_state(String::default);
    let command_history_handle = use_mut_ref(Vec::<String>::new); // store history of commands
    let last_command = (*last_command_handle).clone();
    let command_history = (*command_history_handle).clone();
    let command_node_ref = use_node_ref();

    let oncommandinput = {
        let command_node_ref = command_node_ref.clone();
        let last_command_handle = last_command_handle.clone();

        Callback::from(move |_| {
            let input = command_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                last_command_handle.set(input.value());
            }
        })
    };

    let onkeyenter = {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let target = e.target_dyn_into::<HtmlInputElement>();
                if let Some(target) = target {
                    let input = target.value();
                    target.set_value("");
                    (*command_history_handle.borrow_mut()).push(input.clone());
                    last_command_handle.set(input);
                }
            }
        })
    };

    html! {
        <>
            <Global css={css!("
            :root {
                --text-color-main: rgb(200, 216, 230);
                --text-color-secondary: rgb(35, 92, 145);
                --text-color-third: rgb(78, 55, 135);
                --text-color-quad: rgb(43, 122, 99);
                --text-color-fifth: rgb(89, 98, 107);
                --text-color-incorrect: rgb(156, 31, 31);
                --background-color: rgb(36,37,38);
            }

            body {
                color: var(--text-color-main);
                border-style: solid;
                border-width: 2px;
                border-color: rgb(200, 216, 230);
                border-radius: 7px;
                margin: 5px;
                padding: 15px;
                flex-grow: 1; 
            }

            html {
                background-color: var(--background-color);
                height: 100vh;
                display: flex;
                alight-items: stretch;
                font-family: CascadiaCode,monospace;
            }

            div.main-screen {
                align-items: stretch;
                flex-direction: column;
                justify-content: flex-start;
            } 

            div.command-single-line {
                display: flex;
                flex-direction: row;
                justify-content: flex-start;
            }

            div.command-input {
                flex-direction: column;
                justify-content: center;
                margin-left: 5px;
                width: 100%;
            }

            input.command-input {
                width: 100%;
            }

            input.command-input {
                background-color: var(--background-color);
                border: none;
                color: var(--text-color-main);
                outline: none;
                font-size: 1rem;
                font-family: CascadiaCode, monospace;
            }

        ")}/>
            <div class="main-screen">
                <h1>{ "Hello World!" }</h1>
                <CommandEnterLine
                    on_input={oncommandinput}
                    node_ref = {command_node_ref}
                    on_key_enter = {onkeyenter}
                />
                <p> {command_history.borrow().clone()}</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
