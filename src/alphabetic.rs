// TODO: Custom error type.
// TODO: Figure out better conversions.

use crate::{LetterCase, NotAlphabeticError, Result};
use std::fmt::Display;

/// A type representing a letter of Latin-script alphabet.
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct AlphabeticLetter {
    index: u8,
    case: LetterCase,
}

impl AlphabeticLetter {
    /// Number of letters in an alphabet.
    pub const ALPHABET_SIZE: u8 = 26;

    /**
    Constructs [`AlphabeticLetter`] from position in alphabet and letter case.

    # Example
    ```
    # use alphabetic::{AlphabeticLetter,LetterCase};
    let letter = AlphabeticLetter::from_index(2, LetterCase::Uppercase);
    assert_eq!(char::from(letter),'C');

    ```
    */
    #[must_use]
    pub fn from_index(index: u8, case: LetterCase) -> Self {
        AlphabeticLetter { index, case }
    }

    /**
    Constructs [`AlphabeticLetter`] from a string.

    # Example
    ```
    # use alphabetic::{AlphabeticLetter,LetterCase};
    let vector = AlphabeticLetter::from_string("Hi").unwrap();
    assert_eq!(char::from(vector.get(0).unwrap()),'H');
    assert_eq!(char::from(vector.get(1).unwrap()),'i');
    ```

    # Errors
    Function will error if `input` contains any non-alphabetic characters.

     */
    pub fn from_string(input: &str) -> Result<Vec<AlphabeticLetter>> {
        input
            .chars()
            .map(AlphabeticLetter::try_from)
            .collect::<Result<Vec<AlphabeticLetter>>>()
    }

    /**
    Returns position in alphabet.

    # Example
    ```
    # use alphabetic::{AlphabeticLetter, Result};
    # fn main() -> Result<()> {
    let letter = AlphabeticLetter::try_from('d')?;
    assert_eq!(letter.index(),3);
    # Ok(())
    # }
    ```
    */
    #[must_use]
    pub fn index(&self) -> u8 {
        self.index
    }

