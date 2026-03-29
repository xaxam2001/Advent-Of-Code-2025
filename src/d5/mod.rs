use std::collections::HashSet;

pub fn d5p1(s: &str) -> usize {
    d5p1_v1(s)
}

pub fn d5p1_v1(s: &str) -> usize {
    let mut range_browsed = false;

    // let mut fresh_ids = HashSet::new();
    let mut lower_bounds = Vec::<usize>::new();
    let mut higher_bounds  = Vec::<usize>::new();
    let mut valid_id = HashSet::<usize>::new();

    for line in s.lines() {
        if line.is_empty() {
            range_browsed = true;
            continue;
        }

        if range_browsed {
            let Ok(id) =  line.parse::<usize>() else { panic!("couldn't parse number") };
            // if fresh_ids.contains(&id) {total_fresh +=1}
            for index in 0..lower_bounds.len() {
                if id >= lower_bounds[index] && id <= higher_bounds[index] {
                    valid_id.insert(id);
                }
            }
        } else {
            let mut parts = line.split('-');

            let Some(first_bound) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
            let Some(end_bound) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

            let Ok(first_bound) =  first_bound.parse::<usize>() else { panic!("couldn't parse number") };
            let Ok(end_bound) =  end_bound.parse::<usize>() else { panic!("couldn't parse number") };

            // NOTE: allocation of 79_164_837_199_888 bytes failed (79TB)
            // fresh_ids.extend(first_bound..=end_bound);
            lower_bounds.push(first_bound);
            higher_bounds.push(end_bound);
        }
    }

    valid_id.len()
}

pub fn d5p2(s: &str) -> usize {
    d5p2_v1(s)
}

pub fn d5p2_v1(s: &str) -> usize {
    // storing each number of each range is not possible because the numbers are way too big
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for line in s.lines() {
        if line.is_empty() { // smarter than line == ""
            break;
        }

        let mut parts = line.split('-');

        let Some(first_bound) = parts.next() else { panic!("Invalid input no number found for first part of a range") };
        let Some(end_bound) = parts.next() else { panic!("Invalid input no number found for second part of a range") };

        let Ok(first_bound) =  first_bound.parse::<usize>() else { panic!("couldn't parse number") };
        let Ok(end_bound) =  end_bound.parse::<usize>() else { panic!("couldn't parse number") };

        ranges.push((first_bound, end_bound));
    }

    ranges.sort_unstable_by_key(|&(lower_bound, _)| lower_bound);

    // merge overlapping or adjacent ranges
    let mut merged: Vec<(usize, usize)> = Vec::new();
    merged.push(ranges[0]);

    for &(lower_bound, end_bound) in &ranges[1..] {
        // last range added into the list
        let Some(last) = merged.last_mut() else {panic!("error while going through the ranges")};

        // if ranges are touching each other or overlapping then we can merge them
        // (using saturating add to avoid overflow)
        if lower_bound <= last.1.saturating_add(1) {
            last.1 = last.1.max(end_bound);
        } else {
            merged.push((lower_bound, end_bound));
        }
    }

    let mut valid_id = 0;

    for (lower_bound, end_bound) in merged{
        valid_id += (end_bound - lower_bound) + 1;
    }

    valid_id
}

#[cfg(test)]
mod tests {
    use crate::d5::{d5p1, d5p2};

    #[test]
    fn d5p1_test(){
        let s = include_str!("d5_test.txt");
        let result = d5p1(s);
        assert_eq!(3, result);
    }

    #[test]
    fn d5p2_test(){
        let s = include_str!("d5_test.txt");
        let result = d5p2(s);
        assert_eq!(14, result);
    }
}