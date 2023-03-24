use std::io::BufRead;

use vampirc_uci::{parse, MessageList, UciMessage};

use chess_engine::Engine;

fn main() {
    let stdin = std::io::stdin();

    let mut uci_mode = false;

    let engine = Engine::default();

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
                    }
                    UciMessage::Go {
                        time_control,
                        search_control,
                    } => {
                        if !uci_mode {
                            continue;
                        }

                        // Search for and return the next move.
                    }
                    UciMessage::Stop => {
                        if !uci_mode {
                            continue;
                        }

                        // Stop thinking, but keep the current best move.
                    }
                    UciMessage::Quit => break 'main_loop,
                    _ => {}
                }
            }
        }
    }
}
