use proconio::input;

// 部分和問題
//
// 整数 a1, a2, ... an からいくつかを選び、その和をちょうど k にすることができるかを判定する
pub fn solve() {
    println!("> n, k, a...");

    input! {
      n: usize,
      k: usize,
      a: [usize; n]
    }

    // Depth-First Search
    fn dfs(i: usize, sum: usize, n: &usize, k: &usize, a: &Vec<usize>) -> bool {
        if i == *n {
            return sum == *k;
        }

        // not select a[i]
        if dfs(i + 1, sum, n, k, a) {
            return true;
        }

        // select a[i]
        if dfs(i + 1, sum + a[i], n, k, a) {
            return true;
        }

        return false;
    }

    let ans = dfs(0, 0, &n, &k, &a);

    println!("answer is {}", ans)
}
