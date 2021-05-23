# BytesStr

An immutable UTF8 string backed by [Bytes](https://crates.io/crates/bytes).

Useful utility for storing views into UTF8 strings without allocating and copying.

This implementation has function that allow to create BytesStr from data that is partially UTF8 encoded.