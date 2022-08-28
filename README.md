# Dioxus-i18n

internationalization for dioxus.

## Example

```rust
fn SubComponent(cx: Scope) -> Element {
    cx.render(rsx! {
        Text { "hello_world" }
    })
}

fn app(cx: Scope) -> Element {
    let en = "hello_world = Hello World!";

    let mut langs = HashMap::new();

    let mut bundle = FluentBundle::<FluentResource>::new(vec![langid!("en-GB")]);
    bundle.add_resource(FluentResource::try_new(en.to_string()).unwrap()).unwrap();

    langs.insert(langid!("en-GB"), bundle);

    cx.render(rsx! {
        TranslationsProvider {
            provider: Cell::new(Some(Fluent::new(langs))),
            default_locale: langid!("en-GB"),
            SubComponent {}
        }
    })
}
```
