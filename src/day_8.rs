use std::fs;
use std::collections::HashMap;

fn find<'a>(
    parent: &mut HashMap<&'a (i64, i64, i64), &'a (i64, i64, i64)>,
    x: &'a (i64, i64, i64)
) -> &'a (i64, i64, i64) {
    if parent[x] != x {
        let root = find(parent, parent[x]);
        parent.insert(x, root); // path compression
    }
    parent[x]
}

pub fn part1(input: &str, max_pairs: usize) -> usize {
    let content = fs::read_to_string(input).expect("failed to read input file");
    let boxes: Vec<(i64, i64, i64)> = content
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
            (parts[0], parts[1], parts[2])
        })
        .collect();
    let mut pairs: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let dx = boxes[i].0 - boxes[j].0;
            let dy = boxes[i].1 - boxes[j].1;
            let dz = boxes[i].2 - boxes[j].2;
            let dist = ((dx*dx + dy*dy + dz*dz) as f64).sqrt();
            pairs.push((i, j, dist));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let selected_pairs = &pairs[..max_pairs.min(pairs.len())];
    let mut parent: HashMap<&(i64,i64,i64), &(i64,i64,i64)> = HashMap::new();
    for b in &boxes {
        parent.insert(b, b);
    }
    for &(i, j, _) in selected_pairs {
        let root1 = find(&mut parent, &boxes[i]);
        let root2 = find(&mut parent, &boxes[j]);
        if root1 != root2 {
            parent.insert(root1, root2);
        }
    }
    let mut circuits: HashMap<&(i64,i64,i64), Vec<&(i64,i64,i64)>> = HashMap::new();
    for b in &boxes {
        let root = find(&mut parent, b);
        circuits.entry(root).or_default().push(b);
    }
    let mut lengths: Vec<usize> = circuits.values().map(|c| c.len()).collect();
    lengths.sort_by(|a, b| b.cmp(a));
    let top3 = &lengths[..3.min(lengths.len())];
    top3.iter().product()
}


pub fn part2(input: &str) -> usize {
    let content = std::fs::read_to_string(input).expect("failed to read input file");

    // Parse boxes
    let boxes: Vec<(i64, i64, i64)> = content
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            (parts[0], parts[1], parts[2])
        })
        .collect();

    let n = boxes.len();

    // Generate all pairs with distances
    let mut pairs: Vec<(usize, usize, f64)> = Vec::with_capacity(n*(n-1)/2);
    for i in 0..n {
        for j in i+1..n {
            let dx = boxes[i].0 - boxes[j].0;
            let dy = boxes[i].1 - boxes[j].1;
            let dz = boxes[i].2 - boxes[j].2;
            let dist = ((dx*dx + dy*dy + dz*dz) as f64).sqrt();
            pairs.push((i, j, dist));
        }
    }

    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut parent: Vec<usize> = (0..n).collect();
    let mut component_count = n;

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]); // path compression
        }
        parent[x]
    }

    let mut last_pair = (0, 0, 0.0);

    for &(i, j, dist) in &pairs {
        let root_i = find(&mut parent, i);
        let root_j = find(&mut parent, j);

        if root_i != root_j {
            parent[root_i] = root_j;
            component_count -= 1;
            last_pair = (i, j, dist);

            if component_count == 1 {
                break;
            }
        }
    }

    // Return product of x-coordinates of the last merged pair
    (boxes[last_pair.0].0 * boxes[last_pair.1].0) as usize
}