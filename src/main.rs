use stylist::yew::{use_style, Global};
use stylist::{css, style};
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod basic_components;
mod command_parsing;

use basic_components::UsernameText;
use command_parsing::{valid_command, PreviousCommands};

#[derive(Properties, PartialEq)]
pub struct CommandEnterLineProps {
    pub on_input: Callback<web_sys::InputEvent>,
    pub node_ref: NodeRef,
    pub on_key_enter: Callback<KeyboardEvent>,
    pub input_style: String,
}

/// Function component for text extry line where user puts in commands
#[function_component(CommandEnterLine)]
fn command_enter_line(props: &CommandEnterLineProps) -> Html {
    let input_node_ref = props.node_ref.clone();

    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let oninput = props.on_input.reform(move |e| e);
    let onkeyenter = props.on_key_enter.reform(move |e| e);

    let input_node_ref_clone = input_node_ref.clone();
    use_effect(move || {
        if let Some(input) = input_node_ref_clone.cast::<HtmlInputElement>() {
            input.focus();
        }
    });

    html!(
        <div class="command-single-line">
            <UsernameText/>
            <div class="command-input">
                <label for="command-input">
                    <input ref={input_node_ref}
                        oninput={oninput}
                        onkeyup={onkeyenter}
                        id="command-input"
                        type="text"
                        class={&props.input_style.clone()}
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
    let command_history_handle = use_mut_ref(|| vec![String::from("head")]); // store history of commands and start with head command "already run"
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

    // text coloring if the command is invalid
    let input_text_style = if valid_command(&last_command) || last_command == String::new() {
        String::from("command-input")
    } else {
        String::from("command-input-incorrect")
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
                height: 100%;
                overflow-y: auto;
                overflow-x: hidden;
                bottom: 0px;
            } 

            div.command-single-line {
                display: flex;
                flex-direction: row;
                justify-content: flex-start;
                align-items: center;
            }

            div.command-input {
                flex-direction: column;
                justify-content: center;
                margin-left: 5px;
                width: 100%;
            }

            input.command-input {
                background-color: var(--background-color);
                border: none;
                color: var(--text-color-main);
                outline: none;
                font-size: 1rem;
                font-family: CascadiaCode, monospace;
                width: 100%;
                padding: 0px;
                padding-left: 2px;
            }

            input.command-input-incorrect {
                background-color: var(--background-color);
                border: none;
                color: var(--text-color-incorrect);
                outline: none;
                font-size: 1rem;
                font-family: CascadiaCode, monospace;
                caret-color: var(--text-color-main);
                width: 100%;
                padding: 0px;
                padding-left: 2px;
            }

            div.user-command {
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                margin: 0px;
                margin-bottom: 1rem;
            }

            p {
                margin-top: 0px;
                margin-bottom: 0px;
            }

            ul.command-history {
                margin: 0px;
                padding: 0px;
            }

        ")}/>
            <div class="main-screen">
                <PreviousCommands command_history={command_history.borrow().clone()}/>
                <CommandEnterLine
                    on_input={oncommandinput}
                    node_ref = {command_node_ref}
                    on_key_enter = {onkeyenter}
                    input_style = {input_text_style}
                />
                // <p> {command_history.borrow().clone()}</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
