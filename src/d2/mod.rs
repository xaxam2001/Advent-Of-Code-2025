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
            let id_str = &range.to_string(); // allocation new string heap
            let length = id_str.len();

            if !length % 2 == 0 {
                continue;
            }
            else if length % 2 == 0 && length > 0
            {
                let middle = length / 2; // middle index
                // découpage de chaîne
                let a = &id_str[..middle];
                let b = &id_str[middle..length];

                //comparaison de caractère
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
            // calcul du nombre de chiffres par division
            let mut temp = range;
            let mut nombre_chiffres = 0; // Plus de chaines de caracterse donc 0 Allocations
            while temp > 0 {
                temp /= 10;
                nombre_chiffres += 1;
            }

            if nombre_chiffres > 0 && nombre_chiffres % 2 == 0 {
                let moitie = nombre_chiffres / 2;

                // création du diviseur pour séparer le nombre
                let mut diviseur = 1;
                for _ in 0..moitie {
                    diviseur *= 10;
                }

                let gauche = range / diviseur;
                let droite = range % diviseur;

                // compare deux entier
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
        let mut nb_chiffres = 0;
        while temp > 0
        {
            temp /= 10; nb_chiffres += 1;
        }

        // seuil pour éviter de recompter les chiffres à chaque itération
        let mut seuil = 1;
        for _ in 0..nb_chiffres
        {
            seuil *= 10;
        } // evite de recompter le nombre de chiffre à chaque fois

        for range in a..=b {
            //si on chnage de puissiance : maj du nb_chiffres uniquement au changement d'unité
            if range >= seuil {
                nb_chiffres += 1;
                seuil *= 10;
            }

            if nb_chiffres % 2 == 0 {
                let mut diviseur = 1;
                for _ in 0..(nb_chiffres / 2) { diviseur *= 10; }

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

pub fn d2p1_v4(s: &str) -> usize {
    let mut result = 0;

    // travail sur des octets donc plus de vérif utf8
    for product_id in s.as_bytes().split(|&b| b == b',') //suite d'octet
    {
        let ids:Vec<&[u8]> = product_id.split(|&b| b == b'-').collect(); //check le '-' directement mais .collect donc allo heap

        // parsing manuel
        let mut a = 0;
        for &chiffre in ids[0]
        {
            a = a * 10 + (chiffre - b'0') as usize;
        }

        let mut b = 0;
        for &chiffre in ids[1]
        {
            b = b * 10 + (chiffre - b'0') as usize;
        } // passage en octet en usize = + rapide que parse

        let mut temp = a;
        let mut nb_chiffres = 0;
        while temp > 0
        {
            temp /= 10; nb_chiffres += 1;
        }

        let mut seuil = 1;
        for _ in 0..nb_chiffres
        {
            seuil *= 10;
        }

        for range in a..=b {
            //si on chnage de puissiance
            if range >= seuil {
                nb_chiffres += 1;
                seuil *= 10;
            }

            if nb_chiffres % 2 == 0 {
                let mut diviseur = 1;
                for _ in 0..(nb_chiffres / 2) { diviseur *= 10; }

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

pub fn d2p1_v5(s: &str) -> usize {
    let mut result = 0;

    for product_id in s.as_bytes().split(|&b| b == b',')
    {
        let ids:Vec<&[u8]> = product_id.split(|&b| b == b'-').collect();

        let mut a = 0;
        for &chiffre in ids[0]
        {
            a = a * 10 + (chiffre - b'0') as usize;
        }

        let mut b = 0;
        for &chiffre in ids[1]
        {
            b = b * 10 + (chiffre - b'0') as usize;
        }

        let mut temp = a;
        let mut nb_chiffres = 0;
        while temp > 0
        {
            temp /= 10; nb_chiffres += 1;
        }

        let mut seuil = 1;
        for _ in 0..nb_chiffres
        {
            seuil *= 10;
        }
        // pré-calcul du diviseur avant la boucle
        let mut diviseur = 1;
        if nb_chiffres % 2 == 0
        {
            diviseur = 10_usize.pow((nb_chiffres / 2) as u32); //new : utilisation de pow ou table interne
        }

        for range in a..=b
        {
            //si on chnage de puissiance
            if range >= seuil
            {
                nb_chiffres += 1;
                seuil *= 10;

                // recalcul du diviseur uniquement si nécessaire
                if nb_chiffres % 2 == 0 {
                    diviseur = 10_usize.pow((nb_chiffres / 2) as u32);
                } else {
                    diviseur = 0; // Impair : pas de diviseur utile
                }
            }

            if diviseur != 0 // diviseur est impair, ignore
            {
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

pub fn d2p2_v2(s: &str) -> usize {
    //std::thread::sleep(Duration::from_millis(40));
    let mut result = 0;

    for product_id in s.split(',') {
        let parts: Vec<&str> = product_id.split('-').collect();
        let a = parts[0].parse::<usize>().unwrap();
        let b = parts[1].parse::<usize>().unwrap();

        let mut temp = a;
        let mut nb_chiffres = 0;
        while temp > 0 {
            temp /= 10;
            nb_chiffres += 1;
        }

        let mut diviseurs_puissances = Vec::new();
        for i in 1..=(nb_chiffres / 2) {
            if nb_chiffres % i == 0 {
                diviseurs_puissances.push(10_usize.pow(i as u32));
            }
        }

        for range in a..=b
        {
            let id_str = &range.to_string();
            let chars:Vec<char> = range.to_string().chars().collect();
            let length = chars.len();

            let middle = length / 2;

            for range in a..=b {
                for &puissance in &diviseurs_puissances {
                    let sequence = range % puissance;

                    let mut test_repetition = 0;
                    let mut multiplicateur = 1;

                    let mut est_valide = true;
                    let mut n = range;
                    let motif = range % puissance;

                    while n > 0 {
                        if n % puissance != motif {
                            est_valide = false;
                            break;
                        }
                        n /= puissance;
                    }

                    if est_valide {
                        result += range;
                        break; // On a trouvé, on passe au nombre suivant
                    }
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
    d2p2_v2(s)
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
