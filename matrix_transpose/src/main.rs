fn transpose_matrix(mut matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    for i in 0..3 {
        for j in i + 1..3 {
            (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j])
        }
    }
    matrix
}

fn print_matrix(matrix: [[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}");
    }
}

fn main() {
    // println!("Hello, world!");
    let mut mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    print_matrix(mat);
    mat = transpose_matrix(mat);
    print_matrix(mat)
}
