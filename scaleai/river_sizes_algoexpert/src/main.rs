fn dfs(
    graph: &[Vec<i32>],
    visited: &mut [Vec<bool>],
    at: (usize, usize),
    components: &mut [Vec<(usize, usize)>],
    count: usize,
) {
    let (row, col) = at;
    if visited[row][col] {
        return;
    }
    visited[row][col] = true;
    components[count].push((row, col));

    let row_len = graph.len();
    let col_len = graph[0].len();

    let h_direction = [-1, 1, 0, 0];
    let v_direction = [0, 0, 1, -1];
    for i in 0..4 {
        let new_row: i32 = row as i32 + h_direction[i];
        let new_col: i32 = col as i32 + v_direction[i];

        if new_row >= 0
            && new_col >= 0
            && new_row < row_len as i32
            && new_col < col_len as i32
            && graph[new_row as usize][new_col as usize] == 1
        {
            dfs(
                graph,
                visited,
                (new_row as usize, new_col as usize),
                components,
                count,
            );
        }
    }
}

// matrix like
// 1 0 0 1 0
// 1 0 1 0 0
// 0 0 1 0 1
// 1 0 1 0 1
// 1 0 1 1 0
//
// find the connected components (1's)
// which are the rivers
fn river_sizes(matrix: &[Vec<i32>]) -> Vec<Vec<(usize, usize)>> {
    let row_len = matrix.len();
    let col_len = matrix[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; col_len]; row_len];
    let mut connected: Vec<Vec<(usize, usize)>> = vec![vec![]; row_len];
    let mut count: usize = 0;

    for i in 0..row_len {
        for j in 0..col_len {
            if !visited[i][j] && matrix[i][j] == 1 {
                dfs(matrix, &mut visited, (i, j), &mut connected, count);
                count += 1;
            }
        }
    }

    connected
}

fn main() {
    let graph: Vec<Vec<i32>> = vec![
        vec![1, 0, 0, 1, 0],
        vec![1, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1],
        vec![1, 0, 1, 1, 0],
    ];
    let connected = river_sizes(&graph);
    for i in 0..connected.len() {
        println!("{}", connected[i].len());
    }
}
