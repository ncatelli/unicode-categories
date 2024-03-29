# unicode-categories
This crate provides a method and Extension Trait on top of the `char` type
for returning a corresponding Unicode General Category as defined in the
[latest standard](https://www.unicode.org/versions/Unicode15.0.0/UnicodeStandard-15.0.pdf).

## Including the crate
For now the crate can be included by branch or by tag via git.

```toml
#[dependencies]
unicode-categories = { git = "https://github.com/ncatelli/unicode-categories.git", branch = "main" }
```

## Examples

Using the `char` type extension trait.

```rust
use unicode_categories::*;

assert_eq!(Some(Category::Lu), 'A'.unicode_category());
assert_eq!(Some(Category::Ll), 'a'.unicode_category());
```

Using the included conversion method:

```rust
use unicode_categories::*;

assert_eq!(Some(Category::Lu), unicode_category_from_char('A'));
assert_eq!(Some(Category::Ll), unicode_category_from_char('a'));
```
