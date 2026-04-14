mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;

fn main() {
    println!("D1P1: {}", d1::d1p1(include_str!("d1/d1.txt")));
    println!("D1P2: {}", d1::d1p2(include_str!("d1/d1.txt")));
    println!("D2P1: {}", d2::d2p1(include_str!("d2/d2.txt")));
    println!("D2P2: {}", d2::d2p2(include_str!("d2/d2.txt")));
    println!("D3P1: {}", d3::d3p1(include_str!("d3/d3.txt")));
    println!("D3P2: {}", d3::d3p2(include_str!("d3/d3.txt")));
    println!("D4P1: {}", d4::d4p1(include_str!("d4/d4.txt")));
    println!("D4P2: {}", d4::d4p2(include_str!("d4/d4.txt")));
    println!("D5P1: {}", d5::d5p1(include_str!("d5/d5.txt")));
    println!("D5P2: {}", d5::d5p2(include_str!("d5/d5.txt")));
    println!("D6P1: {}", d6::d6p1(include_str!("d6/d6.txt")));
    println!("D6P2: {}", d6::d6p2(include_str!("d6/d6.txt")));
    println!("D7P1: {}", d7::d7p1(include_str!("d7/d7.txt")));
    println!("D7P2: {}", d7::d7p2(include_str!("d7/d7.txt")));
}
