use std::fs;

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

pub fn part2(input: &str) -> i32 {
    0
}
