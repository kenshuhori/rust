use proconio::input;

// input
// 11
// 10 9 10 3 100 100 90 80 10 30 10

fn main() {
    let mut last_earning: Option<i32> = None;

    input! {
        n: usize,
        earnings: [i32; n],
    }

    for earning in earnings {
        if last_earning == None {
            last_earning = Some(earning);
            continue;
        }

        if earning == last_earning.unwrap() {
            println!("stay");
        } else if earning > last_earning.unwrap() {
            println!("up {}", earning - last_earning.unwrap());
        } else {
            println!("down {}", last_earning.unwrap() - earning);
        }

        last_earning = Some(earning);
    }
}
