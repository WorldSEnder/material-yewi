use material_yewi::typography::{Typography, TypographyVariant};
use material_yewi_documentation_macros::document_example;
use yew::prelude::*;

#[function_component(Doc)]
pub fn doc() -> Html {
    let example_text = document_example! {"example_text.rs"};
    let example_contained = document_example! {"example_contained.rs"};
    let example_vertical = document_example! {"example_vertical.rs"};
    html! {
        <>
            <Typography variant={TypographyVariant::H2}>
                {"Button Groups"}
            </Typography>
            <Typography variant={TypographyVariant::Paragraph}>
                {"Below you can find examples showing the "}<pre style="display:inline;">{"ButtonGroup"}</pre>{" component."}
            </Typography>
            {example_text}
            {example_contained}
            {example_vertical}
        </>
    }
}
