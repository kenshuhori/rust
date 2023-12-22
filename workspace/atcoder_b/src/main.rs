use proconio::input;

// input例
// 11
// 10 9 10 3 100 100 90 80 10 30 10

// output例
// down 1
// up 1
// down 7
// up 97
// stay
// down 10
// down 10
// down 70
// up 20
// down 20

fn main() {
    let mut last_earning: Option<i32> = None;

    input! {
        n: usize,
        earnings: [i32; n],
    }

    for earning in earnings {
        match last_earning {
            None => {
                last_earning = Some(earning);
            },
            Some(last_earning) => {
                compare_and_print(last_earning, earning);
            }
        }
        last_earning = Some(earning);
    }
}

fn compare_and_print(yesterday: i32, today: i32) {
    if yesterday == today {
        println!("stay")
    } else if yesterday > today {
        println!("down {}", yesterday - today)
    } else {
        println!("up {}", today - yesterday)
    }
}
