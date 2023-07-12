# Quill
Quill is a (barely) human-readable database language. Quill encompasses both the data-definion and CRUD syntax.

This executable is meant to work with flat-files, but in theory a server could be supported as well.

## Primitive Types
* `list`  (flat array)
* `table` (key/value pairs)
* `constant` (value of any type, including a list or table initializer)

A `list` can contain any Primitive Type, including others lists or Tables.

# Syntax
The syntax is "keyword <name> {definition} //comment", like so:

**Table Example**
```
table tableName {identifier1: 5, identifier2: 3}; // Binds tableName to the table constant denoted by {}'s`
```

**List Example**
```
list listName [5, 6, 8, *tableName] // Binds listName to the list constant denoted by []'s`. * uses a reference to a quill item rather than a constant.
```

## Keywords
* `reverse` - Reverse a list
* `remove` [list / table / "value from x"] - Remove a value from an atom
* `add` x `to` [list] or `add` x `to` [table] `as` y - Add a value to an atom
* `change` x `in` [table or list] `to` y - Change a value in an atom
* `print` x `in` {columns / aggregate} - Pretty-print or aggregate values
* `times{x}`[statement] - Repeat the statement in `{}` `x` times in a row

# Example
This example will create the word "quill" in a list called quillChars

    list quillChars [q, u, i]
	table otherChars {lastLetter: l}
	times{2}[add *otherChars:lastLetter to quillChars]
	print quillChars in aggregate // Prints "quill" to stdout

