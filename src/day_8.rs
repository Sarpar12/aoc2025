use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct BoxPair {
    box_1: (i64, i64, i64),
    box_2: (i64, i64, i64),
    distance: f64,
}

impl BoxPair {
    pub fn new(box_1: &(i64, i64, i64), box_2: &(i64, i64, i64)) -> Self {
        let distance = ((
            (box_1.0 - box_2.0).pow(2) + 
            (box_1.1 - box_2.1).pow(2) + 
            (box_1.2 - box_2.2).pow(2)) as f64).sqrt();
        Self {box_1: *box_1, box_2: *box_2, distance}
    }
}

// Union-Find helper
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
    let mut pairs: Vec<BoxPair> = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let pair = BoxPair::new(&boxes[i], &boxes[j]);
            pairs.push(pair);
        }
    }
    pairs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    let selected_pairs = &pairs[..max_pairs.min(pairs.len())];
    
    let mut parent: HashMap<&(i64,i64,i64), &(i64,i64,i64)> = HashMap::new();
    for b in &boxes {
        parent.insert(b, b);
    }
    for pair in selected_pairs {
        let root1 = find(&mut parent, &pair.box_1);
        let root2 = find(&mut parent, &pair.box_2);
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
    let product: usize = top3.iter().product();
    product
}

pub fn part2(input: &str) -> usize {
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
    let mut pairs: Vec<BoxPair> = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let pair = BoxPair::new(&boxes[i], &boxes[j]);
            pairs.push(pair);
        }
    }
    pairs.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    let mut last_merge: Option<(&BoxPair)> = None;
    
    let mut parent: HashMap<&(i64,i64,i64), &(i64,i64,i64)> = HashMap::new();
    for b in &boxes {
        parent.insert(b, b);
    }
    for pair in &pairs {
        let root1 = find(&mut parent, &pair.box_1);
        let root2 = find(&mut parent, &pair.box_2);
        if root1 != root2 {
            parent.insert(root1, root2);
            last_merge = Some(pair);
        }
    }
    let last = last_merge.unwrap();
    (last.box_1.0 * last.box_2.0) as usize
    // let mut circuits: HashMap<&(i64,i64,i64), Vec<&(i64,i64,i64)>> = HashMap::new();
    // for b in &boxes {
    //     let root = find(&mut parent, b);
    //     circuits.entry(root).or_default().push(b);
    // }
    // let mut lengths: Vec<usize> = circuits.values().map(|c| c.len()).collect();
    // lengths.sort_by(|a, b| b.cmp(a));
    // let top3 = &lengths[..3.min(lengths.len())];
    // let product: usize = top3.iter().product();
    // product
}