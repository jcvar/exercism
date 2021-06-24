pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // unimplemented!(
    //     "find the saddle points of the following matrix: {:?}",
    //     input
    // )
    let mut points = Vec::new();
    let rows = input.len();
    for r in 0..rows {
        if let Some(row_max) = input[r].iter().max() {
            for (c, _) in input[r].iter().enumerate().filter(|(_, v)| v == &row_max) {
                if (0..rows).all(|i| input[i][c] >= *row_max) {
                    points.push((r, c));
                }
            }
        }
    }
    points
}
