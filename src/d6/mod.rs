pub fn d6p1(s: &str) -> usize {
    d6p1_v1(s)
}

pub fn d6p1_v1(s: &str) -> usize {
    let mut values = Vec::<Vec::<usize>>::new();
    let mut operators = Vec::<&str>::new();

    let mut i = 0;

    for line in s.lines() {

        let nums = line.split_whitespace();


        let mut j = 0;
        for num in nums{
            // we add the operators if it's the last line
            if i == s.lines().count() - 1 {
                operators.push(num);
            } else { // otherwise we add the number into it's corresponding column
                let Ok(num) = num.parse::<usize>() else { continue };

                if i == 0 {
                    values.push(Vec::<usize>::new()); // if it's the first line we instantiate each vec that contains a column
                }

                values[j].push(num);
            }
            j+=1;
        }

        i+=1;
    }

    let mut total = 0;

    for i in 0..values.len(){
        let mut local_total= 0;
        if operators[i] == "*" {
            local_total = 1;
            for num in &values[i]{
                local_total *= num;
            }
        } else if operators[i] == "+" {
            local_total = 0;
            for num in &values[i]{
                local_total += num;
            }
        }

        total += local_total;
    }

    total
}

pub fn d6p2(s: &str) -> usize {
    d6p2_v1(s)
}

pub fn d6p2_v1(s: &str) -> usize {
    let lines: Vec<&str> = s.lines().collect();

    let mut total= 0;

    // Find the length of the longest line to avoid going out of bounds
    // can be deleted here for performance
    let max_len = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    let mut values = Vec::<usize>::new();
    let mut operator = ' ';

    for col in 0..max_len {

        let mut number = String::new();

        for (line_count, line) in lines.iter().enumerate(){
            // Use .nth() to get the character at the column index
            if let Some(c) = line.chars().nth(col) {
                if line_count == lines.len() - 1 && c != ' ' {
                    operator = c;
                }
                else {

                    number.push(c);
                }
            }
            else { panic!("col out of bounds") }
        }

        if let Ok(num) = number.trim().parse::<usize>() { values.push(num) }

        if number.trim().is_empty() || col == max_len - 1{
            // OPTIM: compute the result col by col without storing in a buffer
            let mut local_total = 0;
            if operator == '+' {
                for val in &values {
                    local_total += val;
                }
            }
            else if operator == '*' {
                local_total = 1;
                // using reference here bc otherwise iter is considered mutable and already mutable in push up there
                for val in &values {
                    local_total *= val;
                }
            }
            total += local_total;
            values = Vec::<usize>::new(); // reset the buffer
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::d6::{d6p1, d6p2};

    #[test]
    fn d6p1_test(){
        let s = include_str!("d6_test.txt");
        let result = d6p1(s);
        assert_eq!(4277556, result);
    }

    #[test]
    fn d6p2_test(){
        let s = include_str!("d6_test.txt");
        let result = d6p2(s);
        assert_eq!(3263827, result);
    }
}