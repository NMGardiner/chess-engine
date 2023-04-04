use crate::board::*;

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
    Count = PieceType::King as isize + 1,
}

impl PieceType {
    pub fn val(&self) -> usize {
        *self as usize
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Side {
    White = 0,
    Black = 1,
    Count = 2,
}

impl Side {
    pub fn val(&self) -> usize {
        *self as usize
    }

    pub fn flip(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
            _ => Side::Count,
        }
    }
}

pub trait CheckIndex {
    fn check_index(&self, index: usize) -> bool;
}

impl CheckIndex for Bitboard {
    fn check_index(&self, index: usize) -> bool {
        self & 1 << index == 1 << index
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub from: u32,
    pub to: u32,
    pub promote: Option<PieceType>,
}

pub struct Engine {
    // Which type of piece, if any, is on a given square.
    squares_by_type: [Option<PieceType>; 64],

    board: Board,
}

impl Engine {
    pub fn name(&self) -> &str {
        "Chess Engine"
    }

    pub fn author(&self) -> &str {
        "Nathan Gardiner"
    }

    pub fn set_initial_position(&mut self) {
        for file in 1..=8 {
            self.set_square(7 + file, Side::White, Some(PieceType::Pawn));
            self.set_square(47 + file, Side::Black, Some(PieceType::Pawn));
        }

        self.set_square(1, Side::White, Some(PieceType::Knight));
        self.set_square(6, Side::White, Some(PieceType::Knight));
        self.set_square(57, Side::Black, Some(PieceType::Knight));
        self.set_square(62, Side::Black, Some(PieceType::Knight));

        self.set_square(2, Side::White, Some(PieceType::Bishop));
        self.set_square(5, Side::White, Some(PieceType::Bishop));
        self.set_square(58, Side::Black, Some(PieceType::Bishop));
        self.set_square(61, Side::Black, Some(PieceType::Bishop));

        self.set_square(0, Side::White, Some(PieceType::Rook));
        self.set_square(7, Side::White, Some(PieceType::Rook));
        self.set_square(56, Side::Black, Some(PieceType::Rook));
        self.set_square(63, Side::Black, Some(PieceType::Rook));

        self.set_square(3, Side::White, Some(PieceType::Queen));
        self.set_square(59, Side::Black, Some(PieceType::Queen));

        self.set_square(4, Side::White, Some(PieceType::King));
        self.set_square(60, Side::Black, Some(PieceType::King));
    }

    fn set_square(&mut self, square_idx: usize, side: Side, piece_type: Option<PieceType>) {
        self.squares_by_type[square_idx] = piece_type;

        if let Some(piece_type) = piece_type {
            // Set the square.
            self.board.bitboard_by_side[side.val()] |= 1 << square_idx;
            self.board.bitboard_by_side[side.flip().val()] &= !(1 << square_idx);

            self.board.bitboard_by_piece[piece_type.val()] |= 1 << square_idx;
        } else {
            // Clear the square.
            self.board.bitboard_by_side[side.val()] &= !(1 << square_idx);

            for i in 0..(PieceType::Count.val()) {
                self.board.bitboard_by_piece[i] &= !(1 << square_idx);
            }
        }
    }

    pub fn make_move(&mut self, piece_move: Move) {
        let from_index = piece_move.from as usize;
        let to_index = piece_move.to as usize;

        // Ascertain which side is making the move.
        let side = if self.board.bitboard_by_side[Side::White.val()] & (1 << piece_move.from) != 0 {
            Side::White
        } else {
            Side::Black
        };

        // Ascertain the piece type.
        let from_piece_type = self.squares_by_type[from_index];

        if from_piece_type.is_none() {
            println!("Invalid move made! Square {} has no piece!", from_index);
            return;
        }

        let to_piece_type = piece_move
            .promote
            .unwrap_or_else(|| from_piece_type.unwrap());

        self.set_square(from_index, side, None);
        self.set_square(to_index, side, Some(to_piece_type));
    }

    pub fn print_board(&self) {
        let mut out = String::from("    a   b   c   d   e   f   g   h\n");
        out += "  +---+---+---+---+---+---+---+---+\n";

        for rank in (1..=8).rev() {
            out += format!("{} |", rank).as_str();

            for file in 0..8 {
                let index = ((rank - 1) * 8) + file;

                let c = if let Some(piece_type) = self.squares_by_type[index] {
                    let side = if self.board.bitboard_by_side[Side::White.val()].check_index(index)
                    {
                        Side::White
                    } else {
                        Side::Black
                    };

                    self.char_from_piece(piece_type, side)
                } else {
                    ' '
                };

                out += format!(" {} |", c).as_str();
            }

            out += format!(" {}\n", rank).as_str();
            out += "  +---+---+---+---+---+---+---+---+\n";
        }

        out += "    a   b   c   d   e   f   g   h";

        println!("{}", out);
    }

    fn char_from_piece(&self, piece_type: PieceType, side: Side) -> char {
        let char = match piece_type {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
            PieceType::Count => '?',
        };

        if side == Side::White {
            char
        } else {
            char.to_ascii_lowercase()
        }
    }

    pub fn generate_moves(&self, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        let mut pawn_moves = self.board.generate_pawn_moves(side);
        moves.append(&mut pawn_moves);

        // Knight moves are not yet fully implemented, so leave out for now.
        //let mut knight_moves = self.board.generate_knight_moves(side);
        //moves.append(&mut knight_moves);

        moves
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            squares_by_type: [None; 64],
            board: Board::new(),
        }
    }
}
