use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    fs::{self, File},
    io::BufReader,
};

pub fn read_input_simple() -> String {
    let contents = fs::read_to_string("input/day_3.txt").expect("Able to read file");

    contents
}

/*
Regex resources:
- https://www.sitepoint.com/learn-regex/.
- https://regex101.com/
Very proud to not have made a regex with AI.
*/
pub fn extract_multiplications(contents: String) -> Result<i32> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)")?;

    let res = re.find_iter(&contents).fold(0, |acc, m| {
        let multiplication = m.as_str();
        match compute_multiplication(multiplication) {
            Ok(val) => acc + val,
            Err(_) => acc,
        }
    });

    Ok(res)
}

/// Extracts don't(), do(), and mul(x,y) from contents
pub fn extract_multiplications_with_rules(contents: String) -> Result<Vec<String>> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|don\'t\(\)|do\(\)")?;
    let res = re
        .find_iter(&contents)
        .map(|m| m.as_str().to_owned())
        .collect();

    Ok(res)
}

pub fn calculate_multiplications_with_rules(
    multiplications_with_rules: Vec<String>,
) -> Result<i32> {
    dbg!(multiplications_with_rules.len());

    let mut enable: Option<bool> = None;
    let mut final_res = 0;

    for r in multiplications_with_rules {
        match r {
            r if r == "don't()".to_string() => {
                enable = Some(false);
            }
            r if r == "do()".to_string() => {
                enable = Some(true);
            }
            _ => {
                if enable.is_some_and(|f| !f) {
                    continue;
                }

                match compute_multiplication(&r) {
                    Ok(res) => {
                        final_res += res;
                    }
                    Err(err) => eprintln!("{err}"),
                };
            }
        }
    }

    Ok(final_res)
}

/// Computes multiplication from mul(x,y)
pub fn compute_multiplication(mul: &str) -> Result<i32> {
    let re = Regex::new(r"[0-9]+")?;

    // https://doc.rust-lang.org/rust-by-example/error/iter_result.html
    let nums: Vec<i32> = re
        .find_iter(mul)
        .filter_map(|m| {
            let num_str = m.as_str();
            num_str.parse::<i32>().ok()
        })
        .collect();

    match (nums.get(0), nums.get(1)) {
        (Some(n1), Some(n2)) => {
            return Ok(n1 * n2);
        }
        _ => {
            return Err(anyhow!("Unable to extract numbers"));
        }
    }
}

#[test]
fn test_day_three_part_2() {
    let contents = read_input_simple();
    let multiplications_with_rules =
        extract_multiplications_with_rules(contents).expect("able to extract");
    let res = calculate_multiplications_with_rules(multiplications_with_rules);
    assert_eq!(res.unwrap(), 92082041);
}

#[test]
fn test_day_three_part_1() {
    let res = read_input_simple();
    let res = extract_multiplications(res);
    assert_eq!(res.unwrap(), 191183308);
}

#[test]
fn test_extract_multiplication_with_rules() {
    let res = read_input_simple();
    let res = extract_multiplications_with_rules(res);
}

#[test]
fn test_compute_multiplication() {
    let mul = "mul(3,2)";
    let res = compute_multiplication(mul);
    assert_eq!(res.unwrap(), 6);
}
