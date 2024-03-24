use std::io;

fn main() {
    let room_width = 21;
    let room_height = 10;

    let mut player_pos: (i32, i32) = (2, room_height / 2);
    let mut snake_pos: (i32, i32) = (room_width - 2, room_height / 2);

    loop {
        print_room(room_width, room_height, player_pos, snake_pos);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Couldn't read line.");


    }
}

fn print_room(width: i32, height: i32, player_pos: (i32, i32), snake_pos: (i32, i32)) {
    for y in 0..=height {
        for x in 0..=width {
            if x == width && y == height / 2 {
                print!("]");
                continue;
            }

            if snake_pos == (x, y) {
                print!("S");
                continue;
            }

            if player_pos == (x, y) {
                print!("P");
                continue;
            }

            if x == 0 || x == width || y == 0 || y == height {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
