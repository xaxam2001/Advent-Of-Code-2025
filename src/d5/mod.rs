use std::time::Duration;

pub fn d5p1_v1(s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

    let mut total = 0;

    let sample:Vec<&str> = s.split("\r\n\r\n").collect();
    let sample_a:Vec<&str> = sample[0].split_whitespace().collect();
    let sample_b:Vec<&str> = sample[1].split_whitespace().collect();

    let mut ids_available:Vec<i32> = Vec::new();

    for sample in sample_b
    {
        let ids_str:Vec<&str> = sample.split(',').collect();

        for id in ids_str
        {
            let n = id.parse::<i32>().unwrap();

            ids_available.push(n);
        }
    }

    for i in 0..ids_available.len()
    {
        let mut is_fresh: bool = false;

        for sample in &sample_a
        {
            let range:Vec<&str> = sample.split('-').collect();
            let min = range[0].parse::<usize>().unwrap();
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
    std::thread::sleep(Duration::from_millis(40));

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
            let range:Vec<&str> = sample.split('-').collect();
            let min = range[0].parse::<usize>().unwrap();
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


pub fn d5p2_v1(s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

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


pub fn d5p2_v2(s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

    let mut total = 0;

    // 1. Découpage classique en deux blocs
    let sample: Vec<&str> = s.split("\n\n").collect();
    if sample.len() < 2 { return 0; }

    // Bloc A : Les plages de fraîcheur
    let sample_a: Vec<&str> = sample[0].split_whitespace().collect();
    // Bloc B : Les IDs d'ingrédients disponibles
    let sample_b: Vec<&str> = sample[1].split_whitespace().collect();

    // 2. On prépare nos plages pour ne pas les "parser" 1000 fois
    let mut parsed_ranges: Vec<(usize, usize)> = Vec::new();
    for s_range in sample_a {
        let parts: Vec<&str> = s_range.split('-').collect();
        if parts.len() == 2 {
            let min = parts[0].parse::<usize>().unwrap_or(0);
            let max = parts[1].parse::<usize>().unwrap_or(0);
            parsed_ranges.push((min, max));
        }
    }

    // 3. On traite les IDs disponibles un par un
    let mut processed_ids: Vec<usize> = Vec::new();

    for id_str in sample_b {
        let current_id = id_str.parse::<usize>().unwrap_or(0);

        // Éviter de compter deux fois le même ingrédient [cite: 28]
        if processed_ids.contains(&current_id) {
            continue;
        }
        processed_ids.push(current_id);

        // 4. On vérifie si cet ID est dans l'UNE des plages
        let mut found_fresh = false;
        for (min, max) in &parsed_ranges {
            if current_id >= *min && current_id <= *max {
                found_fresh = true;
                break; // On a trouvé, inutile de regarder les autres plages
            }
        }

        if found_fresh {
            total += 1;
        }
    }

    total
}


pub fn d5p2_vx(s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

    // 1. Découpage pour isoler le bloc des plages
    let sample: Vec<&str> = s.split("\n\n").collect();
    if sample.is_empty() { return 0; }

    let lines: Vec<&str> = sample[0].split_whitespace().collect();
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    // 2. Extraction des plages (min, max)
    for line in lines {
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(min), Ok(max)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                ranges.push((min, max));
            }
        }
    }

    // 3. Tri des plages par leur valeur minimale
    // C'est l'étape clé pour pouvoir fusionner les chevauchements
    ranges.sort_by_key(|r| r.0);

    // 4. Fusion des plages qui se chevauchent ou se touchent
    let mut merged: Vec<(usize, usize)> = Vec::new();

    for range in ranges {
        if let Some(last) = merged.last_mut() {
            // Si le début de la plage actuelle est inférieur ou égal à la fin de la précédente + 1
            if range.0 <= last.1 + 1 {
                // On étend la plage précédente si la nouvelle finit plus loin
                if range.1 > last.1 {
                    last.1 = range.1;
                }
            } else {
                // Pas de contact, on ajoute une nouvelle plage
                merged.push(range);
            }
        } else {
            // Première plage à insérer
            merged.push(range);
        }
    }

    // 5. Calcul du total des identifiants uniques
    let mut total = 0;
    for (min, max) in merged {
        total += (max - min) + 1;
    }

    total
}

pub fn d5p1(s: &str) -> usize {
    d5p1_v2(s)
}

pub fn d5p2(s: &str) -> usize {
    d5p2_v1(s)
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
