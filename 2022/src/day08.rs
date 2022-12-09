use ndarray::Array2;

type Data = Array2<u8>;

pub fn parse_input(input: &str) -> Data {
    let input_data: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect();

    Array2::from_shape_vec(
        (input_data.len(), input_data[0].len()),
        input_data.iter().flatten().cloned().collect(),
    )
    .unwrap()
}

pub fn part1(input: &Data) -> usize {
    let mut is_tree_visible = 0;

    for row in 0..input.shape()[0] {
        for col in 0..input.shape()[1] {
            let tree_height = input[[row, col]];

            'directions: for [dy, dx] in [[-1, 0], [0, 1], [1, 0], [0, -1]] {
                let mut x = col as isize;
                let mut y = row as isize;

                'cur_direction: loop {
                    x += dx;
                    y += dy;

                    if x < 0 || x >= input.shape()[1] as isize || y < 0 || y >= input.shape()[0] as isize {
                        is_tree_visible += 1;
                        break 'directions;
                    }

                    if input[[y as usize, x as usize]] >= tree_height {
                        break 'cur_direction;
                    }
                }
            }
        }
    }

    is_tree_visible
}

pub fn part2(input: &Data) -> usize {
    let mut max_scenic_score = 0;

    for row in 1..input.shape()[0] - 1 {
        for col in 1..input.shape()[1] - 1 {
            let mut scenic_score = 1;

            let tree_height = input[[row, col]];

            for [dy, dx] in [[-1, 0], [0, 1], [1, 0], [0, -1]] {
                let mut x = col as isize;
                let mut y = row as isize;

                let mut visible_trees = 0;

                'cur_direction: loop {
                    x += dx;
                    y += dy;

                    if x < 0 || x >= input.shape()[1] as isize || y < 0 || y >= input.shape()[0] as isize {
                        break;
                    }

                    let cur_tree_height = input[[y as usize, x as usize]];
                    visible_trees += 1;

                    if cur_tree_height >= tree_height {
                        // Last tree visible in this direction
                        break 'cur_direction;
                    }
                }

                scenic_score *= visible_trees;
            }

            max_scenic_score = max_scenic_score.max(scenic_score);
        }
    }

    max_scenic_score
}
