use crate::demo::Demo;
use css_in_rust::bindings::yew::gen_unique_name;
use css_in_rust::bindings::yew::use_scopes;
use material_yewi::typography::{Typography, TypographyVariant};
use std::convert::TryInto;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub code_sample: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    let tab_styles = use_scopes(
        "tab",
        r#"
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
        }"#
        .to_string()
        .try_into()
        .expect("unexpected error in css"),
    );

    let tab_content_style = use_scopes(
        "tab-content",
        r#"
        background-color: transparent;
        left: 0;
        -webkit-transform: translateY(-3px);
                transform: translateY(-3px);
        border-radius: 6px;

        position: fixed;
        height: 0;
        opacity: 0;
        visibility: hidden;
        "#
        .to_string()
        .try_into()
        .expect("unexpected error in css"),
    );
    let wrapper_style = use_scopes("tab-wrapper", Default::default());

    let mk_tab_content_active = |i| {
        format!(r#"
        .{wrapper} > .{class_tab}:checked:nth-of-type({i}) ~ &:nth-of-type({i}) {{
            position: relative;
            height: auto;
            visibility: initial;
            opacity: 1;

            -webkit-transition: visibility 0s, 0.5s opacity ease-in, 0.8s -webkit-transform ease;
            transition: visibility 0s, 0.5s opacity ease-in, 0.8s -webkit-transform ease;
            transition: visibility 0s, 0.5s opacity ease-in, 0.8s transform ease;
            transition: visibility 0s, 0.5s opacity ease-in, 0.8s transform ease, 0.8s -webkit-transform ease;
            -webkit-transform: translateY(0px);
                    transform: translateY(0px);
        }}
        "#,
            class_tab = tab_styles.to_string(),
            wrapper = wrapper_style.to_string(),
            i = i,
        )
        .try_into()
        .expect("unexpected error in css")
    };
    let code_styles = use_scopes("tab-code", mk_tab_content_active(1));
    let result_style = use_scopes("tab-results", mk_tab_content_active(2));

    let tabgroup_id = &*use_state(|| gen_unique_name("code-example"));
    let code_id = &*use_state(|| gen_unique_name("code-sample"));
    let results_id = &*use_state(|| gen_unique_name("code-results"));

    ::yew::html! {
        <div class={classes![&wrapper_style]}>
            <input type="radio" id={code_id.clone()} name={tabgroup_id.clone()} class={classes![&tab_styles]} />
            <label for={code_id.clone()}><Typography variant={TypographyVariant::Button}>{"Code sample"}</Typography></label>
            <input type="radio" id={results_id.clone()} name={tabgroup_id.clone()} class={classes![&tab_styles]} checked={true} />
            <label for={results_id.clone()}><Typography variant={TypographyVariant::Button}>{"Results"}</Typography></label>

            <div class={classes![&tab_content_style, &code_styles]}>
                <pre>{props.code_sample.clone()}</pre>
            </div>
            <div class={classes![&tab_content_style, &result_style]}>
                <Demo>
                    { for props.children.iter() }
                </Demo>
            </div>
        </div>
    }
}
