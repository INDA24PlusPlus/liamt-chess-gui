use arvidkr_chess::*;
use chess_networking as net;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self};
use ggez::{glam::*, Context, ContextBuilder, GameResult};
use std::env;
use std::path;
use std::str::FromStr;
use std::time::Duration;

mod chess;
use chess::*;

mod draw;
use draw::*;

mod network;
use network::*;

const TILE_SIZE: f32 = 100.0;
const OFFSET: f32 = 100.0;

fn main() {
    let resource_dir = path::PathBuf::from("./resources");

    let mode = ggez::conf::WindowMode::default().dimensions(1000.0, 1000.0);

    let (mut ctx, event_loop) = ContextBuilder::new("chess", "Laim")
        .add_resource_path(resource_dir)
        .window_mode(mode)
        .window_setup(ggez::conf::WindowSetup::default().title("ULTIMEATE CHESS GAME!!?1"))
        .build()
        .expect("gg, could not create ggez context :(");

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <addr> <client/server>");
        std::process::exit(1);
    }

    let addr = &args[1];
    let role = &args[2];

    if role == "client" {
        let mut client = Client::new(addr);
        let test = net::Ack {
            ok: true,
            end_state: None,
        };
        let test_serialized: Vec<u8> = test.try_into().unwrap();
        client.send(test_serialized);
    } else if role == "server" {
        let mut server = Server::new(addr);
        loop {
            server.receive();
            std::thread::sleep(Duration::from_secs(1));
        }
    } else {
        println!("Invalid role, must be client or server");
        std::process::exit(1);
    }

    let chess = Chess::new(&mut ctx);

    event::run(ctx, event_loop, chess);
}

struct Chess {
    piece_images: Vec<(String, graphics::Image)>,
    board: Board,
    board_str: String,
    selected_piece: Option<usize>,
    dragging: bool,
    mouse_pos: (f32, f32),
    valid_moves: [Vec<usize>; 64],
    turn: Color,
    grid: graphics::Mesh,
    reset_button_rect: graphics::Rect,
    reset_button_mesh: graphics::Mesh,
    piece_mesh: graphics::Mesh,
    valid_circle_mesh: graphics::Mesh,
    check_circle_mesh: graphics::Mesh,
    status: Status,
}

impl Chess {
    pub fn new(ctx: &mut Context) -> Chess {
        let mb = &mut graphics::MeshBuilder::new();
        for row in 0..8 {
            for col in 0..8 {
                let tile_color = if (row + col) % 2 == 0 {
                    graphics::Color::from_rgb(255, 255, 255)
                } else {
                    graphics::Color::from_rgb(0, 0, 0)
                };

                let rect = graphics::Rect::new(
                    col as f32 * TILE_SIZE,
                    row as f32 * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                );
                mb.rectangle(graphics::DrawMode::fill(), rect, tile_color)
                    .expect("Failed to build grid tile");
            }
        }

        let grid = graphics::Mesh::from_data(ctx, mb.build());

        let mut board = Board::new();
        board.init_board();

        let board_str = invert_boardstr(board.get_boardinfo()[7..71].to_string());

        let piece_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(5.0),
            graphics::Rect::new(0.0, 0.0, TILE_SIZE, TILE_SIZE),
            graphics::Color::from_rgba(199, 38, 239, 255),
        )
        .unwrap();

        let valid_circle_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            10.0,
            0.1,
            graphics::Color::from_rgba(199, 38, 239, 100),
        )
        .unwrap();

        let check_circle_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            25.0,
            0.1,
            graphics::Color::from_rgba(255, 0, 0, 100),
        )
        .unwrap();

        let reset_button_rect = graphics::Rect::new(50.0, 25.0, 150.0, 30.0);

        let reset_button_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            reset_button_rect,
            graphics::Color::from_rgba(255, 255, 255, 255),
        )
        .unwrap();

        let valid_moves = generate_valid_moves(&mut board);
        Chess {
            status: Status::Active,
            piece_images: load_piece_images(ctx),
            board,
            board_str,
            selected_piece: None,
            dragging: false,
            mouse_pos: (0.0, 0.0),
            valid_moves,
            turn: Color::White,
            grid,
            reset_button_mesh,
            reset_button_rect,
            piece_mesh,
            valid_circle_mesh,
            check_circle_mesh,
        }
    }

    fn update_board(&mut self) {
        let info = self.board.get_boardinfo();
        self.turn = if &info[2..3] == "W" {
            Color::White
        } else {
            Color::Black
        };
        self.board_str = invert_boardstr((info[7..71]).to_string());

        self.valid_moves = generate_valid_moves(&mut self.board);

        let is_over = is_over(&mut self.board);

        match is_over {
            0 => (),
            1 => self.status = Status::Checkmate,
            2 => self.status = Status::Stalemate,
            3 => self.status = Status::ThreefoldRepetition,
            4 => self.status = Status::FiftyMoveRule,
            _ => (),
        }
    }
}

