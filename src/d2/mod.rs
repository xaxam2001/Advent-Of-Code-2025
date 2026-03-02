pub fn d2p1(s: &str) -> usize {
    d2p1_v1(s)
}

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

pub fn d2p2(s: &str) -> usize {
    d2p2_v1(s)
}

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