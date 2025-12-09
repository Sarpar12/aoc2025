use std::fs;

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let contents = fs::read_to_string(input).unwrap();
    let normalized = contents.replace("\r\n", "\n");
    let mut sections = normalized.split("\n");

    let num1_str = sections.next().unwrap_or("");
    let num2_str = sections.next().unwrap_or("");
    let num3_str = sections.next().unwrap_or("");
    let num4_str = sections.next().unwrap_or("");
    let ops_str = sections.next().unwrap_or("");

    let num1 = num1_str.split_whitespace();
    let num2 = num2_str.split_whitespace();
    let num3 = num3_str.split_whitespace();
    let num4 = num4_str.split_whitespace();
    let ops = ops_str.split_whitespace();

    let zipped = num1
        .zip(num2.zip(num3.zip(num4.zip(ops))))
        .map(|(a, (b, (c, (d, op))))| (a, b, c, d, op));

    let mut sum: i64 = 0;

    for (n1, n2, n3, n4, op) in zipped {
        let a: i64 = n1.parse().unwrap();
        let b: i64 = n2.parse().unwrap();
        let c: i64 = n3.parse().unwrap();
        let d: i64 = n4.parse().unwrap();

        let result = match op {
            "+" => a + b + c + d,
            "-" => a - b - c - d,
            "*" => a * b * c * d,
            "/" => a / b / c / d,
            _ => 0,
        };

        sum += result;
    }

    sum
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let contents = fs::read_to_string(input).unwrap();
    let normalized = contents.replace("\r\n", "\n");
    let lines: Vec<&str> = normalized.lines().collect();

    let num_rows: Vec<Vec<char>> = lines
        .iter()
        .take(lines.len().saturating_sub(1))
        .map(|line| line.chars().collect())
        .collect();

    let ops_chars: Vec<char> = lines.last().unwrap_or(&"").chars().collect();

    let max_len = num_rows
        .iter()
        .map(|row| row.len())
        .max()
        .unwrap_or(0)
        .max(ops_chars.len());

    let mut sum: i64 = 0;
    let mut stack: Vec<i64> = Vec::new();
    let mut ops_str: String = ops_chars.iter().collect();

    // Read from right to left
    for i in (-1..=max_len as isize).rev() {
        let mut num_buf: i64 = 0;
        let mut has_digit = false;

        for row in &num_rows {
            if i >= 0 {
                if let Some(&c) = row.get(i as usize) {
                    if c.is_ascii_digit() {
                        num_buf = num_buf * 10 + (c as i64 - '0' as i64);
                        has_digit = true;
                    }
                }
            }
        }

        if has_digit {
            stack.push(num_buf);
        }

        if (!has_digit) && !stack.is_empty() {
            let current_op = ops_str.trim().chars().last().unwrap_or(' ');
            ops_str = ops_str.trim().trim_end_matches(current_op).to_string();

            let mut acc = stack.pop().unwrap();
            while let Some(val) = stack.pop() {
                acc = match current_op {
                    '+' => acc + val,
                    '-' => acc - val,
                    '*' => acc * val,
                    '/' => {
                        if val != 0 {
                            acc / val
                        } else {
                            acc
                        }
                    }
                    _ => acc,
                };
            }

            sum += acc;
        }
    }

    sum
}
