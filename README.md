fixbomb - A FIXME bomb rustc plugin
===================================

`fixbomb` is a `rustc` plugin which trigger a compilation error whenever it finds an expired `fixbomb` attribute.

Usage
-----

In order to ignite the fixbomb, you need to annotate the "fixed" item with the `fixbomb` attribute, with some named arguments:
- `date`: Required. The date at which the bomb will trigger, must be formatted in ISO 8601.
- `message`: Optional. A message you want to display when the bomb trigger.

An example:

`Cargo.toml`:
```toml
[dependencies]
fixbomb = { git = "https://github.com/KokaKiwi/rust-fixbomb" }
```

`lib.rs`:
```rust
#![feature(plugin)]
#![plugin(fixbomb)]

#[fixbomb(date = "2016-07-27T20:37:23+02:00", message = "You did something wrong, remember?")]
fn bombed() {
    // FIXME: I did something wrong.
}
```

And when compiling:
```
error: Fixbomb triggered: You did something wrong, remember?
 --> examples/simple.rs:4:1
  |>
4 |> #[fixbomb(date = "2016-06-27T16:29:47+02:00", message = "You did something wrong, remember?")]
  |> ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
```
