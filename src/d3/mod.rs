pub fn d3p1(s: &str) -> usize {
    d3p1_v1(s)
}

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

pub fn d3p2(s: &str) -> usize {
    d3p2_v1(s)
}

pub fn d3p2_v1(s: &str) -> usize {
    let mut total_joltage = 0;

    for line in s.lines() {

        let mut drops_left = line.len() - 12;
        let mut stack: Vec<char> = Vec::new();

        for current_char in line.chars(){
            while let Some(&top_char) = stack.last() {
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