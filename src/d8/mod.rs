pub fn d8p1(s: &str, n_shortest_connection: usize) -> usize {
    d8p1_v3(s, n_shortest_connection)
}

// Helper function to find the root of a network
fn find(mut i: usize, parent: &mut [usize]) -> usize {
    while i != parent[i] {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

#[allow(unused)]
pub fn d8p1_v1(s: &str, n_shortest_connection: usize) -> usize {
    let n = s.lines().count();

    // This vec contains (distance_squared, index1, index2)
    let mut distances: Vec<(f64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);

    for (i, line) in s.lines().enumerate() {
        let mut parts_1 = line.split(',');
        let parts_1: [i32; 3] = [
            parts_1.next().unwrap().parse().unwrap(),
            parts_1.next().unwrap().parse().unwrap(),
            parts_1.next().unwrap().parse().unwrap(),
        ];

        for j in (i + 1)..n {
            let line_j = s.lines().nth(j).unwrap();

            let mut parts_2 = line_j.split(',');
            let parts_2: [i32; 3] = [
                parts_2.next().unwrap().parse().unwrap(),
                parts_2.next().unwrap().parse().unwrap(),
                parts_2.next().unwrap().parse().unwrap(),
            ];

            // Compute the distance
            let dx = f64::from(parts_1[0] - parts_2[0]);
            let dy = f64::from(parts_1[1] - parts_2[1]);
            let dz = f64::from(parts_1[2] - parts_2[2]);
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();

            // Add into a vec associating indices and distance
            distances.push((dist, i, j));
        }
    }

    // sort the distance from smallest to biggest
    // sort_unstable here because we don't care about the ties
    // for floats had to use total_cmp which move NaN at the end
    distances.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));

    // network representation
    // 'parent' tracks who connect to who and 'size' tracks sizes of each networks
    let mut parent: Vec<usize> = (0..n).collect(); // at first each one is it's own parent
    let mut size: Vec<usize> = vec![1; n];


    // Process only the top 1000 shortest connections
    for &(_, i, j) in distances.iter().take(n_shortest_connection) {
        let root_i = find(i, &mut parent);
        let root_j = find(j, &mut parent);

        // If they aren't already in the same network we merge then
        if root_i != root_j {
            parent[root_j] = root_i;       // connect j's network to i's network
            size[root_i] += size[root_j];  // add their sizes together
            size[root_j] = 0;              // old root is zero now so we don't double count it
        }
    }

    // Multiply the size of the 3 largest networks together
    // Sort sizes in descending order
    size.sort_unstable_by(|a, b| b.cmp(a)); // found in the doc of sort_unstable

    // Take the top 3 and multiply them
    size.iter().take(3).product()
}

#[allow(unused)]
pub fn d8p1_v2(s: &str, n_shortest_connection: usize) -> usize {

    // create the points ahead to avoid parsing twice the same values
    let points: Vec<[i32; 3]> = s
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            [
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();

    let n = points.len(); // get the number of points

    // This vec contains (distance_squared, index1, index2)
    let mut distances: Vec<(f64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);

    // loop over all the lines (the O(n2) part)
    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = points[i];
            let p2 = points[j];

            // Compute the distances
            let dx = f64::from(p1[0] - p2[0]);
            let dy = f64::from(p1[1] - p2[1]);
            let dz = f64::from(p1[2] - p2[2]);
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();

            // Add into a vec associating indices and distance
            distances.push((dist, i, j));
        }
    }

    // sort the distance from smallest to biggest
    // sort_unstable here because we don't care about the ties
    // for floats had to use total_cmp which move NaN at the end
    distances.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));


    // network representation
    // 'parent' tracks who connect to who and 'size' tracks sizes of each networks
    let mut parent: Vec<usize> = (0..n).collect(); // at first each one is it's own parent
    let mut size: Vec<usize> = vec![1; n];


    // Process only the top 1000 shortest connections
    for &(_, i, j) in distances.iter().take(n_shortest_connection) {
        let root_i = find(i, &mut parent);
        let root_j = find(j, &mut parent);

        // If they aren't already in the same network we merge then
        if root_i != root_j {
            parent[root_j] = root_i;       // connect j's network to i's network
            size[root_i] += size[root_j];  // add their sizes together
            size[root_j] = 0;              // old root is zero now so we don't double count it
        }
    }

    // Multiply the size of the 3 largest networks together
    // Sort sizes in descending order
    size.sort_unstable_by(|a, b| b.cmp(a)); // found in the doc of sort_unstable

    // Take the top 3 and multiply them
    size.iter().take(3).product()
}

