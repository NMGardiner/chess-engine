use crate::{Move, PieceType, Side};

pub use u64 as Bitboard;

pub trait BitboardOps {
    fn get_ls1b(&self) -> Self;
    fn remove_ls1b(&self) -> Self;
    fn iter(&self) -> BitboardIterator;
}

impl BitboardOps for Bitboard {
    /// Get the least significant 1 bit in the bitboard.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_engine::{Bitboard, BitboardOps};
    ///
    /// let bitboard: Bitboard = 0x000000FF00FF0000;
    /// assert_eq!(bitboard.get_ls1b(), 0x0000000000010000);
    /// ```
    fn get_ls1b(&self) -> Self {
        self & self.wrapping_neg()
    }

    /// Remove the least significant 1 bit from the bitboard.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_engine::{Bitboard, BitboardOps};
    ///
    /// let bitboard: Bitboard = 0x000000FF00FF0000;
    /// assert_eq!(bitboard.remove_ls1b(), 0x000000FF00FE0000);
    /// ```
    fn remove_ls1b(&self) -> Self {
        self & (self - 1)
    }

    fn iter(&self) -> BitboardIterator {
        BitboardIterator { current: *self }
    }
}

pub struct BitboardIterator {
    pub current: Bitboard,
}

impl Iterator for BitboardIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current != 0 {
            let ls1b = Some(self.current.get_ls1b());
            self.current = self.current.remove_ls1b();

            ls1b
        } else {
            None
        }
    }
}

pub const RANK_1: Bitboard = 0x00000000000000FF;
pub const RANK_2: Bitboard = 0x000000000000FF00;
pub const RANK_3: Bitboard = 0x0000000000FF0000;
pub const RANK_4: Bitboard = 0x00000000FF000000;
pub const RANK_5: Bitboard = 0x000000FF00000000;
pub const RANK_6: Bitboard = 0x0000FF0000000000;
pub const RANK_7: Bitboard = 0x00FF000000000000;
pub const RANK_8: Bitboard = 0xFF00000000000000;

// Rank by index.
pub const RANKS: [Bitboard; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];

pub const FILE_A: Bitboard = 0x0101010101010101;
pub const FILE_B: Bitboard = 0x0202020202020202;
pub const FILE_C: Bitboard = 0x0404040404040404;
pub const FILE_D: Bitboard = 0x0808080808080808;
pub const FILE_E: Bitboard = 0x1010101010101010;
pub const FILE_F: Bitboard = 0x2020202020202020;
pub const FILE_G: Bitboard = 0x4040404040404040;
pub const FILE_H: Bitboard = 0x8080808080808080;

// File by index.
pub const FILES: [Bitboard; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
    NNE,
    NEE,
    SEE,
    SSE,
    SSW,
    SWW,
    NWW,
    NNW,
}

// All directions a knight can attack. Used for iteration.
pub const KNIGHT_ATTACKS_DIRECTIONS: [Direction; 8] = [
    Direction::NNE,
    Direction::NEE,
    Direction::SEE,
    Direction::SSE,
    Direction::SSW,
    Direction::SWW,
    Direction::NWW,
    Direction::NNW,
];

const fn bb_shift(bitboard: Bitboard, direction: Direction) -> Bitboard {
    match direction {
        // Cardinal moves.
        Direction::N => bitboard << 8,
        Direction::E => (bitboard << 1) & !FILE_A,
        Direction::S => bitboard >> 8,
        Direction::W => (bitboard >> 1) & !FILE_H,

        // Diagonal moves.
        Direction::NE => (bitboard << 9) & !FILE_A,
        Direction::SE => (bitboard >> 7) & !FILE_A,
        Direction::SW => (bitboard >> 9) & !FILE_H,
        Direction::NW => (bitboard << 7) & !FILE_H,

        // Knight moves.
        Direction::NNE => (bitboard << 17) & !FILE_A,
        Direction::NEE => (bitboard << 10) & !FILE_A,
        Direction::SEE => (bitboard >> 6) & !FILE_A,
        Direction::SSE => (bitboard >> 15) & !FILE_A,
        Direction::SSW => (bitboard >> 17) & !FILE_H,
        Direction::SWW => (bitboard >> 10) & !FILE_H,
        Direction::NWW => (bitboard << 6) & !FILE_H,
        Direction::NNW => (bitboard << 15) & !FILE_H,
    }
}

/// Returns a bitboard of single pawn push destinations for the given side.
///
/// # Arguments
///
/// * `pawns` - The bitboard of pawns belonging to the given side.
/// * `empty` - The bitboard of all empty squares on the board.
/// * `side` - The side to push pawns for (should match the side of the pawns in `pawns`).
pub const fn single_pawn_push(pawns: Bitboard, empty: Bitboard, side: Side) -> Bitboard {
    match side {
        Side::White => bb_shift(pawns, Direction::N) & empty,
        Side::Black => bb_shift(pawns, Direction::S) & empty,
        _ => 0,
    }
}

/// Returns a bitboard of double pawn push destinations for the given side.
///
/// # Arguments
///
/// * `pawns` - The bitboard of pawns belonging to the given side.
/// * `empty` - The bitboard of all empty squares on the board.
/// * `side` - The side to push pawns for (should match the side of the pawns in `pawns`).
pub const fn double_pawn_push(pawns: Bitboard, empty: Bitboard, side: Side) -> Bitboard {
    match side {
        Side::White => {
            bb_shift(single_pawn_push(pawns & RANK_2, empty, side), Direction::N) & empty
        }
        Side::Black => {
            bb_shift(single_pawn_push(pawns & RANK_7, empty, side), Direction::S) & empty
        }
        _ => 0,
    }
}

