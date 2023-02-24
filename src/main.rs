use std::{collections::HashMap, io};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
struct Coordinate {
    file: i8,
    rank: i8,
}

impl Coordinate {
    fn new(f: char, r: i8) -> Option<Coordinate> {
        let f = (f as u8 - 96) as i8;

        if f > 0 && f < 9 && r > 0 && r < 9 {
            Some(Coordinate { file: f, rank: r })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    color: Color,
    doubled_last_turn: bool,
    has_moved: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
    from: Coordinate,
    to: Coordinate,
}

impl Move {
    fn alg(&self) -> String {
        format!(
            "{}{}{}{}",
            (self.from.file as u8 + 96) as char,
            self.from.rank,
            (self.to.file as u8 + 96) as char,
            self.to.rank
        )
    }
}

struct Board {
    pieces: HashMap<Coordinate, Piece>,
    turn_color: Color,
    moves: Vec<Move>,
}

impl Board {
    fn new() -> Board {
        let mut board = HashMap::new();

        for c in [Color::White, Color::Black] {
            let pawn_rank = match c {
                Color::White => 2,
                Color::Black => 7,
            };
            let other_rank = match c {
                Color::White => 1,
                Color::Black => 8,
            };

            for f in 1..=8 {
                board.insert(
                    Coordinate {
                        file: f,
                        rank: pawn_rank,
                    },
                    Piece {
                        piece_type: PieceType::Pawn,
                        color: c,
                        doubled_last_turn: false,
                        has_moved: false,
                    },
                );
            }
            for f in [1, 8] {
                board.insert(
                    Coordinate {
                        file: f,
                        rank: other_rank,
                    },
                    Piece {
                        piece_type: PieceType::Rook,
                        color: c,
                        doubled_last_turn: false,
                        has_moved: false,
                    },
                );
            }
            for f in [2, 7] {
                board.insert(
                    Coordinate {
                        file: f,
                        rank: other_rank,
                    },
                    Piece {
                        piece_type: PieceType::Knight,
                        color: c,
                        doubled_last_turn: false,
                        has_moved: false,
                    },
                );
            }
            for f in [3, 6] {
                board.insert(
                    Coordinate {
                        file: f,
                        rank: other_rank,
                    },
                    Piece {
                        piece_type: PieceType::Bishop,
                        color: c,
                        doubled_last_turn: false,
                        has_moved: false,
                    },
                );
            }
            board.insert(
                Coordinate {
                    file: 4,
                    rank: other_rank,
                },
                Piece {
                    piece_type: PieceType::Queen,
                    color: c,
                    doubled_last_turn: false,
                    has_moved: false,
                },
            );
            board.insert(
                Coordinate {
                    file: 5,
                    rank: other_rank,
                },
                Piece {
                    piece_type: PieceType::King,
                    color: c,
                    doubled_last_turn: false,
                    has_moved: false,
                },
            );
        }
        Board {
            pieces: board,
            turn_color: Color::White,
            moves: Vec::new(),
        }
    }

    fn print_position(&self) {
        println!("{}", String::from_iter(['-'; 16]));
        for r in (1..=8).rev() {
            for f in 1..=8 {
                match self.pieces.get(&Coordinate { file: f, rank: r }) {
                    Some(piece) => print!(
                        "|{}",
                        match piece.color {
                            Color::White => match piece.piece_type {
                                PieceType::Pawn => "♙",
                                PieceType::Knight => "♘",
                                PieceType::Bishop => "♗",
                                PieceType::Rook => "♖",
                                PieceType::Queen => "♕",
                                PieceType::King => "♔",
                            },
                            Color::Black => match piece.piece_type {
                                PieceType::Pawn => "♟︎",
                                PieceType::Knight => "♞",
                                PieceType::Bishop => "♝",
                                PieceType::Rook => "♜",
                                PieceType::Queen => "♛",
                                PieceType::King => "♚",
                            },
                        }
                    ),
                    None => print!("| "),
                }
            }
            println!("|{}", r);
            println!("{}", String::from_iter(['-'; 16]));
        }
        println!(" a b c d e f g h")
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for p in self.pieces.iter().filter(|p| p.1.color == self.turn_color) {
            match p.1.piece_type {
                PieceType::Pawn => {
                    let dir = match p.1.color {
                        Color::White => 1,
                        Color::Black => -1,
                    };
                    let up1 = {
                        let mut temp = *p.0;
                        temp.rank += dir;
                        temp
                    };
                    let up2 = {
                        let mut temp = up1;
                        temp.rank += dir;
                        temp
                    };
                    let upl = {
                        let mut temp = up1;
                        temp.file -= dir;
                        temp
                    };
                    let upr = {
                        let mut temp = up1;
                        temp.file += dir;
                        temp
                    };
                    if self.pieces.get(&up1).is_none() {
                        moves.push(Move {
                            from: *p.0,
                            to: up1,
                        });
                    }
                    if !p.1.has_moved && self.pieces.get(&up2).is_none() {
                        moves.push(Move {
                            from: *p.0,
                            to: up2,
                        });
                    }
                    if self.pieces.get(&upl).is_some()
                        && self.pieces.get(&upl).unwrap().color != self.turn_color
                    {
                        moves.push(Move {
                            from: *p.0,
                            to: upl,
                        })
                    };
                    if self.pieces.get(&upr).is_some()
                        && self.pieces.get(&upr).unwrap().color != self.turn_color
                    {
                        moves.push(Move {
                            from: *p.0,
                            to: upr,
                        })
                    };
                }
                PieceType::Knight => {
                    for (x, y) in [
                        (-2, -1),
                        (-1, -2),
                        (-2, 1),
                        (-1, 2),
                        (2, 1),
                        (1, 2),
                        (2, -1),
                        (1, -2),
                    ] {
                        let d = {
                            let mut temp = *p.0;
                            temp.file += x;
                            temp.rank += y;
                            temp
                        };
                        if d.file > 0
                            && d.file < 9
                            && d.rank > 0
                            && d.rank < 9
                            && match self.pieces.get(&d) {
                                Some(piece) => piece.color != self.turn_color,
                                None => true,
                            }
                        {
                            moves.push(Move { from: *p.0, to: d })
                        }
                    }
                }
                _ => {}
            }
        }
        moves
    }

    fn make_move(&mut self, mov: &Move) -> Result<(), ()> {
        if self.legal_moves().contains(mov) {
            let mut p = *self.pieces.get(&mov.from).unwrap();
            p.has_moved = true;
            for dbl in self.pieces.iter_mut().filter(|d| d.1.doubled_last_turn) {
                dbl.1.doubled_last_turn = false;
            }
            if p.piece_type == PieceType::Pawn && (mov.to.rank - mov.from.rank).abs() == 2 {
                p.doubled_last_turn = true;
            }

            self.pieces.remove(&mov.from);
            self.pieces.insert(mov.to, p);

            self.moves.push(*mov);

            if self.turn_color == Color::White {
                self.turn_color = Color::Black;
            } else {
                self.turn_color = Color::White
            }

            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_alg(&self, s: &String) -> Result<Move, ()> {
        let mut mov = Move {
            from: Default::default(),
            to: Default::default(),
        };

        mov.from = match s.chars().nth(0) {
            Some(file) => match s.chars().nth(1) {
                Some(rank) => match Coordinate::new(
                    file,
                    match rank.to_digit(9) {
                        Some(d) => d,
                        None => return Err(()),
                    } as i8,
                ) {
                    Some(c) => c,
                    None => return Err(()),
                },
                None => return Err(()),
            },
            None => return Err(()),
        };

        mov.to = match s.chars().nth(2) {
            Some(file) => match s.chars().nth(3) {
                Some(rank) => match Coordinate::new(
                    file,
                    match rank.to_digit(9) {
                        Some(d) => d,
                        None => return Err(()),
                    } as i8,
                ) {
                    Some(c) => c,
                    None => return Err(()),
                },
                None => return Err(()),
            },
            None => return Err(()),
        };

        Ok(mov)
    }
}

fn main() {
    let mut board = Board::new();

    board.print_position();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match board.parse_alg(&input) {
            Ok(mov) => match board.make_move(&mov) {
                Ok(()) => board.print_position(),
                Err(()) => println!("Illegal move. Please make a legal move."),
            },
            Err(()) => println!("Error parsing move. Please use valid long algebraic notation."),
        }
    }
}
