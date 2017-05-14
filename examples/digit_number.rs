use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

/**
桁数
与えられた２つの整数 a と b の和の桁数を出力するプログラムを作成して下さい。

Input
複数のデータセットが与えられます。各データセットは １ 行に与えられます。各データセットは２つの整数 a と b が１つのスペースで区切られて与えられます。入力の終わりまで処理して下さい。

Constraints
0 ≤ a, b ≤ 1,000,000
データセットの数 ≤ 200
Output
各データセットごとに、a+b の桁数を出力して下さい
 */

fn main() {
    let path = Path::new("/Users/kouhei/Project/SampleData/digitNumber/1.txt");
    // 読み込んだパスを取得
    let display = path.display();

    // pathを読み込み専用モードで開く。これは`io::Result<File>`を返す。
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // ファイルの中身を文字列に読み込む。`io::Result<useize>`を返す。
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        // Ok(_) => print!("{} contains:\n{}", display, s),
        Ok(_) => parse_content(s),
    }
}

fn parse_content(content: String) {

    let mut reader = content.lines();

    loop {
        match reader.next() {
            None => break,
            Some(data) => {
                calcurate_digit(data);
            }
        }
    }
}

fn calcurate_digit(data: &str) {
    let v: Vec<&str> = data.split(' ').collect();

    let x: i32 = FromStr::from_str(v[0]).unwrap();
    let y: i32 = FromStr::from_str(v[1]).unwrap();

    let result: String = (x + y).to_string();

    println!("{:?}", result.len());
}
