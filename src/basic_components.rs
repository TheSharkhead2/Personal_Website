use stylist::yew::use_style;
use yew::prelude::*;

/// Component for the user information and whatnot before the command input section
#[function_component(UsernameText)]
pub fn username_text() -> Html {
    // define basic color styles for use here
    let style_main = use_style!("color: var(--text-color-fifth); margin: 0px;");
    let style1 = use_style!("color: var(--text-color-third);");
    let style2 = use_style!("color: var(--text-color-secondary);");
    let style3 = use_style!("color: var(--text-color-quad);");

    html! {
        <p class={style_main}>
            <span class={style1}>{"guest"}</span> // possibly temp, with user login this would change
            {"@"}
            <span class={style2}>{"theorode.com"}</span>
            {":"}
            <span class={style3}>{"~"}</span> // temp, change to dir later
            {"$"}
        </p>
    }
}
