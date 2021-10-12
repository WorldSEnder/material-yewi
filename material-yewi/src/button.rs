use material_styles_yew::use_theme;
use material_styles_yew::CssColor;
use material_styles_yew::Theme;
use stylist::ast::{sheet, ScopeContent, Sheet};
use yew::function_component;
use yew::html;
use yew::Callback;
use yew::Children;
use yew::Properties;

pub use crate::button_base::ButtonPressedEvent;
use crate::button_base::{ButtonBase, CLASS_DISABLED, CLASS_FOCUS_VISIBLE};
// FIXME: ripple effects

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for ButtonStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ButtonColor {
    Primary,
    Secondary,
    Inherit,
}

impl Default for ButtonColor {
    fn default() -> Self {
        ButtonColor::Primary
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ButtonVariant {
    Text,
    Outlined,
    Contained,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Text
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Medium
    }
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct ButtonProperties {
    #[prop_or_default]
    pub class: Sheet,
    #[prop_or_default]
    pub children: Children,
    /// Event fired when the button is considered pressed.
    /// This currently only includes clicking it, but would be expanded to keyboard events Enter and Spacebar
    ///  for screen reading compatibility. Also works correctly with Touch events on devices without a pointer.
    #[prop_or_default]
    pub on_pressed: Callback<ButtonPressedEvent>,
    #[prop_or_default]
    pub color: ButtonColor,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or(false)]
    pub disabled: bool,
}

struct DefaultStyles {
    root_inline: Sheet,
    // sizing
    size_text_small: Sheet,
    size_text_medium: Sheet,
    size_text_large: Sheet,
    size_outlined_small: Sheet,
    size_outlined_medium: Sheet,
    size_outlined_large: Sheet,
    size_contained_small: Sheet,
    size_contained_medium: Sheet,
    size_contained_large: Sheet,
    // coloring
    color_text_inherit: Sheet,
    color_text_primary: Sheet,
    color_text_secondary: Sheet,
    color_outlined_inherit: Sheet,
    color_outlined_primary: Sheet,
    color_outlined_secondary: Sheet,
    color_contained_inherit: Sheet,
    color_contained_primary: Sheet,
    color_contained_secondary: Sheet,
    // disabled
    outlined: Sheet,
    contained: Sheet,
    // overrides
    root_override: Sheet,
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    // FIXME: push into theme
    let disabled_color = "rgba(0, 0, 0, 0.26)".to_string();
    let disabled_background_color = "rgba(0, 0, 0, 0.12)".to_string();
    let gray_300: CssColor = CssColor::rgb(0xe0, 0xe0, 0xe0);
    let gray_a100: CssColor = CssColor::rgb(0xd5, 0xd5, 0xd5);

    let shadows2 = &theme.shadows[2];
    let shadows4 = &theme.shadows[4];
    let shadows6 = &theme.shadows[6];
    let shadows8 = &theme.shadows[8];

    // FIXME: transition from theme
    let root_basebox = sheet!(
        min-width: 64px;
        border-radius: ${theme.shape.border_radius.clone()};
        transition:
            background-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
            box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
            border-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
            color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
        &.${CLASS_DISABLED} {
            color: ${&disabled_color};
        }
    );
    let root_hover = sheet!(
        &:hover { text-decoration: none; }
    );
    let mut root_inline = vec![];
    root_inline.extend_from_slice(&theme.typography.button);
    root_inline.extend_from_slice(&root_basebox);
    root_inline.extend_from_slice(&root_hover);
    let root_inline = Sheet::from(root_inline);

    let size_text_small = sheet!(
        padding: 4px 5px;
        font-size: ${theme.typography.pixels_to_rem(13.0)};
    );
    let size_text_medium = sheet!(
        padding: 6px 8px;
    );
    let size_text_large = sheet!(
        padding: 8px 11px;
        font-size: ${theme.typography.pixels_to_rem(15.0)};
    );

    let size_outlined_small = sheet!(
        padding: 3px 9px;
        font-size: ${theme.typography.pixels_to_rem(13.0)};
    );
    let size_outlined_medium = sheet!(
        padding: 5px 15px;
    );
    let size_outlined_large = sheet!(
        padding: 7px 21px;
        font-size: ${theme.typography.pixels_to_rem(15.0)};
    );

    let size_contained_small = sheet!(
        padding: 4px 10px;
        font-size: ${theme.typography.pixels_to_rem(13.0)};
    );
    let size_contained_medium = sheet!(
        padding: 6px 16px;
    );
    let size_contained_large = sheet!(
        padding: 8px 22px;
        font-size: ${theme.typography.pixels_to_rem(15.0)};
    );

    let outlined = sheet!(
        &.${CLASS_DISABLED} {
            border: 1px solid ${" "}${&disabled_background_color};
        }
    );
    let contained = sheet!(
        &.${CLASS_DISABLED} {
            color: ${&disabled_color};
            box-shadow: none;
            background-color: ${&disabled_background_color};
        }
        &.${CLASS_FOCUS_VISIBLE} {
            box-shadow: ${shadows6};
        }
    );

    let to_hover = |c: CssColor| c.alpha_multiply(theme.palette.actions.hover_opacity);
    let color_primary = &theme.palette.primary;
    let color_secondary = &theme.palette.secondary;

