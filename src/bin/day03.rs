use advent_of_code_2020::rows_to_vector;

static DAY_3_INPUT: &str = include_str!(r"../../inputs/day03.txt");

fn main() {
    let v = rows_to_vector(DAY_3_INPUT);
    let a1 = solve_first(&v);
    println!("Day 3\n first puzzle: {}", a1.unwrap());
    let d1_2 = solve_second(&v);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn solve_first(rows: &Vec<&str>) -> Result<usize, &'static str> {
    solver(rows, 3, 1)
}

fn solve_second(rows: &Vec<&str>) -> Result<usize, &'static str> {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let res = slopes
        .iter()
        .map(|x| solver(rows, (*x).0, (*x).1).unwrap())
        .product();
    Ok(res)
}

fn solver(rows: &Vec<&str>, x_step: usize, y_step: usize) -> Result<usize, &'static str> {
    let height = rows.len();
    let width = rows[0].len();

    let get_at_closure = |x, y| -> &str {
        let map_x = x % width;
        let map_y = y % height;
        let row: &str = (*rows)[map_y]; // [map_x..map_x+1];
        let map = &(row)[map_x..];
        map
    };

    let mut x = 0;
    let mut y = 0;
    let mut sum = 0;

    loop {
        if y >= height {
            return Ok(sum);
        }

        if get_at_closure(x, y).starts_with("#") {
            sum += 1;
        }

        x += x_step;
        y += y_step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_first() {
        assert_eq!(solve_first(&rows_to_vector(DAY_3_INPUT)).unwrap(), 216);
    }
    #[test]
    fn solution_second() {
        assert_eq!(
            solve_second(&rows_to_vector(DAY_3_INPUT)).unwrap(),
            6708199680
        );
    }

    #[test]
    fn test_first() {
        let input = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];

        assert_eq!(solve_first(&input).unwrap(), 5);
    }
}
