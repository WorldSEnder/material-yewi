use css_in_rust::bindings::yew::use_scopes;
use css_in_rust::style::ast::Scopes;
use std::convert::TryInto;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Callback;
use yew::Children;
use yew::KeyboardEvent;
use yew::MouseEvent;
use yew::Properties;
use yew_styles::use_theme;
use yew_styles::CssColor;
use yew_styles::Theme;

// FIXME: ripple effects

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonStyleRoot {
    css_scopes: Scopes,
}

impl From<Scopes> for ButtonStyleRoot {
    fn from(scopes: Scopes) -> Self {
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
    root_base: Scopes,
    disabled_root_base: Scopes,

    root_inline: Scopes,
    // sizing
    size_text_small: Scopes,
    size_text_medium: Scopes,
    size_text_large: Scopes,
    size_outlined_small: Scopes,
    size_outlined_medium: Scopes,
    size_outlined_large: Scopes,
    size_contained_small: Scopes,
    size_contained_medium: Scopes,
    size_contained_large: Scopes,
    // coloring
    color_text_inherit: Scopes,
    color_text_primary: Scopes,
    color_text_secondary: Scopes,
    color_outlined_inherit: Scopes,
    color_outlined_primary: Scopes,
    color_outlined_secondary: Scopes,
    color_contained_inherit: Scopes,
    color_contained_primary: Scopes,
    color_contained_secondary: Scopes,
    // disabled
    disabled_outlined: Scopes,
    disabled_contained: Scopes,
    disabled_outlined_secondary: Scopes,
    // overrides
    root_override: Scopes,
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    let root_base: Scopes = r#"
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
        "#
    .to_string()
    .try_into()
    .expect("error in css parsing");
    let disabled_root_base: Scopes = r#"
        pointer-events: none;
        cursor: default;
        "#
    .to_string()
    .try_into()
    .expect("error in css parsing");

    let mut root_inline = Scopes::default();
    root_inline.append(theme.typography.button.clone());
    root_inline.append(
        // FIXME: transition from theme
        format!(
            r#"
            min-width: 64px;
            border-radius: {brdr};
            transition:
                background-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
                box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
                border-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,
                color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
            "#,
            brdr = theme.shape.border_radius
        )
        .try_into()
        .expect("unexpected css error"),
    );
    // hover
    root_inline.append(
        r#"&:hover { text-decoration: none; }"#
            .to_string()
            .try_into()
            .expect("error in css parsing"),
    );

    let size_text_small: Scopes = format!(
        r#"
        padding: 4px 5px;
        font-size: {sz};
        "#,
        sz = theme.typography.pixels_to_rem(13.0)
    )
    .try_into()
    .expect("error in css parsing");
    let size_text_medium: Scopes =
        r#"padding: 6px 8px;"#.to_string().try_into().expect("error in css parsing");
    let size_text_large: Scopes = format!(
        r#"
        padding: 8px 11px;
        font-size: {sz};
        "#,
        sz = theme.typography.pixels_to_rem(15.0)
    )
    .try_into()
    .expect("error in css parsing");

    let size_outlined_small: Scopes = format!(
        r#"
        padding: 3px 9px;
        font-size: {sz};
        "#,
        sz = theme.typography.pixels_to_rem(13.0)
    )
    .try_into()
    .expect("error in css parsing");
    let size_outlined_medium: Scopes =
        r#"padding: 5px 15px;"#.to_string().try_into().expect("error in css parsing");
    let size_outlined_large: Scopes = format!(
        r#"
        padding: 7px 21px;
        font-size: {sz};
        "#,
        sz = theme.typography.pixels_to_rem(15.0)
    )
    .try_into()
    .expect("error in css parsing");

    let size_contained_small: Scopes = format!(
        r#"
        padding: 4px 10px;
        font-size: {sz};
        "#,
        sz = theme.typography.pixels_to_rem(13.0)
    )
    .try_into()
    .expect("error in css parsing");
    let size_contained_medium: Scopes =
        r#"padding: 6px 16px;"#.to_string().try_into().expect("error in css parsing");
    let size_contained_large: Scopes = format!(
        r#"
        padding: 8px 22px;
        font-size: {sz};
        "#,
        sz = theme.typography.pixels_to_rem(15.0)
    )
    .try_into()
    .expect("error in css parsing");

    // FIXME: push into theme
    let disabled_color = "rgba(0, 0, 0, 0.26)".to_string();
    let disabled_background_color = "rgba(0, 0, 0, 0.12)".to_string();

    let disabled_outlined: Scopes = format!(
        r#"border: 1px solid {c_dis};"#,
        c_dis = disabled_background_color,
    )
    .try_into()
    .expect("error in css parsing");

    let disabled_outlined_secondary: Scopes =
        format!(r#"border: 1px solid {c_dis};"#, c_dis = disabled_color,)
            .try_into()
            .expect("error in css parsing");
    let disabled_contained: Scopes = format!(
        r#"
        color: {c_dis};
        box-shadow: none;
        background-color: {c_dis_bck};
        "#,
        c_dis = disabled_color,
        c_dis_bck = disabled_background_color,
    )
    .try_into()
    .expect("error in css parsing");

    let to_hover = |c: CssColor| {
        c.alpha_multiply(theme.palette.actions.hover_opacity)
            .to_css_value()
    };
    let color_primary = &theme.palette.primary;
    let color_secondary = &theme.palette.secondary;

    let color_text_inherit: Scopes = format!(
        r#"
        color: inherit;
        &:hover {{
            background-color: {c_bck};
        }}
        @media (hover: none) {{
            &:hover {{ background-color: transparent; }}
        }}
        "#,
        c_bck = to_hover(theme.palette.text.primary),
    )
    .try_into()
    .expect("error in css parsing");
    let color_text_primary: Scopes = format!(
        r#"
        color: {c_txt};
        &:hover {{
            background-color: {c_bck};
        }}
        @media (hover: none) {{
            &:hover {{ background-color: transparent; }}
        }}
        "#,
        c_txt = color_primary.main.to_css_value(),
        c_bck = to_hover(color_primary.main),
    )
    .try_into()
    .expect("error in css parsing");
    let color_text_secondary: Scopes = format!(
        r#"
        color: {c_txt};
        &:hover {{
            background-color: {c_bck};
        }}
        @media (hover: none) {{
            &:hover {{ background-color: transparent; }}
        }}
        "#,
        c_txt = color_secondary.main.to_css_value(),
        c_bck = to_hover(color_secondary.main),
    )
    .try_into()
    .expect("error in css parsing");

    let color_outlined_inherit: Scopes = format!(
        r#"
        color: inherit;
        border: 1px solid {c_brd};
        &:hover {{
            background-color: {c_bck};
        }}
        @media (hover: none) {{
            &:hover {{ background-color: transparent; }}
        }}
        "#,
        // FIXME: push into theme
        c_brd = CssColor::rgba(0, 0, 0, 0.32).to_css_value(),
        c_bck = to_hover(theme.palette.text.primary),
    )
    .try_into()
    .expect("error in css parsing");
    let color_outlined_primary: Scopes = format!(
        r#"
        color: {c_txt};
        border: 1px solid {c_brd};
        &:hover {{
            border: 1px solid {c_brd_hover};
            background-color: {c_bck};
        }}
        @media (hover: none) {{
            &:hover {{ background-color: transparent; }}
        }}
        "#,
        c_txt = color_primary.main.to_css_value(),
        c_brd = color_primary.main.alpha_multiply(0.5).to_css_value(),
        c_brd_hover = color_primary.main.to_css_value(),
        c_bck = to_hover(color_primary.main),
    )
    .try_into()
    .expect("error in css parsing");
    let color_outlined_secondary: Scopes = format!(
        r#"
        color: {c_txt};
        border: 1px solid {c_brd};
        &:hover {{
            border: 1px solid {c_brd_hover};
            background-color: {c_bck};
        }}
        @media (hover: none) {{
            &:hover {{ background-color: transparent; }}
        }}
        "#,
        c_txt = color_secondary.main.to_css_value(),
        c_brd = color_secondary.main.alpha_multiply(0.5).to_css_value(),
        c_brd_hover = color_secondary.main.to_css_value(),
        c_bck = to_hover(color_secondary.main),
    )
    .try_into()
    .expect("error in css parsing");

    // FIXME: push into theme
    let gray_300: CssColor = "#e0e0e0".try_into().unwrap();
    let gray_a100: CssColor = "#d5d5d5".try_into().unwrap();
    let shadows2 = "0px 3px 1px -2px rgba(0,0,0,0.2),0px 2px 2px 0px rgba(0,0,0,0.14),0px 1px 5px 0px rgba(0,0,0,0.12)";
    let shadows4 = "0px 2px 4px -1px rgba(0,0,0,0.2),0px 4px 5px 0px rgba(0,0,0,0.14),0px 1px 10px 0px rgba(0,0,0,0.12)";
    let shadows8 = "0px 5px 5px -3px rgba(0,0,0,0.2),0px 8px 10px 1px rgba(0,0,0,0.14),0px 3px 14px 2px rgba(0,0,0,0.12)";

    let color_contained_inherit: Scopes = format!(
        r#"
        color: {c_txt};
        background-color: {c_bck};
        box-shadow: {shadow};
        &:hover {{
            background-color: {c_bck_hover};
            box-shadow: {shadow_hover};
        }}
        &:active {{
            box-shadow: {shadow_active};
        }}
        @media (hover: none) {{
            &:hover {{
                background-color: {c_bck};
                box-shadow: {shadow};
            }}
        }}
        "#,
        c_txt = theme.palette.contrast_text_color(gray_300).to_css_value(),
        c_bck = gray_300.to_css_value(),
        c_bck_hover = gray_a100.to_css_value(),
        shadow = shadows2,
        shadow_hover = shadows4,
        shadow_active = shadows8,
    )
    .try_into()
    .expect("error in css parsing");
    let color_contained_primary: Scopes = format!(
        r#"
        color: {c_txt};
        background-color: {c_bck};
        box-shadow: {shadow};
        &:hover {{
            background-color: {c_bck_hover};
            box-shadow: {shadow_hover};
        }}
        &:active {{
            box-shadow: {shadow_active};
        }}
        @media (hover: none) {{
            &:hover {{
                background-color: {c_bck};
                box-shadow: {shadow};
            }}
        }}
        "#,
        c_txt = color_primary.contrast.to_css_value(),
        c_bck = color_primary.main.to_css_value(),
        c_bck_hover = color_primary.dark.to_css_value(),
        shadow = shadows2,
        shadow_hover = shadows4,
        shadow_active = shadows8,
    )
    .try_into()
    .expect("error in css parsing");
    let color_contained_secondary: Scopes = format!(
        r#"
        color: {c_txt};
        background-color: {c_bck};
        box-shadow: {shadow};
        &:hover {{
            background-color: {c_bck_hover};
            box-shadow: {shadow_hover};
        }}
        &:active {{
            box-shadow: {shadow_active};
        }}
        @media (hover: none) {{
            &:hover {{
                background-color: {c_bck};
                box-shadow: {shadow};
            }}
        }}
        "#,
        c_txt = color_secondary.contrast.to_css_value(),
        c_bck = color_secondary.main.to_css_value(),
        c_bck_hover = color_secondary.dark.to_css_value(),
        shadow = shadows2,
        shadow_hover = shadows4,
        shadow_active = shadows8,
    )
    .try_into()
    .expect("error in css parsing");

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
    fn build_root_style(&self, props: &ButtonProperties) -> Scopes {
        use ButtonColor::*;
        use ButtonSize::*;
        use ButtonVariant::*;

        let mut collected_scopes = Scopes::default();
        collected_scopes.append(self.root_base.clone());
        collected_scopes.append(self.root_inline.clone());
        collected_scopes.append(match (props.variant, props.size) {
            (Text, Small) => self.size_text_small.clone(),
            (Text, Medium) => self.size_text_medium.clone(),
            (Text, Large) => self.size_text_large.clone(),
            (Outlined, Small) => self.size_outlined_small.clone(),
            (Outlined, Medium) => self.size_outlined_medium.clone(),
            (Outlined, Large) => self.size_outlined_large.clone(),
            (Contained, Small) => self.size_contained_small.clone(),
            (Contained, Medium) => self.size_contained_medium.clone(),
            (Contained, Large) => self.size_contained_large.clone(),
        });
        collected_scopes.append(match (props.variant, props.color) {
            (Text, Inherit) => self.color_text_inherit.clone(),
            (Text, Primary) => self.color_text_primary.clone(),
            (Text, Secondary) => self.color_text_secondary.clone(),
            (Outlined, Inherit) => self.color_outlined_inherit.clone(),
            (Outlined, Primary) => self.color_outlined_primary.clone(),
            (Outlined, Secondary) => self.color_outlined_secondary.clone(),
            (Contained, Inherit) => self.color_contained_inherit.clone(),
            (Contained, Primary) => self.color_contained_primary.clone(),
            (Contained, Secondary) => self.color_contained_secondary.clone(),
        });
        collected_scopes.append(self.root_override.clone());

        collected_scopes
    }
    fn build_disabled_style(&self, props: &ButtonProperties) -> Scopes {
        use ButtonColor::*;
        use ButtonVariant::*;

        let mut collected_scopes = Scopes::default();
        collected_scopes.append(self.disabled_root_base.clone());
        collected_scopes.append(match (props.variant, props.color) {
            (Contained, _) => self.disabled_contained.clone(),
            (Outlined, Secondary) => self.disabled_outlined_secondary.clone(),
            (Outlined, _) => self.disabled_outlined.clone(),
            _ => Default::default(),
        });

        collected_scopes
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let styles = use_theme(derive_styles_from_theme);

    let root_style = use_scopes("Mwi-button-root", styles.build_root_style(props));
    let disabled_style = use_scopes("Mwi-button-disabled", styles.build_disabled_style(props));
    let disabled_style = if props.disabled {
        Some(&disabled_style)
    } else {
        None
    };

    let onclick = props.on_pressed.reform(ButtonPressedEvent::MousePress);

    html! {
        <button class={classes![&root_style, disabled_style]} onclick={onclick}>
            { for props.children.iter() }
        </button>
    }
}
