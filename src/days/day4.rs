use crate::utils::read_lines;

pub fn solve() 
{
    solve_p1();
    solve_p2();
}

fn solve_p2()
{
    let input = read_lines("./input_files/day4_input.txt").unwrap();
    let input_bytes = input.map(|line| line.unwrap());
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input_bytes {
        let mut line_vec: Vec<char> = Vec::new();

        for c in line.chars() {
            line_vec.push(c);
        }
        matrix.push(line_vec);
    }


    let ref_matrix = matrix.clone();

    let mut sum = 0;

    for (i, line) in matrix.into_iter().enumerate() {
        for (j, _c) in line.into_iter().enumerate() {
                sum += check_x_mas((i,j), &ref_matrix);
        }
    }

    println!("Part 2: {}", sum);

}

fn check_x_mas((i, j): (usize, usize), matrix: &Vec<Vec<char>>) -> i32 
{
    //value fits in matrix bounds
    if i < 1 || j < 1 || i > matrix.len() - 2 || j > matrix[i].len() - 2 {
        return 0;
    }

    //correct center caracter
    if matrix[i][j] != 'A' {
        return 0;
    }

    let chars = vec!['M', 'A', 'S'];

    //check top left to bottom right diagonal
    {
        let mut value_to_check:Vec<char> = Vec::new();
        value_to_check.push(matrix[i-1][j-1]);
        value_to_check.push(matrix[i][j]);
        value_to_check.push(matrix[i+1][j+1]);

        if value_to_check != chars {
            value_to_check.reverse();
            if value_to_check != chars {
                return 0;
            }
        }
    }

    //check bottom left to top right diagonal
    {
        let mut value_to_check:Vec<char> = Vec::new();
        value_to_check.push(matrix[i+1][j-1]);
        value_to_check.push(matrix[i][j]);
        value_to_check.push(matrix[i-1][j+1]);

        if value_to_check != chars {
            value_to_check.reverse();
            if value_to_check != chars {
                return 0;
            }
        }
    }


    return 1;
}

fn solve_p1() 
{
    let input = read_lines("./input_files/day4_input.txt").unwrap();
    let input_bytes = input.map(|line| line.unwrap());
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input_bytes {
        let mut line_vec: Vec<char> = Vec::new();

        for c in line.chars() {
            line_vec.push(c);
        }
        matrix.push(line_vec);
    }


    let ref_matrix = matrix.clone();

    let mut sum = 0;

    for (i, line) in matrix.into_iter().enumerate() {
        for (j, _c) in line.into_iter().enumerate() {
                sum += count_xmas((i,j), &ref_matrix);
        }
    }

    println!("Part 1: {}", sum);
}

fn count_xmas((i, j): (usize, usize), matrix: &Vec<Vec<char>>) -> i32 {
    let chars = vec!['X', 'M', 'A', 'S'];
    let mut sum = 0;
    // let mut value_to_check:Vec<char> = Vec::new();

    //look forwards
    if j + 3 < matrix[i].len(){
        let mut value_to_check:Vec<char> = Vec::new();

        for idx in 0..4 {
            value_to_check.push(matrix[i][j+idx]);
        }
        if value_to_check == chars {
            sum += 1;
        }
    }
    // look backwards
    if j >= 3 {
        let mut value_to_check:Vec<char> = Vec::new();
        for idx in 0..4 {
            value_to_check.push(matrix[i][j-idx]);
        }
        if value_to_check == chars {
            sum += 1;
        }
    }

    // //look up
    if i >= 3 {
        let mut value_to_check:Vec<char> = Vec::new();
        for idx in 0..4 {
            value_to_check.push(matrix[i-idx][j]);
        }
        if value_to_check == chars {
            sum += 1;
        }
    }

    // //lood down
    if i <= matrix.len() - 4 {
        let mut value_to_check:Vec<char> = Vec::new();
        for idx in i..i+4 {
            value_to_check.push(matrix[idx][j]);
        }
        if value_to_check == chars {
            sum += 1;
        }
    }

    // //look diagonal up and forward
    if i >= 3 && j + 3 < matrix[i].len() {
        let mut value_to_check:Vec<char> = Vec::new();
        for idx in 0..4 {
            value_to_check.push(matrix[i-idx][j+idx]);
        }
        if value_to_check == chars {
            sum += 1;
        }
    }

    // //lood diagonal down and forward
    if i + 3 < matrix.len() && j + 3 < matrix[i].len() {
        let mut value_to_check:Vec<char> = Vec::new();
        for idx in 0..4 {
            value_to_check.push(matrix[i+idx][j+idx]);
        }
        if value_to_check == chars {
            sum += 1;
        }
    }

    // //lood diagonal up and back
    if i >= 3 && j >= 3 {
        let mut value_to_check:Vec<char> = Vec::new();
        for idx in 0..4 {
            value_to_check.push(matrix[i-idx][j-idx]);
        }
        if value_to_check == chars {
            sum += 1;
        }
    }

    // //look diagonal down and back
    if i + 3 < matrix.len() && j >= 3 {
        let mut value_to_check:Vec<char> = Vec::new();
        for idx in 0..4 {
            value_to_check.push(matrix[i+idx][j-idx]);
        }
        if value_to_check == chars {
            sum += 1;
        }
    }

    return sum;
}