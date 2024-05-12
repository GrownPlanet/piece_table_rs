enum PieceType {
    Added,
    Original,
}

struct Piece {
    piece_type: PieceType,
    start: usize,
    length: usize,
    newlines: Vec<usize>,
}

impl Piece {
    pub fn new(piece_type: PieceType, start: usize, length: usize, newlines: Vec<usize>) -> Self {
        Self {
            piece_type,
            start,
            length,
            newlines,
        }
    }
}

pub struct PieceTable {
    original: String,
    added: String,
    pieces: Vec<Piece>,
}

impl PieceTable {
    pub fn new(string: String) -> Self {
        let newlines = newline_positions(&string);
        let piece = Piece::new(PieceType::Original, 0, string.len(), newlines);

        Self {
            original: string,
            added: String::new(),
            pieces: vec![piece],
        }
    }

    pub fn gen_string(&self, from: usize, to: usize) -> Vec<&str> {
        let mut strings: Vec<&str> = vec![];

        for piece in self.pieces.iter() {
            match piece.piece_type {
                PieceType::Added => {
                    strings.push(&self.added[piece.start..(piece.start + piece.length)])
                }
                PieceType::Original => {
                    strings.push(&self.original[piece.start..(piece.start + piece.length)])
                }
            }
        }

        todo!()
    }
}

fn newline_positions(string: &str) -> Vec<usize> {
    string
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '\n')
        .map(|(i, _)| i)
        .collect()
}
