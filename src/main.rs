use gloo_console::log;
use stylist::yew::Global;
use stylist::{css, Style};
use web_sys::{window, Element, HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

mod about;
mod auto_complete;
mod basic_components;
mod command_parsing;
mod echo;
mod education;
mod github;
mod head;
mod help;
mod linkedin;
mod projects;
mod resume;

use auto_complete::auto_complete;
use basic_components::UsernameText;
use command_parsing::{valid_command, PreviousCommands};

#[derive(Properties, PartialEq)]
pub struct CommandEnterLineProps {
    pub on_input: Callback<web_sys::InputEvent>,
    pub node_ref: NodeRef,
    pub on_key_enter: Callback<KeyboardEvent>,
    pub input_style: String,
    pub current_value: String,
    pub autocomplete_text: Option<String>,
}

/// Function component for text extry line where user puts in commands
#[function_component(CommandEnterLine)]
fn command_enter_line(props: &CommandEnterLineProps) -> Html {
    let input_node_ref = props.node_ref.clone();

    let oninput = props.on_input.reform(move |e| e);
    let onkeyenter = props.on_key_enter.reform(move |e| e);

    let input_node_ref_clone = input_node_ref.clone();
    use_effect(move || {
        if let Some(input) = input_node_ref_clone.cast::<HtmlInputElement>() {
            input.focus().ok();
        }
    });

    // prevent up and down arrow from changing place of cursor
    let onkeydown = {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "ArrowUp" || e.key() == "ArrowDown" {
                e.prevent_default();
            }
        })
    };

    let autocomplete_text = match &props.autocomplete_text {
        Some(text) => String::from(text),
        None => String::from(""),
    };

    let difference_from_actual =
        (props.current_value.len() as i32) - (autocomplete_text.len() as i32);

    let autocomplete_hangover_text = if difference_from_actual < 0 {
        String::from(&autocomplete_text[(props.current_value.len())..])
    } else {
        String::from("")
    };

    let overlapping_div_class = css!("margin: 0px; position: relative; width: 100%; height: 100%;");
    let autocomplete_div_class = css!("position: absolute; left: 2px; top: 0px; margin: 0px;");

    let mut margin_change = (difference_from_actual) + (autocomplete_text.len() as i32);

    // add space if string starts with space because p tag removes leading spaces
    if autocomplete_hangover_text.len() > 0 {
        if &autocomplete_hangover_text[..1] == " " {
            margin_change += 1;
        }
    }

    let p_autocomplete_class_string = format!(
        "left: {}ch; position: absolute; margin: 0px; width: 100vw; color: var(--text-color-main-transparent-rec);",
        margin_change.to_string()
    );

    let overlapping_p_autocomplete_text_class = Style::new(p_autocomplete_class_string).unwrap();

    html!(
        <div class="command-single-line">
            <UsernameText/>
            <div class="command-input">
                <div class={overlapping_div_class}>
                    <div class={autocomplete_div_class}>
                        <p class={overlapping_p_autocomplete_text_class}>
                            {autocomplete_hangover_text}
                        </p>
                    </div>
                    <label for="command-input">
                        <input ref={input_node_ref}
                            oninput={oninput}
                            onkeyup={onkeyenter}
                            onkeydown={onkeydown}
                            id="command-input"
                            type="text"
                            class={&props.input_style.clone()}
                            maxlength="100"
                        />
                    </label>
                </div>
            </div>
        </div>
    )
}

