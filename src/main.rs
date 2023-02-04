use stylist::css;
use stylist::yew::{use_style, Global};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CommandEnterLineProps {
    pub on_command_entry: Callback<web_sys::Event>,
}

/// Function component for text extry line where user puts in commands
#[function_component(CommandEnterLine)]
fn command_enter_line(props: &CommandEnterLineProps) -> Html {
    let input_node_ref = use_node_ref();

    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let oninput = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let onchange = props.on_command_entry.reform(move |e| e);

    let style_main = use_style!("color: var(--text-color-fifth);");
    let style1 = use_style!("color: var(--text-color-third);");
    let style2 = use_style!("color: var(--text-color-secondary);");
    let style3 = use_style!("color: var(--text-color-quad);");

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
            <label for="command-input">
                <input ref={input_node_ref}
                    oninput={oninput}
                    onchange={onchange}
                    id="command-input"
                    type="text"
                    value={input_value.clone()}
                />
            </label>
            <p> {input_value.clone()} </p>
        </div>
    )
}

#[function_component(App)]
fn app() -> Html {
    let last_command_handle = use_state(String::default);
    let last_command = (*last_command_handle).clone();

    let on_command_entry: Callback<web_sys::Event> = {
        let last_command_handle = last_command_handle.clone();

        Callback::from(move |command: Event| {
            let input = command.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                last_command_handle.set(input.value());
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
                background-color: rgb(36, 37, 38);
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

            div.command-single=line {

            }

        ")}/>
            <div class="main-screen">
                <h1>{ "Hello World!" }</h1>
                <CommandEnterLine {on_command_entry}/>
                <p> {last_command.clone()}</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
