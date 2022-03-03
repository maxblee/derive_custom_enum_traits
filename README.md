# derive_custom_enum_traits

This is a set of (currently one) derive macros for Rust for dealing with enums. Its current macro, `DeriveIndex`
creates two functions for an enum, `to_index`, which converts an enum variant into its 0-indexed order; and
`from_index` which converts a `usize` into an `Option<Variant>`.
