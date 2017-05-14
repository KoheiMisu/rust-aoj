use std::env;

/**
文字列反転
文字列 str を入力し、その文字列を逆順に出力するプログラムを作成して下さい。文字は半角英数字のみで、20 文字以内とします。

Input
文字列 str が１行に与えられる。

Output
str を逆順にした文字列を１行に出力する。
 */

fn main() {
    for (index, argument) in env::args().enumerate() {
        if index == 1 {
            reverse(argument);
        }
    }
}

fn reverse(target: String) {
    if 20 < target.len() {
        panic!("argument is too long");
    }

    /**
     * Stringをchar型に変換
     * 逆順にしてRevオブジェクトに詰められる
     * オブジェクト内を操作してStringにする
     *
     * Stringを文字の集合として捉えることができるので可能
     */
    println!("{:?}", target.chars().rev().collect::<String>());
}
