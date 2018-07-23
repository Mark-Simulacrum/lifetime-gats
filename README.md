# Lifetime-based emulation of Generic Associated Types

**This implementation has not been verified as correct. It may not be sound.**

This crate provides two primary types, `Reference` and `ReferenceMut` which permit writing traits
and implementations that are generic over lifetimes.

You can see the example which implements such a trait with normal reference return types and another
implementation which returns structs parameterized over the lifetime. This is a zero-cost
abstraction.
