# piece table rs
A implementation of a piece table in rust

## usage
`fn new(string: String) -> PieceTable`
create a new piece table

`fn generate_string(&self) -> String`
create a string from the piece table

`fn len(&self) -> usize`
get the lenght of the piece table

`fn insert(&mut self, index: usize, string: &str) -> Result<(), String>`
insert a value in the piece table, returns an error if the index is out of bounds

`fn delete(&mut self, index: usize) -> Result<(), String>`
delete a character on a given index, returns an error if the index is out of bounds

`fn delete_range(&mut self, from: usize, to: usize) -> Result<(), String>`
delete a range of characters (g), returns an error if the index is out of bounds or if from is bigger than to
