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

        if total <= 0 && total.abs() != value {count_zero += 1 ; }
        count_zero += (total / 100).unsigned_abs() as usize;

        total = total.rem_euclid(100);
    }

    count_zero
}

pub fn d1p1(s: &str) -> usize {
    d1p1_v1(s)
}

pub fn d1p2(s: &str) -> usize {
    d1p2_v1(s)
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