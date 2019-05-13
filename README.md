# baked_fluent [![CircleCI](https://circleci.com/gh/kazimuth/baked_fluent.svg?style=svg)](https://circleci.com/gh/kazimuth/baked_fluent)

**THIS CRATE IS NOT YET FUNCTIONAL.** (give it a few days until a public beta.)

A system for dead-easy i18n in rust. Bakes [Fluent](https://projectfluent.org) source files into executables and provides an easy API to use them. Intended for server-side use; integrates with most [web frameworks](#frameworks) and [templating libraries](#templating).

## Example usage (with `actix-web`)

`Cargo.toml`:

```toml
# ...

[dependencies]
actix-web = "1.0.0-beta3"
baked_fluent = { version = "0.1.0", features = ["with-actix"]}
```

`src/main.rs`:

```rust
use actix_web::{web, App, HttpServer, Result};
use baked_fluent::{impl_localize, localize};

// Create a struct called `Localizer` implementing `baked_fluent::Localize`
impl_localize! {
    #[path("tests/i18n")]
    #[default_locale("en_US")]
    pub struct Localizer(_);
}

fn index((loc, info): (Localizer, web::Path<(String, isize)>)) -> Result<String> {
    Ok(localize!(
        loc,
        greeting,
        name = &info.0[..],
        friends = info.1
    )?)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/{name}/{friend_count}/").to(index)))
        .bind("localhost:8080")?
        .run()
}
```

`i18n/en_US/hello.ftl`:

```ftl
# english translations
greeting = Hello { $name }! { $friends ->
    [one] You have a friend!
    [zero] You have no friends yet 😞
   *[other] You have {$friends} friends.
}
```

`i18n/es_MX/hello.ftl`:

```ftl
# traducciones a español
greeting = ¡Hola, { $name }! { $friends ->
    [one] ¡Tienes un amigo!
    [zero] Todavía no tienes amigos 😞
   *[other] Tienes {$friends} amigos.
}
```

In the terminal:

```sh
$ cargo run &
...
$ curl -s -H 'Accept-Language: en' http://localhost:8080/Jamie/12
Hello, Jamie! You have 12 friends.
$ curl -s -H 'Accept-Language: es' http://localhost:8080/Jamie/12
¡Hola, Jamie! Tienes 12 amigos.
$ curl -s -H 'Accept-Language: es' http://localhost:8080/Jamie/0
¡Hola, Jamie! Todavía no tienes amigos 😞
$ curl -s -H 'Accept-Language: de_DE,de,en_UK,en_US,en' http://localhost:8080/Jamie/1
Hello, Jamie! You have 1 friend.
```

## FAQ

#### Is this gettext?

No, Project Fluent is an alternative API that does similar things to `gettext` but is designed to support a broader range of languages easily. If you really want gettext, [check crates.io](https://crates.io/search?q=gettext).

#### How does the fluent language work?

See the [guide](https://projectfluent.org/fluent/guide/) and the [other docs](https://github.com/projectfluent/fluent/wiki).

#### Can I use this crate from WebAssembly?

No! Don't do that. This crate bakes all translations into the output, it will significantly swell your wasm module sizes. Instead, use [fluent-bundle](https://crates.io/crates/fluent-locale) directly, and load your translations from a static file server. (There's room for a small utility crate that does this, if you feel like implementing one.) There's also [fluent-js](https://github.com/projectfluent/fluent.js).

You can, however, use this crate to negotiate a locale chain. Simply create a `Localize` instance as normal and then call `Localize::get_locale_chain()` to get the list of locales used by the instance. (You can also use [fluent-locale](https://crates.io/crates/fluent-locale) to do this on the frontend.)

#### Why isn't framework [Z] supported?

There's a lot of web frameworks / templating libraries. If you want to improve support for one of them, make a PR! Also, the `Localize` API is simple enough that it shouldn't be hard to just use it from your app.

#### How do I add support for a framework?

To add support for an imaginary web framework / templating library "xyz":

- Fork this repo.
- Add a feature "with-xyz" that brings in xyz as a dependency to baked_fluent.
- Add "with-xyz" as a dependency to the "full" or the "full-nightly" baked_fluent feature, depending on
  whether xyz requires nightly to compile.
- Add a module `integrations::xyz`, and add a doc comment explaining how to use xyz with baked_fluent.
  Make sure you've got working doctests (they should at least build.)
- If necessary, add a feature "with-xyz" to baked_fluent_codegen, which should be automatically activated by the
  baked_fluent with-xyz feature. Then generate whatever supporting code you need.
- If necessary, add configuration options for xyz support in the form of an annotation in impl_localize.
  (The parsing for these options should go in baked_fluent_codegen/src/input.rs).

  ```rust
  impl_localize! {
      #[xyz(option_a = "bees", option_b = "global warming")]
      struct Localizer(_);
  }
  ```

- Open a PR!

#### How fast is this crate?

```sh
# a simple locale negotiation:
negotiate-fast          time:   [4.5704 us 4.6237 us 4.6789 us]
# a locale negotiation with a long chain:
negotiate-slow          time:   [24.855 us 25.130 us 25.443 us]
# localizing a message with no arguments:
localize-simple         time:   [890.92 ns 901.70 ns 912.54 ns]
# localizing a message with a few arguments and a conditional:
localize-moderate       time:   [4.1089 us 4.1585 us 4.2078 us]
```

On an intel core i5 from 2014.

Performance mostly depends on the underlying `fluent-rs` implementation.
We accept performance-related PRs.

Note: this project loads all available translations into memory on the first call to `Localize::new`. This shouldn't add significant memory overhead, unless you have a _lot_ of translations.

#### Why doesn't this crate use #[derive(Localize)] instead of this weird macro thing?

It needs to control the contents of the `Localize` struct, so that won't work.
