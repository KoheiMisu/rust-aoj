use std::collections::HashMap;
use std::fmt;

const SENTENCE: &str = "Thank you for your mail and your lectures";

struct MaxAppeared {
    count: i32,
    longest_word: i32
}

struct Word {
    name: String,
    appeared: i32,
    word_length: i32,
}

#[derive(Default)]
struct ParseResult {
    most_appeared_word: String,
    longest_word: String
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Word {{ name: {}, appeared: {}, word_length: {} }}", self.name, self.appeared, self.word_length)
    }
}

impl fmt::Debug for ParseResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseResult {{ most_appeared_word: {}, longest_word: {} }}", self.most_appeared_word, self.longest_word)
    }
}

/**
英語の文章
英語の文章に出現する単語の中で、出現頻度が最も高い単語と、文字数が最も多い単語を出力するプログラムを作成してください。入力データには英文字、スペースのみが含まれているものとします。単語とは、スペースで区切られた連続する英文字の列です。

Input
英語の文章を表現する文字列（半角の英文字、スペース）が１行に与えられます。

文章の文字数は 1000 文字以下です。一つの単語の文字数は 32 文字以下です。出現頻度が最も高い単語、最長の文字数を持つ単語はそれぞれ文中にただ一つだけ存在するものとします。

Output
最も出現頻度が高い単語と、最も文字数が多い単語を１つのスペースで区切って１行に出力して下さい。

Sample Input
Thank you for your mail and your lectures

Output for the Sample Input
your lectures
*/
fn main() {
    let result = find_most_appeared_word();
    println!("text: {}", SENTENCE);
    println!("most appeared word: {}, longest word: {}", result.most_appeared_word, result.longest_word);
}

/**
 * find_most_appeared_word
 */
fn find_most_appeared_word() -> ParseResult {
    let v: Vec<&str> = SENTENCE.split(' ').collect();
    let mut x = v.iter().cloned().collect::<Vec<&str>>();

    /**
     * @Todo
     * value assigned to `word_appeared_count` is never read
     * 解決する
     */
    let mut word_appeared_count = 0;
    let mut max_appeared = MaxAppeared { count: 0, longest_word: 0 };
    let mut words = HashMap::new();

    /**
     * @Todo こいつうまく書くタイミング。。
     */
    x.sort();
    x.dedup();

    for index in 0..x.len() {
        word_appeared_count = 0;

        for val in &v {
            // @Todo このto_string()どうにかならないかな、、
            if x[index].to_string() == val.to_string() {
                word_appeared_count += 1
            }
        }

        if max_appeared.count < word_appeared_count {
            max_appeared.count = word_appeared_count;
        }

        // usize を i32として受け取る
        let word_length = x[index].len() as i32;

        if max_appeared.longest_word < word_length {
            max_appeared.longest_word = word_length;
        }

        let word = Word {
            name: x[index].to_string(),
            appeared: word_appeared_count,
            word_length: word_length,
        };

        words.insert(index, word);
    }

    let mut result = ParseResult::default();

    for (_, word) in &words {
        if word.appeared == max_appeared.count {
            result.most_appeared_word = word.name.to_string();
        }
    }

    for (_, word) in &words {
        if word.word_length == max_appeared.longest_word {
            result.longest_word = word.name.to_string();
        }
    }

    result
}