impl EventHandler<ggez::GameError> for Chess {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
            let x = (i % 8) as f32 * TILE_SIZE + OFFSET;
            let y = (i / 8) as f32 * TILE_SIZE + OFFSET;

            let mut piece_dst = Vec2::new(x, y);

            // START HANDLE SELECTED PIECE
            if selected_piece_idx == i {
                // DRAW SELECTION BORDER AROUND PIECE
                let dest = Vec2::new(x, y);
                canvas.draw(&self.piece_mesh, graphics::DrawParam::new().dest(dest));

                // IF DRAGGING, MOVE PIECE TO MOUSE POSITION
                if self.dragging {
                    piece_dst = Vec2::new(
                        self.mouse_pos.0 - (TILE_SIZE / 2.0),
                        self.mouse_pos.1 - (TILE_SIZE / 2.0),
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
                let dest = Vec2::new(x + TILE_SIZE / 2.0, y + TILE_SIZE / 2.0);
                canvas.draw(
                    &self.valid_circle_mesh,
                    graphics::DrawParam::new().dest(dest),
                );
            }

            // DRAW NUMERS BELOW BOARD
            if i % 8 == 0 {
                let mut text = graphics::Text::new(format!("{}", 8 - i / 8));
                text.set_scale(graphics::PxScale::from(40.0));
                let text_dest = Vec2::new(54.0, y + TILE_SIZE / 2.0 - (TILE_SIZE / 5.0));

                canvas.draw(&text, graphics::DrawParam::new().dest(text_dest));
            }

            // DRAW LETTERS BESIDE BOARD
            if i < 8 {
                let mut text = graphics::Text::new(format!("{}", (65 + (i % 8)) as u8 as char));
                text.set_scale(graphics::PxScale::from(40.0));
                let text_dest = Vec2::new(x + TILE_SIZE / 2.0 - (TILE_SIZE / 8.0), 918.0);

                canvas.draw(&text, graphics::DrawParam::new().dest(text_dest));
            }

            let k1 = i % 8;
            let k2 = 8 - i / 8 - 1;
            let k = (k1 + k2 * 8) as i64;

            if (c == 'K' || c == 'k') && in_check(&mut self.board, k) {
                let dest = Vec2::new(x + TILE_SIZE / 2.0, y + TILE_SIZE / 2.0);
                canvas.draw(
                    &self.check_circle_mesh,
                    graphics::DrawParam::new().dest(dest),
                );
            }
        }

        // DRAW TURN TEXT
        let mut text = graphics::Text::new(format!("Turn: {:?}", self.turn));
        text.set_scale(graphics::PxScale::from(40.0));
        text.set_layout(graphics::TextLayout::center());
        let text_dest = Vec2::new(500.0, 50.0);
        canvas.draw(&text, graphics::DrawParam::new().dest(text_dest));

        // DRAW RESET BUTTON
        canvas.draw(&self.reset_button_mesh, graphics::DrawParam::new());

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

        // DRAW STATUS TEXT
        if self.status != Status::Active {
            let mut text = graphics::Text::new(format!("{:?}", self.status));
            text.set_scale(graphics::PxScale::from(100.0));
            text.set_layout(graphics::TextLayout::center());
            let text_dest = Vec2::new(500.0, 500.0);
            canvas.draw(
                &text,
                graphics::DrawParam::new()
                    .dest(text_dest)
                    .color(graphics::Color::RED),
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

        let x2 = (x - OFFSET) as i32 / TILE_SIZE as i32;
        let y2 = (y - OFFSET) as i32 / TILE_SIZE as i32;
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
                self.update_board();
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
            self.status = Status::Active;
            self.update_board();
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

        let x2 = (x - OFFSET) as i32 / TILE_SIZE as i32;
        let y2 = (y - OFFSET) as i32 / TILE_SIZE as i32;

        let idx = y2 as usize * 8 + x2 as usize;

        // IF PIECE EXISTS
        if self.selected_piece.is_some()
            && self.valid_moves[self.selected_piece.unwrap()].contains(&idx)
        {
            // IF PIECE IS SELECTED AND POSITION IS VALID, MOVE PIECE
            move_piece(&mut self.board, self.selected_piece.unwrap(), idx);
            self.update_board();
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
