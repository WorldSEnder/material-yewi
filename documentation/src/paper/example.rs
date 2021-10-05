use material_yewi::paper::{Paper, PaperEdgeStyle, PaperVariant};
use material_yewi::typography::Typography;
use yew::{html, Html};

pub fn render() -> Html {
    html! {
        <PaperListing>
            <Paper variant={PaperVariant::Outlined}>
                <Typography>{"Outlined"}</Typography>
            </Paper>
            <Paper>
                <Typography>{"Default"}</Typography>
            </Paper>
            <Paper variant={PaperVariant::Elevated(3)}>
                <Typography>{"Elevated(3)"}</Typography>
            </Paper>
            <Paper variant={PaperVariant::Elevated(12)}>
                <Typography>{"Elevated(12)"}</Typography>
            </Paper>
            <Paper variant={PaperVariant::Elevated(24)}>
                <Typography>{"Elevated(24)"}</Typography>
            </Paper>
            <Paper edge_style={PaperEdgeStyle::Square}>
                <Typography>{"Square"}</Typography>
            </Paper>
        </PaperListing>
    }
}

use yew::{classes, function_component, Children, Properties};

#[derive(Clone, PartialEq, Properties)]
struct WrapperProps {
    children: Children,
}

#[function_component(PaperListing)]
fn paper_listing(props: &WrapperProps) -> Html {
    // A small utility component for documentation, renders as div
    let wrapper_class = stylist::yew::use_style!(/* "paper-listing", */
        max-width: 500px;
        display: flex;
        flex-wrap: wrap;
        & > :not(style) {
            margin: 8px;
            width: 128px;
            height: 128px;
            text-align: center;
            line-height: 128px;
        }
        &${" "}p { line-height: inherit; }
    );
    html! {
        <div class={classes![wrapper_class]}>
            { for props.children.iter() }
        </div>
    }
}
