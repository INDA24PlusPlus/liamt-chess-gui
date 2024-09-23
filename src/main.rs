use arvidkr_chess::*;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self};
use ggez::{glam::*, Context, ContextBuilder, GameResult};
use std::path;
use std::str::FromStr;

const TILE_SIZE: i32 = 100;

/* #[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
} */

fn main() {
    let resource_dir = path::PathBuf::from("./resources");

    let mode = ggez::conf::WindowMode::default().dimensions(1000.0, 1000.0);
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("chess", "Laim")
        .add_resource_path(resource_dir)
        .window_mode(mode)
        .build()
        .expect("gg, could not create ggez context :(");

    let my_game = Chess::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

fn load_piece_images(ctx: &Context) -> Vec<(String, graphics::Image)> {
    /* let pieces = vec![
        (Piece::Pawn(Color::White), "/wP.png"),
        (Piece::Rook(Color::White), "/wR.png"),
        (Piece::Knight(Color::White), "/wN.png"),
        (Piece::Bishop(Color::White), "/wB.png"),
        (Piece::Queen(Color::White), "/wQ.png"),
        (Piece::King(Color::White), "/wK.png"),
        (Piece::Pawn(Color::Black), "/bP.png"),
        (Piece::Rook(Color::Black), "/bR.png"),
        (Piece::Knight(Color::Black), "/bN.png"),
        (Piece::Bishop(Color::Black), "/bB.png"),
        (Piece::Queen(Color::Black), "/bQ.png"),
        (Piece::King(Color::Black), "/bK.png"),
    ]; */

    let pieces = vec![
        ("P", "/wP.png"),
        ("R", "/wR.png"),
        ("N", "/wN.png"),
        ("B", "/wB.png"),
        ("Q", "/wQ.png"),
        ("K", "/wK.png"),
        ("p", "/bP.png"),
        ("r", "/bR.png"),
        ("n", "/bN.png"),
        ("b", "/bB.png"),
        ("q", "/bQ.png"),
        ("k", "/bK.png"),
    ];

    let mut piece_images = Vec::new();

    for (piece, path) in pieces {
        let img = graphics::Image::from_path(ctx, path).unwrap();
        piece_images.push((String::from_str(piece).unwrap(), img));
    }

    piece_images
}

struct Chess {
    piece_images: Vec<(String, graphics::Image)>,
    grid: graphics::Mesh,
    board: Board,
    // Your state here...
}

impl Chess {
    /* fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        // Move piece on board
        make_move
    } */

    pub fn new(ctx: &mut Context) -> Chess {
        let mb = &mut graphics::MeshBuilder::new();
        for row in 0..8 {
            for col in 0..8 {
                let tile_color = if (row + col) % 2 == 0 {
                    graphics::Color::from_rgb(237, 14, 118)
                } else {
                    graphics::Color::from_rgb(0, 0, 0)
                };

                let rect = graphics::Rect::new(
                    (col * TILE_SIZE) as f32,
                    (row * TILE_SIZE) as f32,
                    TILE_SIZE as f32,
                    TILE_SIZE as f32,
                );
                mb.rectangle(graphics::DrawMode::fill(), rect, tile_color)
                    .expect("Failed to build grid tile");
            }
        }

        let grid = graphics::Mesh::from_data(ctx, mb.build());

        let mut board = Board::new();
        board.init_board();

        println!("{:?}", board.get_board());

        Chess {
            piece_images: load_piece_images(ctx),
            grid,
            board,
            // ...
        }
    }
}

impl EventHandler for Chess {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let dst = Vec2::new(100.0, 100.0);
        canvas.draw(&self.grid, graphics::DrawParam::new().dest(dst));

        let board_str = &self.board.get_boardinfo()[7..71];

        for (i, char) in board_str.chars().enumerate() {
            let x = (i % 8) as f32 * TILE_SIZE as f32 + 100.0;
            let y = (i / 8) as f32 * TILE_SIZE as f32 + 100.0;

            let img = &self
                .piece_images
                .iter()
                .find(|(piece, _)| piece == &char.to_string());

            if let Some((_, img)) = img {
                let dst = Vec2::new(x, y);
                canvas.draw(img, graphics::DrawParam::new().dest(dst));
            }
        }

        canvas.finish(ctx)
    }
}
