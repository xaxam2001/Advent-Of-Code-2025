#[allow(unused)]
pub fn d1p1_v1(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero = 0;

    for line in s.lines() {
        let mut value = line.chars();
        value.next();

        let Ok(value) =  value.as_str().parse::<i32>() else {
            panic!("couldn't parse number from ")
        };

        if line.starts_with('L') {
            total -= value;
        }else if line.starts_with('R') {
            total += value;
        }else {
            panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line")
        }

        total = total.rem_euclid(100);

        if total == 0 {count_zero += 1}
    }

    count_zero
}

#[allow(unused)]
pub fn d1p1_v2(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero = 0;

    // trying to use bytes_vec since it's ascii
    // allocation on the heap here!
    let bytes_vec: Vec<u8> = s.as_bytes().to_vec();

    // bytes_vec.lines doesn't exist so has to manually separate the lines
    for line in bytes_vec.split(|&b| b == b'\n') {
        let mut value = 0;

        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to usize (char_byte - ascii code of 0)
            value = value * 10 + i32::from(b - b'0');
        }

        // match directly on the byte value
        match line[0] {
            b'L' => total -= value,
            b'R' => total += value,
            _ => panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line"),
        }

        total = total.rem_euclid(100);

        if total == 0 {count_zero += 1}
    }

    count_zero
}
#[allow(unused)]
pub fn d1p1_v3(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero = 0;

    // trying to use bytes_vec since it's ascii
    // no allocation on the heap here, only manipulating variable on the stack
    for line in s.as_bytes().split(|&b| b == b'\n') {
        let mut value = 0;

        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to usize (char_byte - ascii code of 0)
            value = value * 10 + i32::from(b - b'0');
        }

        // match directly on the byte value
        match line[0] {
            b'L' => total -= value,
            b'R' => total += value,
            _ => panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line"),
        }

        total = total.rem_euclid(100);

        if total == 0 {count_zero += 1}
    }

    count_zero
}

#[allow(unused)]
pub fn d1p1_v4(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero = 0;

    // trying to use bytes_vec since it's ascii
    // no allocation on the heap here, only manipulating variable on the stack
    for line in s.as_bytes().split(|&b| b == b'\n') {
        let mut value = 0;

        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to i32 (char_byte - ascii code of 0)
            value = value * 10 + i32::from(b - b'0');
        }

        // match directly on the byte value
        match line[0] {
            // Prevent going negative to use the standard %
            b'L' => total = (total  + 100 - (value % 100)) % 100,
            b'R' => total = (total + value) % 100,
            _ => panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line"),
        }

        if total == 0 {count_zero += 1}
    }

    count_zero
}

#[allow(unused)]
pub fn d1p1_v5(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero = 0;

    // trying to use bytes_vec since it's ascii
    // no allocation on the heap here, only manipulating variable on the stack
    for line in s.as_bytes().split(|&b| b == b'\n') {
        let mut value = 0;

        #[allow(clippy::cast_lossless)]
        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to usize (char_byte - ascii code of 0)
            value = value * 10 + (b - b'0') as i32;
        }

        // match directly on the byte value
        match line[0] {
            // Prevent going negative, allowing us to use standard %
            b'L' => total = (total - (value % 100) + 100) % 100,
            b'R' => total = (total + value) % 100,
            _ => panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line"),
        }

        if total == 0 {count_zero += 1}
    }

    count_zero
}

#[allow(unused)]
pub fn d1p1_v6(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero = 0;

    // trying to use bytes_vec since it's ascii
    // no allocation on the heap here, only manipulating variable on the stack
    for line in s.as_bytes().split(|&b| b == b'\n') {
        // should make the compiler not do any bound check here because it's guaranteed not to be empty
        if line.is_empty() {
            continue;
        }

        let mut value = 0;

        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to usize (char_byte - ascii code of 0)
            value = value * 10 + i32::from(b - b'0');
        }

        // match directly on the byte value
        match line[0] {
            // Prevent going negative, allowing us to use standard %
            b'L' => total = (total - (value % 100) + 100) % 100,
            b'R' => total = (total + value) % 100,
            _ => panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line"),
        }

        if total == 0 {count_zero += 1}
    }

    count_zero
}

#[allow(unused)]
pub fn d1p1_v7(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero = 0;

    // trying to use bytes_vec since it's ascii
    // no allocation on the heap here, only manipulating variable on the stack
    for line in s.as_bytes().split(|&b| b == b'\n') {

        let mut value = 0;

        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to usize (char_byte - ascii code of 0)
            value = value * 10 + i32::from(b - b'0');
        }

        // match directly on the byte value
        match line[0] {
            // Prevent going negative, allowing us to use standard %
            b'L' => total = (total - (value % 100) + 100) % 100,
            b'R' => total = (total + value) % 100,
            _ => continue, // removing panic from method v5
        }

        if total == 0 {count_zero += 1}
    }

    count_zero
}

