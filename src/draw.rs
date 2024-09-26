use crate::*;

pub fn load_piece_images(ctx: &Context) -> Vec<(String, graphics::Image)> {
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
