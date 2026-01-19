#![warn(clippy::pedantic)]
#![warn(missing_docs, rustdoc::all)]

use std::fmt::Display;
/// A type representing a letter case.
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
pub enum LetterCase {
    /// Lower case letter (e.g. 'a', 'b')
    #[default]
    Lowercase,
    /// Upper case letter (e.g. 'A', 'B')
    Uppercase,
}
/// A type representing a letter of Latin-script alphabet.
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct AlphabeticLetter {
    index: u8,
    case: LetterCase,
}

impl AlphabeticLetter {
    /// Number of letters in alphabet.
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
    Returns position in alphabet.

    # Example
    ```
    # use alphabetic::{AlphabeticLetter};
    # fn main() -> Result<(),&'static str> {
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
    Shifts [`index`](AlphabeticLetter::index) by `amount` places forward or backward in alphabet. Wraps around when reaching the end.

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
    ```
    # use alphabetic::{AlphabeticLetter};
    # fn main() -> Result<(),&'static str> {
    let mut letter = AlphabeticLetter::try_from('z')?;
    letter.shift(2);
    assert_eq!(char::from(letter),'b');
    # Ok(())
    # }
    ```
    ```
    # use alphabetic::{AlphabeticLetter};
    # fn main() -> Result<(),&'static str> {
    let mut letter = AlphabeticLetter::try_from('o')?;
    letter.shift(-3);
    assert_eq!(char::from(letter),'l');
    # Ok(())
    # }
    ```
    */
    pub fn shift(&mut self, amount: i32) {
        // Casting here should be safe, because of modulo and adding positive integer.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let offset: u8 =
            (amount % i32::from(Self::ALPHABET_SIZE) + i32::from(Self::ALPHABET_SIZE)) as u8;
        self.index = (self.index + offset) % Self::ALPHABET_SIZE;
    }
}

impl TryFrom<u8> for AlphabeticLetter {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
            Err("not a letter")
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
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_alphabetic() {
            return Err("not a letter");
        }
        let Ok(byte) = u8::try_from(value) else {
            return Err("not a letter");
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
}
