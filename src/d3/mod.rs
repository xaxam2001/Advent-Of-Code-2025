#[allow(unused)]
pub fn d3p1_v1(s: &str) -> usize {
    let mut total_joltage = 0;

    for line in s.lines(){

        let mut highest_joltage = 0;

        for i in 0..line.chars().count() - 1{
            let remaining_of_line = line[i..].to_string();
            let Some(first_char) = remaining_of_line.chars().next() else {panic!("Couldn't find any char at the beginning of the line")};
            let mut first_char = first_char.to_string();

            for j in 1..remaining_of_line.chars().count() {
                let Some(second_char) = remaining_of_line.chars().nth(j) else { panic!("Couldn't find a char, error while parsing")};
                first_char.push(second_char);
                let joltage = match first_char.parse::<usize>() {
                    Ok(joltage) => joltage,
                    Err(e) => panic!("Error found when parsing number: {e}")
                };

                if joltage > highest_joltage {highest_joltage=joltage}

                first_char.pop();
            }
        }
        total_joltage += highest_joltage;
    }

    total_joltage
}

#[allow(unused)]
pub fn d3p1_v2(s: &str) -> usize {
    let mut total_joltage = 0;

    for line in s.lines(){

        let mut highest_joltage = 0;

        for (i, first_char) in line.chars().take(line.len() - 1).enumerate() {
            let mut first_char = first_char.to_string();

            for second_char in line.chars().skip(i+1) { // clippy :)
                first_char.push(second_char);
                let joltage = match first_char.parse::<usize>() {
                    Ok(joltage) => joltage,
                    Err(e) => panic!("Error found when parsing number: {e}")
                };

                if joltage > highest_joltage {highest_joltage=joltage;}

                first_char.pop();
            }
        }
        total_joltage += highest_joltage;
    }
    total_joltage
}


#[allow(unused)]
pub fn d3p1_v3(s: &str) -> usize {
    let mut total_joltage = 0;

    for line in s.as_bytes().split(|&b| b == b'\n'){

        let mut highest_joltage = 0;

        for i in 0..line.len() - 1{
            let first_digit = usize::from(line[i] - b'0') * 10;

            for second_digit in line.iter().skip(i+1) {
                let joltage = first_digit + usize::from(second_digit - b'0');

                if joltage > highest_joltage {highest_joltage=joltage;}
            }
        }
        total_joltage += highest_joltage;
    }

    total_joltage
}

#[allow(unused)]
pub fn d3p1_v4(s: &str) -> usize {
    let mut total_joltage = 0;
    let joltage_size = 2;

    for line in s.as_bytes().split(|&b| b == b'\n'){

        let mut max_first_seen = usize::from(line[0] - b'0');
        let mut highest_joltage = 0;

        for (i, current_digit) in line.iter().skip(1).enumerate() {
            let current_digit = usize::from(current_digit - b'0');

            // we pair the current digit with the biggest number we've seen to its left
            let potential_joltage = max_first_seen * 10 + current_digit;

            if potential_joltage > highest_joltage {
                highest_joltage = potential_joltage;
            }

            // Check if the current digit is better than the past ones
            if current_digit > max_first_seen {
                max_first_seen = current_digit;
            }
        }
        
        total_joltage += highest_joltage;
    }

    total_joltage
}

#[allow(unused)]
pub fn d3p1_v5(s: &str) -> usize {
    let mut total_joltage = 0;
    let joltage_size = 2;

    for line in s.as_bytes().split(|&b| b == b'\n'){

        let mut max_first_seen = line[0] - b'0';
        let mut highest_joltage = 0;

        for (i, current_digit) in line.iter().skip(1).enumerate() {
            let current_digit = current_digit - b'0';

            // we pair the current digit with the biggest number we've seen to its left
            let potential_joltage = usize::from(max_first_seen) * 10 + usize::from(current_digit);

            if potential_joltage > highest_joltage {
                highest_joltage = potential_joltage;
            }

            // Check if the current digit is better than the past ones
            if current_digit > max_first_seen {
                max_first_seen = current_digit;
            }
        }

        total_joltage += highest_joltage;
    }

    total_joltage
}

#[allow(unused)]
pub fn d3p1_v6(s: &str) -> usize {
    let mut total_joltage = 0;
    let joltage_size = 2;

    let bytes = s.as_bytes(); // get the raw bytes directly

    let mut max_first_seen = bytes[0] - b'0';
    let mut highest_joltage = 0;

    let mut new_line_index = 0;

    for (i, &b) in bytes.iter().enumerate() {

        if i == new_line_index {continue};

        match b {
            b'0'..=b'9' => {
                let current_digit = b - b'0';
                // we pair the current digit with the biggest number we've seen to its left
                let potential_joltage = usize::from(max_first_seen) * 10 + usize::from(current_digit);

                if potential_joltage > highest_joltage {
                    highest_joltage = potential_joltage;
                }

                // Check if the current digit is better than the past ones
                if current_digit > max_first_seen {
                    max_first_seen = current_digit;
                }
            }
            b'\n' => {
                // if switching to next number add the value of the highest joltage
                total_joltage += highest_joltage;

                // reset the highest joltage digit using the next first one
                max_first_seen = bytes[i+1] - b'0';
                highest_joltage = 0;

                // we set the new line index
                new_line_index = i + 1;
            }
            _ => ()
        }
    }

    // add final line calculation because file doesn't end with a \n
    total_joltage += highest_joltage;
    
    total_joltage
}


pub fn d3p1(s: &str) -> usize {
    d3p1_v6(s)
}

pub fn d3p2_v1(s: &str) -> usize {
    let mut total_joltage = 0;

    for line in s.lines() {

        let mut drops_left = line.len() - 12; //number of digits that can be dropped
        let mut stack: Vec<char> = Vec::new(); // final number

        for current_char in line.chars(){
            // as long as there are still char we compare to the next one
            while let Some(&top_char) = stack.last() {
                // if the char in the remaining part is smaller than the current char and that we can still drop digits we remove it 
                if top_char < current_char && drops_left > 0{
                    stack.pop();
                    drops_left -= 1;
                } else {
                    break;
                }
            }

            stack.push(current_char);

        }

        stack.truncate(12); // if there is still numbers at the end like in 98765432111111

        let number : String = stack.into_iter().collect();

        total_joltage += match number.parse::<usize>() {
            Ok(num) => num,
            Err(e) => panic!("Error found when parsing number: {e}")
        }
    }

    total_joltage
}

pub fn d3p2(s: &str) -> usize {
    d3p2_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d3::{d3p1, d3p2};

    #[test]
    fn d3p1_test(){
        let s = include_str!("d3_test.txt");
        let result = d3p1(s);
        assert_eq!(357, result);
    }

    #[test]
    fn d3p2_test(){
        let s = include_str!("d3_test.txt");
        let result = d3p2(s);
        assert_eq!(3121910778619, result);
    }
}