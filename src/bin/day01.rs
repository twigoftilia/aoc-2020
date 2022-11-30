use advent_of_code_2020::integer_rows_to_vector;

static DAY_1_INPUT: &str = include_str!(r"../../inputs/day01.txt");
static FAIL_STRING: &str = "Could not solve task";

fn main() {
    let v = integer_rows_to_vector(DAY_1_INPUT);
    let d1_1 = solve_first(&v);
    println!("Day 1\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve_second(&v);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn solve_first(numbers: &[i32]) -> Result<i32, &'static str> {
    let last_position = numbers.len() - 1;
    for (i, j) in numbers.iter().enumerate() {
        if i != last_position {
            let the_rest = &numbers[i + 1..];
            for (_i, j2) in the_rest.iter().enumerate() {
                if *j + *j2 == 2020 {
                    return Ok(*j * (*j2));
                }
            }
        }
    }
    Err(FAIL_STRING)
}

fn solve_second(numbers: &Vec<i32>) -> Result<i32, &'static str> {
    let n = numbers.clone();

    let mut v = n
        .iter()
        .filter(|i| **i <= 2020)
        .map(|&x| x)
        .collect::<Vec<i32>>();

    v.sort();

    let last_position = v.len() - 1;

    for (i, j) in v.iter().enumerate() {
        //   dbg!(i, j);
        if i == last_position - 1 {
            break;
        }
        let the_seconds = &v[(i + 1)..];
        for (i2, j2) in the_seconds.iter().enumerate() {
            //      dbg!(i2, j2);
            if (j + (2 * j2)) > 2020 {
                break;
            }
            let the_thirds = &v[(i2 + 1)..];

            for (_i3, j3) in the_thirds.iter().enumerate() {
                let sum = j + j2 + j3;

                if sum < 2020 {
                    continue;
                };
                if sum > 2020 {
                    break;
                };
                return Ok(j * j2 * j3);
            }
        }
    }

    Err(FAIL_STRING)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_solution() {
        assert_eq!(
            solve_first(&integer_rows_to_vector(DAY_1_INPUT)).unwrap(),
            181044
        );
    }

    #[test]
    fn second_solution() {
        assert_eq!(
            solve_second(&integer_rows_to_vector(DAY_1_INPUT)).unwrap(),
            82660352
        );
    }
    #[test]
    fn example_first() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve_first(&input).unwrap(), 514579);
    }
    #[test]
    fn test_1() {
        let input = vec![0, 2020];
        assert_eq!(solve_first(&input).unwrap(), 0);
        let input = vec![2020];
        assert_eq!(solve_first(&input).is_ok(), false);
    }
    #[test]
    fn test_2() {
        let input = vec![0, 0, 2020];
        assert_eq!(solve_second(&input).unwrap(), 0);
        let input = vec![1, 2, 2017];
        assert_eq!(solve_second(&input).unwrap(), 2017 * 2);
        let input = vec![300, 301, 2020 - 300 - 301];
        assert_eq!(
            solve_second(&input).unwrap(),
            300 * 301 * (2020 - 300 - 301)
        );

        let input = vec![0, 1, 2020];
        assert_eq!(solve_second(&input).is_ok(), false);
    }
}