#[function_component(App)]
fn app() -> Html {
    let last_command_handle = use_state(String::default);
    let autocomplete_result_handle = use_state(Option::<String>::default);
    let arrow_keys_command_history_index_handle = use_state(|| 0); // which index in the history the up/down arrow actions are
    let entire_command_history_handle = use_mut_ref(Vec::<String>::new); // entire command history that can't be wiped
    let command_history_handle = use_mut_ref(|| vec![String::from("head")]); // store history of commands and start with head command "already run"
    let last_command = (*last_command_handle).clone();
    let arrow_keys_command_history_index = *arrow_keys_command_history_index_handle;
    let command_history = (*command_history_handle).clone();
    let entire_command_history = (*entire_command_history_handle).clone();
    let command_node_ref = use_node_ref();

    // focus input element when clicking on screen
    let on_screen_click = {
        let input_node_ref = command_node_ref.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                input.focus().ok();
            }
        })
    };

    let onclearcommand = {
        let command_history_handle = command_history_handle.clone();
        let last_command_handle = last_command_handle.clone();

        Callback::from(move |_: bool| {
            *command_history_handle.borrow_mut() = vec![];
            last_command_handle.set(String::from("clear"));
        })
    };

    let oncommandinput = {
        let command_node_ref = command_node_ref.clone();
        let last_command_handle = last_command_handle.clone();
        let autocomplete_result_handle = autocomplete_result_handle.clone();

        Callback::from(move |_| {
            let input = command_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                autocomplete_result_handle.set(auto_complete(input.value().clone()));

                last_command_handle.set(input.value());
            }
        })
    };

    let onkeyenter = {
        let command_node_ref = command_node_ref.clone();
        let last_command_handle = last_command_handle.clone();

        let autocomplete_result_handle = autocomplete_result_handle.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let target = e.target_dyn_into::<HtmlInputElement>();
                if let Some(target) = target {
                    let input = target.value();
                    target.set_value("");
                    if input.split_whitespace().next().is_some() {
                        // if last command is same don't save it
                        if entire_command_history_handle.borrow().last() != Some(&input) {
                            (*entire_command_history_handle.borrow_mut()).push(input.clone());
                            arrow_keys_command_history_index_handle
                                .set(entire_command_history.borrow().len() + 1);
                            // increment the up/down arrow thingy by 1
                        } else {
                            arrow_keys_command_history_index_handle
                                .set(entire_command_history.borrow().len());
                        }

                        // always push to total history as this is rendered off of
                        (*command_history_handle.borrow_mut()).push(input.clone());

                        // check to see if a link needs to be openned with this command
                        let command = input.split_whitespace().next().unwrap(); // already know this is something
                        let window = window();
                        if let Some(window) = window {
                            if command == "github" {
                                window
                                    .open_with_url_and_target(
                                        "https://github.com/TheSharkhead2",
                                        "_blank",
                                    )
                                    .unwrap(); // don't really care if this errors
                            } else if command == "linkedin" {
                                window
                                    .open_with_url_and_target(
                                        "https://www.linkedin.com/in/theorode/",
                                        "_blank",
                                    )
                                    .unwrap();
                            // don't really care if this errors
                            } else if command == "resume" {
                                window
                                    .open_with_url_and_target(
                                        "https://github.com/TheSharkhead2/Resume/blob/main/resume.pdf",
                                        "_blank",
                                    )
                                    .unwrap(); // don't really care if this errors
                            }
                        }

                        // update this in order to update render (without this it won't re-render)
                        last_command_handle.set(input);
                    }
                }

                // scroll to new element added from command
                let command_ref = command_node_ref.cast::<Element>();
                if let Some(command_ref) = command_ref {
                    command_ref.scroll_into_view_with_bool(false);
                }
            } else if e.key() == "ArrowUp" {
                // don't run this logic if at top of history
                if arrow_keys_command_history_index > 0 {
                    let target = e.target_dyn_into::<HtmlInputElement>();
                    if let Some(target) = target {
                        target.set_value(
                            &entire_command_history.borrow()[arrow_keys_command_history_index - 1]
                                [..],
                        ); // set input box value
                        last_command_handle.set(
                            entire_command_history.borrow()[arrow_keys_command_history_index - 1]
                                .clone(),
                        );
                        arrow_keys_command_history_index_handle
                            .set(arrow_keys_command_history_index - 1); // decrement
                    }
                }
            } else if e.key() == "ArrowDown" {
                // only run if not at last index
                if arrow_keys_command_history_index < entire_command_history.borrow().len() - 1 {
                    let target = e.target_dyn_into::<HtmlInputElement>();
                    if let Some(target) = target {
                        target.set_value(
                            &entire_command_history.borrow()[arrow_keys_command_history_index + 1]
                                [..],
                        );
                        last_command_handle.set(
                            entire_command_history.borrow()[arrow_keys_command_history_index + 1]
                                .clone(),
                        );
                        arrow_keys_command_history_index_handle
                            .set(arrow_keys_command_history_index + 1);
                    }
                }
            } else if e.key() == "Tab" {
                let target = e.target_dyn_into::<HtmlInputElement>(); // cast to input element to set and get current text
                if let Some(target) = target {
                    let input = target.value();

                    // if the auto complete returns a suggestion, update the input box
                    if let Some(autocomplete_suggestion) = auto_complete(input) {
                        target.set_value(&autocomplete_suggestion[..]);
                        last_command_handle.set(autocomplete_suggestion);
                    }
                }
            }

            // reset autocomplete
            let input = command_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                autocomplete_result_handle.set(auto_complete(input.value().clone()));

                last_command_handle.set(input.value());
            }
        })
    };

    // text coloring if the command is invalid
    let input_text_style = if valid_command(&last_command)
        || last_command == String::new()
        || auto_complete(last_command.clone()).is_some()
    {
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
                --text-color-main-transparent: rgba(200, 216, 230, 0.5);
                --text-color-main-transparent-rec: rgba(200, 216, 230, 0.4);
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
                overflow: hidden;
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
                padding-left: 2px;
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
                margin-top: 1px;
                margin-bottom: 1px;
            }

            ul.command-history {
                margin: 0px;
                padding: 0px;
            }

        ")}/>
            <div class="main-screen" onclick={on_screen_click}>
                <PreviousCommands
                    command_history={command_history.borrow().clone()}
                    clear_command_callback={onclearcommand}
                />

                <CommandEnterLine
                    on_input={oncommandinput}
                    node_ref = {command_node_ref}
                    on_key_enter = {onkeyenter}
                    input_style = {input_text_style}
                    current_value = {(*last_command_handle).clone()}
                    autocomplete_text = {(*autocomplete_result_handle).clone()}
                />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