    /**
    Shifts [`AlphabeticLetter`] by `amount` places forward or backward in alphabet. Wraps around when reaching the end.

    # Examples
    ```
    # use alphabetic::{AlphabeticLetter, Result};
    # fn main() -> Result<()> {
    let mut letter = AlphabeticLetter::try_from('A')?;
    assert_eq!(char::from(letter.shift(5)),'F');
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
    pub fn shift(&mut self, amount: i32) -> &Self {
        // Casting here should be safe, because of modulo and adding positive integer.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let offset: u8 =
            (amount % i32::from(Self::ALPHABET_SIZE) + i32::from(Self::ALPHABET_SIZE)) as u8;
        self.index = (self.index + offset) % Self::ALPHABET_SIZE;
        self
    }
}

impl TryFrom<u8> for AlphabeticLetter {
    type Error = NotAlphabeticError;

    fn try_from(value: u8) -> Result<AlphabeticLetter> {
        if value.is_ascii_lowercase() {
            Ok(AlphabeticLetter {
                index: value - b'a',
                case: LetterCase::Lowercase,
            })
        } else if value.is_ascii_uppercase() {
            Ok(AlphabeticLetter {
                index: value - b'A',
                case: LetterCase::Uppercase,
            })
        } else {
            Err(NotAlphabeticError)
        }
    }
}

impl From<AlphabeticLetter> for u8 {
    fn from(val: AlphabeticLetter) -> Self {
        match val.case {
            LetterCase::Lowercase => val.index + b'a',
            LetterCase::Uppercase => val.index + b'A',
        }
    }
}

impl TryFrom<char> for AlphabeticLetter {
    type Error = NotAlphabeticError;

    fn try_from(value: char) -> Result<Self> {
        if !value.is_ascii_alphabetic() {
            return Err(NotAlphabeticError);
        }
        let Ok(byte) = u8::try_from(value) else {
            return Err(NotAlphabeticError);
        };
        if value.is_ascii_lowercase() {
            return Ok(AlphabeticLetter {
                index: byte - b'a',
                case: LetterCase::Lowercase,
            });
        } else if value.is_ascii_uppercase() {
            return Ok(AlphabeticLetter {
                index: byte - b'A',
                case: LetterCase::Uppercase,
            });
        }
        unreachable!()
    }
}

impl From<AlphabeticLetter> for char {
    fn from(val: AlphabeticLetter) -> Self {
        match val.case {
            LetterCase::Lowercase => (val.index + b'a').into(),
            LetterCase::Uppercase => (val.index + b'A').into(),
        }
    }
}

impl From<&AlphabeticLetter> for char {
    fn from(val: &AlphabeticLetter) -> Self {
        match val.case {
            LetterCase::Lowercase => (val.index + b'a').into(),
            LetterCase::Uppercase => (val.index + b'A').into(),
        }
    }
}

impl Display for AlphabeticLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self.to_owned()))
    }
}

#[cfg(test)]
mod tests {

    use super::{AlphabeticLetter, LetterCase};

    #[test]
    fn from_u8_lowercase() -> Result<(), Box<dyn std::error::Error>> {
        let char_a = AlphabeticLetter::try_from(b'a')?;
        let char_z = AlphabeticLetter::try_from(b'z')?;
        assert_eq!(char_a.index, 0);
        assert_eq!(char_a.case, LetterCase::Lowercase);
        assert_eq!(char_z.index, 25);
        assert_eq!(char_a.case, LetterCase::Lowercase);
        Ok(())
    }
    #[test]
    fn from_u8_uppercase() -> Result<(), Box<dyn std::error::Error>> {
        let char_a = AlphabeticLetter::try_from(b'A')?;
        let char_z = AlphabeticLetter::try_from(b'Z')?;
        assert_eq!(char_a.index, 0);
        assert_eq!(char_a.case, LetterCase::Uppercase);
        assert_eq!(char_z.index, 25);
        assert_eq!(char_a.case, LetterCase::Uppercase);
        Ok(())
    }
    #[test]
    fn into_u8() {
        let char_a = AlphabeticLetter {
            index: 0,
            case: LetterCase::Lowercase,
        };
        let char_z = AlphabeticLetter {
            index: 25,
            case: LetterCase::Uppercase,
        };
        assert_eq!(u8::from(char_a), b'a');
        assert_eq!(u8::from(char_z), b'Z');
    }
    #[test]
    fn from_char() -> Result<(), Box<dyn std::error::Error>> {
        let char_b = AlphabeticLetter::try_from('b')?;
        let char_c = AlphabeticLetter::try_from('C')?;
        assert_eq!(char_b.index, 1);
        assert_eq!(char_b.case, LetterCase::Lowercase);
        assert_eq!(char_c.index, 2);
        assert_eq!(char_c.case, LetterCase::Uppercase);
        Ok(())
    }
    #[test]
    fn into_char() {
        let char_z = AlphabeticLetter {
            index: 25,
            case: LetterCase::Lowercase,
        };
        let char_x = AlphabeticLetter {
            index: 23,
            case: LetterCase::Uppercase,
        };
        assert_eq!(char::from(char_z), 'z');
        assert_eq!(char::from(char_x), 'X');
    }
    #[test]
    fn same_char_lowercase() -> Result<(), Box<dyn std::error::Error>> {
        let char = AlphabeticLetter::try_from('r')?;
        assert_eq!(char::from(char), 'r');
        Ok(())
    }
    #[test]
    fn same_char_uppercase() -> Result<(), Box<dyn std::error::Error>> {
        let char = AlphabeticLetter::try_from('R')?;
        assert_eq!(char::from(char), 'R');
        Ok(())
    }
    #[test]
    fn display() -> Result<(), Box<dyn std::error::Error>> {
        let char = AlphabeticLetter::try_from('B')?;
        assert_eq!(format!("{char}"), "B");
        Ok(())
    }
    #[test]
    fn empty_string() {
        let vector = AlphabeticLetter::from_string("").unwrap();
        assert_eq!(vector.len(), 0);
    }
}
