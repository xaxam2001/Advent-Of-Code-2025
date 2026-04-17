use std::time::Duration;

pub fn d1p1_v1(s: &str) -> usize {
    //std::thread::sleep(Duration::from_millis(40));
    let mut result: i32 = 0;
    let mut position:i32 = 0;

    let rotations = s.split_whitespace();

    for rotation in  rotations {
        let (dir, rot) = rotation.split_at(1); //ok car L et R = 1 octet
        let rot: i32 = rot.parse().unwrap_or(0); //conversion entier

        if dir == "L" {
            position = (position - rot).rem_euclid(100);
        } else {
            position = (position + rot).rem_euclid(100);
        }
        if position == 0 { result += 1 }
    }

    result as usize
}

pub fn d1p1_v2(s: &str) -> usize {
    let mut result = 0;
    let mut position: i32 = 0;

    for rotation in s.split_whitespace() {
        let bytes = rotation.as_bytes();
        let dir = bytes[0];

        let rot = rotation[1..].parse::<i32>().unwrap_or(0);

        if dir == b'L' {
            position = (position - rot).rem_euclid(100);
        } else {
            position = (position + rot).rem_euclid(100);
        }

        if position == 0 { result += 1; }
    }
    result as usize
}

pub fn d1p1_v3(s: &str) -> usize {
    let mut result = 0;
    let mut position: i32 = 0;

    for rotation in s.split_whitespace() {
        let bytes = rotation.as_bytes();
        let dir = bytes[0];

        let rot = rotation[1..].parse::<i32>().unwrap_or(0);

        if dir == b'R' {
            position = (position + rot).rem_euclid(100);
        } else {
            position = (position - rot).rem_euclid(100);
        }

        if position == 0 { result += 1; }
    }
    result as usize
}

pub fn d1p1_v4(s: &str) -> usize {
    let mut result = 0;
    let mut position: i32 = 0;

    for rotation in s.split_whitespace() {
        let bytes = rotation.as_bytes();
        let dir = bytes[0];

        let rot = rotation[1..].parse::<i32>().unwrap_or(0);

        if dir == b'L' {
            position = (position - rot + 100) % 100;
        } else {
            position = (position + rot) % 100;
        }

        if position == 0 { result += 1; }
    }
    result as usize
}


pub fn d1p2_v1(s: &str) -> usize {
    //std::thread::sleep(Duration::from_millis(30));

    let mut result: i32 = 0;
    let mut position:i32 = 50;
    let rotations = s.split_whitespace();

    for rotation in  rotations {
        let (dir, rot) = rotation.split_at(1);
        let rot: i32 = rot.parse().unwrap_or(0);

        for _ in 0..rot {
            if dir == "L" {
                position = (position - 1).rem_euclid(100);

            } else {
                position = (position + 1).rem_euclid(100);
            }

            if position == 0 { result += 1 }
        }
    }

    result as usize
}

pub fn d1p2(s: &str) -> usize {
    d1p2_v1(s)
}

pub fn d1p1(s: &str) -> usize {
    d1p1_v3(s)
}

#[cfg(test)]
mod tests {
    use crate::d1::{d1p1, d1p2};

    #[test]
    fn d1p1_test()
    {
        let s = include_str!("d1.txt");
        let result:usize = d1p1(s);
        assert_eq!(964, result);
        println!("Partie 1 : {} ", result);
    }

    #[test]
    fn d1p2_test()
    {
        let s = include_str!("d1.txt");
        let result:usize = d1p2(s);
        assert_eq!(5872, result);
        println!("Partie 2 : {} ", result);
    }
}