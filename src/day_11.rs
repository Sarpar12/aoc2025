use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn part1(input: &str) -> i32 {
    let contents = fs::read_to_string(input).unwrap(); 
    let mut mappings: HashMap<&str, Vec<&str>> = HashMap::new();
    let start: &str = "you";
    let end: &str = "out";
    for line in contents.lines().filter(|l| !l.is_empty()) {
        let mut part: Vec<&str> = line.split_whitespace().collect();
        if let Some(first) = part.first_mut() {
            *first = &first[..first.len() - 1];
        }
        let key = part[0];
        let values = part[1..].to_vec();
        mappings.insert(key, values);
    }
    let mut sum: i32 = 0;
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if node == end {
            sum += 1;
            continue;
        }
        if let Some(neighbors) = mappings.get(node) {
            for &next in neighbors {
                // optional: avoid immediately going back to `start`
                if next != start {
                    queue.push_back(next);
                }
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let contents = fs::read_to_string(input).unwrap(); 
    let mut mappings: HashMap<&str, Vec<&str>> = HashMap::new();
    let start: &str = "svr";
    let end: &str = "out";
    for line in contents.lines().filter(|l| !l.is_empty()) {
        let mut part: Vec<&str> = line.split_whitespace().collect();
        if let Some(first) = part.first_mut() {
            *first = &first[..first.len() - 1];
        }
        let key = part[0];
        let values = part[1..].to_vec();
        mappings.insert(key, values);
    }

    // memo key type: (node, seen_dac, seen_fft)
    #[derive(Hash, Eq, PartialEq, Clone, Copy)]
    struct State<'a> {
        node: &'a str,
        seen_dac: bool,
        seen_fft: bool,
    }
    let mut memo: HashMap<State, i64> = HashMap::new();
    fn dfs<'a>(
        st: State<'a>,
        end: &str,
        mappings: &HashMap<&'a str, Vec<&'a str>>,
        memo: &mut HashMap<State<'a>, i64>,
    ) -> i64 {
        if let Some(&v) = memo.get(&st) {
            return v;
        }

        if st.node == end {
            let res = if st.seen_dac && st.seen_fft { 1 } else { 0 };
            memo.insert(st, res);
            return res;
        }

        let mut total = 0i64;

        if let Some(neighbors) = mappings.get(st.node) {
            for &next in neighbors {
                let next_state = State {
                    node: next,
                    seen_dac: st.seen_dac || next == "dac",
                    seen_fft: st.seen_fft || next == "fft",
                };
                total += dfs(next_state, end, mappings, memo);
            }
        }
        memo.insert(st, total);
        total
    }

    let start_state = State {
        node: start,
        seen_dac: start == "dac",
        seen_fft: start == "fft",
    };

    dfs(start_state, end, &mappings, &mut memo) as i64
}
