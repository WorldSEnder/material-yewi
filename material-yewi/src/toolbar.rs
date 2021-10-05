use material_styles_yew::use_theme;
use material_styles_yew::Theme;
use stylist::ast::ScopeContent;
use stylist::ast::{sheet, Sheet};
use stylist::yew::use_style;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Children;
use yew::Properties;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolbarStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for ToolbarStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ToolbarVariant {
    Dense,
    Regular,
}

impl Default for ToolbarVariant {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ToolbarGutters {
    Enabled,
    Disabled,
}

impl Default for ToolbarGutters {
    fn default() -> Self {
        Self::Enabled
    }
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct ToolbarProperties {
    #[prop_or_default]
    pub class: Sheet,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: ToolbarVariant,
    #[prop_or_default]
    pub gutters: ToolbarGutters,
}

struct DefaultStyles {
    root_style: Sheet,
    gutters_enabled: Sheet,
    variant_dense: Sheet,
    variant_regular: Sheet,
    root_override: Sheet,
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    let root_style = sheet!(
        position: relative;
        display: flex;
        align-items: center;
    );
    let spacing2 = "16px"; // TODO: put into theme
    let spacing3 = "16px";
    let small_break = "(min-width:600px)";
    let gutters_enabled = sheet!(
        padding-left: ${&spacing2};
        padding-right: ${&spacing2};
        @media ${small_break} {
            padding-left: ${&spacing3};
            padding-right: ${&spacing3};
        }
    );
    let variant_dense = sheet!(
        min-height: 48px;
    );
    let variant_regular = sheet!(
        min-height: 56px;
        @media (min-width:0px) and (orientation: landscape) {
            min-height:48px;
        }
        @media (min-width:600px) {
            min-height:64px;
        }
    );

    let root_override = theme
        .components
        .search_override::<ToolbarStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();

    DefaultStyles {
        root_style,
        gutters_enabled,
        variant_dense,
        variant_regular,
        root_override,
    }
}

impl DefaultStyles {
    fn build_root_style(&self, props: &ToolbarProperties) -> Vec<ScopeContent> {
        use ToolbarGutters::*;
        use ToolbarVariant::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(&self.root_style);
        collected_scopes.extend_from_slice(match props.gutters {
            Enabled => &self.gutters_enabled,
            Disabled => &[],
        });
        collected_scopes.extend_from_slice(match props.variant {
            Dense => &self.variant_dense,
            Regular => &self.variant_regular,
        });
        collected_scopes.extend_from_slice(&self.root_override);

        collected_scopes
    }
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProperties) -> Html {
    let styles = use_theme(derive_styles_from_theme);

    let mut root_style = styles.build_root_style(props);
    root_style.extend_from_slice(&props.class);
    let root_style = Sheet::from(root_style);
    let root_style = use_style(/* ""Mwi-toolbar-root", */ root_style);

    html! {
        <div class={classes![root_style]}>
            { for props.children.iter() }
        </div>
    }
}
