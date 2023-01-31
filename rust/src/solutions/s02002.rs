use proconio::input;

// Lake Counting (POJ No.2386)
// http://poj.org/problem?id=2386
pub fn solve() {
    print!(
        "> n, m, w...
e.g)
10 12
1 0 0 0 0 0 0 0 0 1 1 0
0 1 1 1 0 0 0 0 0 1 1 1
0 0 0 0 1 1 0 0 0 1 1 0
0 0 0 0 0 0 0 0 0 1 1 0
0 0 0 0 0 0 0 0 0 1 0 0
0 0 1 0 0 0 0 0 0 1 0 0
0 1 0 1 0 0 0 0 0 1 1 0
1 0 1 0 1 0 0 0 0 0 1 0
0 1 0 1 0 0 0 0 0 0 1 0
0 0 1 0 0 0 0 0 0 0 1 0"
    );

    input! {
      n: usize,
      m: usize,
      w: [[u8; m]; n],
    }

    // make garden
    let mut field: Vec<Vec<String>> = Vec::new();
    for i in 0..n {
        field.push(Vec::new());
        for j in 0..m {
            if w[i][j] == 1 {
                field[i].push("W".to_string())
            } else {
                field[i].push("".to_string())
            }
        }
    }

    fn dfs(x: usize, y: usize, field: &mut Vec<Vec<String>>) {
        field[x][y] = ".".to_string();

        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;
                if nx < 0 || ny < 0 || nx >= field.len() as i64 || ny >= field[0].len() as i64 {
                    continue;
                }

                if field[nx as usize][ny as usize] == "W" {
                    dfs(nx as usize, ny as usize, field)
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if field[i][j] == "W" {
                dfs(i, j, &mut field);
                res += 1;
            }
        }
    }

    print!("\nanswer is {}", res);
}
