mod day_1;
mod day_2;

fn main() {
    // Part 1
    let p1_out = match day_2::part1("./src/Inputs/D2.txt") {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    println!("Part 1 output: {p1_out}!");
    // Part 2
    let p2_out = match day_1::part2("./src/Inputs/D1I1.txt") {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    };
    println!("Part 2 output: {p2_out}!");
}
