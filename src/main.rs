use std::fs;

fn main() {
    let mut result: i32 = 0;
    let mut position:i32 = 50;
    let contents = fs::read_to_string(r"F:\Projets\Advent-Of-Code-2025\src\d1\input.txt").expect("Something went wrong reading the file");

    let rotations = contents.split_whitespace();

    for rotation in  rotations {
        let (dir, rot) = rotation.split_at(1);
        let rot: i32 = rot.parse().unwrap_or(0);

        if dir == "L" {
            position = (position - rot).rem_euclid(100);
        } else {
            position = (position + rot).rem_euclid(100);
        }
        if position == 0 { result += 1 }
    }

    println!("{}", result);
}
/*
fn clamp_dial(input: i32) -> i32
{
    if (input > 100)
    {
        100 - input
    } else if (input < 0) {
        100 + input
    } else {
        0 + input
    }
}*/