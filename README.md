# code-spells
Do you think Rust macros are a bit like magic? I do too!  
This crate aliases some common (and some less common) code snippets to macros with thematically appropriate names.  
E.g. the `drop` function can now be invoked with `obliviate!`, `Pin::new(&mut x)` can now be written as `immobulus!(&mut x)`, and if you want protection from harmful dementors the crate lets you cast `expecto_patronum!(expr, "message")` instead of calling `expr.expect("message")`.

```rust
let v1 = vec![erecto!(i32); 5];
let mut v2 = geminio!(&v1);
obliviate!(v1);
accio!(expecto_patronum!(v2.get_mut(0), "Dementors B-gone!")) = 5;
```
Also aliases `unsafe` to the macro `unforgivable!`, because what could be more unforgivable than undefined behaviour?

# Related crates
[expecto-patronum](https://crates.io/crates/expecto-patronum)