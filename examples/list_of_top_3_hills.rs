use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

/**
山の高さ
山の高さをメートル単位の整数で表した 10 個のデータがあります。その 10 個のデータを読み込み、その中で、高い順から３つ出力するプログラムを作成して下さい。

Input
山の高さ1
山の高さ2
     .
     .
山の高さ10
Constraints
0 ≤ 山の高さ ≤ 10,000
 */

fn main() {
    let path = Path::new("/Users/kouhei/Project/SampleData/listHills/1.txt");
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
        Ok(_) => parse_content(s),
    }
}

fn parse_content(content: String) {

    /**
     * テキストをvecに区切って格納
     */
    let mut reader = content.lines();
    let mut list: Vec<i32> = Vec::new(); // 長さを決める場合は、Vec::with_capacity(n)

    loop {
        match reader.next() {
            None => break,
            Some(num) => {
                // &str を i32型に変換して詰める
                list.push(FromStr::from_str(num).unwrap());
            }
        }
    }

    /**
     * sort
     */
    let mut made_changes = true;
    let mut item_count = list.len();

    while made_changes {
        made_changes = false;
        item_count -= 1;
        let mut i = 0;
        while i < item_count {
            if list[i] < list[i + 1] {
                list.swap(i, i + 1);
                made_changes = true;
            }
            i += 1;
        }
    }

    // 結果を出力
    for i in 0..3 {
        println!("{}番目に高い山の高さ{}", i + 1, list[i]);
    }
}
