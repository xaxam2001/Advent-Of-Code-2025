
#[allow(unused)]
pub fn d2p1_v1(s: &str) -> usize {
    let ranges = s.split(',');

    let mut total = 0usize;

    for range  in ranges {
        let mut parts = range.split('-');

        let Some(first_bound) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_bound) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        let Ok(first_bound) =  first_bound.parse::<usize>() else { panic!("couldn't parse number") };
        let Ok(end_bound) =  end_bound.parse::<usize>() else { panic!("couldn't parse number") };

        for id in first_bound..=end_bound {
            let id_str = id.to_string();

            if id_str.chars().count() % 2 == 1 { continue; }

            let second_part_index = id_str.chars().count()/2;

            let first_part = &id_str[..second_part_index];
            let second_part = &id_str[second_part_index..];

            if first_part == second_part {total += id}
        }
    }

    total
}

#[allow(unused)]
pub fn d2p1_v2(s: &str) -> usize {

    let mut total = 0usize;

    // using u8 representation as well here
    for range in s.as_bytes().split(|&b| b == b',') {
        let mut parts = range.split(|&b| b == b'-');

        let Some(first_part) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_part) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        // Fast manual parsing from bytes to usize
        let mut first_bound = 0usize;
        for &b in first_part {
            first_bound = first_bound * 10 + (b - b'0') as usize;
        }

        let mut end_bound = 0usize;
        for &b in end_part {
            end_bound = end_bound * 10 + (b - b'0') as usize;
        }

        for id in first_bound..=end_bound {
            // convert the number to a String to split it
            let id_str = id.to_string();
            let id_bytes = id_str.as_bytes();

            let len = id_bytes.len();

            // odd number of digits can't be divided
            if len % 2 != 0 { continue; }

            let mid = len / 2;

            // getting each half and comparing
            let left_half = &id_bytes[..mid];
            let right_half = &id_bytes[mid..];

            if left_half == right_half {
                total += id;
            }
        }
    }

    total
}

#[allow(unused)]
pub fn d2p1_v3(s: &str) -> usize {

    let mut total = 0usize;

    // using u8 representation as well here
    for range in s.as_bytes().split(|&b| b == b',') {
        let mut parts = range.split(|&b| b == b'-');

        let Some(first_part) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_part) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        // Fast manual parsing from bytes to usize
        let mut first_bound = 0usize;
        for &b in first_part {
            first_bound = first_bound * 10 + (b - b'0') as usize;
        }

        let mut end_bound = 0usize;
        for &b in end_part {
            end_bound = end_bound * 10 + (b - b'0') as usize;
        }

        for id in first_bound..=end_bound {
            if id == 0 { continue; }

            // .ilog10() is an instruction to find the number of digits - 1
            let digits = id.ilog10() + 1;

            if digits % 2 != 0 { continue; } // if number of digits is odd then we can't split it in half

            // get the middle of the number
            let half_digits = digits / 2;
            let divisor = 10usize.pow(half_digits); // need to precise usize for rust to get the right pow

            // I wouldn't have thought of this
            // Split the number mathematically
            // 1212 / 100 = 12 (Left half)
            // 1212 % 100 = 12 (Right half)
            if id / divisor == id % divisor {
                total += id;
            }
        }
    }

    total
}

#[allow(unused)]
pub fn d2p2_v1(s: &str) -> usize {
    let ranges = s.split(',');

    let mut total = 0usize;

    for range  in ranges {
        let mut parts = range.split('-');

        let Some(first_bound) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_bound) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        let Ok(first_bound) =  first_bound.parse::<usize>() else { panic!("couldn't parse number") };
        let Ok(end_bound) =  end_bound.parse::<usize>() else { panic!("couldn't parse number") };

        for id in first_bound..=end_bound {
            let id_str = id.to_string();

            // [optimization] store the str count in a variable

            // try if it's divisible from 2 to id_str.chars().count() and check if every part are equal
            for div in 2..=id_str.chars().count(){
                if id_str.chars().count() % div != 0 { continue; } // if you can't divide it then the different part can't be equal

                let mut all_same = true;

                for part in 0..div-1{
                    let section_size = id_str.chars().count()/div;

                    let first_part = &id_str[section_size*part..section_size*(part+1)];
                    let second_part = &id_str[section_size*(part+1)..section_size*(part+2)];

                    if first_part != second_part {all_same = false; break;}
                }

                if all_same {total+=id; break;}
            }
        }
    }

    total
}

#[allow(unused)]
pub fn d2p2_v2(s: &str) -> usize {
    let ranges = s.split(',');

    let mut total = 0usize;

    for range  in s.as_bytes().split(|&b| b == b',') {
        let mut parts = range.split(|&b| b == b'-');

        let Some(first_part) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_part) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        // Fast manual parsing from bytes to usize
        let mut first_bound = 0usize;
        for &b in first_part {
            first_bound = first_bound * 10 + (b - b'0') as usize;
        }

        let mut end_bound = 0usize;
        for &b in end_part {
            end_bound = end_bound * 10 + (b - b'0') as usize;
        }

        for id in first_bound..=end_bound {
            if id == 0 { continue; }

            // .ilog10() is an instruction to find the number of digits - 1
            let digits = id.ilog10() + 1;
            // try if it's divisible from 2 to id_str.chars().count() and check if every part are equal
            for div in 2..=digits{
                if digits % div != 0 { continue; } // if you can't divide it then the different part can't be equal
                
                let mut all_same = true;

                let section_size = digits/div; // moved section_size out of the for loop
                let modulo = 10usize.pow(section_size);

                let mut right_part = id % modulo;

                for part in 1..div {
                    let divisor = 10usize.pow(section_size * part);
                    let left_part = id/divisor % modulo;

                    if right_part != left_part {all_same = false; break;}

                    right_part = left_part;
                }
                if all_same {total+=id; break;}
            }
        }
    }

    total
}

