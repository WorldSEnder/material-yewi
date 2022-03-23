use material_styles_yew::use_theme;
use material_styles_yew::ShadowSpec;
use material_styles_yew::Theme;
use stylist::ast::ScopeContent;
use stylist::ast::{sheet, Sheet};
use stylist::yew::use_style;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Children;
use yew::Html;
use yew::Properties;

#[derive(Debug, Clone, PartialEq)]
pub struct PaperStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for PaperStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PaperVariant {
    Outlined,
    Elevated(u8),
}

impl Default for PaperVariant {
    fn default() -> Self {
        Self::Elevated(1)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PaperEdgeStyle {
    Square,
    Rounded,
}

impl Default for PaperEdgeStyle {
    fn default() -> Self {
        Self::Rounded
    }
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct PaperProperties {
    #[prop_or_default]
    pub class: Sheet,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: PaperVariant,
    #[prop_or_default]
    pub edge_style: PaperEdgeStyle,
}

struct DefaultStyles {
    root_style: Sheet,
    rounded_style: Sheet,
    outlined_style: Sheet,
    shadows: [ShadowSpec; 25],
    root_override: Sheet,
}

fn derive_styles_from_theme(theme: &Theme) -> DefaultStyles {
    // TODO: add color options
    let root_style = sheet!(
        background-color: ${theme.palette.background.paper};
        color: ${theme.palette.text.primary};
        transition: box-shadow 300ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
    );
    let rounded_style = sheet!(
        border-radius: ${&theme.shape.border_radius};
    );
    let outlined_style = sheet!(
        border: 1px solid${" "}${&theme.palette.divider};
    );
    // TODO: "dark" mode, gradients in background, etc...

    let root_override = theme
        .components
        .search_override::<PaperStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();

    DefaultStyles {
        root_style,
        rounded_style,
        outlined_style,
        shadows: theme.shadows.0.clone(),
        root_override,
    }
}

impl DefaultStyles {
    fn build_root_style(&self, props: &PaperProperties) -> Vec<ScopeContent> {
        use PaperEdgeStyle::*;
        use PaperVariant::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(&self.root_style);
        match props.edge_style {
            Rounded => collected_scopes.extend_from_slice(&self.rounded_style),
            Square => {}
        };
        match props.variant {
            Outlined => collected_scopes.extend_from_slice(&self.outlined_style),
            Elevated(hgt) if hgt < 25 => collected_scopes.extend_from_slice(&sheet!(
                box-shadow: ${&self.shadows[hgt as usize]};
            )),
            // FIXME: panic in debug?
            Elevated(_illegal) => collected_scopes.extend_from_slice(&sheet!(
                box-shadow: ${&self.shadows[24]};
            )),
        };
        collected_scopes.extend_from_slice(&self.root_override);

        collected_scopes
    }
}

#[function_component]
pub fn Paper(props: &PaperProperties) -> Html {
    let styles = use_theme(derive_styles_from_theme);

    let mut root_style = styles.build_root_style(props);
    root_style.extend_from_slice(&props.class);
    let root_style = Sheet::from(root_style);
    let root_style = use_style(/* ""Mwi-paper-root", */ root_style);

    html! {
        <div class={classes![root_style]}>
            { for props.children.iter() }
        </div>
    }
}
