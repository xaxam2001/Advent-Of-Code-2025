use std::time::Duration;

pub fn d5p1_v1(s: &str) -> usize {
    let mut total = 0;

    let sample:Vec<&str> = s.split("\r\n\r\n").collect();
    let sample_a:Vec<&str> = sample[0].split_whitespace().collect();
    let sample_b:Vec<&str> = sample[1].split_whitespace().collect();

    let mut ids_available:Vec<u64> = Vec::new();

    for sample in sample_b
    {
        let ids_str:Vec<&str> = sample.split(',').collect();

        for id in ids_str
        {
            let n = id.parse::<u64>().unwrap();

            ids_available.push(n);
        }
    }

    for i in 0..ids_available.len()
    {
        let mut is_fresh: bool = false;

        for sample in &sample_a
        {
            let range:Vec<&str> = sample.split('-').collect(); //allocation + découpage
            let min = range[0].parse::<usize>().unwrap();  //conversion en texte
            let max = range[1].parse::<usize>().unwrap();

            if ids_available[i] >= min.try_into().unwrap() && ids_available[i] <= max.try_into().unwrap()
            {
                is_fresh = true;
            }
        }

        if is_fresh
        {
            total += 1;
        }
    }

    total as usize
}

pub fn d5p1_v2(s: &str) -> usize {
    let mut total = 0;

    let sections: Vec<&str> = s.split("\r\n\r\n").collect();
    let sample_a = sections[0];
    let sample_b = sections[1];

    let mut ids_available = Vec::new();

    for sample in sample_a.split_whitespace()
    {
        let bornes: Vec<&str> = sample.split('-').collect();
        let min = bornes[0].parse::<u64>().unwrap();
        let max = bornes[1].parse::<u64>().unwrap();

        ids_available.push((min, max));
    }

    for sample in sample_b.split_whitespace()
    {
        for id_str in sample.split(',')
        {
            let id = id_str.parse::<u64>().unwrap();

            for (min, max) in &ids_available //ids déja prêt => comparaison binaire
            {
                if id >= *min && id <= *max
                {
                    total += 1;
                    break; // si un produit est valide, on sort (- de test à faire !)
                }
            }
        }
    }

    total
}

pub fn d5p2_v1(s: &str) -> usize {
    //std::thread::sleep(Duration::from_millis(40));

    let mut total = 0;

    let sample:Vec<&str> = s.split("\r\n\r\n").collect();
    let sample_a:Vec<&str> = sample[0].split_whitespace().collect();

    let mut ids_checked: Vec<usize> = Vec::new();

    for sample in &sample_a
    {
        let range:Vec<&str> = sample.split('-').collect();
        let min = range[0].parse::<usize>().unwrap();
        let max = range[1].parse::<usize>().unwrap();

        for j in min..=max
        {
            if ids_checked.contains(&j){
                continue;
            }
            ids_checked.push(j);
            total += 1;
        }
    }

    total
}

pub fn d5p1(s: &str) -> usize {
    d5p1_v2(s)
}

pub fn d5p2(s: &str) -> usize {
    return 0 //d5p2_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d5::{d5p1, d5p2};

    #[test]
    fn d5p1_test()
    {
        let s = include_str!("d5p1_test.txt");
        let result:usize = d5p1(s);
        println!("P1 : {}", result);
    }

    #[test]
    fn d5p2_test()
    {
        let s = include_str!("d5p1_test.txt");
        let result:usize = d5p2(s);
        println!("P2 : {}", result);
    }
}
