use std::time::Duration;

pub fn d3p1_v1(s: &str) -> usize {
    //std::thread::sleep(Duration::from_millis(40));

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

pub fn d3p1_v2(s: &str) -> usize {
    let mut somme_total = 0;

    for batterie in s.split_whitespace()
    {
        let bytes = batterie.as_bytes(); //accès mémoire

        let mut max_banque = 0;

        for i in 0..bytes.len()
        {
            let dizaine = (bytes[i] - b'0') as i32; // conversion

            for j in i+1..bytes.len()
            {
                let unite = (bytes[j] - b'0') as i32;

                let joltage = dizaine * 10 + unite;

                if joltage > max_banque { max_banque = joltage; }
            }
        }

        somme_total += max_banque;

    }
    somme_total as usize
}

pub fn d3p1_v3(s: &str) -> usize {

    let mut somme_total = 0;

    for batterie in s.split_whitespace()
    {
        let bytes = batterie.as_bytes();
        let mut max_banque = 0;
        let mut max_chiffre_vu = -1;

        for &b in bytes
        {
            let chiffre = (b - b'0') as i32;

            if max_chiffre_vu != -1
            {
                let combo = max_chiffre_vu * 10 + chiffre;
                if combo > max_banque
                {
                    max_banque = combo;
                }
            }

            if chiffre > max_chiffre_vu { max_chiffre_vu = chiffre; }
        }
        somme_total += max_banque;
    }
    somme_total as usize
}

pub fn d3p1_v4(s: &str) -> usize {

    let mut somme_total = 0;
    let bytes = s.as_bytes();

    let mut max_banque = 0;
    let mut max_chiffre_vu = -1;

    for &b in bytes
    {
        if b == b' ' || b == b'\n' || b == b'\r' {
            if max_banque > 0 {
                somme_total += max_banque as usize;
                // Reset pour la batterie suivante
                max_banque = 0;
                max_chiffre_vu = -1;
            }
            continue;
        }

        let chiffre = (b - b'0') as i32;

        if max_chiffre_vu != -1 {
            let combo = max_chiffre_vu * 10 + chiffre;
            if combo > max_banque {
                max_banque = combo;
            }
        }

        if chiffre > max_chiffre_vu {
            max_chiffre_vu = chiffre;
        }
    }
    somme_total += max_banque as usize;
    somme_total
}

pub fn d3p2_v1(_s: &str) -> usize {
    //std::thread::sleep(Duration::from_millis(40));

   0
}

pub fn d3p1(s: &str) -> usize {
    d3p1_v4(s)
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
