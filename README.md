# spellrs
Do you feel like macros are magic? I agree!
This crate creates some macro aliases for various thematically appropriate code snippets.  
E.g. the `drop` function can now be invoked with `obliviate!`, and `Pin::new(&mut x)` can now be written as `immobulus!(&mut x)`.  
If you want protection from harmful dementors the crate lets you cast `expecto_patronum!(expr, "message")` instead of calling `expr.expect("message")`.

# Related crates
[expecto-patronum](https://crates.io/crates/expecto-patronum)