use std::io::BufRead;

use rand::Rng;
use vampirc_uci::{parse, MessageList, UciMessage, UciMove, UciPiece, UciSquare};

use chess_engine::{Engine, Move, PieceType, Side};

fn main() {
    let stdin = std::io::stdin();

    let mut uci_mode = false;

    let mut engine = Engine::default();
    let mut side = Side::White;

    'main_loop: loop {
        for line in stdin.lock().lines() {
            let messages: MessageList = parse(&line.unwrap());

            for message in messages {
                match message {
                    UciMessage::Uci => {
                        // The engine is now running in UCI mode.
                        uci_mode = true;

                        // Send identification message, and report as ready.
                        println!("{}", UciMessage::id_name(engine.name()));
                        println!("{}", UciMessage::id_author(engine.author()));
                        println!("{}", UciMessage::UciOk);
                    }
                    UciMessage::IsReady => {
                        if !uci_mode {
                            continue;
                        }

                        // Immediately send a readyok message back, no reason not to at the moment.
                        println!("{}", UciMessage::ReadyOk);
                    }
                    UciMessage::Position {
                        startpos,
                        fen,
                        moves,
                    } => {
                        if !uci_mode {
                            continue;
                        }
                        // Set up the given position.
                        if startpos {
                            engine.set_initial_position();
                        }

                        side = Side::White;

                        for uci_move in moves {
                            engine.make_move(uci_move_to_move(&uci_move));

                            if side == Side::White {
                                side = Side::Black;
                            } else {
                                side = Side::White;
                            }
                        }

                        engine.print_board();
                    }
                    UciMessage::Go {
                        time_control,
                        search_control,
                    } => {
                        if !uci_mode {
                            continue;
                        }

                        // Search for and return the next move.
                        let moves = engine.generate_moves(side);
                        if !moves.is_empty() {
                            let chosen_move = &moves[rand::thread_rng().gen_range(0..moves.len())];
                            let move_string = UciMessage::BestMove {
                                best_move: move_to_uci_move(chosen_move),
                                ponder: None,
                            };
                            println!("{}", move_string);
                        }
                    }
                    UciMessage::Stop => {
                        if !uci_mode {
                            continue;
                        }

                        // Stop thinking, but keep the current best move.

                        break 'main_loop;
                    }
                    UciMessage::Quit => break 'main_loop,
                    _ => {}
                }
            }
        }
    }
}

fn uci_piece_to_piece(piece: UciPiece) -> PieceType {
    match piece {
        UciPiece::Pawn => PieceType::Pawn,
        UciPiece::Knight => PieceType::Knight,
        UciPiece::Bishop => PieceType::Bishop,
        UciPiece::Rook => PieceType::Rook,
        UciPiece::Queen => PieceType::Queen,
        UciPiece::King => PieceType::King,
    }
}

fn piece_to_uci_piece(piece: PieceType) -> UciPiece {
    match piece {
        PieceType::Pawn => UciPiece::Pawn,
        PieceType::Knight => UciPiece::Knight,
        PieceType::Bishop => UciPiece::Bishop,
        PieceType::Rook => UciPiece::Rook,
        PieceType::Queen => UciPiece::Queen,
        PieceType::King => UciPiece::King,
        PieceType::Count => UciPiece::King,
    }
}

fn uci_move_to_move(uci_move: &UciMove) -> Move {
    let from_idx = (8 * (uci_move.from.rank - 1)) + ((uci_move.from.file as u8) - b'a');
    let to_idx = (8 * (uci_move.to.rank - 1)) + ((uci_move.to.file as u8) - b'a');

    Move {
        from: from_idx as u32,
        to: to_idx as u32,
        promote: uci_move.promotion.map(uci_piece_to_piece),
    }
}

fn move_to_uci_move(engine_move: &Move) -> UciMove {
    let from = UciSquare {
        rank: ((engine_move.from / 8) + 1) as u8,
        file: ((engine_move.from % 8) as u8 + b'a') as char,
    };

    let to = UciSquare {
        rank: ((engine_move.to / 8) + 1) as u8,
        file: ((engine_move.to % 8) as u8 + b'a') as char,
    };

    UciMove {
        from,
        to,
        promotion: engine_move.promote.map(piece_to_uci_piece),
    }
}
