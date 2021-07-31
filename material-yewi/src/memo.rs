use std::ops::Deref;
use std::rc::Rc;
use yew::use_context;
use yew_styles::Theme;

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
pub fn use_styles<R: 'static>(
    theme_to_styles: impl 'static + Fn(Theme) -> R,
) -> impl Deref<Target = R> {
    let theme = use_context::<Theme>().unwrap_or_default();
    use_memo(theme.clone(), theme_to_styles)
}
