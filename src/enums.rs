/// A type representing a letter case.
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
pub enum LetterCase {
    /// Lower case letter (e.g. 'a', 'b')
    #[default]
    Lowercase,
    /// Upper case letter (e.g. 'A', 'B')
    Uppercase,
}
