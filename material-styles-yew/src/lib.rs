use dependent_map::{DebugEntry, DynClone, DynPartialEq, HashableAny};
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::hash::Hasher;
use std::ops::Deref;
use std::rc::Rc;
use stylist::ast::Sheet;
use yew::use_context;

mod color;
pub use color::*;

#[derive(Debug)]
pub struct Typography {
    /// Css-scopes applied to button like text elements
    pub button: Sheet,

    pub body1: Sheet,
    pub body2: Sheet,
    pub caption: Sheet,
    pub h1: Sheet,
    pub h2: Sheet,
    pub h3: Sheet,
    pub h4: Sheet,
    pub h5: Sheet,
    pub h6: Sheet,
    pub overline: Sheet,
    pub subtitle1: Sheet,
    pub subtitle2: Sheet,
}

fn standard_text_css(weight: u32, size: &str, line_height: &str, letter_spacing: &str) -> Sheet {
    stylist::ast::sheet!(
        font-family: "Roboto", "Helvetica", "Arial", sans-serif;
        font-weight: ${weight};
        font-size: ${size};
        line-height: ${line_height};
        letter-spacing: ${letter_spacing};
    )
}

fn uppercase_text_css(weight: u32, size: &str, line_height: &str, letter_spacing: &str) -> Sheet {
    stylist::ast::sheet!(
        font-family: "Roboto", "Helvetica", "Arial", sans-serif;
        font-weight: ${weight};
        font-size: ${size};
        line-height: ${line_height};
        letter-spacing: ${letter_spacing};
        text-transform: uppercase;
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

        let button = uppercase_text_css(weight_medium, "0.875rem", "1.75", "0.02857em");
        let h1 = standard_text_css(weight_light, "6rem", "1.167", "-0.01562em");
        let h2 = standard_text_css(weight_light, "3.75rem", "1.2", "-0.00833em");
        let h3 = standard_text_css(weight_regular, "3rem", "1.167", "0em");
        let h4 = standard_text_css(weight_regular, "2.125rem", "1.235", "0.00735em");
        let h5 = standard_text_css(weight_regular, "1.5rem", "1.334", "0em");
        let h6 = standard_text_css(weight_medium, "1.25rem", "1.6", "0.0075em");
        let body1 = standard_text_css(weight_regular, "1rem", "1.5", "0.00938em");
        let body2 = standard_text_css(weight_regular, "0.875rem", "1.43", "0.01071em");
        let caption = standard_text_css(weight_regular, "0.75rem", "1.66", "0.03333em");
        let overline = uppercase_text_css(weight_regular, "0.75rem", "2.66", "0.08333em");
        let subtitle1 = standard_text_css(weight_regular, "1rem", "1.75", "0.00938em");
        let subtitle2 = standard_text_css(weight_medium, "0.875rem", "1.57", "0.00714em");

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
    pub shadows: Shadows,
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
    pub light: CssColor,
    pub main: CssColor,
    pub dark: CssColor,
    pub contrast: CssColor,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextColorSpec {
    pub primary: CssColor,
    pub secondary: CssColor,
    pub disabled: CssColor,
    pub hint: CssColor,
}

impl Default for TextColorSpec {
    fn default() -> Self {
        Self {
            primary: CssColor::rgba(0, 0, 0, 0.87),
            secondary: CssColor::rgba(0, 0, 0, 0.54),
            disabled: CssColor::rgba(0, 0, 0, 0.38),
            hint: CssColor::rgba(0, 0, 0, 0.38),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaletteActions {
    pub hover_opacity: f32,
}

impl Default for PaletteActions {
    fn default() -> Self {
        Self {
            hover_opacity: 0.04,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaletteBackground {
    pub paper: CssColor,
}

impl Default for PaletteBackground {
    fn default() -> Self {
        Self {
            paper: "#fff".try_into().expect(""),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShadowSpec(&'static str);

impl Display for ShadowSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shadows(pub [ShadowSpec; 25]);

impl Default for Shadows {
    fn default() -> Self {
        Self([
            ShadowSpec("none"),
            ShadowSpec("0px 2px 1px -1px rgba(0,0,0,0.2),0px 1px 1px 0px rgba(0,0,0,0.14),0px 1px 3px 0px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 3px 1px -2px rgba(0,0,0,0.2),0px 2px 2px 0px rgba(0,0,0,0.14),0px 1px 5px 0px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 3px 3px -2px rgba(0,0,0,0.2),0px 3px 4px 0px rgba(0,0,0,0.14),0px 1px 8px 0px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 2px 4px -1px rgba(0,0,0,0.2),0px 4px 5px 0px rgba(0,0,0,0.14),0px 1px 10px 0px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 3px 5px -1px rgba(0,0,0,0.2),0px 5px 8px 0px rgba(0,0,0,0.14),0px 1px 14px 0px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 3px 5px -1px rgba(0,0,0,0.2),0px 6px 10px 0px rgba(0,0,0,0.14),0px 1px 18px 0px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 4px 5px -2px rgba(0,0,0,0.2),0px 7px 10px 1px rgba(0,0,0,0.14),0px 2px 16px 1px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 5px 5px -3px rgba(0,0,0,0.2),0px 8px 10px 1px rgba(0,0,0,0.14),0px 3px 14px 2px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 5px 6px -3px rgba(0,0,0,0.2),0px 9px 12px 1px rgba(0,0,0,0.14),0px 3px 16px 2px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 6px 6px -3px rgba(0,0,0,0.2),0px 10px 14px 1px rgba(0,0,0,0.14),0px 4px 18px 3px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 6px 7px -4px rgba(0,0,0,0.2),0px 11px 15px 1px rgba(0,0,0,0.14),0px 4px 20px 3px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 7px 8px -4px rgba(0,0,0,0.2),0px 12px 17px 2px rgba(0,0,0,0.14),0px 5px 22px 4px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 7px 8px -4px rgba(0,0,0,0.2),0px 13px 19px 2px rgba(0,0,0,0.14),0px 5px 24px 4px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 7px 9px -4px rgba(0,0,0,0.2),0px 14px 21px 2px rgba(0,0,0,0.14),0px 5px 26px 4px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 8px 9px -5px rgba(0,0,0,0.2),0px 15px 22px 2px rgba(0,0,0,0.14),0px 6px 28px 5px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 8px 10px -5px rgba(0,0,0,0.2),0px 16px 24px 2px rgba(0,0,0,0.14),0px 6px 30px 5px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 8px 11px -5px rgba(0,0,0,0.2),0px 17px 26px 2px rgba(0,0,0,0.14),0px 6px 32px 5px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 9px 11px -5px rgba(0,0,0,0.2),0px 18px 28px 2px rgba(0,0,0,0.14),0px 7px 34px 6px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 9px 12px -6px rgba(0,0,0,0.2),0px 19px 29px 2px rgba(0,0,0,0.14),0px 7px 36px 6px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 10px 13px -6px rgba(0,0,0,0.2),0px 20px 31px 3px rgba(0,0,0,0.14),0px 8px 38px 7px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 10px 13px -6px rgba(0,0,0,0.2),0px 21px 33px 3px rgba(0,0,0,0.14),0px 8px 40px 7px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 10px 14px -6px rgba(0,0,0,0.2),0px 22px 35px 3px rgba(0,0,0,0.14),0px 8px 42px 7px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 11px 14px -7px rgba(0,0,0,0.2),0px 23px 36px 3px rgba(0,0,0,0.14),0px 9px 44px 8px rgba(0,0,0,0.12)"),
            ShadowSpec("0px 11px 15px -7px rgba(0,0,0,0.2),0px 24px 38px 3px rgba(0,0,0,0.14),0px 9px 46px 8px rgba(0,0,0,0.12)"),
        ])
    }
}

impl<T> std::ops::Index<T> for Shadows
where
    [ShadowSpec; 25]: std::ops::Index<T, Output = ShadowSpec>,
{
    type Output = ShadowSpec;

    fn index(&self, index: T) -> &Self::Output {
        self.0.index(index)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Palette {
    pub primary: ColorSpec,
    pub secondary: ColorSpec,
    pub error: ColorSpec,
    pub warning: ColorSpec,
    //
    pub text: TextColorSpec,
    //
    pub actions: PaletteActions,
    pub background: PaletteBackground,
    pub divider: CssColor,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            primary: ColorSpec {
                light: "#7986cb".try_into().expect(""),
                main: "#3f51b5".try_into().expect(""),
                dark: "#303f9f".try_into().expect(""),
                contrast: "#fff".try_into().expect(""),
            },
            secondary: ColorSpec {
                light: "#ff4081".try_into().expect(""),
                main: "#f50057".try_into().expect(""),
                dark: "#c51162".try_into().expect(""),
                contrast: "#fff".try_into().expect(""),
            },
            error: ColorSpec {
                light: "#e57373".try_into().expect(""),
                main: "#f44336".try_into().expect(""),
                dark: "#d32f2f".try_into().expect(""),
                contrast: "#fff".try_into().expect(""),
            },
            warning: ColorSpec {
                light: "#ffb74d".try_into().expect(""),
                main: "#ff9800".try_into().expect(""),
                dark: "#f57c00".try_into().expect(""),
                contrast: CssColor::rgba(0, 0, 0, 0.87),
            },
            text: Default::default(),
            actions: Default::default(),
            background: Default::default(),
            divider: CssColor::rgba(0, 0, 0, 0.12),
        }
    }
}

impl Palette {
    pub fn contrast_text_color(&self, _background: CssColor) -> CssColor {
        // FIXME: return light color for dark background
        self.text.primary
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
    use_memo(theme, theme_to_styles)
}
