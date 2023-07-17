
use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{Argument, get_element_id, hooks::use_translations};

#[derive(Props)]
pub struct TextProps<'a> {
    #[props(optional)]
    args: Option<HashMap<&'a str, Argument<'a>>>,
    children: Element<'a>
}

#[allow(non_snake_case)]
pub fn Text<'a>(cx: Scope<'a, TextProps<'a>>) -> Element<'a> {
    let attrs = cx.props.args.as_ref().cloned().unwrap_or_default();

    let id = get_element_id(cx.props.children.as_ref());

    let translations = use_translations(&cx);

    let text = translations.format_string(id, attrs);

    cx.render(rsx! {
        "{text}"
    })
}
