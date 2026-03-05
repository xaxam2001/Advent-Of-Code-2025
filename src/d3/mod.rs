use std::time::Duration;

pub fn d3p1_v1(s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

    let mut somme_total = 0;

    for batterie in s.split_whitespace()
    {
        let mut numbers:Vec<i32> = Vec::new();

        let mut max_banque = 0;

        for c in batterie.chars()
        {
            numbers.push(c.to_digit(10).unwrap() as i32);
        }

        for i in 0..numbers.len()-1
        {
            let valeur_dizaine = numbers[i];

            for j in i+1..numbers.len()
            {
                let valeur_unite = numbers[j];

                let joltage_combine:i32 = valeur_dizaine * 10 + valeur_unite;

                if joltage_combine > max_banque
                {
                    max_banque = joltage_combine;
                }
            }
        }

        somme_total += max_banque;
    }

    somme_total.try_into().unwrap()
}

pub fn d3p2_v1(_s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

   0
}

pub fn d3p1(s: &str) -> usize {
    d3p1_v1(s)
}

pub fn d3p2(s: &str) -> usize {
    d3p2_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d3::{d3p1, d3p2};

    #[test]
    fn d3p1_test()
    {
        let s = include_str!("d3p1_test.txt");
        let result:usize = d3p1(s);
        println!("P1 : {}", result);
    }

    #[test]
    fn d3p2_test()
    {
        let s = include_str!("d3p1_test.txt");
        let result:usize = d3p2(s);
        println!("P2 : {}", result);
    }
}
