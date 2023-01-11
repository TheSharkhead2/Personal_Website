use stylist::css;
use stylist::yew::{Global, use_style};
use yew::prelude::*;
use web_sys::HtmlInputElement;

/// Function component for text extry line where user puts in commands
#[function_component(CommandEnterLine)]
fn command_enter_line() -> Html {
    let input_node_ref = use_node_ref();

    let current_value: String = "".to_string();

    let onchange = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                // do something with value
                current_value = value;
            }
        })
    };

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
                    {onchange}
                    id="command-input"
                    type="text"
                />
            </label>
        </div>
    )
}

#[function_component(App)]
fn app() -> Html {
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
                <CommandEnterLine/>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}