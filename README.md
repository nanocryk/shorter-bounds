# `shorter-bounds`

[![shorter-bounds crate](https://img.shields.io/crates/v/shorter-bounds?label=shorter-bounds)](https://crates.io/crates/shorter-bounds)
[![shorter-bounds documentation](https://img.shields.io/docsrs/shorter-bounds/latest?label=shorter-bounds%20docs)](https://docs.rs/shorter-bounds)

Provides a macro to easily define a trait alias, implemented automatically for all types that implement
the super traits. It supports both supertrait bounds (`trait Foo : Bounds`) and type parameters
bounds (`trait Foo<T: (Bounds)>`). The latter requires parenthesis around the bounds to easility
parse them and support any valid Rust bound syntax.

Since Rust 1.79, you can add bounds to traits associated types, which will also imply that bound
when the trait alias is used (contrary to using a `where` clause). This allows to define powerful
trait aliases that avoids repeating many `<Foo as Bar>::Baz : Traits`.

## Syntax

```rust
shorter_bounds::alias!(
    pub // Optional
    trait
    MyTraitAlias
    // Can be followed by type parameters
    <
        Foo,
        // Type paramter can have bounds, but they must be wrapped inside parenthesis
        Bar: (Clone + Iterator<Item: Clone>),
    >
    :
    // List of traits being aliased
    Clone +
        Iterator<Item = (Foo, Bar)>
);
```

## Exemple

```rust
shorter_bounds::alias!(pub trait IterableOfClonable: Iterator<Item: Clone>);
fn exemple(iter: impl IterableOfClonable) {
    for x in iter {
        let _ = x.clone();
    }
}
exemple([42, 63].into_iter());
```