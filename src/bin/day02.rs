use advent_of_code_2020::rows_to_vector;
static DAY_2_INPUT: &str = include_str!(r"../../inputs/day02.txt");

fn main() {
    let v = rows_to_vector(DAY_2_INPUT);
    let d1_1 = solve_first(&v);
    println!("Day 2\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve_second(&v);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn solve_first(rows: &Vec<&str>) -> Result<usize, &'static str> {
    // example: // 2-9 c: ccccccccc
    let fold_bad_pwds = |sum, line: &&str| -> usize {
        let mut c_i = (*line).chars();

        let mut first_done: bool = false;
        let mut min = 0;
        let mut max = 0;
        let mut the_char: Option<char> = None;
        let mut count = 0;

        loop {
            let s = c_i.next();
            if s.is_none() {
                break;
            }
            let c = s.unwrap();
            match c {
                '0'..='9' => {
                    let int_val = c.to_string().parse::<u64>().unwrap();
                    if !first_done {
                        min = min * 10 + int_val;
                    } else {
                        max = max * 10 + int_val;
                    }
                }
                '-' => {
                    first_done = true;
                }
                'a'..='z' | 'A'..='Z' => {
                    if the_char.is_none() {
                        the_char.replace(c);
                    } else {
                        if c == the_char.unwrap() {
                            count += 1;
                        }
                    }
                }
                ' ' | ':' => {
                    // ignore spaces etc. Worthless
                }
                _ => {
                    return sum + 1;
                }
            }
        }
        if count < min || count > max {
            return sum + 1;
        }
        return sum;
    };
    let sum = (*rows).iter().fold(0, fold_bad_pwds);
    Ok(rows.len() - sum)
}

fn solve_second(rows: &Vec<&str>) -> Result<usize, &'static str> {
    // example: // 2-9 c: ccccccccc
    let fold_bad_pwds = |sum, line: &&str| -> usize {
        let mut c_i = (*line).chars().enumerate();

        let mut first_done: bool = false;
        let mut min = 0;
        let mut max = 0;
        let mut the_char: Option<char> = None;
        //  let mut count = 0;

        loop {
            let e = c_i.next();
            if e.is_none() {
                break;
            }

            let pos = e.unwrap().0;
            let c = e.unwrap().1;
            match c {
                '0'..='9' => {
                    let int_val = c.to_string().parse::<u64>().unwrap();
                    if !first_done {
                        min = min * 10 + int_val;
                    } else {
                        max = max * 10 + int_val;
                    }
                }
                '-' => {
                    first_done = true;
                }
                'a'..='z' | 'A'..='Z' => {
                    if the_char.is_none() {
                        the_char.replace(c);
                    } else {
                        let first = line.chars().nth(pos + min as usize - 1);
                        let second = line.chars().nth(pos + max as usize - 1);

                        //   dbg!(first, second, the_char);

                        if first.is_none() || second.is_none() {
                            panic!("noting at expected index");
                        }
                        let mut hits = 0;
                        if first.unwrap() == the_char.unwrap() {
                            hits += 1;
                        }
                        //    dbg!(hits);
                        if second.unwrap() == the_char.unwrap() {
                            hits += 1;
                        }
                        //dbg!(hits, sum);
                        if hits == 1 {
                            // all fine
                            return sum;
                        }
                        return sum + 1;
                    }
                }
                ' ' | ':' => {
                    // ignore spaces etc. Worthless
                }
                _ => {
                    panic!("Parse errors in AOC, gosh...");
                }
            }
        }
        panic!("omg omg omg");
    };
    let sum = (*rows).iter().fold(0, fold_bad_pwds);
    Ok(rows.len() - sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_first() {
        assert_eq!(solve_first(&rows_to_vector(DAY_2_INPUT)).unwrap(), 474);
    }
    #[test]
    fn solution_second() {
        assert_eq!(solve_second(&rows_to_vector(DAY_2_INPUT)).unwrap(), 745);
    }
    // let v = rows_to_vector(DAY_2_INPUT);
    // let d1_1 = solve_first(&v);
    // println!("Day 2\n first puzzle: {}", d1_1.unwrap());
    // let d1_2 = solve_second(&v);
    // println!(" second puzzle: {}", d1_2.unwrap());

    #[test]
    fn test_first() {
        let input = vec!["2-9 c: ccccccccc"];
        assert_eq!(solve_first(&input).unwrap(), 1);
        let input = vec!["2-8 c: ccccccccc"];
        assert_eq!(solve_first(&input).unwrap(), 0);
        let input = vec!["9-9 c: ccccccccX"];
        assert_eq!(solve_first(&input).unwrap(), 0);

        let input = vec![
            "1-1 X: ccccccccX",
            "1-1 X: Xcccccccc",
            "1-2 X: Xcccccccc",
            "1-2 X: XcccXcccc",
        ];
        assert_eq!(solve_first(&input).unwrap(), 4);
        let input = vec!["1-2 X: XcccXXccX"];
        assert_eq!(solve_first(&input).unwrap(), 0);
        let input = vec!["1-2 X: ccXXXXccc"];
        assert_eq!(solve_first(&input).unwrap(), 0);
    }

    #[test]
    fn example_second() {
        let input = vec!["1-2 b: abcd"];
        assert_eq!(solve_second(&input).unwrap(), 1);

        let input = vec!["1-2 c: cXccccccc", "1-9 c: Xcccccccc", "9-1 c: Xcccccccc"];
        assert_eq!(solve_second(&input).unwrap(), 3);

        let input = vec!["1-2 c: cXccccccc"];
        assert_eq!(solve_second(&input).unwrap(), 1);
        let input = vec!["1-9 c: XcccccccX"];
        assert_eq!(solve_second(&input).unwrap(), 0);
        let input = vec!["1-9 c: XccccccXc"];
        assert_eq!(solve_second(&input).unwrap(), 1);
    }
}
