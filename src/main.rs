mod day_1;

fn main() {
    // Part 1
    let p1_out = match day_1::part1("./Inputs/D1I1.txt") {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    println!("Part 1 output: {p1_out}!")
    // Part 2(Placeholder)
}
