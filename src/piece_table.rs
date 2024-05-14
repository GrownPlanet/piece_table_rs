#[derive(Copy, Clone)]
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
        let mut newlines = count_newlines(&string);

        // the last line won't be returned if there isn't a newline at the end
        if string.as_bytes()[string.len() - 1] as char != '\n' {
            newlines.push(string.len() - 1);
        }

        let piece = Piece::new(PieceType::Original, 0, string.len(), newlines);

        Self {
            original: string,
            added: String::new(),
            pieces: vec![piece],
        }
    }

    // TODO: 
    // - add some more error handling
    // - error handling when to or from is too big
    pub fn gen_string(&self, from: usize, to: usize) -> Vec<String> {
        if from > to {
            println!("`from` (= {}) is greater than `to` (= {})", from, to);
            std::process::exit(1);
        }

        let mut strings: Vec<String> = vec![String::new()];

        let mut passed_newlines = 0;

        for piece in self.pieces.iter() {
            let new_newlines = passed_newlines + piece.newlines.len();

            if new_newlines < from {
                continue;
            } else if passed_newlines >= to {
                return strings;
            }

            let start = if passed_newlines < from && from < new_newlines {
                from - passed_newlines
            } else {
                0
            };

            let end = if passed_newlines < to && to < new_newlines {
                to - passed_newlines
            } else {
                piece.newlines.len()
            };

            let buf = match piece.piece_type {
                PieceType::Added => &self.added,
                PieceType::Original => &self.original,
            };

            let i = strings.len() - 1;

            let mut to_push = match start {
                0 => &buf[piece.start..piece.newlines[start]],
                _ => &buf[(piece.newlines[start - 1] + 1)..=piece.newlines[start]],
            };

            strings[i].push_str(to_push);

            for i in (start + 1)..end {
                to_push = &buf[(piece.newlines[i - 1] + 1)..=piece.newlines[i]];
                strings.push(to_push.to_string());
            }

            passed_newlines = new_newlines;
        }

        return strings;
    }

    fn split_at(&mut self, at: usize) -> Result<usize, String> {
        let mut passed_size = 0;

        for (i, piece) in self.pieces.iter().enumerate() {
            if passed_size < at && at < passed_size + piece.length {
                let buf = match piece.piece_type {
                    PieceType::Added => &self.added,
                    PieceType::Original => &self.original,
                };

                let p1_len = at - passed_size;
                let p1_newlines = count_newlines(&buf[piece.start..(piece.start + p1_len)]);
                let p1 = Piece::new(piece.piece_type, piece.start, p1_len, p1_newlines);

                let p2_len = (passed_size + piece.length) - at;
                let p2_newlines = count_newlines(&buf[p1_len..(piece.start + p2_len)]);
                let p2 = Piece::new(piece.piece_type, piece.start, p2_len, p2_newlines);

                self.pieces[i] = p1;
                self.pieces.insert(i + 1, p2);

                return Ok(i + 1);
            } else if at == passed_size {
                return Ok(i);
            } else if at == passed_size + piece.length {
                return Ok(i + 1);
            }
            passed_size += piece.length;
        }

        Err(String::from("`split_at` failed!"))
    }

    pub fn insert(&mut self, at: usize, string: &str) -> Result<(), String> {
        let start_index = self.added.len();
        let newlines = count_newlines(string);

        self.added.push_str(string);

        let i = self.split_at(at)?;

        self.pieces.insert(
            i,
            Piece::new(PieceType::Added, start_index, self.added.len() - start_index, newlines),
        );

        Ok(())
    }
}

fn count_newlines(string: &str) -> Vec<usize> {
    string
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '\n')
        .map(|(i, _)| i)
        .collect()
}
