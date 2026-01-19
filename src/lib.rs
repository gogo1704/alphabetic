#![warn(clippy::pedantic)]
#![warn(missing_docs, rustdoc::all)]
#![warn(clippy::cargo)]
/*!
# Alphabetic

"Alphabetic" is simple crate featuring [`AlphabeticLetter`] struct, that represents a letter in Latin-script alphabet.

Provides [`shift()`](AlphabeticLetter::shift()) method that shifts [`AlphabeticLetter`] forward or backward in alphabet. Wraps around when reaching the end.

# Examples
```
# use alphabetic::{AlphabeticLetter};
# fn main() -> Result<(),&'static str> {
let mut letter = AlphabeticLetter::try_from('A')?;
letter.shift(5);
assert_eq!(char::from(letter),'F');
# Ok(())
# }
```
Changing first letter of a string:
```
# use alphabetic::{AlphabeticLetter,LetterCase};
let mut vector = AlphabeticLetter::from_string("Rust").unwrap();
vector.first_mut().unwrap().shift(-5);
let new_string = vector.into_iter().map(char::from).collect::<String>();
assert_eq!(new_string, "Must");
```
*/
mod alphabetic;
mod enums;
mod error;
pub use crate::alphabetic::*;
pub use crate::enums::*;
pub use crate::error::*;
