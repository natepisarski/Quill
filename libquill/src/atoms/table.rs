//! Tables are bindings between identifiers and values, analogous to "maps" in
//! some programming languages. The rval of a table value can be any atom, including
//! other tables.

use atoms::atom::*;

pub struct Table<T: Atom> {
    pub bindings: Vec<(String, T)>
}

impl<T: Atom> Atom for Table<T> { 
    fn get_type() -> QuillType {
        QuillType::Table
    }
}
