pub fn d4p1(s: &str) -> usize {
    d4p1_v1(s)
}

pub fn d4p1_v1(s: &str) -> usize {
    let mut total_accessible = 0;

    let mut line_index = 0;

    for line in s.lines() {
        let mut char_index = 0;

        for char in line.chars(){
            if char == '.' {
                char_index += 1;
                continue;
            }

            let mut adjacent_paper = 0;
            for i in -1..=1 {
                let Ok(neighbor_line_index) = (line_index+i).try_into() else {continue;};
                for j in -1..=1 {
                    if i == 0 && j == 0 {continue;}
                    let Ok(char_line_index) = (char_index+j).try_into() else {continue;};
                    match s.lines().nth(neighbor_line_index) {
                        None => {}
                        Some(neighbor_line) => {match neighbor_line.chars().nth(char_line_index) {
                            None => {}
                            Some(neighbor_char) => {if neighbor_char == '@' {adjacent_paper+=1}}
                        }}
                    }
                }
            }

            char_index += 1;

            if adjacent_paper < 4 {total_accessible+=1;}
        }

        line_index+=1;
    }

    total_accessible
}

pub fn d4p2(s: &str) -> usize {
    d4p2_v1(s)
}

pub fn d4p2_v1(s: &str) -> usize {
    let mut total_accessible = 0;

    let mut state = s.to_string();

    loop {
        let mut accessible_in_this_state = 0;
        let mut line_index = 0;

        let state_copy = state.clone(); // Take a quick snapshot
        for line in state_copy.lines() {
            let mut char_index = 0;

            let line_length = line.chars().count();

            for char in line.chars() {
                if char == '.' {
                    char_index += 1;
                    continue;
                }

                let mut adjacent_paper = 0;
                for i in -1..=1 {
                    let Ok(line_index) = i32::try_from(line_index) else { panic!() };
                    let Ok(neighbor_line_index) = (line_index + i).try_into() else { continue; };
                    for j in -1..=1 {
                        if i == 0 && j == 0 { continue; }
                        let Ok(char_index) = i32::try_from(char_index) else { panic!() };
                        let Ok(char_line_index) = (char_index + j).try_into() else { continue; };
                        match state.lines().nth(neighbor_line_index) {
                            None => {}
                            Some(neighbor_line) => {
                                match neighbor_line.chars().nth(char_line_index) {
                                    None => {}
                                    Some(neighbor_char) => { if neighbor_char == '@' { adjacent_paper += 1 } }
                                }
                            }
                        }
                    }
                }

                if adjacent_paper < 4 {
                    accessible_in_this_state += 1;

                    let absolute_index: usize = char_index + line_index * (line_length + 1);

                    state.replace_range(absolute_index..=absolute_index, ".");
                }

                char_index += 1;

            }

            line_index += 1;
        }

        total_accessible += accessible_in_this_state;

        if accessible_in_this_state == 0 {break}

    }
    total_accessible
}

#[cfg(test)]
mod tests {
    use crate::d4::{d4p1, d4p2};

    #[test]
    fn d4p1_test(){
        let s = include_str!("d4_test.txt");
        let result = d4p1(s);
        assert_eq!(13, result);
    }

    #[test]
    fn d4p2_test(){
        let s = include_str!("d4_test.txt");
        let result = d4p2(s);
        assert_eq!(43, result);
    }
}