#[allow(unused)]
pub fn d1p2_v1(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero:usize = 0;

    for line in s.lines() {
        let mut value = line.chars();
        value.next();
        
        let Ok(value) =  value.as_str().parse::<i32>() else {
            panic!("couldn't parse number from ")
        };

        if line.starts_with('L') {
            total -= value;
        }else if line.starts_with('R') {
            total += value;
        }else {
            panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line")
        }

        // count an extra pass to zero if the total is negative and we were didn't stop on zero
        if total <= 0 && total.abs() != value {count_zero += 1 ; }

        // the number of time we pass by zero correspond to the number of time we can divide the total by q 100
        count_zero += (total / 100).unsigned_abs() as usize;

        total = total.rem_euclid(100);
    }

    count_zero
}

#[allow(unused)]
pub fn d1p2_v2(s: &str) -> usize {

    let mut total = 50;
    let mut count_zero:usize = 0;

    // using bytes like in function d1p1_v3
    for line in  s.as_bytes().split(|&b| b == b'\n') {
        let mut value = 0;

        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to usize (char_byte - ascii code of 0)
            value = value * 10 + i32::from(b - b'0');
        }

        match line[0] {
            b'L' => total -= value,
            b'R' => total += value,
            _ => panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line"),
        }

        // count an extra pass to zero if the total is negative and we were didn't stop on zero
        if total <= 0 && total.abs() != value {count_zero += 1 ; }

        // the number of time we pass by zero correspond to the number of time we can divide the total by q 100
        count_zero += (total / 100).unsigned_abs() as usize;

        total = total.rem_euclid(100);
    }

    count_zero
}

#[allow(unused)]
pub fn d1p2_v3(s: &str) -> usize {
    let mut total = 50;
    let mut count_zero:usize = 0;

    // using bytes like in function d1p1_v3
    for line in  s.as_bytes().split(|&b| b == b'\n') {
        let mut value = 0;

        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to usize (char_byte - ascii code of 0)
            value = value * 10 + i32::from(b - b'0');
        }

        #[allow(clippy::cast_sign_loss)] // to avoid clippy here "count_zero += (new_pos / 100) as usize;" since it's all positive
        match line[0] {
            // separating the computation into two because going forward costs less
            b'R' => {
                let new_pos = total + value;
                count_zero += (new_pos / 100) as usize;
                total = new_pos % 100;
            }
            // keeping the substraction as it was for now
            b'L' => {
                let new_pos = total - value;
                // count an extra pass to zero if the total is negative and we were didn't stop on zero
                if new_pos <= 0 && new_pos.abs() != value {count_zero += 1 ; }

                // the number of time we pass by zero correspond to the number of time we can divide the new_pos by q 100
                count_zero += (new_pos / 100).unsigned_abs() as usize;

                total = new_pos.rem_euclid(100);
            }
            _ => panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line"),
        }


    }

    count_zero
}

#[allow(unused)]
pub fn d1p2_v4(s: &str) -> usize {
    let mut total = 50;
    let mut count_zero:usize = 0;

    // using bytes like in function d1p1_v3
    for line in  s.as_bytes().split(|&b| b == b'\n') {
        let mut value = 0;

        for &b in &line[1..] { // start from 1 to skip L or R
            // convert ASCII character to usize (char_byte - ascii code of 0)
            value = value * 10 + u32::from(b - b'0');
        }

        match line[0] {
            // separating the computation into two because going forward costs less
            b'R' => {
                let new_pos = total + value;
                count_zero += (new_pos / 100) as usize;
                total = new_pos % 100;
            }
            // keeping the substraction as it was for now
            b'L' => {
                if value < total {
                    //didn't even reach 0, just subtract
                    total -= value;
                } else {
                    // this is the distance we went through after crossing zero the first time
                    let distance_after_zero = value - total; // this is positive

                    // count when we pass by zero and add 1 only if the last position was different from 0 (avoid counting twice)
                    count_zero += (distance_after_zero / 100) as usize + usize::from(total != 0);

                    // distance_after_zero % 100 gives what we truly moved
                    // invert it like if we were coming from a 100
                    // last modulo in case distance_after_zero % 100 = 0
                    total = (100 - (distance_after_zero % 100)) % 100;
                }
            }
            _ => panic!("problem parsing, couldn't find 'L' or 'R' at the beginning of the line"),
        }


    }

    count_zero
}

pub fn d1p1(s: &str) -> usize {
    d1p1_v7(s)
}

pub fn d1p2(s: &str) -> usize {
    d1p2_v4(s)
}

#[cfg(test)]
mod tests {
    use crate::d1::{d1p1, d1p2};

    #[test]
    fn d1p1_test(){
        let s = include_str!("d1_test.txt");
        let result = d1p1(s);
        assert_eq!(3, result);
    }

    #[test]
    fn d1p2_test(){
        let s = include_str!("d1_test.txt");
        let result = d1p2(s);
        assert_eq!(6, result);
    }
}