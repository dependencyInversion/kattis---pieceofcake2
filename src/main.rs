struct Input {
    length: u32,
    vertical_cut_from_left: u32,
    horzinontal_cut_from_left: u32,
}

struct Piece {
    x: u32,
    y: u32,
    z: u32,
    surface_area: f64,
    volume: f64,
}

impl Piece {
    fn new(x: u32, y: u32) -> Self {
        Piece {
            x,
            y,
            z: 4,
            surface_area: x as f64 * y as f64,
            volume: x as f64 * y as f64 * 4.0,
        }
    }
}

fn main() {
    let mut buf: String = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    let mut piece_vec: Vec<Piece> = Vec::new();

    let mut input_iter = buf.trim().split_ascii_whitespace();

    let input = Input {
        length: input_iter.next().unwrap().parse::<u32>().unwrap(),
        vertical_cut_from_left: input_iter.next().unwrap().parse::<u32>().unwrap(),
        horzinontal_cut_from_left: input_iter.next().unwrap().parse::<u32>().unwrap(),
    };

    piece_vec.push(Piece::new(
        input.horzinontal_cut_from_left.clone(),
        input.vertical_cut_from_left.clone(),
    ));
    piece_vec.push(Piece::new(
        input.horzinontal_cut_from_left,
        input.length - input.vertical_cut_from_left,
    ));
    piece_vec.push(Piece::new(
        input.length - input.horzinontal_cut_from_left,
        input.vertical_cut_from_left,
    ));
    piece_vec.push(Piece::new(
        input.length - input.horzinontal_cut_from_left,
        input.length - input.vertical_cut_from_left,
    ));

    let mut max_volume: f64 = piece_vec.first().unwrap().volume;

    for piece in piece_vec.into_iter().skip(1) {
        if max_volume < piece.volume {
            max_volume = piece.volume;
        }
    }

    println!("{}", max_volume);
}
