const SIZE: usize = 4;
const X: i32 = 10;
const Y: i32 = 10;
fn main() {
    for _ in 0..Y {
        for z in 1..=SIZE {
            for _ in 0..X {
                print_hex_line(true, z);
            }
            println!("");
        }
        for z in (1..=SIZE).rev() {
            for _ in 0..X {
                print_hex_line(false, z);
            }
            println!("");
        }
    }
}

fn print_hex_line(is_top: bool, level: usize) {
    let leading_slash = if is_top { "/" } else { r"\" };
    let trailing_slash = if is_top { r"\" } else { "/" };
    let space = String::from(" ");
    let underscore = "_";

    let string = if is_top && level == SIZE {
        leading_slash.to_string()
            + &space.repeat((SIZE * 3) - 2)
            + trailing_slash
            + &underscore.repeat(SIZE)
    } else if !is_top && level == 1 {
        space.repeat(SIZE - level)
            + leading_slash
            + &underscore.repeat(SIZE)
            + trailing_slash
            + &space.repeat((2 * SIZE) - 1)
    } else {
        space.repeat(SIZE - level)
            + leading_slash
            + &space.repeat((level * 2) + SIZE - 2)
            + trailing_slash
            + &space.repeat((SIZE * 2) - level)
    };
    print!("{}", string);
}
