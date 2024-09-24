use arvidkr_chess::*;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self};
use ggez::{glam::*, Context, ContextBuilder, GameResult};
use std::path;
use std::str::FromStr;

const TILE_SIZE: i32 = 100;
const OFFSET: f32 = 100.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    Black,
    None,
}

fn main() {
    let resource_dir = path::PathBuf::from("./resources");

    let mode = ggez::conf::WindowMode::default().dimensions(1000.0, 1000.0);

    let (mut ctx, event_loop) = ContextBuilder::new("chess", "Laim")
        .add_resource_path(resource_dir)
        .window_mode(mode)
        .window_setup(ggez::conf::WindowSetup::default().title("ULTIMEATE CHESS GAME!!?1"))
        .build()
        .expect("gg, could not create ggez context :(");

    let chess = Chess::new(&mut ctx);

    event::run(ctx, event_loop, chess);
}

fn load_piece_images(ctx: &Context) -> Vec<(String, graphics::Image)> {
    let pieces = vec![
        ("p", "/wP.png"),
        ("r", "/wR.png"),
        ("n", "/wN.png"),
        ("b", "/wB.png"),
        ("q", "/wQ.png"),
        ("k", "/wK.png"),
        ("P", "/bP.png"),
        ("R", "/bR.png"),
        ("N", "/bN.png"),
        ("B", "/bB.png"),
        ("Q", "/bQ.png"),
        ("K", "/bK.png"),
    ];

    let mut piece_images = Vec::new();

    for (piece, path) in pieces {
        let img = graphics::Image::from_path(ctx, path).unwrap();
        piece_images.push((String::from_str(piece).unwrap(), img));
    }

    piece_images
}

fn str_to_idx(s: &str) -> usize {
    let s = s.to_lowercase();
    let x = s.chars().next().unwrap() as usize - 'a' as usize;
    let y = s.chars().nth(1).unwrap() as usize - '1' as usize;
    (7 - y) * 8 + x
}

fn idx_to_str(idx: usize) -> String {
    let x = idx % 8;
    let y = 7 - idx / 8;
    let x = (x as u8 + b'a') as char;
    let y = (y as u8 + b'1') as char;
    format!("{}{}", x, y)
}

fn generate_valid_moves(board: &mut Board) -> [Vec<usize>; 64] {
    const ARRAY_REPEAT_VALUE: Vec<usize> = Vec::new();
    let mut valid_moves = [ARRAY_REPEAT_VALUE; 64];

    let moves = filtered_moves(board);

    for m in moves.iter() {
        let idx = str_to_idx(&m[0..2]);
        valid_moves[idx].push(str_to_idx(&m[2..4]));
    }

    valid_moves
}

fn get_piece_color(piece: char) -> Color {
    match piece {
        'p' | 'r' | 'n' | 'b' | 'q' | 'k' => Color::White,
        'P' | 'R' | 'N' | 'B' | 'Q' | 'K' => Color::Black,
        _ => Color::None,
    }
}

fn move_piece(board: &mut Board, from: usize, to: usize) {
    let from = idx_to_str(from);
    let to = idx_to_str(to);

    let movi = format!("{}{}", from, to);

    println!("Move: {}", movi);

    make_move(board, movi);

    println!("Str: {}", board.get_boardinfo())
}

fn invert_boardstr(boardstr: String) -> String {
    //reverse every 8 characters
    let mut new_boardstr = String::new();
    for i in (0..64).rev().step_by(8) {
        let row = &boardstr[i - 7..i + 1];
        new_boardstr.push_str(row);
    }
    new_boardstr
}

struct Chess {
    piece_images: Vec<(String, graphics::Image)>,
    grid: graphics::Mesh,
    board: Board,
    board_str: String,
    reset_button_rect: graphics::Rect,
    selected_piece: Option<usize>,
    dragging: bool,
    mouse_pos: (f32, f32),
    valid_moves: [Vec<usize>; 64],
    turn: Color,
}

impl Chess {
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

        let board_str = (board.get_boardinfo()[7..71]).to_string();

        let reset_button_rect = graphics::Rect::new(50.0, 25.0, 150.0, 30.0);

        const ARRAY_REPEAT_VALUE: Vec<usize> = Vec::new();
        Chess {
            piece_images: load_piece_images(ctx),
            grid,
            reset_button_rect,
            board,
            board_str,
            selected_piece: None,
            dragging: false,
            mouse_pos: (0.0, 0.0),
            valid_moves: [ARRAY_REPEAT_VALUE; 64],
            turn: Color::White,
        }
    }
}

impl EventHandler<ggez::GameError> for Chess {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let info = self.board.get_boardinfo();
        self.turn = if &info[2..3] == "W" {
            Color::White
        } else {
            Color::Black
        };
        self.board_str = invert_boardstr((info[7..71]).to_string());
        self.valid_moves = generate_valid_moves(&mut self.board);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let selected_piece_idx = self.selected_piece.unwrap_or(69); // nice

        // START DRAW GRID
        let dst = Vec2::new(100.0, 100.0);
        canvas.draw(&self.grid, graphics::DrawParam::new().dest(dst));

