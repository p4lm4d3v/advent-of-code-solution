use typedef::Update;

pub fn is_before(update: &Update, x: i32, y: i32) -> bool {
    let pos_x = update.iter().position(|&e| e == x).unwrap();
    let pos_y = update.iter().position(|&e| e == y).unwrap();
    pos_x < pos_y
}
