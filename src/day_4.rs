use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use token_read::{self, ReadTokensError, TokenReader};

/// Create 2D matrix from input
fn create_matrix() -> Vec<Vec<char>> {
    let f = File::open("input/day_4.txt").expect("Able to read file");
    let mut reader = BufReader::new(f);
    let mut matrix: Vec<Vec<char>> = Vec::new();

    loop {
        let mut line = String::new();
        if let Err(err) = reader.read_line(&mut line) {
            eprintln!("{}", err.kind());
            break;
        };

        if line.is_empty() {
            break;
        }

        matrix.push(line.trim().to_ascii_lowercase().chars().collect());
    }

    matrix
}

/*
x a a x
m m s a
a s a x
s a x s
Result: 2, vertical, horizontal, diagonal
DFS on each x
*/
fn count_xmas(matrix: Vec<Vec<char>>) -> (i32, i32) {
    let direction: Vec<(isize, isize)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut count_xmas = 0;
    let mut count_x_mas = 0;

    let first_validation: Vec<(char, (isize, isize))> = vec![
        ('m', (-1, -1)),
        ('m', (1, -1)),
        ('s', (-1, 1)),
        ('s', (1, 1)),
    ];
    let second_validation: Vec<(char, (isize, isize))> = vec![
        ('s', (-1, -1)),
        ('m', (1, -1)),
        ('s', (-1, 1)),
        ('m', (1, 1)),
    ];
    let third_validation: Vec<(char, (isize, isize))> = vec![
        ('s', (-1, -1)),
        ('s', (1, -1)),
        ('m', (-1, 1)),
        ('m', (1, 1)),
    ];
    let fourth_validation: Vec<(char, (isize, isize))> = vec![
        ('m', (-1, -1)),
        ('s', (1, -1)),
        ('m', (-1, 1)),
        ('s', (1, 1)),
    ];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'x' {
                direction.iter().for_each(|dir| {
                    count_xmas +=
                        traverse_xmas_straight(&matrix, i as isize, j as isize, dir.to_owned());
                });
            }
            if matrix[i][j] == 'a' {
                if validate_x_mas(&matrix, i as isize, j as isize, &first_validation)
                    || validate_x_mas(&matrix, i as isize, j as isize, &second_validation)
                    ||validate_x_mas(&matrix, i as isize, j as isize, &third_validation)
                    || validate_x_mas(&matrix, i as isize, j as isize, &fourth_validation)
                {
                    count_x_mas += 1;
                }
            }
        }
    }

    (count_xmas, count_x_mas)
}

type Validation = Vec<(char, (isize, isize))>;

fn validate_x_mas(
    matrix: &Vec<Vec<char>>,
    row: isize,
    col: isize,
    validation: &Validation,
) -> bool {
    let row_len = matrix.len() as isize;
    let col_len = matrix[0].len() as isize;

    for (expected_char, (r, c)) in validation {
        let (n_r, n_c) = (row + r, col + c);

        // Case: out of bounds index
        if n_r < 0 || n_c < 0 || n_r >= row_len || n_c >= col_len {
            return false;
        }

        let curr_char = matrix[n_r as usize][n_c as usize];
        if !curr_char.eq(&expected_char) {
            // dbg!(curr_char, expected_char, (row, col), (n_r, n_c));
            return false;
        }

        println!();
    }

    true
}

fn traverse_xmas_straight(
    matrix: &Vec<Vec<char>>,
    row: isize,
    col: isize,
    dir: (isize, isize),
) -> i32 {
    let char_map: HashMap<char, char> = HashMap::from([('x', 'm'), ('m', 'a'), ('a', 's')]);

    let mut stack = vec![(row, col)];
    let mut xmas_count = 0;

    let row_len = matrix.len() as isize;
    let col_len = matrix[0].len() as isize;

    while let Some((s_r, s_c)) = stack.pop() {
        let curr_char = matrix[s_r as usize][s_c as usize];

        // found 'xmas'
        if curr_char.eq(&'s') {
            xmas_count += 1;
            continue;
        }

        let (r, c) = dir;
        let (n_r, n_c) = (s_r + r, s_c + c);

        // Case: out of bounds index
        if n_r < 0 || n_c < 0 || n_r >= row_len || n_c >= col_len {
            continue;
        }
        // Case: non-matching next character
        let next_char = char_map
            .get(&curr_char)
            .expect("input guarantees xmas characters");

        // Case: matching next character
        if matrix[n_r as usize][n_c as usize].eq(next_char) {
            stack.push((n_r, n_c));
        }
    }

    xmas_count
}

