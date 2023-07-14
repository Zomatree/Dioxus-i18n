use dioxus::{prelude::*, core::Attribute};
use dioxus::core::AttributeValue;
use std::{rc::Rc, cell::{Cell, RefCell}, collections::HashMap};
use unic_langid::LanguageIdentifier;

#[cfg(feature = "fluent")]
mod fluent;
#[cfg(feature = "fluent")]
pub use crate::fluent::Fluent;

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

pub trait TranslationsProvider {
    fn format_string<'a>(&'a self, id: &'a str, locale: &'a LanguageIdentifier, args: HashMap<&'a str, Argument<'a>>) -> String;
}

pub struct Translations(RefCell<TranslationsInner>);

impl Translations {
    pub fn change_locale(&self, new_locale: LanguageIdentifier) {
        self.0.borrow_mut().current_locale = new_locale
    }

    pub fn current_locale(&self) -> LanguageIdentifier {
        self.0.borrow().current_locale.clone()
    }

    pub fn format_string<'a>(&'a self, id: &'a str, args: HashMap<&'a str, Argument<'a>>) -> String {
        self.0.borrow().format_string(id, args)
    }
}

pub struct TranslationsInner {
    provider: Box<dyn TranslationsProvider>,
    current_locale: LanguageIdentifier
}

impl TranslationsInner {
    pub fn format_string<'a>(&'a self, id: &'a str, args: HashMap<&'a str, Argument<'a>>) -> String {
        self.provider.format_string(id, &self.current_locale, args)
    }
}

pub fn use_translations(cx: &ScopeState) -> &Rc<Translations> {
    cx.use_hook(|| cx.consume_context::<Rc<Translations>>()
        .expect("use_translations called before use_translations_provider called"))
}

pub fn use_current_locale<T: TranslationsProvider + 'static>(cx: &ScopeState, default: LanguageIdentifier) -> LanguageIdentifier {
    match cx.consume_context::<Rc<Translations>>() {
        Some(translations) => translations.current_locale(),
        None => {
            #[cfg(feature = "web")]
            {
                match storage::get::<_, LanguageIdentifier>("i18n-locale") {
                    Some(current) => current,
                    None => {
                        storage::set("i18n-locale", default.clone());
                        default
                    }
                }
            }
            #[cfg(feature = "desktop")]
            {
                default
            }
        }
    }
}

#[cfg(feature = "web")]
mod storage {
    use gloo::storage::{LocalStorage, Storage};
    use serde::{Serialize, Deserialize};

    pub fn set<K: AsRef<str>, V: Serialize>(key: K, value: V) {
        LocalStorage::set(key, value).unwrap()
    }

    pub fn get<K: AsRef<str>, V: for <'a> Deserialize<'a>>(key: K) -> Option<V> {
        LocalStorage::get(key).ok()
    }
}

#[cfg(feature = "desktop")]
mod desktop {
    // future persistant desktop storage here
}

pub fn use_translations_provider<P: TranslationsProvider + 'static>(cx: &ScopeState, default: LanguageIdentifier, hook: impl FnOnce() -> P) -> &mut Rc<Translations> {
    cx.use_hook(|| {
        let current_locale = use_current_locale::<P>(&cx, default);

        let inner = TranslationsInner {
            provider: Box::new(hook()),
            current_locale
        };

        cx.provide_context(Rc::new(Translations(RefCell::new(inner))))
    })
}

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

fn get_element_id<'a>(node: Option<&'a VNode>) -> &'a str {
    let node = node.expect("Missing Text id");
    let template = node.template.get();
    let root = template.roots.get(0).expect("No children on Text");

    match root {
        TemplateNode::Text { text } => *text,
        _ => panic!("Expected static text for id")
    }
}
