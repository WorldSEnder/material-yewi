use dependent_map::{DebugEntry, DynClone, DynPartialEq, HashableAny};
use std::hash::Hasher;
use std::fmt::Debug;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Theme {
    pub shape: Shape,
    pub breakpoints: Breakpoints,
    pub direction: Direction,
    pub palette: Palette,
    // shadows?: unknown;
    pub spacing: Spacing,
    // transitions?: unknown;
    pub components: Components,
    // mixins?: unknown;
    // typography?: unknown;
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
        dependent_map::EntryAt<E, A>: Eq + Clone,
        dependent_map::ValueAt<E, A>: Debug,
        dependent_map::KeyAt<E, A>: Debug,
);
type ComponentMap = dependent_map::Map<
    dependent_map::families::Singleton,
    dependent_map::DefaultHashBuilder,
    dyn ComponentsMapEntry<dependent_map::DefaultHasher>
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
    pub fn add_override<C: 'static + PartialEq + Clone + Debug>(&mut self, component_props: C) -> Option<C> {
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
    fn eq(&self, _rhs: &Components) -> bool {
        true
    }
}

pub type ThemeProvider = yew::ContextProvider<Theme>;
