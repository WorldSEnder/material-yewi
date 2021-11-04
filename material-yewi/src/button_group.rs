use crate::button::{Button, ButtonColor, ButtonProperties, ButtonVariant, CLASS_DISABLED};
use material_styles_yew::{use_theme, CssColor, Theme};
use stylist::{
    ast::{sheet, ScopeContent, Sheet},
    yew::use_style,
};
use yew::{classes, function_component, html, html_nested, ChildrenWithProps, Properties};

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonGroupStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for ButtonGroupStyleRoot {
    fn from(scopes: Sheet) -> Self {
        Self { css_scopes: scopes }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonGroupProperties {
    #[prop_or_default]
    pub class: Sheet,
    #[prop_or_default]
    pub children: ChildrenWithProps<Button>,
    #[prop_or_default]
    pub orientation: Orientation,
}

struct DefaultStyles {
    root_inline: Sheet,
    root_vertical: Sheet,
    // button props
    button: Sheet,
    button_horizontal: Sheet,
    button_vertical: Sheet,
    // variant::text
    button_horizontal_text: Sheet,
    button_vertical_text: Sheet,
    button_text_color_primary: Sheet,
    button_text_color_secondary: Sheet,
    // variant::contained
    button_horizontal_contained: Sheet,
    button_vertical_contained: Sheet,
    button_contained_color_primary: Sheet,
    button_contained_color_secondary: Sheet,
    // variant::outlined
    button_horizontal_outlined: Sheet,
    button_vertical_outlined: Sheet,
    // overrides
    root_override: Sheet,
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    let gray_400: CssColor = CssColor::rgb(0xbd, 0xbd, 0xbd);

    let root_inline = sheet!(
        display: inline-flex;
        border-radius: ${&theme.shape.border_radius};
    );
    let root_vertical = sheet!(
        flex-direction: column;
    );

    let button = sheet!(
        min-width: 40px;
    );
    let button_horizontal = sheet!(
        &:not(:first-of-type) {
            border-top-left-radius: 0;
            border-bottom-left-radius: 0;
        }
        &:not(:last-of-type) {
            border-top-right-radius: 0;
            border-bottom-right-radius: 0;
        }
    );
    let button_vertical = sheet!(
        &:not(:first-of-type) {
            border-top-left-radius: 0;
            border-top-right-radius: 0;
        }
        &:not(:last-of-type) {
            border-bottom-left-radius: 0;
            border-bottom-right-radius: 0;
        }
    );
    let button_horizontal_text = sheet!(
        &:not(:last-of-type) {
            // TODO: get color from theme palette
            border-right: 1px solid rgba(255, 255, 255, 0.23);
        }
    );
    let button_vertical_text = sheet!(
        &:not(:last-of-type) {
            // TODO: get color from theme palette
            border-bottom: 1px solid rgba(255, 255, 255, 0.23);
        }
    );
    let button_text_color_primary = sheet!(
        &:not(:last-of-type) {
            border-color: ${theme.palette.primary.main.alpha_multiply(0.5)};
        }
    );
    let button_text_color_secondary = sheet!(
        &:not(:last-of-type) {
            border-color: ${theme.palette.secondary.main.alpha_multiply(0.5)};
        }
    );
    let button_horizontal_contained = sheet!(
        &:not(:last-of-type) {
            border-right: 1px solid${" "}${gray_400};
        }
        &:not(:last-of-type).${CLASS_DISABLED} {
            border-right: 1px solid${" "}${theme.palette.actions.disabled};
        }
    );
    let button_vertical_contained = sheet!(
        &:not(:last-of-type) {
            border-bottom: 1px solid${" "}${gray_400};
        }
        &:not(:last-of-type).${CLASS_DISABLED} {
            border-bottom: 1px solid${" "}${theme.palette.actions.disabled};
        }
    );
    let button_contained_color_primary = sheet!(
        &:not(:last-of-type) {
            border-color: ${theme.palette.primary.dark};
        }
    );
    let button_contained_color_secondary = sheet!(
        &:not(:last-of-type) {
            border-color: ${theme.palette.secondary.dark};
        }
    );
    let button_horizontal_outlined = sheet!(
        &:not(:first-of-type) {
            margin-left: -1px;
        }
        &:not(:last-of-type) {
            border-right-color: transparent;
        }
        &:not(:last-of-type):hover {
            border-right-color: currentColor;
        }
    );
    let button_vertical_outlined = sheet!(
        &:not(:first-of-type) {
            margin-top: -1px;
        }
        &:not(:last-of-type) {
            border-bottom-color: transparent;
        }
        &:not(:last-of-type):hover {
            border-bottom-color: currentColor;
        }
    );

    let root_override = theme
        .components
        .search_override::<ButtonGroupStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();

    DefaultStyles {
        root_inline,
        root_vertical,
        button,
        button_horizontal,
        button_vertical,
        button_text_color_primary,
        button_text_color_secondary,
        button_horizontal_text,
        button_vertical_text,
        button_contained_color_primary,
        button_contained_color_secondary,
        button_horizontal_contained,
        button_vertical_contained,
        button_horizontal_outlined,
        button_vertical_outlined,
        root_override,
    }
}

impl DefaultStyles {
    fn build_root_style(&self, props: &ButtonGroupProperties) -> Vec<ScopeContent> {
        use Orientation::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(&self.root_inline);
        collected_scopes.extend_from_slice(match props.orientation {
            Horizontal => &[],
            Vertical => &self.root_vertical,
        });
        collected_scopes.extend_from_slice(&self.root_override);
        collected_scopes
    }

    fn build_button_style(
        &self,
        group_props: &ButtonGroupProperties,
        button_props: &ButtonProperties,
    ) -> Vec<ScopeContent> {
        use ButtonColor::*;
        use ButtonVariant::*;
        use Orientation::*;

        let mut collected_scopes = vec![];
        collected_scopes.extend_from_slice(&self.button);
        collected_scopes.extend_from_slice(match group_props.orientation {
            Horizontal => &self.button_horizontal,
            Vertical => &self.button_vertical,
        });
        collected_scopes.extend_from_slice(match (group_props.orientation, button_props.variant) {
            (Horizontal, Text) => &self.button_horizontal_text,
            (Vertical, Text) => &self.button_vertical_text,
            (Horizontal, Contained) => &self.button_horizontal_contained,
            (Vertical, Contained) => &self.button_vertical_contained,
            (Horizontal, Outlined) => &self.button_horizontal_outlined,
            (Vertical, Outlined) => &self.button_vertical_outlined,
        });
        collected_scopes.extend_from_slice(match (button_props.variant, button_props.color) {
            (Text, Primary) => &self.button_text_color_primary,
            (Text, Secondary) => &self.button_text_color_secondary,
            (Contained, Primary) => &self.button_contained_color_primary,
            (Contained, Secondary) => &self.button_contained_color_secondary,
            (Outlined, Primary) => &[],
            (Outlined, Secondary) => &[],
            (_, Inherit) => &[],
        });
        collected_scopes
    }
}

#[function_component(ButtonGroup)]
pub fn button_group(props: &ButtonGroupProperties) -> Html {
    let styles = use_theme(derive_styles_from_theme);

    let mut root_style = styles.build_root_style(props);
    root_style.extend_from_slice(&props.class);
    let root_style = Sheet::from(root_style);
    let root_style = use_style(/* "Mwi-button-group", */ root_style);

    html! {
        <div class={classes![root_style]}>
            {
                for props.children.iter().map(|button| {
                    let mut button_style = styles.build_button_style(props, button.props.as_ref());
                    button_style.extend_from_slice(&button.props.class);
                    let button_style = Sheet::from(button_style);
                    html_nested! {
                        <Button class={button_style} ..button.props.as_ref().clone() />
                    }
                })
            }
        </div>
    }
}
