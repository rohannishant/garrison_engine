use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    file: i8,
    rank: i8,
}

impl Coordinate {
    fn new(f: char, r: i8) -> Coordinate {
        Coordinate {
            file: (f as u8 - 96) as i8,
            rank: r,
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
    capture: bool,
    piece_type: PieceType,
}

impl Move {
    fn alg(&self) -> String {
        format!(
            "{}{}{}{}",
            match self.piece_type {
                PieceType::Pawn => "",
                PieceType::Knight => "N",
                PieceType::Bishop => "B",
                PieceType::Rook => "R",
                PieceType::Queen => "Q",
                PieceType::King => "K",
            },
            if self.capture { "x" } else { "" },
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
                    if self.pieces.get(&up1).is_none() {
                        moves.push(Move {
                            from: *p.0,
                            to: up1,
                            capture: false,
                            piece_type: PieceType::Pawn,
                        });
                    }
                    if !p.1.has_moved && self.pieces.get(&up2).is_none() {
                        moves.push(Move {
                            from: *p.0,
                            to: up2,
                            capture: false,
                            piece_type: PieceType::Pawn,
                        });
                    }
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

                        let mut cap = false;
                        if d.file > 0
                            && d.file < 9
                            && d.rank > 0
                            && d.rank < 9
                            && match self.pieces.get(&d) {
                                Some(piece) => {
                                    cap = true;
                                    piece.color != self.turn_color
                                }
                                None => true,
                            }
                        {
                            moves.push(Move {
                                from: *p.0,
                                to: d,
                                capture: cap,
                                piece_type: PieceType::Knight,
                            })
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
            from: Coordinate::new('a', 2),
            to: Coordinate::new('a', 3),
            capture: false,
            piece_type: PieceType::Pawn,
        };

        mov.piece_type = match s.chars().nth(0) {
            Some(c) => match c {
                'N' => PieceType::Knight,
                'B' => PieceType::Bishop,
                'R' => PieceType::Rook,
                'Q' => PieceType::Queen,
                'K' => PieceType::King,
                _ => {
                    let pawn_from = match self
                        .pieces
                        .iter()
                        .filter(|p| {
                            p.1.color == self.turn_color
                                && p.1.piece_type == PieceType::Pawn
                                && p.0.file == (c as u8 - 96) as i8
                        })
                        .next()
                    {
                        Some(pn) => pn.0,
                        None => return Err(()),
                    };

                    mov.from = *pawn_from;
                    mov.to = Coordinate::new(
                        c,
                        match s.chars().nth(1) {
                            Some(c2) => match String::from(c2).parse() {
                                Ok(n) => n,
                                Err(_) => return Err(()),
                            },
                            None => return Err(()),
                        },
                    );

                    PieceType::Pawn
                }
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
            Err(()) => println!("Could not parse move. Please use valid algebraic notation."),
        }
    }
}
