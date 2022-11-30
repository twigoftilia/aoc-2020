// use advent_of_code_2020::rows_to_vector;

static INPUT: &str = include_str!("../../inputs/day04.txt");
const EYE_COLORS: &'static [&'static str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    let a1 = solve_first(INPUT);
    println!("Day 4\n first puzzle: {}", a1.unwrap());
    let a2 = solve_second(INPUT);
    println!(" second puzzle: {}", a2.unwrap());
}

/*
byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.
*/
fn test_inclusive_range(str: &str, min: &i32, max: &i32) -> bool {
    //    dbg!(str);
    let i = str.parse::<i32>();
    if i.is_ok() {
        let i = i.unwrap();
        return i.ge(min) && i.le(max);
    }
    false
}

fn validate(fields_vev: &Vec<&str>, extended: bool) -> bool {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    //   let mut cid = false;

    for &str in fields_vev {
        let data_part = str.split(":").skip(1).take(1).next().unwrap();
        if str.starts_with("byr") {
            if !extended {
                byr = true;
            } else {
                if data_part.len() == 4 {
                    byr = test_inclusive_range(data_part, &1920, &2002);
                }
            }
        } else if str.starts_with("iyr") {
            if !extended {
                iyr = true;
            } else {
                if data_part.len() == 4 {
                    iyr = test_inclusive_range(data_part, &2010, &2020);
                }
            }
        // iyr = true;
        } else if str.starts_with("eyr") {
            if !extended {
                eyr = true;
            } else {
                if data_part.len() == 4 {
                    eyr = test_inclusive_range(data_part, &2020, &2030);
                }
            }
        } else if str.starts_with("hgt") {
            if !extended {
                hgt = true;
            } else {
                let val_part = &data_part[..(data_part.len() - 2)];
                if data_part.ends_with("in") {
                    hgt = test_inclusive_range(val_part, &59, &76);
                } else if data_part.ends_with("cm") {
                    hgt = test_inclusive_range(val_part, &150, &193);
                }
            }
        } else if str.starts_with("hcl") {
            if !extended {
                hcl = true;
            } else {
                if data_part.len() == 7 {
                    for (i, c) in data_part.chars().enumerate() {
                        if i == 0 {
                            if c != '#' {
                                break;
                            }
                        } else {
                            if !(c >= '0' && c <= '9' || c >= 'a' && c <= 'f') {
                                break;
                            }
                            if i == 6 {
                                hcl = true;
                            }
                        }
                    }
                }
            }
        } else if str.starts_with("ecl") {
            if !extended {
                ecl = true;
            } else {
                let s = EYE_COLORS.iter().map(|s| *s).find(|s| (*s).eq(data_part));
                if s.is_some() {
                    ecl = true;
                }
            }
        } else if str.starts_with("pid") {
            if !extended {
                pid = true;
            } else {
                if data_part.len() == 9 {
                    for (i, c) in data_part.chars().enumerate() {
                        if !(c >= '0' && c <= '9') {
                            break;
                        }
                        if i == 8 {
                            pid = true;
                        }
                    }
                }
            }
        };
    }
    if byr && iyr && eyr && hgt && hcl && ecl && pid
    /* && cid */
    {
        return true;
    }

    false
}

fn solve_first(rows: &str) -> Result<usize, &'static str> {
    solver(rows, false)
}

fn solve_second(rows: &str) -> Result<usize, &'static str> {
    solver(rows, true)
}

fn solver(rows: &str, extra_step_2_valiatione: bool) -> Result<usize, &'static str> {
    let mut valid_passports = 0;
    let passport_iterator: Vec<&str> = rows.split("\n\n").collect();
    for passport_raw in passport_iterator {
        let mut raw_fields: Vec<&str> = vec![];
        for line in passport_raw.split("\n").collect::<Vec<&str>>() {
            raw_fields.append(&mut line.split(" ").collect::<Vec<&str>>());
        }

        if validate(&raw_fields, extra_step_2_valiatione) {
            valid_passports += 1;
        }
    }

    Ok(valid_passports)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_first() {
        assert_eq!(solve_first(INPUT).unwrap(), 190);
    }
    #[test]
    fn solution_second() {
        assert_eq!(solve_second(INPUT).unwrap(), 121);
    }
    #[test]
    fn inkl() {
        assert_eq!(test_inclusive_range("1", &1, &4), true);
        assert_eq!(test_inclusive_range("4", &1, &4), true);
        assert_eq!(test_inclusive_range("0", &1, &4), false);
        assert_eq!(test_inclusive_range("5", &1, &4), false);
    }
}
