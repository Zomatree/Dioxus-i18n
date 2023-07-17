use std::{collections::HashMap, cell::RefCell};

use unic_langid::LanguageIdentifier;

use crate::{storage, Argument};


pub trait TranslationsProvider {
    fn format_string<'a>(&'a self, id: &'a str, locale: &'a LanguageIdentifier, args: HashMap<&'a str, Argument<'a>>) -> String;
}

pub struct Translations(pub(crate) RefCell<TranslationsInner>);

impl Translations {
    pub fn change_locale(&self, new_locale: LanguageIdentifier) {
        self.0.borrow_mut().current_locale = new_locale.clone();

        storage::set("i18n-locale", new_locale);
    }

    pub fn current_locale(&self) -> LanguageIdentifier {
        self.0.borrow().current_locale.clone()
    }

    pub fn format_string<'a>(&'a self, id: &'a str, args: HashMap<&'a str, Argument<'a>>) -> String {
        self.0.borrow().format_string(id, args)
    }
}

pub struct TranslationsInner {
    pub(crate) provider: Box<dyn TranslationsProvider>,
    pub(crate) current_locale: LanguageIdentifier
}

impl TranslationsInner {
    pub fn format_string<'a>(&'a self, id: &'a str, args: HashMap<&'a str, Argument<'a>>) -> String {
        self.provider.format_string(id, &self.current_locale, args)
    }
}
