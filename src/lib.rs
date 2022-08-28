use dioxus::{prelude::*, core::Attribute};
use dioxus::core::AttributeValue;
use std::{rc::Rc, cell::{Cell, RefCell}, ops::Deref, collections::HashMap};
use unic_langid::LanguageIdentifier;

#[cfg(feature = "fluent")]
mod fluent;
#[cfg(feature = "fluent")]
pub use crate::fluent::Fluent;

#[cfg(all(feature = "web", feature = "desktop"))]
compile_error!("Only one of the web and desktop features can be used");

#[cfg(not(any(feature = "web", feature = "desktop")))]
compile_error!("No features set, one of web or desktop must be used");

pub enum Argument<'a> {
    String(&'a str),
    Number(f64)
}

pub trait TranslationsProvider {
    fn format_string<'a>(&'a self, id: &'a str, locale: &'a LanguageIdentifier, args: HashMap<&'a str, Argument<'a>>) -> String;
}

pub struct Translations<T: TranslationsProvider>(RefCell<TranslationsInner<T>>);

impl<T: TranslationsProvider> Deref for Translations<T> {
    type Target = RefCell<TranslationsInner<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: TranslationsProvider> Translations<T> {
    pub fn change_locale(&self, new_locale: LanguageIdentifier) {
        self.0.borrow_mut().current_locale = new_locale
    }
}

pub struct TranslationsInner<T: TranslationsProvider> {
    provider: T,
    current_locale: LanguageIdentifier
}

impl<T: TranslationsProvider> TranslationsInner<T> {
    pub fn format_string<'a>(&'a self, id: &'a str, args: HashMap<&'a str, Argument<'a>>) -> String {
        self.provider.format_string(id, &self.current_locale, args)
    }
}

pub fn use_translations<T: TranslationsProvider + 'static>(cx: &ScopeState) -> &Rc<Translations<T>> {
    cx.use_hook(|| cx.consume_context::<Rc<Translations<T>>>()
        .expect("use_translations called outside of Translations component"))
}

pub fn use_current_locale<T: TranslationsProvider + 'static>(cx: &ScopeState, default: LanguageIdentifier) -> LanguageIdentifier {
    match cx.consume_context::<Rc<Translations<T>>>() {
        Some(translations) => translations.borrow().current_locale.clone(),
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
                todo!()
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

}

#[derive(Props)]
pub struct TranslationsProviderProps<'a, T: TranslationsProvider> {
    provider: Cell<Option<T>>,
    default_locale: LanguageIdentifier,
    children: Element<'a>
}

#[allow(non_snake_case)]
pub fn TranslationsProvider<'a, T: TranslationsProvider + 'static>(cx: Scope<'a, TranslationsProviderProps<'a, T>>) -> Element<'a> {
    cx.use_hook(|| {
        let current_locale = use_current_locale::<T>(&cx, cx.props.default_locale.clone());

        let inner = TranslationsInner {
            provider: cx.props.provider.take().unwrap(),
            current_locale
        };

        cx.provide_context(Rc::new(Translations(RefCell::new(inner))))
    });

    cx.render(rsx! {
        &cx.props.children
    })
}

#[derive(Props)]
pub struct TextProps<'a, T: TranslationsProvider> {
    id: &'a str,
    translations: &'a Rc<Translations<T>>,

    #[props(optional)]
    attributes: Option<&'a [Attribute<'a>]>
}

#[allow(non_snake_case)]
pub fn Text<'a, T: TranslationsProvider>(cx: Scope<'a, TextProps<'a, T>>) -> Element<'a> {
    let translations = cx.props.translations;

    let attrs = cx.props.attributes.map_or_else(HashMap::new, |attrs| attrs.iter().map(|attr| {
        (attr.name, match attr.value {
            AttributeValue::Text(text) => Argument::String(text),
            AttributeValue::Float32(n) => Argument::Number(n as f64),
            AttributeValue::Float64(n) => Argument::Number(n),
            AttributeValue::Int32(n) => Argument::Number(n as f64),
            AttributeValue::Int64(n) => Argument::Number(n as f64),
            AttributeValue::Uint32(n) => Argument::Number(n as f64),
            AttributeValue::Uint64(n) => Argument::Number(n as f64),
            AttributeValue::Bool(b) => Argument::Number(b as u8 as f64),
            _ => panic!("Argument type not supported, only text, numbers and bools are sup")
        })
    }).collect());

    let inner = translations.borrow();

    let text = inner.format_string(cx.props.id, attrs);

    cx.render(rsx! {
        "{text}"
    })
}
