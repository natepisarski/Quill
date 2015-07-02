//! Atoms are the basis of the Quill language. They can be either constants, lists, or tables.
//! The only capability that an atom itself must satiate is returning an enum stating which
//! of these three types of data it is.

/// The three different kinds of data found within quill.
pub enum QuillType {
    Constant,
    Table,
    List    
}

/// What kind of data am I?
pub trait Atom {
    fn get_type() -> QuillType;
}

/// This is the most used Constant in the Quill language. In fact,
/// I cannot at the time of writing this comment think of a time where
/// the base of Quill would want to be implemented for any other kind of
/// constant, but it can be.
impl Atom for String {
    fn get_type() ->  QuillType {
        QuillType::Constant
    }
}

impl Atom for Vec<String> {
    fn get_type() -> QuillType {
        QuillType::Constant
    }
}