#[allow(unused)]
pub fn d8p1_v3(s: &str, n_shortest_connection: usize) -> usize {

    // create the points ahead to avoid parsing twice the same values
    let points: Vec<[i64; 3]> = s
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            [
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();

    let n = points.len(); // get the number of points

    // This vec contains (distance_squared, index1, index2)
    let mut distances: Vec<(i64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);

    // loop over all the lines (the O(n2) part)
    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = points[i];
            let p2 = points[j];

            // Compute the distances
            let dx = p1[0] - p2[0];
            let dy = p1[1] - p2[1];
            let dz = p1[2] - p2[2];
            let dist = dx * dx + dy * dy + dz * dz;

            // Add into a vec associating indices and distance
            distances.push((dist, i, j));
        }
    }

    // can use just sort_unstable here because working with int so no NaN
    distances.sort_unstable();

    // network representation
    // 'parent' tracks who connect to who and 'size' tracks sizes of each networks
    let mut parent: Vec<usize> = (0..n).collect(); // at first each one is it's own parent
    let mut size: Vec<usize> = vec![1; n];


    // Process only the top 1000 shortest connections
    for &(_, i, j) in distances.iter().take(n_shortest_connection) {
        let root_i = find(i, &mut parent);
        let root_j = find(j, &mut parent);

        // If they aren't already in the same network we merge then
        if root_i != root_j {
            parent[root_j] = root_i;       // connect j's network to i's network
            size[root_i] += size[root_j];  // add their sizes together
            size[root_j] = 0;              // old root is zero now so we don't double count it
        }
    }

    // Multiply the size of the 3 largest networks together
    // Sort sizes in descending order
    size.sort_unstable_by(|a, b| b.cmp(a)); // found in the doc of sort_unstable

    // Take the top 3 and multiply them
    size.iter().take(3).product()
}


pub fn d8p2(s: &str) -> usize {
    d8p2_v1(s)
}

#[allow(unused)]
pub fn d8p2_v1(s: &str) -> usize {
    let n = s.lines().count();

    // This vec has (distance_squared, index1, index2, x1, x2)
    let mut distances: Vec<(f64, usize, usize, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);

    for (i, line) in s.lines().enumerate() {
        let mut parts_1 = line.split(',');
        let parts_1: [i32; 3] = [
            parts_1.next().unwrap().parse().unwrap(),
            parts_1.next().unwrap().parse().unwrap(),
            parts_1.next().unwrap().parse().unwrap(),
        ];

        for j in (i + 1)..n {
            let line_j = s.lines().nth(j).unwrap();

            let mut parts_2 = line_j.split(',');
            let parts_2: [i32; 3] = [
                parts_2.next().unwrap().parse().unwrap(),
                parts_2.next().unwrap().parse().unwrap(),
                parts_2.next().unwrap().parse().unwrap(),
            ];

            // Compute the distance
            let dx = f64::from(parts_1[0] - parts_2[0]);
            let dy = f64::from(parts_1[1] - parts_2[1]);
            let dz = f64::from(parts_1[2] - parts_2[2]);
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();

            // Add into a vec associating indices and distance
            distances.push((dist, i, j, parts_1[0] as usize, parts_2[0] as usize ));
        }
    }

    // sort the distance from smallest to biggest
    // sort_unstable here because we don't care about the ties
    // for floats had to use total_cmp which move NaN at the end
    distances.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));

    // network representation
    // 'parent' tracks who connect to who and 'size' tracks sizes of each networks
    let mut parent: Vec<usize> = (0..n).collect(); // at first each one is it's own parent
    let mut size: Vec<usize> = vec![1; n];

    // Process the connections shortest by shortest until they are all connected to one network
    for &(_, i, j, x1, x2) in distances.iter() {
        let root_i = find(i, &mut parent);
        let root_j = find(j, &mut parent);

        // If they aren't already in the same network merge them
        if root_i != root_j {
            parent[root_j] = root_i;       // Connect j's network to i's network
            size[root_i] += size[root_j];  // Add their sizes together
            size[root_j] = 0;              // Zero out the old root so we don't double count it
        }

        if (size.contains(&n)){  // OPTIM: use hashset
            return x1*x2;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::d8::{d8p1, d8p2};

    #[test]
    fn d8p1_test(){
        let s = include_str!("d8_test.txt");
        let result = d8p1(s, 10);
        assert_eq!(40, result);
    }

    #[test]
    fn d8p2_test(){
        let s = include_str!("d8_test.txt");
        let result = d8p2(s);
        assert_eq!(25272, result);
    }
}