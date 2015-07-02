//! Lists are lists of atoms.
use atoms::atom::*;

pub struct List<T: Atom> {
    pub data: Vec<T>,
}

impl<T: Atom> Atom for List<T> {
    fn get_type() -> QuillType {
        QuillType::List 
    }
}
