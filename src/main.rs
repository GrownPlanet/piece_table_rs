use piece_table::PieceTable;

mod piece_table;

fn main() -> Result<(), String> {
    /*
    let original = String::from("I am a piece table!");

    let mut piece_table = PieceTable::new(original);

    // insert values at the start, somewhere in the middle and at the end
    piece_table.insert(0, "start ")?;
    piece_table.insert(12, " somewhere in the middle")?;
    piece_table.insert(piece_table.len(), " end")?;

    // delete last char, first char, and a char somewhere in the middle
    piece_table.delete(piece_table.len() - 1)?;
    piece_table.delete(30)?;
    piece_table.delete(0)?;

    // delete a range of values
    piece_table.delete_range(0, 5)?;

    // generate a string from the piece table
    println!("{}", piece_table.generate_string());

    // get the length of the piece table
    println!("{}", piece_table.len());
    */

    let original = String::from(
        "I am a piece table!\nThis is line 2\nThe third line :D\nLine numero four\nFive lines will be enough\n",
    );

    let mut piece_table = PieceTable::new(original);

    piece_table.insert(0, "now all the other indexes are off >:)\n")?;
    piece_table.insert(70, "<<somewhere in the middle>>")?;
    piece_table.insert(piece_table.len() - 1, "<<I am a piece of text>>")?;

    let strings = piece_table.gen_string(0, 6);

    for (i, string) in strings.iter().enumerate() {
        println!("piece {}: {:?}", i + 1, string);
    }

    Ok(())
}
