use std::collections::HashSet;

// use advent_of_code_2020::rows_to_vector;

static INPUT: &str = include_str!("../../inputs/day06.txt");
// const EYE_COLORS: &'static [&'static str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    let a1 = solve_first(INPUT);
    println!("Day 6\n first puzzle: {}", a1.unwrap());
    let a2 = solve_second(INPUT);
    println!(" second puzzle: {}", a2.unwrap());
}

fn solve_first(rows: &str) -> Result<usize, &'static str> {
    let mut result = 0;
    let group_iterator: Vec<&str> = rows.split("\n\n").collect();
    for group in group_iterator {
        let mut set: HashSet<char> = HashSet::new();
        group.split("\n").flat_map(|s| s.chars()).for_each(|c| {
            set.insert(c);
        });
        result += set.len();
    }
    Ok(result)
}

/*
abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
    In the second group, there is no question to which everyone answered "yes".
    In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
    In the fourth group, everyone answered yes to only 1 question, a.
    In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

*/
fn solve_second(rows: &str) -> Result<usize, &'static str> {
    let mut result = 0;
    let group_iterator: Vec<&str> = rows.split("\n\n").collect();
    for group in group_iterator {
        let mut set_a: HashSet<char> = HashSet::new();
        let mut set_b: HashSet<char> = HashSet::new();

        let mut set_mem_ref = &mut set_a;
        let mut set_new_ref = &mut set_b;

        for line_e in group.split("\n").enumerate() {
            let tmp = set_new_ref;
            set_new_ref = set_mem_ref;
            set_mem_ref = tmp;
            set_new_ref.clear();

            line_e.1.chars().for_each(|c| {
                if line_e.0 == 0 || set_mem_ref.contains(&c) {
                    set_new_ref.insert(c);
                }
            });
        }

        //   dbg!(&set_new_ref);

        result += set_new_ref.len();
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_first() {
        assert_eq!(solve_first(INPUT).unwrap(), 7283);
    }
    #[test]
    fn solution_second() {
        assert_eq!(solve_second(INPUT).unwrap(), 3520);
    }
}
