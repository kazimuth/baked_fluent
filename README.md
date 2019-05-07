# baked_fluent

A system for dead-easy i18n in rust. Bakes [Fluent](https://projectfluent.org) source files into executables and provides an easy API to use them. Intended for server-side use; integrates with most [web frameworks](#frameworks) and [templating libraries](#templating).

Example (with [`actix-web`](https://actix.rs/)):

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

`src/main.rs`:

```rust

use actix_web::{server, App, HttpRequest, Responder};

use baked_fluent::{localize, impl_localize};

// Create a struct called `Localizer` implementing `baked_fluent::Localize`
impl_localize! {
    #[path = "i18n", default_locale = "en_US"]
    pub struct Localizer(_);
}

fn index((loc, name, friend_count): (Localizer, Path<String>, Path<i32>)) -> String {
    localize!(loc, greeting, name=name, friends=friend_count)
}

fn main() {
    let app = App::new().resource(
        "/{name}/{friend_count}",                    // <- define path parameters
        |r| r.method(http::Method::GET).with(index));  // <- use `with` extractor
}
```

In the terminal:

```sh
$ cargo run &
...
$ curl -H 'Accept-Language: en' http://localhost:8088/Jamie/12
Hello, Jamie! You have 12 friends.
$ curl -H 'Accept-Language: es' http://localhost:8088/Jamie/12
¡Hola, Jamie! Tienes 12 amigos.
$ curl -H 'Accept-Language: es' http://localhost:8088/Jamie/0
¡Hola, Jamie! Todavía no tienes amigos 😞
$ curl -H 'Accept-Language: de_DE,de,en_UK,en_US,en' http://localhost:8088/Jamie/1
Hello, Jamie! You have 1 friend.

```
