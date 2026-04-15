pub fn d7p1(s: &str) -> usize {
    d7p1_v1(s)
}

#[allow(unused)]
pub fn d7p1_v1(s: &str) -> usize {
    let mut total_split = 0;

    // Break the string into lines (to be able to modify it)
    let mut lines: Vec<String> = s.lines().map(String::from).collect();

    // Iterate with Index (otherwise it's locked
    for i in 1..lines.len() {
        // clone the previous line to avoid the borrow checker
        let prev_line = lines[i - 1].clone();

        // new temp line to hold the new line
        let mut new_current_line = lines[i].clone();

        // build new line character by character
        for (char_index, c) in lines[i].chars().enumerate() {
            let upper_char = prev_line.chars().nth(char_index).unwrap_or(' ');

            match (c, upper_char) {
                ('.', '|' | 'S') => {
                    new_current_line.replace_range(char_index..=char_index, "|");
                },
                ('^', '|') => {
                    total_split += 1;
                    new_current_line.replace_range(char_index-1..=char_index+1, "|^|");
                },
                _ => {
                }
            }
        }

        // writing new line
        lines[i] = new_current_line;
    }

    total_split
}

pub fn d7p2(s: &str) -> usize {
    d7p2_v1(s)
}

#[allow(unused)]
pub fn d7p2_v1(s: &str) -> usize {

    // Break the string into lines (to be able to modify it)
    let mut lines: Vec<String> = s.lines().map(String::from).collect();

    // store the different timelines each time it is separated
    let mut timelines_count: Vec<usize> = vec![0; lines[0].len()];

    // Iterate with Index (otherwise it's locked
    for i in 1..lines.len() {
        // clone the previous line to avoid the borrow checker
        let prev_line = lines[i - 1].clone();

        // make a snapshot of the previous timeline count to update it
        let mut new_timelines_count =  vec![0; lines[0].len()];

        // new temp line to hold the new line
        let mut new_current_line = lines[i].clone();

        // build new line character by character
        for (char_index, c) in lines[i].chars().enumerate() {
            let upper_char = prev_line.chars().nth(char_index).unwrap_or(' ');

            match (c, upper_char) {
                ('.', 'S') => {
                    new_current_line.replace_range(char_index..=char_index, "|");
                    new_timelines_count[char_index] = 1;
                },
                ('.', '|') => {
                    new_current_line.replace_range(char_index..=char_index, "|");
                    new_timelines_count[char_index] += timelines_count[char_index];
                },
                ('^', '|') => {
                    new_timelines_count[char_index] = 0;
                    new_timelines_count[char_index-1] += timelines_count[char_index];
                    new_timelines_count[char_index+1] += timelines_count[char_index];

                    new_current_line.replace_range(char_index-1..=char_index+1, "|^|");
                },
                _ => {
                }
            }
        }
        
        timelines_count = new_timelines_count;

        // writing new line
        lines[i] = new_current_line;
    }

    timelines_count.iter().sum::<usize>()
}
#[cfg(test)]
mod tests {
    use crate::d7::{d7p1, d7p2};

    #[test]
    fn d7p1_test(){
        let s = include_str!("d7_test.txt");
        let result = d7p1(s);
        assert_eq!(21, result);
    }

    #[test]
    fn d7p2_test(){
        let s = include_str!("d7_test.txt");
        let result = d7p2(s);
        assert_eq!(40, result);
    }
}