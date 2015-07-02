# Quill
Quill is a data storage language, similar to a relational database. The quill client itself interacts with flat files, although with some libquil juggling you could in theory create a server to interact with quill databases.

The Quill language makes a distinction between lists and tables, both of which are Atoms of the quill language. Lists are lists of atoms (lists, tables, or constants). Tables are bindings between identifiers and atoms.

# Syntax
`table tableName {identifier1: 5, identifier2: 3}; // Binds tableName to the table constant denoted by {}'s`

`list listName [5, 6, 8, *tableName] // Binds listName to the list constant denoted by []'s`. * uses a reference to a quill item rather than a constant.

Special keywords:
* reverse - Reverse a list
* remove [list / table / "value from x"] - Remove a value from an atom
* add x to [list] or add x to [table] as y - Add a value to an atom
* change x in [table or list] to y - Change a value in an atom
* print x in {columns / aggregate} - Pretty-print or aggregate values
* times{x}[statement] - Repeat a statement x amount of times. Really, this is a preprocessing step and will literally copy the statement multiple times in the file.

# Example
This example will create the word "quill" in a list called quillChars

    list quillChars [q, u, i]
	table otherChars {lastLetter: l}
	times{2}[add *otherChars:lastLetter to quillChars]
	print quillChars in aggregate // Prints "quill" to stdout