        // LOOP THROUGH BOARD STRING AND DRAW PIECES
        for (i, c) in self.board_str.chars().enumerate() {
            // START CALCULATE POSITION
            let x = (i % 8) as f32 * TILE_SIZE as f32 + OFFSET;
            let y = (i / 8) as f32 * TILE_SIZE as f32 + OFFSET;

            let mut piece_dst = Vec2::new(x, y);

            // START HANDLE SELECTED PIECE
            if selected_piece_idx == i {
                // DRAW SELECTION BORDER AROUND PIECE
                canvas.draw(
                    &graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(5.0),
                        graphics::Rect::new(x, y, TILE_SIZE as f32, TILE_SIZE as f32),
                        graphics::Color::from_rgba(199, 38, 239, 255),
                    )
                    .unwrap(),
                    graphics::DrawParam::new(),
                );

                // IF DRAGGING, MOVE PIECE TO MOUSE POSITION
                if self.dragging {
                    piece_dst = Vec2::new(
                        self.mouse_pos.0 - (TILE_SIZE as f32 / 2.0),
                        self.mouse_pos.1 - (TILE_SIZE as f32 / 2.0),
                    );
                }
            }

            // START DRAW PIECE
            let img = &self
                .piece_images
                .iter()
                .find(|(piece, _)| piece == &c.to_string());

            if let Some((_, img)) = img {
                canvas.draw(img, graphics::DrawParam::new().dest(piece_dst));
            }

            // DRAW VALID MOVES CIRCLE
            if selected_piece_idx != 69 && self.valid_moves[selected_piece_idx].contains(&i) {
                canvas.draw(
                    &graphics::Mesh::new_circle(
                        ctx,
                        graphics::DrawMode::fill(),
                        Vec2::new(x + TILE_SIZE as f32 / 2.0, y + TILE_SIZE as f32 / 2.0),
                        10.0,
                        0.1,
                        graphics::Color::from_rgba(199, 38, 239, 100),
                    )
                    .unwrap(),
                    graphics::DrawParam::new(),
                );
            }

            // DRAW NUMERS BELOW BOARD
            if i % 8 == 0 {
                let mut text = graphics::Text::new(format!("{}", 8 - i / 8));
                text.set_scale(graphics::PxScale::from(40.0));
                let text_dest =
                    Vec2::new(54.0, y + TILE_SIZE as f32 / 2.0 - (TILE_SIZE as f32 / 5.0));

                canvas.draw(&text, graphics::DrawParam::new().dest(text_dest));
            }

            // DRAW LETTERS BESIDE BOARD
            if i < 8 {
                let mut text = graphics::Text::new(format!("{}", (65 + (i % 8)) as u8 as char));
                text.set_scale(graphics::PxScale::from(40.0));
                let text_dest =
                    Vec2::new(x + TILE_SIZE as f32 / 2.0 - (TILE_SIZE as f32 / 8.0), 918.0);

                canvas.draw(&text, graphics::DrawParam::new().dest(text_dest));
            }

            // DRAW TURN TEXT
            let mut text = graphics::Text::new(format!("Turn: {:?}", self.turn));
            text.set_scale(graphics::PxScale::from(40.0));
            let text_dest = Vec2::new((TILE_SIZE as f32) * 4.0 - 15.0, 20.0);
            canvas.draw(&text, graphics::DrawParam::new().dest(text_dest));

            // DRAW RESET BUTTON
            canvas.draw(
                &graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    self.reset_button_rect,
                    graphics::Color::from_rgba(255, 255, 255, 255),
                )
                .unwrap(),
                graphics::DrawParam::new(),
            );

            let mut reset_text = graphics::Text::new("Reset");
            let reset_text_dest = Vec2::new(
                self.reset_button_rect.x + 37.0,
                self.reset_button_rect.y + 3.0,
            );
            reset_text.set_scale(graphics::PxScale::from(30.0));
            canvas.draw(
                &reset_text,
                graphics::DrawParam::new()
                    .dest(reset_text_dest)
                    .color(graphics::Color::BLACK),
            );
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
        if button != MouseButton::Left {
            return Ok(());
        }

        let x2 = (x - OFFSET) as i32 / TILE_SIZE;
        let y2 = (y - OFFSET) as i32 / TILE_SIZE;
        let idx = y2 as usize * 8 + x2 as usize;

        // GET PIECE AT MOUSE POSITION
        let piece = self.board_str.chars().nth(idx);

        // IF PIECE EXISTS
        if let Some(piece) = piece {
            let color = get_piece_color(piece);

            // IF PIECE IS SAME COLOR AS TURN, SELECT PIECE
            if color == self.turn {
                self.selected_piece = Some(idx);
                self.dragging = true;
                self.mouse_pos = (x, y);
            } else if self.selected_piece.is_some()
                && self.valid_moves[self.selected_piece.unwrap()].contains(&idx)
            {
                // IF PIECE IS SELECTED AND POSITION IS VALID, MOVE PIECE
                move_piece(&mut self.board, self.selected_piece.unwrap(), idx);
                self.selected_piece = None;
            } else {
                // ELSE UNSELECT PIECE
                self.selected_piece = None;
            }
        }

        if self.reset_button_rect.contains([x, y]) {
            self.board = Board::new();
            self.board.init_board();
            self.selected_piece = None;
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
        if button != MouseButton::Left {
            return Ok(());
        }

        let x2 = (x - OFFSET) as i32 / TILE_SIZE;
        let y2 = (y - OFFSET) as i32 / TILE_SIZE;

        let idx = y2 as usize * 8 + x2 as usize;

        // IF PIECE EXISTS
        if self.selected_piece.is_some()
            && self.valid_moves[self.selected_piece.unwrap()].contains(&idx)
        {
            // IF PIECE IS SELECTED AND POSITION IS VALID, MOVE PIECE
            move_piece(&mut self.board, self.selected_piece.unwrap(), idx);
            self.selected_piece = None;
        }

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
