**i love toggling bools it's one of my favourite things to do**

Provides `fn toggle(&mut self)` on `bool` for toggling bools.

```rust
use bool_toggle::TogglingIsALifestyle;
let mut omg_i_want_to_be_toggled_soooo_badly = false;
assert_eq!(omg_i_want_to_be_toggled_soooo_badly, false);
omg_i_want_to_be_toggled_soooo_badly.toggle();
assert_eq!(omg_i_want_to_be_toggled_soooo_badly, true);
omg_i_want_to_be_toggled_soooo_badly.toggle();
assert_eq!(omg_i_want_to_be_toggled_soooo_badly, false);
```

```rust
let i_dont_want_to_be_toggled = false;
// That's okay.
```

## Enterprise license

This crate supports a professional re-export of the trait, `BoolToggleExt`.
It is only available when compiling with `--cfg enterprise_license` and obtaining an enterprise license.
For license inquiries, send mail to `/dev/null`.

## Architecture

Tis Crate features the world's most resilient bit filping algorithm. It's code adheres to the highest clean code standards
and aims to provide a maintainable and future proof solution to all the boolean toggling needs there are.

## MSRV

The minimum supported Rust version of this crate is 1.1000.0.
Lower versions might compile but are not supported.