/// Silly goose, it's not a word snake. But pretty cool implementation!
fn traverse_xmas_snake(matrix: &Vec<Vec<char>>, row: isize, col: isize) -> i32 {
    let char_map: HashMap<char, char> = HashMap::from([('x', 'm'), ('m', 'a'), ('a', 's')]);
    let direction: Vec<(isize, isize)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    let mut stack = vec![(row, col)];
    let mut xmas_count = 0;

    let row_len = matrix.len() as isize;
    let col_len = matrix[0].len() as isize;

    let mut visited = HashSet::new();
    visited.insert((row, col));

    while let Some((s_r, s_c)) = stack.pop() {
        // 'as' keyword will wrap around the variable
        let curr_char = matrix[s_r as usize][s_c as usize];

        // Found xmas
        if curr_char.eq(&'s') {
            xmas_count += 1;
            continue;
        }

        direction.iter().for_each(|(r, c)| {
            let (n_r, n_c) = (s_r + r, s_c + c);

            // Case: out of bounds index
            if n_r < 0 || n_c < 0 || n_r >= row_len || n_c >= col_len || !visited.insert((n_r, n_c))
            {
                return;
            }

            // Case: non-matching next character
            let next_char = char_map
                .get(&curr_char)
                .expect("input guarantees xmas characters");

            // Case: matching next character
            if matrix[n_r as usize][n_c as usize].eq(next_char) {
                stack.push((n_r, n_c));
            }
        });
    }

    xmas_count
}

#[test]
fn test_day_1() {
    let res = count_xmas(create_matrix());
    assert_eq!(res.0, 2358);
    assert_eq!(res.1, 1737);
}

#[test]
fn test_small_x_mas() {
    let input = vec![
        vec!['m', 'x', 'm'],
        vec!['m', 'a', 's'],
        vec!['s', 'x', 's'],
    ];

    let correct_count = count_xmas(input);
    assert_eq!(correct_count.0, 0);
    assert_eq!(correct_count.1, 1);
}

#[test]
fn test_traverse_xmas_straight() {
    let input = vec![
        vec!['x', 'm', 'a', 's'],
        vec!['s', 'm', 'a', 's'],
        vec!['x', 'm', 'a', 's'],
        vec!['s', 'm', 'a', 's'],
    ];

    let res = traverse_xmas_straight(&input, 0, 0, (0, 1));
    assert_eq!(res, 1);
    let res = traverse_xmas_straight(&input, 0, 0, (1, 1));
    assert_eq!(res, 1);
    let res = traverse_xmas_straight(&input, 0, 0, (0, -1));
    assert_eq!(res, 0);
    let res = traverse_xmas_straight(&input, 0, 0, (1, -1));
    assert_eq!(res, 0);
}

#[test]
fn test_traverse_xmas() {
    let input = vec![
        vec!['x', 'm', 'a', 's'],
        vec!['s', 'm', 'a', 's'],
        vec!['x', 'm', 'a', 's'],
        vec!['s', 'm', 'a', 's'],
    ];

    let res_snake = traverse_xmas_snake(&input, 0, 0);
    assert_eq!(res_snake, 4);

    // Wrong since it returns 4, which makes it like xmas being a snake formation (not just a straight line)
    // assert_eq!(res, 2);
    let correct_count = count_xmas(input);
    assert_eq!(correct_count.0, 3);
    assert_eq!(correct_count.1, 2);
}

#[test]
fn random_test() {
    let i: isize = -1;
    assert_eq!(i as usize, usize::MAX);
}

#[test]
fn test_count_xmas() {
    let matrix = create_matrix();
    count_xmas(matrix);
}

#[test]
fn test_create_matrix() {
    let res = create_matrix();
    dbg!(res);
}
