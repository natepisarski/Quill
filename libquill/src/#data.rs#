//! Data is the representation of a quill file. It is a "global database"
//! for quill data to interact with itself.

use atoms::atom::*;
use atoms::table::*;

/// This is the top-level binding for the Quill data model. It binds identifiers with
/// quill atoms. Because of how the binding works, this can be represented as a Quill
/// table itself.
pub struct Database<T: Atom> {
    qualified: Table<T>
}