    let color_text_inherit = sheet!(
        color: inherit;
        &:hover {
            background-color: ${to_hover(theme.palette.text.primary)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
    );
    let color_text_primary = sheet!(
        color: ${color_primary.main};
        &:hover {
            background-color: ${to_hover(color_primary.main)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
    );
    let color_text_secondary = sheet!(
        color: ${color_secondary.main};
        &:hover {
            background-color: ${to_hover(color_secondary.main)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
    );

    let color_outlined_inherit = sheet!(
        color: inherit;
        border: 1px solid ${" "}${CssColor::rgba(0, 0, 0, 0.32)};
        &:hover {
            background-color: ${to_hover(theme.palette.text.primary)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
    );
    let color_outlined_primary = sheet!(
        color: ${color_primary.main};
        border: 1px solid ${" "}${color_primary.main.alpha_multiply(0.5)};
        &:hover {
            border: 1px solid ${" "}${color_primary.main};
            background-color: ${to_hover(color_primary.main)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
    );
    let color_outlined_secondary = sheet!(
        color: ${color_secondary.main};
        border: 1px solid ${" "}${color_secondary.main.alpha_multiply(0.5)};
        &:hover {
            border: 1px solid ${" "}${color_secondary.main};
            background-color: ${to_hover(color_secondary.main)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
        &.${CLASS_DISABLED} {
            border: 1px solid ${" "}${&disabled_color};
        }
    );

    let color_contained_inherit = sheet!(
        color: ${theme.palette.contrast_text_color(gray_300)};
        background-color: ${gray_300};
        box-shadow: ${shadows2};
        &:hover {
            background-color: ${gray_a100};
            box-shadow: ${shadows4};
        }
        &:active {
            box-shadow: ${shadows8};
        }
        @media (hover: none) {
            &:hover {
                background-color: ${gray_300};
                box-shadow: ${shadows2};
            }
        }
    );
    let color_contained_primary = sheet!(
        color: ${color_primary.contrast};
        background-color: ${color_primary.main};
        box-shadow: ${shadows2};
        &:hover {
            background-color: ${color_primary.dark};
            box-shadow: ${shadows4};
        }
        &:active {
            box-shadow: ${shadows8};
        }
        @media (hover: none) {
            &:hover {
                background-color: ${color_primary.main};
                box-shadow: ${shadows2};
            }
        }
    );
    let color_contained_secondary = sheet!(
        color: ${color_secondary.contrast};
        background-color: ${color_secondary.main};
        box-shadow: ${shadows2};
        &:hover {
            background-color: ${color_secondary.dark};
            box-shadow: ${shadows4};
        }
        &:active {
            box-shadow: ${shadows8};
        }
        @media (hover: none) {
            &:hover {
                background-color: ${color_secondary.main};
                box-shadow: ${shadows2};
            }
        }
    );

    let root_override = theme
        .components
        .search_override::<ButtonStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();

    DefaultStyles {
        root_inline,
        //
        size_text_small,
        size_text_medium,
        size_text_large,
        size_outlined_small,
        size_outlined_medium,
        size_outlined_large,
        size_contained_small,
        size_contained_medium,
        size_contained_large,
        //
        color_text_inherit,
        color_text_primary,
        color_text_secondary,
        color_outlined_inherit,
        color_outlined_primary,
        color_outlined_secondary,
        color_contained_inherit,
        color_contained_primary,
        color_contained_secondary,
        //
        outlined,
        contained,
        //
        root_override,
    }
}

impl DefaultStyles {
    fn build_root_style(&self, props: &ButtonProperties) -> Vec<ScopeContent> {
        use ButtonColor::*;
        use ButtonSize::*;
        use ButtonVariant::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(&self.root_inline);
        collected_scopes.extend_from_slice(match (props.variant, props.size) {
            (Text, Small) => &self.size_text_small,
            (Text, Medium) => &self.size_text_medium,
            (Text, Large) => &self.size_text_large,
            (Outlined, Small) => &self.size_outlined_small,
            (Outlined, Medium) => &self.size_outlined_medium,
            (Outlined, Large) => &self.size_outlined_large,
            (Contained, Small) => &self.size_contained_small,
            (Contained, Medium) => &self.size_contained_medium,
            (Contained, Large) => &self.size_contained_large,
        });
        collected_scopes.extend_from_slice(match (props.variant, props.color) {
            (Text, Inherit) => &self.color_text_inherit,
            (Text, Primary) => &self.color_text_primary,
            (Text, Secondary) => &self.color_text_secondary,
            (Outlined, Inherit) => &self.color_outlined_inherit,
            (Outlined, Primary) => &self.color_outlined_primary,
            (Outlined, Secondary) => &self.color_outlined_secondary,
            (Contained, Inherit) => &self.color_contained_inherit,
            (Contained, Primary) => &self.color_contained_primary,
            (Contained, Secondary) => &self.color_contained_secondary,
        });
        collected_scopes.extend_from_slice(match (props.variant, props.color) {
            (Contained, _) => &self.contained,
            (Outlined, _) => &self.outlined,
            _ => Default::default(),
        });
        collected_scopes.extend_from_slice(&self.root_override);

        collected_scopes
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let styles = use_theme(derive_styles_from_theme);

    let mut root_style = styles.build_root_style(props);
    root_style.extend_from_slice(&props.class);
    let root_style = Sheet::from(root_style);

    html! {
        <ButtonBase
            class={root_style}
            disabled={props.disabled}
            on_pressed={props.on_pressed.clone()}
        >
            { for props.children.iter() }
        </ButtonBase>
    }
}
