use css_in_rust::style::ast::Scopes;
use dependent_map::{DebugEntry, DynClone, DynPartialEq, HashableAny};
use std::convert::TryInto;
use std::fmt::Debug;
use std::hash::Hasher;
use std::ops::Deref;
use std::rc::Rc;
use yew::use_context;

#[derive(Debug)]
pub struct Typography {
    /// Css-scopes applied to button like text elements
    pub button: Scopes,

    pub body1: Scopes,
    pub body2: Scopes,
    pub caption: Scopes,
    pub h1: Scopes,
    pub h2: Scopes,
    pub h3: Scopes,
    pub h4: Scopes,
    pub h5: Scopes,
    pub h6: Scopes,
    pub overline: Scopes,
    pub subtitle1: Scopes,
    pub subtitle2: Scopes,
}

fn standard_text_css(
    weight: u32,
    size: &str,
    line_height: &str,
    letter_spacing: &str,
    additional: &str,
) -> String {
    format!(
        r#"
        font-family: "Roboto", "Helvetica", "Arial", sans-serif;
        font-weight: {weight};
        font-size: {size};
        line-height: {line_height};
        letter-spacing: {letter_spacing};
        {additional}"#,
        weight = weight,
        size = size,
        line_height = line_height,
        letter_spacing = letter_spacing,
        additional = additional,
    )
}

impl Typography {
    pub fn pixels_to_rem(&self, pixel: f32) -> String {
        let html_font_size = 16f32; // TODO: additional coefficients from font choice.
        format!("{rems:.5}rem", rems = pixel / html_font_size)
    }
}

