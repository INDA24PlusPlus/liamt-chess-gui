use arvidkr_chess::*;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self};
use ggez::{glam::*, Context, ContextBuilder, GameResult};
use std::path;
use std::str::FromStr;

const TILE_SIZE: i32 = 100;
const OFFSET: f32 = 100.0;

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
    board_str: String,
    selected_piece: Option<usize>,
    dragging: bool,
    mouse_pos: (f32, f32),
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
                    graphics::Color::from_rgb(255, 255, 255)
                    //graphics::Color::from_rgb(237, 14, 118)
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

        let board_str = (board.get_boardinfo()[7..71]).to_string();

        Chess {
            piece_images: load_piece_images(ctx),
            grid,
            board,
            board_str,
            selected_piece: None,
            dragging: false,
            mouse_pos: (0.0, 0.0),
        }
    }
}

impl EventHandler<ggez::GameError> for Chess {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.board_str = (self.board.get_boardinfo()[7..71]).to_string();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let dst = Vec2::new(100.0, 100.0);
        canvas.draw(&self.grid, graphics::DrawParam::new().dest(dst));

        for (i, c) in self.board_str.chars().enumerate() {
            let x = (i % 8) as f32 * TILE_SIZE as f32 + OFFSET;
            let y = (i / 8) as f32 * TILE_SIZE as f32 + OFFSET;

            let mut piece_dst = Vec2::new(x, y);

            if let Some(piece_idx) = self.selected_piece {
                if piece_idx == i {
                    canvas.draw(
                        &graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::stroke(5.0),
                            graphics::Rect::new(x, y, TILE_SIZE as f32, TILE_SIZE as f32),
                            graphics::Color::from_rgb(255, 0, 0),
                        )
                        .unwrap(),
                        graphics::DrawParam::new(),
                    );

                    if self.dragging {
                        piece_dst = Vec2::new(self.mouse_pos.0 - 50.0, self.mouse_pos.1 - 50.0);
                    }
                }
            }

            let img = &self
                .piece_images
                .iter()
                .find(|(piece, _)| piece == &c.to_string());

            if let Some((_, img)) = img {
                canvas.draw(img, graphics::DrawParam::new().dest(piece_dst));
            }
        }

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let x2 = (x - OFFSET) as i32 / TILE_SIZE;
        let y2 = (y - OFFSET) as i32 / TILE_SIZE;

        let idx = y2 as usize * 8 + x2 as usize;

        let piece = self.board_str.chars().nth(idx);

        if let Some(piece) = piece {
            println!("Piece: {}", piece);
            self.selected_piece = Some(idx);
            self.dragging = true;
            self.mouse_pos = (x, y);
        }

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let x = (x - OFFSET) as i32 / TILE_SIZE;
        let y = (y - OFFSET) as i32 / TILE_SIZE;

        let idx = y as usize * 8 + x as usize;

        self.dragging = false;

        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        if self.dragging && self.selected_piece.is_some() {
            self.mouse_pos = (x, y);
        }
        Ok(())
    }
}
