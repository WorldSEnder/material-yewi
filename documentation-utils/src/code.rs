use material_styles_yew::use_theme;
use stylist::{ast::sheet, yew::use_style};
use yew::{classes, function_component, html, virtual_dom::AttrValue, Html, Properties};

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct CodeProps {
    pub snippet: AttrValue,
}

#[function_component]
pub fn Code(props: &CodeProps) -> Html {
    let frame_sheet = use_theme(|theme| {
        sheet!(
            box-shadow: ${&theme.shadows[2]};
            border: 0;
            direction: ltr;
            padding: 16px;
            margin: auto;
            display: flex;
            flex-grow: 1;
            background-color: #001E3C;
        )
    });
    let code_style = use_style(/* "demo", */ frame_sheet.clone());

    return html! {
        <pre class={classes![code_style]}><code class="language-rust">{props.snippet.as_ref()}</code></pre>
    };
}
