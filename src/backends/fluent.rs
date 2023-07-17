use fluent_bundle::{FluentBundle, FluentResource, FluentValue, types::{FluentNumber, FluentNumberOptions}};
use unic_langid::LanguageIdentifier;
use std::{collections::HashMap, borrow::{Borrow, Cow}};

use crate::{provider::TranslationsProvider, Argument};

pub struct Fluent<R: Borrow<FluentResource>> {
    bundles: HashMap<LanguageIdentifier, FluentBundle<R>>
}

impl<R: Borrow<FluentResource>> Fluent<R> {
    pub fn new(bundles: HashMap<LanguageIdentifier, FluentBundle<R>>) -> Self {
        Self {
            bundles
        }
    }
}

impl<R: Borrow<FluentResource>> TranslationsProvider for Fluent<R> {
    fn format_string<'a>(&'a self, id: &'a str, locale: &'a unic_langid::LanguageIdentifier, args: HashMap<&'a str, Argument<'a>>) -> String {
        let bundle = self.bundles.get(locale)
            .expect("Invalid locale");

        let message = bundle.get_message(id)
            .unwrap_or_else(|| panic!("No message with tag \"{id}\""));

        let pattern = message.value().expect("No message pattern");

        let mut errors = vec![];  // this is such a dumb design

        let args = args.into_iter().map(|(key, value)| {
            (key, match value {
                Argument::String(s) => FluentValue::String(Cow::Borrowed(s)),
                Argument::Number(n) => FluentValue::Number(FluentNumber::new(n, FluentNumberOptions::default())),
                Argument::Float(f) => FluentValue::Number(FluentNumber::new(f, FluentNumberOptions { style: fluent_bundle::types::FluentNumberStyle::Decimal, ..Default::default() }))
            })
        }).collect();

        let cow = bundle.format_pattern(pattern, Some(&args), &mut errors);

        cow.into_owned()
    }
}
