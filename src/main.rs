mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn main() {
    // Part 1
    let p1_out = day_6::part1("./src/Inputs/D6.txt");
    println!("Part 1 output: {p1_out}!");
    // Part 2
    let p2_out = day_6::part2("./src/Inputs/test.txt");
    println!("Part 2 output: {p2_out}!");
}
