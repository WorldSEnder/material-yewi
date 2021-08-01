use crate::memo::use_styles;
use css_in_rust::bindings::yew::use_scopes;
use css_in_rust::style::ast::Scopes;
use std::convert::TryInto;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Children;
use yew::Properties;
use yew_styles::Theme;

#[derive(Debug, Clone, PartialEq)]
pub struct TypographyStyleRoot {
    css_scopes: Scopes,
}

impl From<Scopes> for TypographyStyleRoot {
    fn from(scopes: Scopes) -> Self {
        Self { css_scopes: scopes }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TypographyVariant {
    Paragraph,
    Body1,
    Body2,
    Caption,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Overline,
    Subtitle1,
    Subtitle2,
    /// Emits a span with styling
    Button,
    /// Emits s span without styling
    Inherit,
    /// Emits a span that is only visible to screen readers
    ScreenReader,
}

impl Default for TypographyVariant {
    fn default() -> Self {
        TypographyVariant::Body1
    }
}

fn variant_to_element(variant: TypographyVariant) -> &'static str {
    use TypographyVariant::*;
    match variant {
        Paragraph | Body1 | Body2 => "p",
        H1 => "h1",
        H2 => "h2",
        H3 => "h3",
        H4 => "h4",
        H5 => "h5",
        H6 | Subtitle1 | Subtitle2 => "h6",
        Caption | Overline | Button | Inherit | ScreenReader => "span",
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TypographyAlign {
    Center,
    Justify,
    Left,
    Right,
    Inherit,
}

impl Default for TypographyAlign {
    fn default() -> Self {
        TypographyAlign::Inherit
    }
}

#[derive(Default, Clone, PartialEq, Debug, Properties)]
pub struct TypographyProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: TypographyVariant,
    #[prop_or_default]
    pub align: TypographyAlign,
    #[prop_or(false)]
    pub no_wrap: bool,
    #[prop_or(false)]
    pub gutter_bottom: bool,
    // TODO: color!
}

#[derive(Default)]
struct DefaultStyles {
    root: Scopes,          // Always applied to the root element
    paragraph: Scopes,     // Applied if variant = Paragraph
    body1: Scopes,         // Applied if variant = Body1
    body2: Scopes,         // Applied if variant = Body2
    caption: Scopes,       // Applied if variant = Caption
    h1: Scopes,            // Applied if variant = H1
    h2: Scopes,            // Applied if variant = H2
    h3: Scopes,            // Applied if variant = H3
    h4: Scopes,            // Applied if variant = H4
    h5: Scopes,            // Applied if variant = H5
    h6: Scopes,            // Applied if variant = H6
    overline: Scopes,      // Applied if variant = Overline
    subtitle1: Scopes,     // Applied if variant = Subtitle1
    subtitle2: Scopes,     // Applied if variant = Subtitle2
    button: Scopes,        // Applied if variant = Button
    screen_reader: Scopes, // Applied if variant = ScreenReader
    align_left: Scopes,    // Applied if align = Left
    align_right: Scopes,   // Applied if align = Right
    align_center: Scopes,  // Applied if align = Center
    align_justify: Scopes, // Applied if align = Justify
    gutter_bottom: Scopes, // Applied if gutter_bottom
    no_wrap: Scopes,       // Applied if no_wrap

    root_override: Scopes, // Overrides applied to root element
}

impl DefaultStyles {
    fn variant_scopes(&self, variant: TypographyVariant) -> Scopes {
        use TypographyVariant::*;
        match variant {
            Paragraph => self.paragraph.clone(),
            Body1 => self.body1.clone(),
            Body2 => self.body2.clone(),
            Caption => self.caption.clone(),
            H1 => self.h1.clone(),
            H2 => self.h2.clone(),
            H3 => self.h3.clone(),
            H4 => self.h4.clone(),
            H5 => self.h5.clone(),
            H6 => self.h6.clone(),
            Overline => self.overline.clone(),
            Subtitle1 => self.subtitle1.clone(),
            Subtitle2 => self.subtitle2.clone(),
            Button => self.button.clone(),
            ScreenReader => self.screen_reader.clone(),
            Inherit => Default::default(),
        }
    }

    fn align_scopes(&self, align: TypographyAlign) -> Scopes {
        use TypographyAlign::*;
        match align {
            Center => self.align_center.clone(),
            Left => self.align_left.clone(),
            Right => self.align_right.clone(),
            Justify => self.align_justify.clone(),
            Inherit => Default::default(),
        }
    }
}

fn derive_styles_from_theme(theme: Theme) -> DefaultStyles {
    let root = r#"margin: 0;"#.to_string().try_into().expect("unexpected error in css parsing");
    let screen_reader = r#"
        position: absolute;
        height: 1;
        width: 1;
        overflow: hidden;"#
        .to_string()
        .try_into()
        .expect("unexpected error in css parsing");
    let no_wrap = r#"
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        "#
    .to_string()
    .try_into()
    .expect("unexpected error in css parsing");
    let paragraph = r#"margin-bottom: 0.35em;"#
        .to_string()
        .try_into()
        .expect("unexpected error in css parsing");
    let gutter_bottom = r#"margin-bottom: 0.35em;"#
        .to_string()
        .try_into()
        .expect("unexpected error in css parsing");
    let align_left = r#"text-align: left;"#
        .to_string()
        .try_into()
        .expect("unexpected error in css parsing");
    let align_right = r#"text-align: right;"#
        .to_string()
        .try_into()
        .expect("unexpected error in css parsing");
    let align_center = r#"text-align: right;"#
        .to_string()
        .try_into()
        .expect("unexpected error in css parsing");
    let align_justify = r#"text-align: right;"#
        .to_string()
        .try_into()
        .expect("unexpected error in css parsing");
    let root_override = theme
        .components
        .search_override::<TypographyStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();
    let display_block: Scopes = r#"display: block;"#
        .to_string()
        .try_into()
        .expect("unexpected error in css parsing");

    let mut button = display_block.clone();
    button.append(theme.typography.button.clone());

    let mut caption = display_block.clone();
    caption.append(theme.typography.caption.clone());

    let mut overline = display_block.clone();
    overline.append(theme.typography.overline.clone());

    DefaultStyles {
        root,
        body1: theme.typography.body1.clone(),
        body2: theme.typography.body2.clone(),
        caption,
        button,
        h1: theme.typography.h1.clone(),
        h2: theme.typography.h2.clone(),
        h3: theme.typography.h3.clone(),
        h4: theme.typography.h4.clone(),
        h5: theme.typography.h5.clone(),
        h6: theme.typography.h6.clone(),
        subtitle1: theme.typography.subtitle1.clone(),
        subtitle2: theme.typography.subtitle2.clone(),
        overline,
        paragraph,
        screen_reader,
        align_left,
        align_right,
        align_center,
        align_justify,
        no_wrap,
        gutter_bottom,
        root_override,
    }
}

#[function_component(Typography)]
pub fn typography(props: &TypographyProperties) -> Html {
    let styles = use_styles(derive_styles_from_theme);

    let component = variant_to_element(props.variant);
    let root_scopes = styles.root.clone();
    let variant_scopes = styles.variant_scopes(props.variant);
    let gutter_scopes = if props.gutter_bottom {
        styles.gutter_bottom.clone()
    } else {
        Default::default()
    };
    let no_wrap_scopes = if props.no_wrap {
        styles.no_wrap.clone()
    } else {
        Default::default()
    };
    let align_scopes = styles.align_scopes(props.align);

    let mut root_styles = Scopes::default();
    // Order matters here! overrides come last
    root_styles.append(root_scopes);
    root_styles.append(variant_scopes);
    root_styles.append(gutter_scopes);
    root_styles.append(no_wrap_scopes);
    root_styles.append(align_scopes);
    root_styles.append(styles.root_override.clone());

    let root_style = use_scopes("Mwi-typography-root", root_styles);

    html! {
        <@{component} class={classes![root_style]}>
            { for props.children.iter() }
        </@>
    }
}
