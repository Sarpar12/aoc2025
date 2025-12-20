use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

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
            .fold(
                0i16,
                |acc, (i, c)| {
                    if c == '#' {
                        acc | (1 << i)
                    } else {
                        acc
                    }
                },
            );

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

    // Base case: if every column is 0, we need no further presses.
    // This corresponds to f(0,0,...,0) = 0 in the writeup.
    if state.iter().all(|&x| x == 0) {
        memo.insert(state.clone(), 0);
        return 0;
    }

    let n = state.len();
    let b = buttons.len();
    let mut best = INF;

    // Recursive strategy (high level):
    // - Try all subsets of buttons to press once in a first phase (like the parity-only solution in Part 1).
    // - Subtract a subset's effects from `state`. If any column would go negative, skip it.
    // - If all remaining values are even, divide them by two and recurse on the halved problem.
    // - The total presses for this subset = presses_in_phase1 + 2 * presses_for_halved.
    // - Take the minimum over all subsets, cache results, and treat INF as unreachable.

    // Temporary vector: how many times each column is incremented by the chosen phase-1 buttons
    // (contrib[j] = number of selected buttons that affect column j)
    let mut contrib = vec![0i64; n];

    // Enumerate all 2^B subsets of buttons as the candidate first-phase presses.
    // Pressing a button more than once in phase 1 is unnecessary (redundant modulo 2),
    // so we consider press-once-or-not (subset membership).
    let total_subsets: u64 = 1u64 << b;

    for subset in 0..total_subsets {
        // reset contribution counts for this subset, and track how many buttons we pressed in phase1
        contrib.fill(0);
        let mut presses_phase1 = 0i64;

        // For each button in the subset, increment `presses_phase1` and add 1 to each column it affects.
        // contrib[j] will end up holding how many times column j is decreased by this phase-1 choice.
        for (btn_idx, cols) in buttons.iter().enumerate() {
            if (subset >> btn_idx) & 1 == 1 {
                presses_phase1 += 1;
                for &col in cols {
                    contrib[col] += 1;
                }
            }
        }

        // After applying this phase-1 choice, compute remaining joltages.
        // If any column would go negative, or the remaining value is odd, this subset is invalid.
        // Otherwise, halve the remaining values and recurse on the reduced instance.
        let mut next_state = Vec::with_capacity(n);
        let mut ok = true;

        for j in 0..n {
            let t = state[j];
            let used = contrib[j];

            // Can't subtract more than we have: this subset would require pressing a button
            // that reduces column j below zero.
            if used > t {
                ok = false;
                break;
            }

            let rem = t - used;

            // Remaining must be even (parity constraint) so we can halve them and recurse.
            if rem & 1 != 0 {
                ok = false;
                break;
            }

            // Divide by 2 to produce the smaller instance for recursion.
            next_state.push(rem / 2);
        }

        if !ok {
            continue;
        }

        // Phase 2: solve the halved instance (sub). If it's reachable, we will repeat
        // the halved sequence twice (hence the 2 * sub factor), and add the presses done
        // in phase 1. Keep the minimum over all subsets.
        let sub = solve_recursive(&next_state, buttons, memo);
        if sub == INF {
            continue;
        }

        let total_presses = presses_phase1 + 2 * sub;
        if total_presses < best {
            best = total_presses;
        }
    }

    // Cache the result (best may be INF to indicate unreachable).
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
