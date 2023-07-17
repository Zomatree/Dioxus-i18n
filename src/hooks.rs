
use std::{rc::Rc, cell::RefCell};

use dioxus::prelude::*;
use unic_langid::LanguageIdentifier;

use crate::{storage, provider::{TranslationsProvider, Translations, TranslationsInner}};

pub fn use_translations_provider<P: TranslationsProvider + 'static>(cx: &ScopeState, default: LanguageIdentifier, hook: impl FnOnce() -> P) -> &mut Rc<Translations> {
    cx.use_hook(|| {
        let current_locale = storage::get("i18n-locale").unwrap_or(default);

        let inner = TranslationsInner {
            provider: Box::new(hook()),
            current_locale
        };

        cx.provide_context(Rc::new(Translations(RefCell::new(inner))))
    })
}

pub fn use_translations(cx: &ScopeState) -> &Rc<Translations> {
    cx.use_hook(|| cx.consume_context::<Rc<Translations>>()
        .expect("use_translations called before use_translations_provider called"))
}

pub fn use_current_locale<T: TranslationsProvider + 'static>(cx: &ScopeState) -> LanguageIdentifier {
    let translations = cx.consume_context::<Rc<Translations>>().expect("use_current_locale called before use_translations_provider");

    translations.current_locale()
}
