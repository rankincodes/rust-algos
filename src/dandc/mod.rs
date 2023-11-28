pub fn matmul_recursive(a: &mut [i32], b: &mut [i32], c: &mut [i32], n: usize) {
    matmul_recursive_helper(a, b, c, n, 0, 0, 0, 0, 0);
}

fn matmul_recursive_helper(
    a: &mut [i32],
    b: &mut [i32],
    c: &mut [i32],
    n: usize,
    a_row: usize,
    a_col: usize,
    b_row: usize,
    b_col: usize,
    c_row: usize,
) {
    if n == 1 {
        c[c_row] += a[a_row] * b[b_row];
        return;
    }
    let n2 = n / 2;
    let a_row2 = a_row + n2;
    let a_col2 = a_col + n2;
    let b_row2 = b_row + n2;
    let b_col2 = b_col + n2;
    let c_row2 = c_row + n2;
    matmul_recursive_helper(a, b, c, n2, a_row, a_col, b_row, b_col, c_row);
    matmul_recursive_helper(a, b, c, n2, a_row, a_col2, b_row2, b_col, c_row);
    matmul_recursive_helper(a, b, c, n2, a_row, a_col, b_row, b_col2, c_row2);
    matmul_recursive_helper(a, b, c, n2, a_row, a_col2, b_row2, b_col2, c_row2);
    matmul_recursive_helper(a, b, c, n2, a_row2, a_col, b_row, b_col, c_row + n2);
    matmul_recursive_helper(a, b, c, n2, a_row2, a_col2, b_row2, b_col, c_row + n2);
    matmul_recursive_helper(a, b, c, n2, a_row2, a_col, b_row, b_col2, c_row2 + n2);
    matmul_recursive_helper(a, b, c, n2, a_row2, a_col2, b_row2, b_col2, c_row2 + n2);
}
