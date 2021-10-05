use material_styles_yew::use_theme;
use material_styles_yew::Theme;
use stylist::ast::ScopeContent;
use stylist::ast::{sheet, Sheet};
use yew::function_component;
use yew::html;
use yew::Children;
use yew::Properties;

use crate::paper::Paper;
use crate::paper::PaperEdgeStyle;
use crate::paper::PaperVariant;

#[derive(Debug, Clone, PartialEq)]
pub struct AppBarStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for AppBarStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AppBarPosition {
    Absolute,
    Fixed,
    Relative,
    Static,
    Sticky,
}

impl Default for AppBarPosition {
    fn default() -> Self {
        Self::Fixed
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AppBarColor {
    Primary,
    Secondary,
    Transparent,
    Inherit,
}

impl Default for AppBarColor {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct AppBarProperties {
    #[prop_or_default]
    pub class: Sheet,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub position: AppBarPosition,
    #[prop_or_default]
    pub color: AppBarColor,
}

struct DefaultStyles {
    root_style: Sheet,
    // position
    position_fixed: Sheet,
    position_absolute: Sheet,
    position_sticky: Sheet,
    position_relative: Sheet,
    position_static: Sheet,
    // color
    color_primary: Sheet,
    color_secondary: Sheet,
    color_transparent: Sheet,
    color_inherit: Sheet,
    // override
    root_override: Sheet,
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    let root_style = sheet!(
        display: flex;
        flex-direction: column;
        width: 100%;
        box-sizing: border-box;
        flex-shrink: 0;
    );
    let position_fixed = sheet!(
        position: fixed;
        top: 0;
        left: auto;
        right: 0;
        z-index: 1100; // TODO: put into theme
        @media print {
            position: absolute;
        }
    );
    let position_absolute = sheet!(
        position: absolute;
        top: 0;
        left: auto;
        right: 0;
        z-index: 1100; // TODO: put into theme
    );
    let position_sticky = sheet!(
        position: sticky;
        top: 0;
        left: auto;
        right: 0;
        z-index: 1100; // TODO: put into theme
    );
    let position_static = sheet!(
        position: static;
    );
    let position_relative = sheet!(
        position: relative;
    );
    let color_primary = sheet!(
        color: ${theme.palette.primary.contrast};
        background-color: ${theme.palette.primary.main};
    );
    let color_secondary = sheet!(
        color: ${theme.palette.secondary.contrast};
        background-color: ${theme.palette.secondary.main};
    );
    let color_inherit = sheet!(
        color: inherit;
    );
    let color_transparent = sheet!(
        color: inherit;
        background-color: transparent;
    );
    // TODO: implement dark mode

    let root_override = theme
        .components
        .search_override::<AppBarStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();

    DefaultStyles {
        root_style,
        position_fixed,
        position_absolute,
        position_sticky,
        position_static,
        position_relative,
        color_primary,
        color_secondary,
        color_inherit,
        color_transparent,
        root_override,
    }
}

impl DefaultStyles {
    fn build_root_style(&self, props: &AppBarProperties) -> Vec<ScopeContent> {
        use AppBarColor::*;
        use AppBarPosition::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(&self.root_style);
        collected_scopes.extend_from_slice(match props.position {
            Absolute => &self.position_absolute,
            Fixed => &self.position_fixed,
            Relative => &self.position_relative,
            Static => &self.position_static,
            Sticky => &self.position_sticky,
        });
        collected_scopes.extend_from_slice(match props.color {
            Primary => &self.color_primary,
            Secondary => &self.color_secondary,
            Transparent => &self.color_transparent,
            Inherit => &self.color_inherit,
        });
        collected_scopes.extend_from_slice(&self.root_override);

        collected_scopes
    }
}

#[function_component(AppBar)]
pub fn app_bar(props: &AppBarProperties) -> Html {
    let styles = use_theme(derive_styles_from_theme);

    let mut root_style = styles.build_root_style(props);
    root_style.extend_from_slice(&props.class);
    let root_style = Sheet::from(root_style);

    html! {
        <Paper class={root_style} variant={PaperVariant::Elevated(4)} edge_style={PaperEdgeStyle::Square}>
            { for props.children.iter() }
        </Paper>
    }
}
