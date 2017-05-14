// @see https://docs.rs/libmath/0.2.0/math/
extern crate math;

use std::env;
use std::f64;
use math::round;

const DEBT: f64 = 100000_f64;
const INTEREST: f64 = 1.05;
const ROUND_UP_DIGIT: i8 = -3;

/**
借金
某国に住んでいる友達がお金に困って、あるヤミ金融業者から 10 万円の借金をしたまま、
全く返済していないといいます。
この業者は、一週間ごとに 5% の利子を借金に加え、さらに借金の 1,000 円未満を切り上げます。

整数 n を入力し、n 週間後の借金の残高を出力するプログラムを作成して下さい。

Input
整数 n (0 ≤ n ≤ 100) が１行に与えられる。

Output
n 週間後の返済額を１行に出力する。
 */

fn main() {
    for (index, argument) in env::args().enumerate() {
        if index == 1 {
            let week: i32 = validate(argument);

            calc_interest_rate(week);
        }
    }
}

fn calc_interest_rate(week: i32) {
    let mut debt = DEBT;

    let mut n = 0;

    while n < week {
        debt = round::ceil(debt * INTEREST, ROUND_UP_DIGIT);

        // Increment counter
        n += 1;
    }

    println!("your debt after {} weeks is {}", week, debt);
}

fn validate(week: String) -> i32 {
    // String to integer
    let num = week.parse::<i32>().unwrap();

    match num {
        1 ... 100 => return num,
        _ => panic!("argument is invalid: {}", num),
    }
}
