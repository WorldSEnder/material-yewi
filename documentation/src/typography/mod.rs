use material_yewi::typography::{Typography, TypographyVariant};
use material_yewi_documentation_macros::document_example;
use yew::prelude::*;

#[function_component]
pub fn Doc() -> Html {
    let example = document_example! {"example.rs"};
    html! {
        <>
            <Typography variant={TypographyVariant::H2}>
                {"Typography"}
            </Typography>
            <Typography variant={TypographyVariant::Paragraph}>
                {"Below you can find examples showing the "}<pre style="display:inline;">{"Typography"}</pre>{" component."}
            </Typography>
            {example}
        </>
    }
}
