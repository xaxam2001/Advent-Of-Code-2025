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

        let mut highest_joltage_first_digit = usize::from(line[0] - b'0');
        let mut highest_joltage_second_digit = usize::from(line[1] - b'0');

        for (i, current_digit) in line.iter().skip(joltage_size).enumerate() {
            let current_digit = usize::from(current_digit - b'0');
            if current_digit > highest_joltage_second_digit
            {
                highest_joltage_second_digit = current_digit;
            }
            if current_digit > highest_joltage_first_digit
                && i+joltage_size < line.len() - 1
            {
                highest_joltage_first_digit = current_digit;
                highest_joltage_second_digit = usize::from(line[i+joltage_size+1] - b'0');
            }

        }
        
        total_joltage += highest_joltage_first_digit * 10 + highest_joltage_second_digit;
    }

    total_joltage
}


pub fn d3p1(s: &str) -> usize {
    d3p1_v4(s)
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