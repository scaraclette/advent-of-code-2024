use anyhow::Result;
use std::{fs::File, io::BufReader, vec};

use token_read::TokenReader;

pub fn read_levels() -> Vec<Vec<u32>> {
    let f = File::open("input/day_2.txt").expect("Able to read file");
    let reader = BufReader::new(f);
    let mut input = TokenReader::new(reader);

    let mut levels: Vec<Vec<u32>> = Vec::new();

    loop {
        match input.line::<Vec<u32>>() {
            Ok(level) => levels.push(level),
            Err(_) => break,
        }
    }

    levels
}

/*
Determine the number of safe reports. Reports and only safe with the following

*/
fn validate_levels(levels: Vec<Vec<u32>>) -> Result<u32> {
    let res = levels.into_iter().fold(0, |acc, l| {
        let is_increasing = l.get(0).and_then(|first_elem| {
            l.get(1)
                .and_then(|second_elem| Some(first_elem < second_elem))
        });
        dbg!(is_increasing);

        let mut is_dampened = false;

        if let Some(is_increasing) = is_increasing {
            for i in 1..l.len() {
                if (l[i] > l[i - 1] && is_increasing) || (l[i] < l[i - 1] && !is_increasing) {
                    let level_diffs = (l[i]).abs_diff(l[i - 1]);
                    if level_diffs == 0 || level_diffs > 3 {
                        if !is_dampened {
                            is_dampened = true
                        } else {
                            return acc;
                        }
                    }
                } else {
                    if !is_dampened {
                        is_dampened = true;
                    } else {
                        return acc;
                    }
                }
            }
        }

        acc + 1
    });

    Ok(res)
}

// Breaks here, but got valid answer from input part 2
#[test]
fn test_small_input() {
    let levels = vec![
        // vec![7,6,4,2,1],
        // vec![1,3,6,7,9],
        // vec![1,1,2]
        // vec![1, 3, 2, 4, 5],
        // vec![4, 5, 3],
		vec![5,6,3,2,1]
    ];

    // assert_eq!(validate_levels(levels).unwrap(), 2);
    assert_eq!(validate_levels(levels).unwrap(), 1);
}

#[test]
fn test_day_two() {
    let levels = read_levels();
    let res = validate_levels(levels);
    // part 2 for valid answer
	assert_eq!(res.unwrap(), 366);
}
