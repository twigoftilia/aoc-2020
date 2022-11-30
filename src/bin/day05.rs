use std::u32;

use advent_of_code_2020::{rows_to_vector, FAIL_STRING};

static INPUT: &str = include_str!("../../inputs/day05.txt");

fn main() {
    let v = rows_to_vector(INPUT);
    let a1 = solve_first(&v);
    println!("Day 5\n first puzzle: {}", a1.unwrap());
    let a2 = solve_second(&v);
    println!(" second puzzle: {}", a2.unwrap());
}

fn get_id(row: &str) -> usize {
    let mut cur_row = 0;
    let mut cur_seat = 0;

    for (i, c) in row.chars().enumerate() {
        if i < 7 {
            if c == 'B' {
                cur_row += 128 / (2 as usize).pow(i as u32) / 2;
            }
        } else {
            if c == 'R' {
                cur_seat += 8 / (2 as usize).pow((i - 7) as u32) / 2;
            }
        }
    }
    cur_row * 8 + cur_seat
}

fn solve_first(rows: &Vec<&str>) -> Result<usize, &'static str> {
    let mut higest = 0;
    for row in rows {
        let id = get_id(row);
        if id > higest {
            higest = id;
        }
    }
    Ok(higest)
}

fn solve_second(rows: &Vec<&str>) -> Result<usize, &'static str> {
    let mut ids = rows.iter().map(|s| get_id(s)).collect::<Vec<usize>>();
    ids.sort();

    let mut prev = None;
    for id in ids.iter() {
        if prev.is_some() && (prev.unwrap() + 1) != *id {
            return Ok(*id - 1);
        }
        prev = Some(*id);
    }
    Err(FAIL_STRING)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_first() {
        assert_eq!(solve_first(&rows_to_vector(INPUT)).unwrap(), 913);
    }

    #[test]
    fn solution_second() {
        assert_eq!(solve_second(&rows_to_vector(INPUT)).unwrap(), 717);
    }
    #[test]
    fn example_first() {
        assert_eq!(
            solve_first(&vec!["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]).unwrap(),
            820
        );
    }
}
