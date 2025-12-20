use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let contents = fs::read_to_string(input).unwrap();

    let mut total = 0i32;

    for line in contents.lines().filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // ---- pattern: keep read-order, right-packed ----
        let pat = &parts[0][1..parts[0].len() - 1];
        let pat_len = pat.len();
        let pattern = pat
            .chars()
            .rev() // last char -> bit 0
            .enumerate()
            .fold(0i16, |acc, (i, c)| {
                if c == '#' {
                    acc | (1 << i)
                } else {
                    acc
                }
            });

        // ---- masks: indices relative to pattern width ----
        let masks: Vec<i16> = parts
            .iter()
            .skip(1)
            .take_while(|p| !p.starts_with('{'))
            .map(|p| {
                p.trim_matches(&['(', ')'][..])
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .fold(0i16, |acc, idx| {
                        let idx: usize = idx.trim().parse().unwrap();
                        acc | (1 << (pat_len - 1 - idx))
                    })
            })
            .collect();

        // ---- BFS: minimum XOR ops from 0 -> pattern ----
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back((0i16, 0i32));
        seen.insert(0i16);

        let mut min_ops = None;

        while let Some((cur, steps)) = queue.pop_front() {
            if cur == pattern {
                min_ops = Some(steps);
                break;
            }

            for &m in &masks {
                let next = cur ^ m;
                if seen.insert(next) {
                    queue.push_back((next, steps + 1));
                }
            }
        }

        let ops = min_ops.expect("pattern unreachable");
        total += ops;
    }
    total
}


const INF: i64 = i64::MAX / 4;

fn solve_recursive(
    state: &Vec<i64>,
    buttons: &[Vec<usize>],
    memo: &mut HashMap<Vec<i64>, i64>,
) -> i64 {
    if let Some(&v) = memo.get(state) {
        return v;
    }

    // Base case: all joltages are 0 -> no presses needed
    if state.iter().all(|&x| x == 0) {
        memo.insert(state.clone(), 0);
        return 0;
    }

    let n = state.len();
    let b = buttons.len();
    let mut best = INF;

    // Temporary vector: how many times each column is incremented in phase 1
    let mut contrib = vec![0i64; n];

    // Enumerate all 2^B subsets of buttons as possible phase-1 sets
    let total_subsets: u64 = 1u64 << b;

    for subset in 0..total_subsets {
        contrib.fill(0);
        let mut presses_phase1 = 0i64;

        // Build contrib[j] = number of buttons in this subset affecting column j
        for (btn_idx, cols) in buttons.iter().enumerate() {
            if (subset >> btn_idx) & 1 == 1 {
                presses_phase1 += 1;
                for &col in cols {
                    contrib[col] += 1;
                }
            }
        }

        // Compute remaining joltages after phase 1, and check if all are even & non-negative
        let mut next_state = Vec::with_capacity(n);
        let mut ok = true;

        for j in 0..n {
            let t = state[j];
            let used = contrib[j];

            // Can't subtract more than we have
            if used > t {
                ok = false;
                break;
            }

            let rem = t - used;

            // After phase 1, remaining joltages must all be even
            if rem & 1 != 0 {
                ok = false;
                break;
            }

            next_state.push(rem / 2);
        }

        if !ok {
            continue;
        }

        // Phase 2: same sequence done twice on the halved problem
        let sub = solve_recursive(&next_state, buttons, memo);
        if sub == INF {
            continue;
        }

        let total_presses = presses_phase1 + 2 * sub;
        if total_presses < best {
            best = total_presses;
        }
    }

    memo.insert(state.clone(), best);
    best
}

pub fn part2(input: &str) -> i64 {
    let contents = fs::read_to_string(input).unwrap();
    let mut total = 0i64;

    for line in contents.lines().filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Pattern is only used to get the width here
        let pat = &parts[0][1..parts[0].len() - 1];
        let width = pat.len();

        // Parse buttons as in part1: masks over the pattern bits
        let raw_masks: Vec<i16> = parts
            .iter()
            .skip(1)
            .take_while(|p| !p.starts_with('{'))
            .map(|p| {
                p.trim_matches(&['(', ')'][..])
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .fold(0i16, |acc, idx| {
                        let idx: usize = idx.trim().parse().unwrap();
                        // same bit convention as part1: index is from left, bit0 is rightmost
                        acc | (1 << (width - 1 - idx))
                    })
            })
            .collect();

        // Convert each mask into a list of column indices it affects (0 = leftmost)
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for &m in &raw_masks {
            let mut cols = Vec::new();
            for col in 0..width {
                let bit = width - 1 - col;
                if (m & (1 << bit)) != 0 {
                    cols.push(col);
                }
            }
            buttons.push(cols);
        }

        // Parse target joltages inside { ... }
        let start = line.find('{').unwrap();
        let end = line.rfind('}').unwrap();
        let inner = &line[start + 1..end];

        let targets: Vec<i64> = inner
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();

        assert_eq!(
            targets.len(),
            width,
            "number of joltages must match pattern width"
        );

        let mut memo: HashMap<Vec<i64>, i64> = HashMap::new();
        let ans = solve_recursive(&targets, &buttons, &mut memo);

        if ans == INF {
            panic!("Unreachable joltage configuration on line: {}", line);
        }

        total += ans;
    }

    total
}