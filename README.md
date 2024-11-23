# code-spells
Do you think Rust macros are a bit like magic? I do too!  
This crate aliases some common (and some less common) code snippets to macros named after thematically appropriate spells from Harry Potter.  
E.g. the `drop` function can now be cast with `obliviate!`, `Pin::new()` with `immobulus!`, and if you want protection from harmful dementors this crate lets you cast `expecto_patronum!(expr, "message")` instead of calling `expr.expect("message")`.

```rust
let v1 = vec![erecto!(i32); 5];
let mut v2 = geminio!(&v1);
obliviate!(v1);
accio!(expecto_patronum!(v2.get_mut(0), "Dementors B-gone!")) = 5;
```
Also aliases `unsafe` to the macro `unforgivable!`, because what could be more unforgivable than undefined behaviour?

## Related crates
[expecto-patronum](https://crates.io/crates/expecto-patronum)

## Name handover

Since this crate is just a joke, if you have an idea for an actually useful crate for which you want this name, contact the author at the email in the `Cargo.toml`.

<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