#[allow(unused)]
pub fn d2p2_v3(s: &str) -> usize {
    let ranges = s.split(',');

    let mut total = 0usize;

    let power_lookup = [1usize,10,100,1000,10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000,
    10_000_000_000, 100_000_000_000, 1_000_000_000_000, 10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000];

    for range  in s.as_bytes().split(|&b| b == b',') {
        let mut parts = range.split(|&b| b == b'-');

        let Some(first_part) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_part) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        // Fast manual parsing from bytes to usize
        let mut first_bound = 0usize;
        for &b in first_part {
            first_bound = first_bound * 10 + (b - b'0') as usize;
        }

        let mut end_bound = 0usize;
        for &b in end_part {
            end_bound = end_bound * 10 + (b - b'0') as usize;
        }

        for id in first_bound..=end_bound {
            if id == 0 { continue; }

            // .ilog10() is an instruction to find the number of digits - 1
            let digits = id.ilog10() + 1;
            // try if it's divisible from 2 to id_str.chars().count() and check if every part are equal
            for div in 2..=digits{
                if digits % div != 0 { continue; } // if you can't divide it then the different part can't be equal

                let mut all_same = true;

                let section_size = digits/div; // moved section_size out of the for loop
                let modulo = power_lookup[section_size as usize];

                let mut right_part = id % modulo;

                for part in 1..div {
                    let divisor = power_lookup[(section_size * part) as usize];
                    let left_part = id/divisor % modulo;

                    if right_part != left_part {all_same = false; break;}

                    right_part = left_part;
                }
                if all_same {total+=id; break;}
            }
        }
    }

    total
}

#[allow(unused)]
pub fn d2p2_v4(s: &str) -> usize {
    let ranges = s.split(',');

    let mut total = 0usize;

    let power_lookup = [1usize,10,100,1000,10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000,
        10_000_000_000, 100_000_000_000, 1_000_000_000_000, 10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000];

    let primes : [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]; // store the 10 first prime number to split the numbers

    for range  in s.as_bytes().split(|&b| b == b',') {
        let mut parts = range.split(|&b| b == b'-');

        let Some(first_part) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_part) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        // Fast manual parsing from bytes to usize
        let mut first_bound = 0usize;
        for &b in first_part {
            first_bound = first_bound * 10 + (b - b'0') as usize;
        }

        let mut end_bound = 0usize;
        for &b in end_part {
            end_bound = end_bound * 10 + (b - b'0') as usize;
        }

        for id in first_bound..=end_bound {
            if id == 0 { continue; }

            // .ilog10() is an instruction to find the number of digits - 1
            let digits = id.ilog10() + 1;
            // try if it's divisible from 2 to id_str.chars().count() and check if every part are equal
            for &div in &primes{
                if div > digits { break; } // if the prime is more than the number of digits, stop the loop
                if digits % div != 0 { continue; } // if you can't divide it then the different part can't be equal

                let mut all_same = true;

                let section_size = digits/div; // moved section_size out of the for loop
                let modulo = power_lookup[section_size as usize];

                let mut right_part = id % modulo;

                for part in 1..div {
                    let divisor = power_lookup[(section_size * part) as usize];
                    let left_part = id/divisor % modulo;

                    if right_part != left_part {all_same = false; break;}

                    right_part = left_part;
                }
                if all_same {total+=id; break;}
            }
        }
    }

    total
}

#[allow(unused)]
pub fn d2p2_v5(s: &str) -> usize {
    let ranges = s.split(',');

    let mut total = 0usize;

    let power_lookup = [1usize,10,100,1000,10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000,
        10_000_000_000, 100_000_000_000, 1_000_000_000_000, 10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000];

    let primes : [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]; // store the 10 first prime number to split the numbers

    for range  in s.as_bytes().split(|&b| b == b',') {
        let mut parts = range.split(|&b| b == b'-');

        let Some(first_part) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_part) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        // Fast manual parsing from bytes to usize
        let mut first_bound = 0usize;
        for &b in first_part {
            first_bound = first_bound * 10 + (b - b'0') as usize;
        }

        let mut end_bound = 0usize;
        for &b in end_part {
            end_bound = end_bound * 10 + (b - b'0') as usize;
        }

        for id in first_bound..=end_bound {
            if id == 0 { continue; }

            // .ilog10() is an instruction to find the number of digits - 1
            let digits = id.ilog10() + 1;
            // try if it's divisible from 2 to id_str.chars().count() and check if every part are equal
            for &div in &primes{
                if div > digits { break; } // if the prime is more than the number of digits, stop the loop
                if digits % div != 0 { continue; } // if you can't divide it then the different part can't be equal

                let section_size = digits/div;

                let divisor = power_lookup[section_size as usize]; // we divide to drop the last section
                let modulo = power_lookup[(digits - section_size) as usize]; // we do a modulo to drop the first block

                // if the overlapping sections are the same then they must be similar
                if id / divisor == id % modulo {total+=id; break;}
            }
        }
    }

    total
}

pub fn d2p1(s: &str) -> usize {
    d2p1_v3(s)
}

pub fn d2p2(s: &str) -> usize {
    d2p2_v5(s)
}

#[cfg(test)]
mod tests {
    use crate::d2::{d2p1, d2p2};

    #[test]
    fn d2p1_test(){
        let s = include_str!("d2_test.txt");
        let result = d2p1(s);
        assert_eq!(1227775554, result);
    }

    #[test]
    fn d2p2_test(){
        let s = include_str!("d2_test.txt");
        let result = d2p2(s);
        assert_eq!(4174379265, result);
    }
}