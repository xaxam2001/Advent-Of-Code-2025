use std::time::Duration;

pub fn d4p1_v1(s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

    let mut total = 0;

    let directions = [
        (-1,-1), (-1,0), (-1,1),
        (0,-1),          (0,1),
        (1,-1),  (1,0),  (1,1),
    ];

    let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cell = grid[y][x];

            if cell == '@'
            {
                let mut voisins = 0;

                for (dx, dy) in directions {

                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    // vérification des limites
                    if nx >= 0 && ny >= 0 &&
                        (nx as usize) < grid[y].len() &&
                        (ny as usize) < grid.len() {

                        if grid[ny as usize][nx as usize] == '@' {
                            voisins += 1;
                        }
                    }
                }

                if voisins < 4 {
                    total += 1;
                }
            }
            else { continue }
        }
    }

    total
}

pub fn d4p2_v1(s: &str) -> usize {
    std::thread::sleep(Duration::from_millis(40));

    let mut deleted = 0;
    let mut modified = true;

    let directions = [
        (-1,-1), (-1,0), (-1,1),
        (0,-1),          (0,1),
        (1,-1),  (1,0),  (1,1),
    ];

    let mut grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

    let mut deleted_pos: Vec<(usize, usize)> = Vec::new();

    while modified {
        modified = false;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let cell = grid[y][x];

                if cell == '@'
                {
                    let mut voisins = 0;

                    for (dx, dy) in directions {

                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        // vérification des limites
                        if nx >= 0 && ny >= 0 &&
                            (nx as usize) < grid[y].len() &&
                            (ny as usize) < grid.len() {

                            if grid[ny as usize][nx as usize] == '@' {
                                voisins += 1;
                            }
                        }
                    }

                    if voisins < 4 {
                        deleted_pos.push((x, y));
                        modified = true;
                    }
                }
                else { continue }
            }
        }

        for (px, py) in &deleted_pos {
            // Vous accédez maintenant directement à px et py
            if grid[*py][*px] == '@' {
                grid[*py][*px] = '.';
                deleted += 1;
            }
        }
    }

    deleted
}

pub fn d4p1(s: &str) -> usize {
    d4p1_v1(s)
}

pub fn d4p2(s: &str) -> usize {
    d4p2_v1(s)
}

#[cfg(test)]
mod tests {
    use crate::d4::{d4p1, d4p2};

    #[test]
    fn d4p1_test()
    {
        let s = include_str!("d4p1_test.txt");
        let result:usize = d4p1(s);
        println!("P1 : {}", result);
    }

    #[test]
    fn d4p2_test()
    {
        let s = include_str!("d4p1_test.txt");
        let result:usize = d4p2(s);
        println!("P2 : {}", result);
    }
}
