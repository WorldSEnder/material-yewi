use material_styles_yew::use_theme;
use material_styles_yew::Theme;
use stylist::ast::{sheet, Sheet};
use stylist::yew::use_style;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Children;
use yew::Properties;
use yew_router::components::Link as RawLink;
use yew_router::Routable;

use crate::typography::{Typography, TypographyVariant};

#[derive(Debug, Clone, PartialEq)]
pub struct LinkStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for LinkStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Underline {
    Always,
    Hover,
    None,
}

impl Default for Underline {
    fn default() -> Self {
        Self::Hover
    }
}
#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct LinkProperties<R: PartialEq> {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub underline: Underline,
    #[prop_or(TypographyVariant::Button)]
    pub variant: TypographyVariant,
    pub route: R,
}

struct DefaultStyles {
    always_underline: Sheet,
    hover_underline: Sheet,
    no_underline: Sheet,
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    // TODO: add color options
    let color = theme.palette.text.primary;

    let always_underline = sheet!(
        text-decoration: underline;
        text-decoration-color: ${color.alpha_multiply(0.4)};
        &:hover {
            text-decoration-color: inherit;
        }
    );
    let hover_underline = sheet!(
        text-decoration: none;
        &:hover {
            text-decoration: underline;
        }
    );
    let no_underline = sheet!(
        text-decoration: none;
    );

    DefaultStyles {
        always_underline,
        hover_underline,
        no_underline,
    }
}

impl DefaultStyles {
    fn build_root_style<R: PartialEq>(&self, props: &LinkProperties<R>) -> Sheet {
        use Underline::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(match props.underline {
            Always => &self.always_underline,
            Hover => &self.hover_underline,
            None => &self.no_underline,
        });

        Sheet::from(collected_scopes)
    }
}

#[function_component(Link)]
pub fn link<R: Routable + Clone + PartialEq + 'static>(props: &LinkProperties<R>) -> Html {
    let styles = use_theme(derive_styles_from_theme);

    let root_style = use_style(/* "Mwi-link-root", */ styles.build_root_style(props));

    html! {
        <RawLink<R> route={props.route.clone()} classes={classes![root_style]}>
            <Typography variant={props.variant}>
                {for props.children.iter()}
            </Typography>
        </RawLink<R>>
    }
}
