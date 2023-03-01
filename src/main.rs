use std::cmp::{max, min};

use chessboard::{Board, Piece, piece_moves, grid_to_coords, bitboard_to_grid};
use raylib::prelude::*;

fn to_grid_coords(x: i32, y: i32) -> Option<(u8, u8)> {
    if max(x, y) > 210 || min(x, y) < 0 {
        return None;
    }
    
    Some((
        (x as u8 - 16) / 24,
        (y as u8 - 16) / 24
    ))
}

fn main() {
    let mut board = Board::default();

    let (mut rl, thread) = raylib::init().size(640, 480).title("Chessboard").build();

    let mut selected = None;
    let mut err = None;

    let mut winner = None;

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            err.take();
            if let Some((x, y)) = to_grid_coords(rl.get_mouse_x(), rl.get_mouse_y()) {
                if let Some((sx, sy)) = selected {
                    if let Err(e) = board.move_piece(sx, sy, x, y, None) {
                        err = Some(e);
                    }
                    selected.take();
                } else {
                    selected = Some((x, y));
                }
            } else {
                selected.take();
            }

            winner.take();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);

        if let Some(e) = &err {
            d.draw_text(e.as_str(), 16, 220, 14, Color::RED);
        }

        let check = board.check();
        if check.0 {
            d.draw_circle(210, 208, 6.5, Color::RED);
        }
        
        if check.1 {
            d.draw_circle(210, 10, 6.5, Color::RED);
        }

        let mate = board.mate();
        if mate.0 {
            winner = Some(true);
        } else if mate.1 {
            winner = Some(false);
        }

        if let Some(true) = winner {
            d.draw_circle(210, 208, 6.5, Color::RED);
            d.draw_circle(210, 10, 6.5, Color::GREEN);
            board = Board::default();
        }
        
        if let Some(false) = winner {
            d.draw_circle(210, 10, 6.5, Color::RED);
            d.draw_circle(210, 208, 6.5, Color::GREEN);
            board = Board::default();
        }


        for (x, row) in board.as_grid().iter().enumerate() {
            for (y, piece) in row.iter().enumerate() {
                d.draw_rectangle_lines(x as i32 * 24 + 10,y as i32 * 24 + 10, 24, 24, Color::BLACK);
                d.draw_text(
                    match piece.0 {
                        Some(p) => match p {
                            Piece::Pawn => "P",
                            Piece::Knight => "N",
                            Piece::Bishop => "B",
                            Piece::Rook => "R",
                            Piece::Queen => "Q",
                            Piece::King => "K",
                        },
                        None => "",
                    },
                    x as i32 * 24 + 16,
                    y as i32 * 24 + 16,
                    20,
                    match piece.1 {
                        chessboard::Color::White => Color::WHITE,
                        _ => Color::BLACK,
                    },
                );
            }
        }

        if let Some((x, y)) = selected {
            for (x, y) in grid_to_coords(bitboard_to_grid(piece_moves(board, x, y))) {
                d.draw_circle(x as i32 * 24 + 16, y as i32 * 24 + 16, 4.0, Color::BLUE);
            }
        }
    }
}
