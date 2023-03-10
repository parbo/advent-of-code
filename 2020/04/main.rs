use std::collections::HashMap;

type Parsed = Vec<HashMap<String, String>>;

fn is_valid(p: &HashMap<String, String>) -> bool {
    if p.len() == 8 {
        true
    } else {
        p.len() == 7 && !p.contains_key("cid")
    }
}

fn is_valid_details(p: &HashMap<String, String>) -> bool {
    let between = |s: &str, least, most| {
        s.parse::<usize>()
            .map_or(false, |v| v >= least && v <= most)
    };
    is_valid(p)
        && p.iter().all(|(k, v)| match k.as_str() {
            "byr" => between(v, 1920, 2002),
            "iyr" => between(v, 2010, 2020),
            "eyr" => between(v, 2020, 2030),
            "hgt" => {
                if v.ends_with("cm") {
                    between(&v[0..v.len() - 2], 150, 193)
                } else if v.ends_with("in") {
                    between(&v[0..v.len() - 2], 59, 76)
                } else {
                    false
                }
            }
            "hcl" => {
                v.len() == 7
                    && v.starts_with('#')
                    && v.chars()
                        .skip(1)
                        .filter(|&c| c.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c))
                        .count()
                        == 6
            }
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()),
            "pid" => v.len() == 9 && v.chars().filter(|c| c.is_numeric()).count() == 9,
            "cid" => true,
            _ => false,
        })
}

fn part1(passports: &Parsed) -> usize {
    passports.iter().filter(|x| is_valid(x)).count()
}

fn part2(passports: &Parsed) -> usize {
    passports.iter().filter(|x| is_valid_details(x)).count()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_by_empty_line(lines)
        .iter()
        .map(|group| group.join(" "))
        .map(|passport| {
            aoc::split(&passport, |c| c == ' ')
                .iter()
                .map(|part| {
                    let x = aoc::split(part, |c| c == ':');
                    (x[0].to_string(), x[1].to_string())
                })
                .collect()
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let invalid: Vec<_> = vec![
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093abcd154719",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect();

        for passport in parse(&invalid) {
            assert_eq!(is_valid_details(&passport), false, "error: {:?}", passport);
        }

        let valid: Vec<_> = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect();

        for passport in parse(&valid) {
            assert_eq!(is_valid_details(&passport), true, "error: {:?}", passport);
        }
    }
}
