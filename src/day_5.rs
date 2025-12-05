use std::fs;

pub fn part1(input: &str) -> Result<i64, std::io::Error> {
    let input = fs::read_to_string(input)?;
    let normalized = input.replace("\r\n", "\n");
    let mut sections = normalized.split("\n\n");

    let ranges_section = sections.next().unwrap_or("");
    let numbers_section = sections.next().unwrap_or("");

    // Parse ranges as (start, end) tuples
    let mut ranges: Vec<(i64, i64)> = ranges_section
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<i64> = line
                .split('-')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if parts.len() >= 2 {
                Some((parts[0], parts[1]))
            } else {
                None
            }
        })
        .collect();

    ranges.sort_by_key(|r| r.0);

    // Merge overlapping ranges
    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    // Count numbers that fall within any range using binary search
    let count = numbers_section
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.trim().parse::<i64>().ok())
        .filter(|&num| is_in_ranges(&merged, num))
        .count() as i64;

    Ok(count)
}

/// Binary search to check if a number is within any of the sorted, merged ranges
fn is_in_ranges(ranges: &[(i64, i64)], num: i64) -> bool {
    // Find the rightmost range whose start <= num
    let idx = ranges.partition_point(|r| r.0 <= num);
    if idx == 0 {
        return false;
    }
    // Check if num falls within that range
    let (start, end) = ranges[idx - 1];
    num >= start && num <= end
}

pub fn part2(path: &str) -> Result<i64, std::io::Error> {
    let input = fs::read_to_string(path)?;
    let normalized = input.replace("\r\n", "\n");
    let mut sections = normalized.split("\n\n");
    let ranges_section = sections.next().unwrap_or("");

    // Parse ranges as (start, end) tuples
    let mut ranges: Vec<(i64, i64)> = ranges_section
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<i64> = line
                .split('-')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if parts.len() >= 2 {
                Some((parts[0], parts[1]))
            } else {
                None
            }
        })
        .collect();

    ranges.sort_by_key(|r| r.0);

    // Merge overlapping ranges
    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    // Count total numbers covered by all merged ranges
    let count: i64 = merged.iter().map(|(start, end)| end - start + 1).sum();

    Ok(count)
}
