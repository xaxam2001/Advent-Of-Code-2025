use std::time::Duration;

pub fn d2p1_v1(s: &str) -> usize {
    //std::thread::sleep(Duration::from_millis(40));
    let mut result = 0;
    let product_ids:Vec<&str> = s.split(',').collect();

    for product_id in &product_ids
    {
        let ids:Vec<&str> = product_id.split('-').collect();
        let a = ids[0].parse::<usize>().unwrap();
        let b = ids[1].parse::<usize>().unwrap();

        for range in a..=b
        {
            let id_str = &range.to_string();
            let length = id_str.len();

            if !length % 2 == 0 {
                continue;
            }
            else if length % 2 == 0 && length > 0
            {
                let middle = length / 2;
                let a = &id_str[..middle];
                let b = &id_str[middle..length];

                if a == b {
                    result += range;
                }
            }
        }
    }

    result
}

pub fn d2p1_v2(s: &str) -> usize {
    let mut result = 0;

    for product_id in s.split(',')
    {
        let ids:Vec<&str> = product_id.split('-').collect();
        let a = ids[0].parse::<usize>().unwrap();
        let b = ids[1].parse::<usize>().unwrap();

        for range in a..=b
        {
            let mut temp = range;
            let mut nombre_chiffres = 0;
            while temp > 0 {
                temp /= 10;
                nombre_chiffres += 1;
            }

            if nombre_chiffres > 0 && nombre_chiffres % 2 == 0 {
                let moitie = nombre_chiffres / 2;

                let mut diviseur = 1;
                for _ in 0..moitie {
                    diviseur *= 10;
                }

                let gauche = range / diviseur;
                let droite = range % diviseur;

                if gauche == droite {
                    result += range;
                }
            }
        }
    }

    result
}

pub fn d2p1_v3(s: &str) -> usize {
    let mut result = 0;

    for product_id in s.split(',')
    {
        let ids:Vec<&str> = product_id.split('-').collect();
        let a = ids[0].parse::<usize>().unwrap();
        let b = ids[1].parse::<usize>().unwrap();

        let mut temp = a;
        let mut nombre_chiffres = 0;
        while temp > 0 {
            temp /= 10;
            nombre_chiffres += 1;
        }

        let mut diviseur = 1;
        if nombre_chiffres % 2 == 0 {
            for _ in 0..(nombre_chiffres / 2) {
                diviseur *= 10;
            }
        }

        for range in a..=b
        {
            if nombre_chiffres % 2 == 0 {
                let gauche = range / diviseur;
                let droite = range % diviseur;

                if gauche == droite {
                    result += range;
                }
            }
        }
    }

    result
}


pub fn d2p2_v1(s: &str) -> usize {
    //std::thread::sleep(Duration::from_millis(40));
    let mut result = 0;
    let product_ids:Vec<&str> = s.split(',').collect();

    for product_id in &product_ids
    {
        let ids:Vec<&str> = product_id.split('-').collect();
        let a = ids[0].parse::<usize>().unwrap();
        let b = ids[1].parse::<usize>().unwrap();

        for range in a..=b
        {
            let id_str = &range.to_string();
            let chars:Vec<char> = range.to_string().chars().collect();
            let length = chars.len();

            let middle = length / 2;

            for i in 1..=middle
            {
                if length % i != 0 && length > 0 { continue; }

                let seq = &id_str[0..i];

                let rep = &seq.repeat(length / i);

                if id_str == rep
                {
                    result += range;
                    break;
                }
            }
        }
    }

    result
}

pub fn d2p1(s: &str) -> usize {
    d2p1_v1(s)
}

pub fn d2p2(s: &str) -> usize {
    d2p2_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d2::{d2p1, d2p2};

    #[test]
    fn d2p1_test()
    {
        let s = include_str!("d2p1_test.txt");
        let result:usize = d2p1(s);
        println!("P1 : {}", result);
    }

    #[test]
    fn d2p2_test()
    {
        let s = include_str!("d2p1_test.txt");
        let result:usize = d2p2(s);
        println!("P2 : {}", result);
    }
}
