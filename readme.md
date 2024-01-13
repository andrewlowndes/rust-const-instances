# Expr macros
An experiment of generating expression structures/enums in macros so they can be used directly in programs. If the properties of the struct are const-compatible the result from the macro can be used in a const context.

## How it works
A trait `ToExpr` is defined in `crates/core` with implementation for std types. The signaure includes a function `to_expr` to print itself as tokens.
```rust
trait ToExpr {
    fn to_expr(&self) -> TokenStream;
}
```

There is a helper derive macro `ToExpr` defined in `crates/macro` that auto-implements the trait on structs and enums when defined.

## Example usage
1. Define a library of structs/enums you will create and use (see `crates/example_lib`)
2. Define a macro that creats object instances and returns their expressions (see `crates/example_macro`)
3. Use the macro and the library in your rust code (see `crates/example_bin`)

Run the demo via `cargo run -p to-expr-example-bin`.

## Where this may be useful
This may prove useful where you want to use const expressions but generate the values for these in macros. Possible other usage would involve parsing config/json files via serde at compile-time.
