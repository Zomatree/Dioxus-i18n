use dioxus::prelude::*;
use std::{cell::RefCell, collections::HashMap};
use unic_langid::LanguageIdentifier;

mod backends;
mod hooks;
mod storage;
mod components;
mod provider;

pub use backends::*;
pub use hooks::*;
pub use components::*;

#[cfg(all(feature = "web", feature = "desktop"))]
compile_error!("Only one of the web and desktop features can be used");

#[cfg(not(any(feature = "web", feature = "desktop")))]
compile_error!("No features set, one of web or desktop must be used");

#[derive(Clone, Copy)]
pub enum Argument<'a> {
    String(&'a str),
    Number(f64),
    Float(f64),
}

fn get_element_id<'a>(node: Option<&'a VNode>) -> &'a str {
    let node = node.expect("Missing Text id");
    let template = node.template.get();
    let root = template.roots.get(0).expect("No children on Text");

    match root {
        TemplateNode::Text { text } => *text,
        _ => panic!("Expected static text for id")
    }
}
