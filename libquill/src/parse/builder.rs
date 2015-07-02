//! Builder is what turns the text itself into Quill atoms. It takes sanitized strings in
//! a given format and churns out a literal.

use citadel::access::encompassing::*;

use atoms::list::*;
use atoms::table::*;
use atoms::atom::*;

macro_rules! str_to {
    ($string:expr, $typ:ty) => ($string.to_string().chars().collect::<$typ>());
}

/// Given properly formatted text, produce a Quill list based on a literal of constants.
/// An example of a properly formatted list literal would be [1, 2, 3]
pub fn list_literal(text: &str) -> List<String> {
    
    let ref raw_data: Vec<char> = encompassed_by(&str_to!(text, Vec<char>), ('[', ']'))[0];
    let mut items: Vec<String> = vec![];

    for item in raw_data.split(|c| {c == &','}) {
        items.push(item.iter().cloned().collect());
    }

    List {data: items}
}

/// Given properly formatted text, produce a Quill table based on a set of literal constants.
/// An example of a properly formatted table literal would be {a: 1, b: 2}
pub fn table_literal(text: &str) -> Table<String> {
    
    let ref raw_data: Vec<char> = encompassed_by(&str_to!(text, Vec<char>), ('{', '}'))[0];
    let mut items: Vec<(String, String)> = vec![];
    
    for item in raw_data.split(|c| {c == &' '}) {
        let pair: Vec<&[char]> = item.split(|c| {c == &':'}).collect();
        items.push((pair[0].iter().cloned().collect(), pair[1].iter().cloned().collect()));
    }

    Table { bindings: items.clone() }
}

// TODO: Citadel Macro for .iter().cloned().collect()
