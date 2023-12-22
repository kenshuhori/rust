use proconio::input;

// 問題
// 3番目に大きい数字を出力する

// input例
//4 18 25 20 9 13

// output例
// 18

fn main() {
    input! {
        mut numbers: [i32; 6],
    }

    numbers.sort_by(|a, b| a.cmp(b));
    println!("{}", numbers[3])
}
