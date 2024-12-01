use anyhow::Result;
use std::{collections::HashMap, fs::File, io::BufReader, iter::zip};
use token_read::TokenReader;

/// Retrieves raw list
fn get_lists() -> Result<(Vec<u32>, Vec<u32>)> {
    let f = File::open("input/day_1.txt").expect("Able to read file");
    let reader = BufReader::new(f);
    let mut input = TokenReader::new(reader);

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    loop {
        match input.line::<(u32, u32)>() {
            Ok((left_num, right_num)) => {
                left_list.push(left_num);
                right_list.push(right_num);
            }
            _ => break,
        }
    }

    Ok((left_list, right_list))
}

/*
Pair the list in ascending order and sum distance from each pair
*/
fn find_distance() -> Result<u32> {
    let (mut left_list, mut right_list) = get_lists()?;
    left_list.sort();
    right_list.sort();

    let res = zip(left_list, right_list).fold(0, |acc, (left, right)| acc + left.abs_diff(right));

    Ok(res)
}

/*
Calculate a total similarity score by adding up each number in the left list after multiplying it
by the number of times that number appears in the right list.
*/
fn find_similarity_score() -> Result<u32> {
    let (left_list, right_list) = get_lists()?;

    let mut right_list_counter = HashMap::new();
    for num in right_list {
        right_list_counter
            .entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let res = left_list.into_iter().fold(0, |acc, left_num| {
        if let Some(right_counter) = right_list_counter.get(&left_num) {
            acc + (left_num * right_counter)
        } else {
            acc
        }
    });

    Ok(res)
}

#[test]
fn test_day_one() {
    let res = find_distance().unwrap();
    dbg!(res);
    let res = find_similarity_score().unwrap();
    dbg!(res);
}
