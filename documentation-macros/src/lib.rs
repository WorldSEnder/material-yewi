extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::quote;
use syn::{Error, Lit, LitStr};
use unindent::unindent;

/// Document an example by showing the code along-side the result.
/// Takes a single string literal that can be parsed into an expression
/// resulting in the example to display.
///
/// Since we can't recover the exact input to a proc-macro, wrapping everything
/// in a string is necessary. Use raw strings with delimiters for good results.
///
/// # Usage
///
/// ```ignore // Ignore since we don't import yew here...
/// document_example!(r##"
/// yew::html! {
///     <span>I result in a span!</span>
/// }
/// "##);
/// ```
///
/// Written the above way, you can quickly uncomment the first and last line
/// to get syntax checking inside the string literal while writing. Uncomment the
/// two lines to insert the additional wrapping around the example that should
/// show up on the webpage along side the example.
///
#[proc_macro]
pub fn document_example(example: TokenStream) -> TokenStream {
    // TODO: implement doc-string like comments that won't be displayed
    // in the output but compiled in?
    let lit: LitStr = syn::parse(example)
        .and_then(|l| match l {
            Lit::Str(s) => Ok(s),
            l => Err(Error::new(l.span(), "expected a string literal")),
        })
        .expect(concat!(
            "This macro takes exactly one literal string.\n",
            "This is because it must emit the example literally into the output html."
        ));

    let example_clode: proc_macro2::TokenStream =
        lit.value().parse().expect("Expected valid rust code");

    let formatted_lit = LitStr::new(&unindent(&lit.value()), lit.span());
    let html_ident = Ident::new("built_documentation_html", Span::mixed_site());

    let macro_result = quote! {
        let #html_ident: ::yew::Html = { #example_clode };

        {
            ::yew::html! {
                <>
                    <pre>{#formatted_lit}</pre>
                    <::material_yewi_documentation_utils::demo::Demo>{#html_ident}</::material_yewi_documentation_utils::demo::Demo>
                </>
            }
        }
    };
    macro_result.into()
}