impl Default for Typography {
    fn default() -> Self {
        let weight_light = 300u32;
        let weight_regular = 400u32;
        let weight_medium = 500u32;
        let _weight_bold = 700u32;

        let button = standard_text_css(
            weight_medium,
            "0.875rem",
            "1.75",
            "0.02857em",
            "text-transform: uppercase;",
        )
        .try_into()
        .expect("unexpected error in css parsing");
        let h1 = standard_text_css(weight_light, "6rem", "1.167", "-0.01562em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let h2 = standard_text_css(weight_light, "3.75rem", "1.2", "-0.00833em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let h3 = standard_text_css(weight_regular, "3rem", "1.167", "0em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let h4 = standard_text_css(weight_regular, "2.125rem", "1.235", "0.00735em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let h5 = standard_text_css(weight_regular, "1.5rem", "1.334", "0em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let h6 = standard_text_css(weight_medium, "1.25rem", "1.6", "0.0075em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let body1 = standard_text_css(weight_regular, "1rem", "1.5", "0.00938em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let body2 = standard_text_css(weight_regular, "0.875rem", "1.43", "0.01071em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let caption = standard_text_css(weight_regular, "0.75rem", "1.66", "0.03333em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let overline = standard_text_css(
            weight_regular,
            "0.75rem",
            "2.66",
            "0.08333em",
            "text-transform: uppercase;",
        )
        .try_into()
        .expect("unexpected error in css parsing");
        let subtitle1 = standard_text_css(weight_regular, "1rem", "1.75", "0.00938em", "")
            .try_into()
            .expect("unexpected error in css parsing");
        let subtitle2 = standard_text_css(weight_medium, "0.875rem", "1.57", "0.00714em", "")
            .try_into()
            .expect("unexpected error in css parsing");

        Typography {
            button,
            h1,
            h2,
            h3,
            h4,
            h5,
            h6,
            body1,
            body2,
            caption,
            overline,
            subtitle1,
            subtitle2,
        }
    }
}

#[derive(Default, Debug)]
pub struct ThemeContents {
    pub shape: Shape,
    pub breakpoints: Breakpoints,
    pub direction: Direction,
    pub palette: Palette,
    // shadows?: unknown;
    pub spacing: Spacing,
    // transitions?: unknown;
    pub components: Components,
    // mixins?: unknown;
    pub typography: Typography,
    // zIndex?: unknown;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub border_radius: String,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            border_radius: "4px".to_string(),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Breakpoints {}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Direction {}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorSpec {
    pub light: String,
    pub main: String,
    pub dark: String,
    pub contrast: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Palette {
    pub primary: ColorSpec,
    pub secondary: ColorSpec,
    pub error: ColorSpec,
    pub warning: ColorSpec,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            primary: ColorSpec {
                light: "#7986cb".to_string(),
                main: "#3f51b5".to_string(),
                dark: "#303f9f".to_string(),
                contrast: "#fff".to_string(),
            },
            secondary: ColorSpec {
                light: "#ff4081".to_string(),
                main: "#f50057".to_string(),
                dark: "#c51162".to_string(),
                contrast: "#fff".to_string(),
            },
            error: ColorSpec {
                light: "#e57373".to_string(),
                main: "#f44336".to_string(),
                dark: "#d32f2f".to_string(),
                contrast: "#fff".to_string(),
            },
            warning: ColorSpec {
                light: "#ffb74d".to_string(),
                main: "#ff9800".to_string(),
                dark: "#f57c00".to_string(),
                contrast: "rgba(0, 0, 0, 0.87)".to_string(),
            },
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Spacing {}

trait ComponentsMapEntry<H: Hasher>: HashableAny<H> + DynClone + DynPartialEq + DebugEntry {}
impl<H: Hasher, T> ComponentsMapEntry<H> for T where
    T: HashableAny<H> + DynClone + DynPartialEq + DebugEntry
{
}
dependent_map::create_entry_impl!(
    ComponentsMapEntry<H> where
        dependent_map::EntryAt<E, A>: PartialEq + Clone,
        dependent_map::ValueAt<E, A>: Debug,
        dependent_map::KeyAt<E, A>: Debug,
);
type ComponentMap = dependent_map::Map<
    dependent_map::families::Singleton,
    dependent_map::DefaultHashBuilder,
    dyn ComponentsMapEntry<dependent_map::DefaultHasher>,
>;

#[derive(Clone, Debug)]
pub struct Components {
    overrides: ComponentMap,
}

impl Default for Components {
    fn default() -> Self {
        Self {
            overrides: dependent_map::Map::new(),
        }
    }
}

impl Components {
    /// Add an override for a component type by providing default properties.
    pub fn add_override<C: 'static + PartialEq + Clone + Debug>(
        &mut self,
        component_props: C,
    ) -> Option<C> {
        match self.overrides.insert(component_props) {
            Some(c) => Some(c.some),
            None => None,
        }
    }
    /// Get the override for a component type, if present.
    pub fn search_override<C: 'static + PartialEq + Clone + Debug>(&self) -> Option<&C> {
        match self.overrides.get_default::<C>() {
            Some(c) => Some(c), // (ab)use deref impl
            None => None,
        }
    }
}

impl PartialEq for Components {
    fn eq(&self, rhs: &Components) -> bool {
        // TODO: is this comparison too expensive to do?
        // Keep in mind that Theme wraps the ThemeContents in an Rc and does a pointer-comparison
        self.overrides == rhs.overrides
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    inner: Rc<ThemeContents>,
}

thread_local!(
    // TODO: use lazy_static! with one global theme instead of a thread_local
    // version?

    /// Global, read-only, default theme. This is used in contexts where no
    /// [`ThemeProvider`] has installed a [`Theme`].
    static DEFAULT_THEME: Theme = {
        Theme {
            inner: Rc::new(Default::default()),
        }
    };
);

impl Default for Theme {
    fn default() -> Self {
        DEFAULT_THEME.with(|t| t.clone())
    }
}

impl Deref for Theme {
    type Target = ThemeContents;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl PartialEq for Theme {
    fn eq(&self, rhs: &Self) -> bool {
        // meh, dont bother :)
        Rc::ptr_eq(&self.inner, &rhs.inner)
    }
}

pub type ThemeProvider = yew::ContextProvider<Theme>;

fn use_memo<Args: 'static, F, R: 'static>(arguments: Args, f: F) -> impl Deref<Target = R>
where
    Args: Clone + PartialEq,
    F: Fn(Args) -> R,
{
    struct Storage<Args, R>(Option<(Args, Rc<R>)>);
    yew::functional::use_hook(
        || Storage(None),
        |state, _| match &state.0 {
            Some(s) if s.0 == arguments => Rc::clone(&s.1),
            _ => {
                let new_result = Rc::new(f(arguments.clone()));
                state.0 = Some((arguments, new_result.clone()));
                new_result
            }
        },
        |_| {},
    )
}

// FIXME: the way we use this for theme derivations, this should not memo per component,
// but memo per theme. Theme incidentially internally keeps a Rc to the actual theme contents,
// so we could use a map from Weak<ThemeContent> to derived styles.
pub fn use_theme<R: 'static>(
    theme_to_styles: impl 'static + Fn(Theme) -> R,
) -> impl Deref<Target = R> {
    let theme = use_context::<Theme>().unwrap_or_default();
    use_memo(theme.clone(), theme_to_styles)
}
