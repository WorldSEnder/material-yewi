use material_styles_yew::use_theme;
use material_styles_yew::CssColor;
use material_styles_yew::Theme;
use std::convert::TryInto;
use stylist::ast::{sheet, Sheet};
use stylist::yew::use_sheet;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Callback;
use yew::Children;
use yew::KeyboardEvent;
use yew::MouseEvent;
use yew::Properties;

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

pub enum ButtonPressedEvent {
    MousePress(MouseEvent),
    EnterPress(KeyboardEvent),
    SpacebarPress(KeyboardEvent),
    // ... more later on?
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
    // TODO: These styles are originally contained in ButtonBase. When we tear them apart,
    // copy them over.
    root_base: Sheet,
    disabled_root_base: Sheet,

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
    disabled_outlined: Sheet,
    disabled_contained: Sheet,
    disabled_outlined_secondary: Sheet,
    // overrides
    root_override: Sheet,
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    // FIXME: push into theme
    let disabled_color = "rgba(0, 0, 0, 0.26)".to_string();
    let disabled_background_color = "rgba(0, 0, 0, 0.12)".to_string();

    let root_base = sheet!(
        display: inline-flex;
        align-items: center;
        justify-content: center;
        position: relative;
        box-sizing: border-box;
        background-color: transparent;

        outline: 0;
        border: 0;
        margin: 0;
        border-radius: 0;
        padding: 0;

        cursor: pointer;
        user-select: none;
        vertical-align: middle;
        text-decoration: none;
        color: inherit;

        -webkit-tap-highlight-color: transparent;
        -moz-appearance: none;
        -webkit-appearance: none;

        &::-moz-focus-inner {
            border-style: none;
        }
        @media print {
            color-adjust: exact;
        }
    );
    let disabled_root_base = sheet!(
        pointer-events: none;
        cursor: default;
        color: ${&disabled_color};
    );

    // FIXME: transition from theme
    let root_basebox = sheet!(
        min-width: 64px;
        border-radius: ${theme.shape.border_radius.clone()};
        transition:
            background-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
            box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
            border-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
            color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
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

    let disabled_outlined = sheet!(
        border: 1px solid ${&disabled_background_color};
    );

    let disabled_outlined_secondary = sheet!(
        border: 1px solid ${&disabled_color};
    );
    let disabled_contained = sheet!(
        color: ${&disabled_color};
        box-shadow: none;
        background-color: ${&disabled_background_color};
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
        border: 1px solid ${CssColor::rgba(0, 0, 0, 0.32)};
        &:hover {
            background-color: ${to_hover(theme.palette.text.primary)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
    );
    let color_outlined_primary = sheet!(
        color: ${color_primary.main};
        border: 1px solid ${color_primary.main.alpha_multiply(0.5)};
        &:hover {
            border: 1px solid ${color_primary.main};
            background-color: ${to_hover(color_primary.main)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
    );
    let color_outlined_secondary = sheet!(
        color: ${color_secondary.main};
        border: 1px solid ${color_secondary.main.alpha_multiply(0.5)};
        &:hover {
            border: 1px solid ${color_secondary.main};
            background-color: ${to_hover(color_secondary.main)};
        }
        @media (hover: none) {
            &:hover { background-color: transparent; }
        }
    );

    // FIXME: push into theme
    let gray_300: CssColor = "#e0e0e0".try_into().unwrap();
    let gray_a100: CssColor = "#d5d5d5".try_into().unwrap();
    let shadows2 = "0px 3px 1px -2px rgba(0,0,0,0.2),0px 2px 2px 0px rgba(0,0,0,0.14),0px 1px 5px 0px rgba(0,0,0,0.12)";
    let shadows4 = "0px 2px 4px -1px rgba(0,0,0,0.2),0px 4px 5px 0px rgba(0,0,0,0.14),0px 1px 10px 0px rgba(0,0,0,0.12)";
    let shadows8 = "0px 5px 5px -3px rgba(0,0,0,0.2),0px 8px 10px 1px rgba(0,0,0,0.14),0px 3px 14px 2px rgba(0,0,0,0.12)";

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
        root_base,
        disabled_root_base,
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
        disabled_outlined,
        disabled_outlined_secondary,
        disabled_contained,
        //
        root_override,
    }
}

impl DefaultStyles {
    fn build_root_style(&self, props: &ButtonProperties) -> Sheet {
        use ButtonColor::*;
        use ButtonSize::*;
        use ButtonVariant::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(&self.root_base);
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
        collected_scopes.extend_from_slice(&self.root_override);

        Sheet::from(collected_scopes)
    }
    fn build_disabled_style(&self, props: &ButtonProperties) -> Sheet {
        use ButtonColor::*;
        use ButtonVariant::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(&self.disabled_root_base);
        collected_scopes.extend_from_slice(match (props.variant, props.color) {
            (Contained, _) => &self.disabled_contained,
            (Outlined, Secondary) => &self.disabled_outlined_secondary,
            (Outlined, _) => &self.disabled_outlined,
            _ => Default::default(),
        });

        Sheet::from(collected_scopes)
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let styles = use_theme(derive_styles_from_theme);

    let root_style = use_sheet("Mwi-button-root", styles.build_root_style(props));
    let disabled_style = use_sheet("Mwi-button-disabled", styles.build_disabled_style(props));
    let disabled_style = if props.disabled {
        Some(disabled_style)
    } else {
        None
    };

    let onclick = props.on_pressed.reform(ButtonPressedEvent::MousePress);

    html! {
        <button class={classes![root_style, disabled_style]} onclick={onclick}>
            { for props.children.iter() }
        </button>
    }
}
