use std::time::Duration;

pub fn d2p1_v1(s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

    let ranges:Vec<&str> = s.split(',').collect();
    let result:usize = 0;

    for range in ranges{
        if !range.chars().all(|c| c.is_ascii_digit()) { continue }

        let longueur = range.len();
        let mut is_repeated:bool = false;

        if longueur % 2 && longueur > 0
        {
            let middle = longueur / 2;
            let partie_A = &range[0..middle];
            let partie_B = &range[middle.. longueur-1];

            if partie_A == partie_A { is_repeated = true; }
        }
    }

    s.len()
}

pub fn d2p1(s: &str) -> usize {
    d2p1_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d2::d2p1;

    #[test]
    fn d2p1_test()
    {
        let s = include_str!("d2p1_test.txt");
        let result:usize = d2p1(s);
        println!("P1 : {}", result);
    }
}