/// Returns a bitboard of the possible pawn East capture targets for the given side.
///
/// # Arguments
///
/// * `pawns` - The bitboard of pawns belonging to the given side.
/// * `enemy_pieces` - The bitboard of all pieces belonging to the other side.
/// * `side` - The side to generate captures for (should match the side of the pawns in `pawns`).
pub const fn pawn_east_attacks(pawns: Bitboard, enemy_pieces: Bitboard, side: Side) -> Bitboard {
    match side {
        Side::White => bb_shift(pawns, Direction::NE) & enemy_pieces,
        Side::Black => bb_shift(pawns, Direction::SE) & enemy_pieces,
        _ => 0,
    }
}

/// Returns a bitboard of the possible pawn West capture targets for the given side.
///
/// # Arguments
///
/// * `pawns` - The bitboard of pawns belonging to the given side.
/// * `enemy_pieces` - The bitboard of all pieces belonging to the other side.
/// * `side` - The side to generate captures for (should match the side of the pawns in `pawns`).
pub const fn pawn_west_attacks(pawns: Bitboard, enemy_pieces: Bitboard, side: Side) -> Bitboard {
    match side {
        Side::White => bb_shift(pawns, Direction::NW) & enemy_pieces,
        Side::Black => bb_shift(pawns, Direction::SW) & enemy_pieces,
        _ => 0,
    }
}

pub struct Board {
    pub attacks_by_piece: [[Bitboard; 64]; 6],
    pub bitboard_by_side: [Bitboard; 2],
    pub bitboard_by_piece: [Bitboard; 6],
}

impl Board {
    pub fn new() -> Self {
        let mut attacks_by_piece = [[0; 64]; 6];

        // Compute all knight attacks for every square.
        for square in 0..64 {
            for direction in KNIGHT_ATTACKS_DIRECTIONS {
                attacks_by_piece[PieceType::Knight.val()][square] |=
                    bb_shift(1 << square, direction);
            }
        }

        Self {
            attacks_by_piece,
            bitboard_by_side: [0; 2],
            bitboard_by_piece: [0; 6],
        }
    }

    pub fn generate_pawn_moves(&self, side: Side) -> Vec<Move> {
        let opp_bitboard = self.bitboard_by_side[side.flip().val()];

        // Single / double pushes.
        let our_pawns =
            self.bitboard_by_piece[PieceType::Pawn.val()] & self.bitboard_by_side[side.val()];

        let empty = !self.bitboard_by_side[side.val()] & !opp_bitboard;

        let promotion_rank = match side {
            Side::White => RANK_8,
            Side::Black => RANK_1,
            _ => !0,
        };

        let mut moves: Vec<Move> = vec![];

        //
        // Pushes
        //

        single_pawn_push(our_pawns, empty, side)
            .iter()
            .for_each(|to_square| {
                let from_square = match side {
                    Side::White => to_square >> 8,
                    Side::Black => to_square << 8,
                    _ => 0,
                };

                let promotion_piece = if to_square & promotion_rank != 0 {
                    Some(PieceType::Queen)
                } else {
                    None
                };

                moves.push(Move {
                    from: from_square.trailing_zeros(),
                    to: to_square.trailing_zeros(),
                    promote: promotion_piece,
                });
            });

        double_pawn_push(our_pawns, empty, side)
            .iter()
            .for_each(|to_square| {
                let from_square = match side {
                    Side::White => to_square >> 16,
                    Side::Black => to_square << 16,
                    _ => 0,
                };

                moves.push(Move {
                    from: from_square.trailing_zeros(),
                    to: to_square.trailing_zeros(),
                    promote: None,
                });
            });

        //
        // Captures
        //

        pawn_east_attacks(our_pawns, opp_bitboard, side)
            .iter()
            .for_each(|target_piece| {
                pawn_west_attacks(target_piece, our_pawns, side.flip())
                    .iter()
                    .for_each(|source_piece| {
                        let promotion_piece = if target_piece & promotion_rank != 0 {
                            Some(PieceType::Queen)
                        } else {
                            None
                        };

                        moves.push(Move {
                            from: source_piece.trailing_zeros(),
                            to: target_piece.trailing_zeros(),
                            promote: promotion_piece,
                        });
                    });
            });

        pawn_west_attacks(our_pawns, opp_bitboard, side)
            .iter()
            .for_each(|target_piece| {
                pawn_east_attacks(target_piece, our_pawns, side.flip())
                    .iter()
                    .for_each(|source_piece| {
                        let promotion_piece = if target_piece & promotion_rank != 0 {
                            Some(PieceType::Queen)
                        } else {
                            None
                        };

                        moves.push(Move {
                            from: source_piece.trailing_zeros(),
                            to: target_piece.trailing_zeros(),
                            promote: promotion_piece,
                        });
                    });
            });

        moves
    }

    pub fn generate_knight_moves(&self, side: Side) -> Vec<Move> {
        let opp_bitboard = self.bitboard_by_side[side.flip().val()];
        let our_bitboard = self.bitboard_by_side[side.val()];

        let empty_bitboard = !our_bitboard & !opp_bitboard;

        let our_knights =
            self.bitboard_by_piece[PieceType::Knight.val()] & self.bitboard_by_side[side.val()];

        let mut moves: Vec<Move> = vec![];

        our_knights.iter().for_each(|knight_source| {
            let source_index = knight_source.trailing_zeros();

            // Empty squares to move the knight to.
            let knight_moves = self.attacks_by_piece[PieceType::Knight.val()]
                [source_index as usize]
                & empty_bitboard;

            knight_moves.iter().for_each(|knight_move| {
                moves.push(Move {
                    from: source_index,
                    to: knight_move.trailing_zeros(),
                    promote: None,
                });
            });
        });

        moves
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
