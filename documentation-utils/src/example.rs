use crate::demo::Demo;
use material_yewi::typography::Typography;
use material_yewi::typography::TypographyVariant;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use stylist::{ast::sheet, yew::use_style};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub code_sample: String,
    #[prop_or_default]
    pub children: Children,
}

fn get_next_style_id() -> u64 {
    static CTR: Lazy<Arc<Mutex<u64>>> = Lazy::new(Arc::default);
    let mut ctr = CTR.lock().expect("Failed to lock Rng.");

    *ctr += 1;
    *ctr
}

fn use_unique_name(suggestion: &'static str) -> UseStateHandle<String> {
    use_state(move || format!("{}-{}", suggestion, get_next_style_id()))
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    let tab_styles = use_style(/* "tab", */ sheet!(
        & {
            display: none;
        }
        & + label {
            cursor: pointer;
            display: inline-block;
            text-align: center;

            -webkit-box-flex: 3;
            -ms-flex-positive: 3;
                    flex-grow: 3;
            -webkit-user-select: none;
            -moz-user-select: none;
            -ms-user-select: none;
                user-select: none;
            -webkit-transition: 0.3s background-color ease, 0.3s box-shadow ease;
            transition: 0.3s background-color ease, 0.3s box-shadow ease;
            height: 50px;
            box-sizing: border-box;
            padding: 15px;
        }
        &:checked + label {
            background-color: #ccc;
        }
    ));

    let tab_content_style = use_style(/* "tab-content", */ sheet!(
        background-color: transparent;
        left: 0;
        border-radius: 6px;

        position: fixed;
        height: 0;
        opacity: 0;
        visibility: hidden;
    ));
    let wrapper_style = use_style(/* "tab-wrapper", */ sheet!());

    let mk_tab_content_active = |i| {
        sheet!(
            .${wrapper_style.get_class_name()}
            > .${tab_styles.get_class_name()}:checked:nth-of-type(${i}) ~ &:nth-of-type(${i}) {
                position: relative;
                height: auto;
                visibility: initial;
                opacity: 1;

                -webkit-transition: visibility 0s, 0.5s opacity ease-in;
                transition: visibility 0s, 0.5s opacity ease-in;
            }
        )
    };
    let code_styles = use_style(/* "tab-code", */ mk_tab_content_active(1));
    let result_style = use_style(/* "tab-results", */ mk_tab_content_active(2));

    let code_id = use_unique_name("code-sample");
    let tabgroup_id = use_unique_name("code-example");
    let results_id = use_unique_name("code-results");

    ::yew::html! {
        <div class={classes![wrapper_style]}>
            <input type="radio" id={code_id.to_string()} name={tabgroup_id.to_string()} class={classes![tab_styles.clone()]} />
            <label for={code_id.to_string()}><Typography variant={TypographyVariant::Button}>{"Code sample"}</Typography></label>
            <input type="radio" id={results_id.to_string()} name={tabgroup_id.to_string()} class={classes![tab_styles]} checked={true} />
            <label for={results_id.to_string()}><Typography variant={TypographyVariant::Button}>{"Results"}</Typography></label>

            <div class={classes![tab_content_style.clone(), code_styles]}>
                <pre>{props.code_sample.clone()}</pre>
            </div>
            <div class={classes![tab_content_style, result_style]}>
                <Demo>
                    { for props.children.iter() }
                </Demo>
            </div>
        </div>
    }
}
