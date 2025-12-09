use std::fs;

pub fn part1(input: &str) -> i64 {
    let contents = fs::read_to_string(input).unwrap();
    let locs: Vec<(i64, i64)> = contents
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect();
    let mut current_max = 0;
    for i in 0..locs.len() {
        for j in i + 1..locs.len() {
            let dx = (locs[i].0 - locs[j].0).abs() + 1;
            let dy = (locs[i].1 - locs[j].1).abs() + 1;
            if (dx * dy) > current_max {
                current_max = dx * dy;
            }
        }
    }
    current_max
}

pub fn part2(input: &str) -> i64 {
    // Read red tile coordinates
    let red: Vec<(i64, i64)> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect();

    // Build green rectangles connecting consecutive red tiles
    let green: Vec<(i64, i64, i64, i64)> = red
        .windows(2)
        .map(|w| {
            let (a, b) = w[0];
            let (c, d) = w[1];
            (a.min(c), b.min(d), a.max(c), b.max(d))
        })
        .collect();

    let mut max_area: i64 = 0;

    // Iterate all pairs of red tiles
    for i in 0..red.len() {
        for j in i + 1..red.len() {
            let (x1, y1) = red[i];
            let (x2, y2) = red[j];

            // Compute bounding box
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);

            let area = (xmax - xmin + 1) * (ymax - ymin + 1);

            // Check all green rectangles: skip if any green rectangle is fully enclosed
            let mut valid = true;
            for &(gx1, gy1, gx2, gy2) in &green {
                if xmin < gx2 && ymin < gy2 && xmax > gx1 && ymax > gy1 {
                    valid = false;
                    break;
                }
            }

            if valid {
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

