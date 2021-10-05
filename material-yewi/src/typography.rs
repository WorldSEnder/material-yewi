use material_styles_yew::use_theme;
use material_styles_yew::Theme;
use stylist::ast::{sheet, Sheet};
use stylist::yew::use_style;
use yew::classes;
use yew::function_component;
use yew::html;
use yew::Children;
use yew::Properties;

#[derive(Debug, Clone, PartialEq)]
pub struct TypographyStyleRoot {
    css_scopes: Sheet,
}

impl From<Sheet> for TypographyStyleRoot {
    fn from(css_scopes: Sheet) -> Self {
        Self { css_scopes }
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
    pub class: Sheet,
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
    root: Sheet,          // Always applied to the root element
    paragraph: Sheet,     // Applied if variant = Paragraph
    body1: Sheet,         // Applied if variant = Body1
    body2: Sheet,         // Applied if variant = Body2
    caption: Sheet,       // Applied if variant = Caption
    h1: Sheet,            // Applied if variant = H1
    h2: Sheet,            // Applied if variant = H2
    h3: Sheet,            // Applied if variant = H3
    h4: Sheet,            // Applied if variant = H4
    h5: Sheet,            // Applied if variant = H5
    h6: Sheet,            // Applied if variant = H6
    overline: Sheet,      // Applied if variant = Overline
    subtitle1: Sheet,     // Applied if variant = Subtitle1
    subtitle2: Sheet,     // Applied if variant = Subtitle2
    button: Sheet,        // Applied if variant = Button
    screen_reader: Sheet, // Applied if variant = ScreenReader
    align_left: Sheet,    // Applied if align = Left
    align_right: Sheet,   // Applied if align = Right
    align_center: Sheet,  // Applied if align = Center
    align_justify: Sheet, // Applied if align = Justify
    gutter_bottom: Sheet, // Applied if gutter_bottom
    no_wrap: Sheet,       // Applied if no_wrap

    root_override: Sheet, // Overrides applied to root element
}

impl DefaultStyles {
    fn variant_scopes(&self, variant: TypographyVariant) -> Sheet {
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

    fn align_scopes(&self, align: TypographyAlign) -> Sheet {
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
    let root = sheet!(margin: 0;);
    let screen_reader = sheet!(
        position: absolute;
        height: 1;
        width: 1;
        overflow: hidden;
    );
    let no_wrap = sheet!(
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    );
    let paragraph = sheet!(margin-bottom: ${"0.35em"};);
    let gutter_bottom = sheet!(margin-bottom: ${"0.35em"};);
    let align_left = sheet!(text-align: left;);
    let align_right = sheet!(text-align: right;);
    let align_center = sheet!(text-align: right;);
    let align_justify = sheet!(text-align: right;);
    let display_block = sheet!(display: block;);

    let mut button = vec![];
    button.extend_from_slice(&display_block);
    button.extend_from_slice(&theme.typography.button);
    let button = Sheet::from(button);

    let mut caption = vec![];
    caption.extend_from_slice(&display_block);
    caption.extend_from_slice(&theme.typography.caption);
    let caption = Sheet::from(caption);

    let mut overline = vec![];
    overline.extend_from_slice(&display_block);
    overline.extend_from_slice(&theme.typography.overline);
    let overline = Sheet::from(overline);

    let root_override = theme
        .components
        .search_override::<TypographyStyleRoot>()
        .map(|c| &c.css_scopes)
        .cloned()
        .unwrap_or_default();

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
    let styles = use_theme(derive_styles_from_theme);

    let component = variant_to_element(props.variant);
    let root_sheet = styles.root.clone();
    let variant_sheet = styles.variant_scopes(props.variant);
    let gutter_sheet = if props.gutter_bottom {
        styles.gutter_bottom.clone()
    } else {
        Default::default()
    };
    let no_wrap_sheet = if props.no_wrap {
        styles.no_wrap.clone()
    } else {
        Default::default()
    };
    let align_sheet = styles.align_scopes(props.align);

    let mut root_styles = vec![];
    // Order matters here! overrides come last
    root_styles.extend_from_slice(&root_sheet);
    root_styles.extend_from_slice(&variant_sheet);
    root_styles.extend_from_slice(&gutter_sheet);
    root_styles.extend_from_slice(&no_wrap_sheet);
    root_styles.extend_from_slice(&align_sheet);
    root_styles.extend_from_slice(&styles.root_override);
    root_styles.extend_from_slice(&props.class);
    let root_styles = Sheet::from(root_styles);

    let root_style = use_style(/* "Mwi-typography-root", */ root_styles);

    html! {
        <@{component} class={classes![root_style]}>
            { for props.children.iter() }
        </@>
    }
}
