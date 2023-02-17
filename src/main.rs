use std::io::BufRead;

use lindera::tokenizer::Tokenizer;

use rand::Rng;


const IS_NECO: bool = true;
const NECO: &str = "./neko.txt";
const WIKI40: &str = "./wiki40b.txt";

fn main() {

    

    let string = if IS_NECO {
        //ファイル全体を一気に読み込み
        println!("start reading file");
        std::fs::read_to_string(NECO).unwrap()
    }else {
        //_START_PARAGRAPH_の次の行だけを文字列として読み込み
        let file = std::fs::File::open(WIKI40).unwrap();
        let lines = std::io::BufReader::new(file);

        let mut paragraphs = String::new();
        let mut next_is_paragraph = false;
        for (i, line) in lines.lines().enumerate() {
            if let Ok(string) = line {
                //paraghaphのみを追加
                if next_is_paragraph == true {
                    paragraphs.push_str(&string);
                    next_is_paragraph = false;

                //そうでなければ次がパラグラフか検証
                }else if string == "_START_PARAGRAPH_" {
                    next_is_paragraph = true;
                }
            }else {
                println!("unable to read line '{}'", i);
            }
        }
        //これをstringに格納する
        paragraphs
    };
    
    //create tokenizer
    let tokenizer = Tokenizer::new().unwrap();
    
    //tokenize the text
    println!("start tokenizeing");
    let mut tokens: Vec<&str> = Vec::new();
    for line in string.lines() {
        let mut temp_tokens = tokenizer.tokenize_str(line).unwrap();
        tokens.append(&mut temp_tokens);
    }
    
    //Ngram辞書の作成
    println!("start counting");
    //まず２個ずつのペアで集計
    use std::collections::HashMap;
    let mut counter: HashMap<(&str, &str), u32> = HashMap::new();
    for (&str1, &str2) in tokens[..tokens.len() - 1].iter().zip(tokens[1..].iter()) {
        let bi_gram = (str1, str2);
        //あれば+1して、無ければ1で初期化。
        counter.entry(bi_gram).and_modify(|counter| *counter += 1).or_insert(1);
    }

    //集計を最初の語で検索できるように転地
    println!("start collecting");
    //HashMap<first_word, Vec<(second_word, count)>
    let mut dict: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();
    for ((str1, str2), &count) in counter.iter() {
        //println!("({},{}): {}", str1, str2, count);

        match dict.contains_key(str1) {
            false => {
                //create vec, and push
                let seconds_new: Vec<(&str, u32)> = vec![(*str2, count)];
                dict.insert(str1, seconds_new);
            },
            true => {
                //if exists, add it
                if let Some(seconds) = dict.get_mut(str1) {
                    seconds.push((str2, count));
                }
            }
        }
    }

    println!("then, generating phase");

    //generate
    let mut input_buf = String::new();
    let mut rng = rand::thread_rng();
    'sentence: loop {
        println!("input first word: ");
        std::io::stdin().read_line(&mut input_buf).unwrap();
        let input = input_buf.trim().to_string();
        input_buf.clear();


        let mut generated_text: Vec<String> = Vec::new();
        generated_text.push(input);

        //続きを作る
        //"。"かnot foundまで続ける
        '_word: loop {
            //2gramの最初の語が文章の後端と一致するものを探す。
            //なんかgetするには&strであるべきらしい
            let mut word_list = match dict.get_mut(generated_text.last().unwrap() as &str) {
                None => {
                    println!("next word is not found");
                    continue 'sentence;
                },
                Some(word_list) => {
                    word_list
                }
            };

            //出現頻度それぞれに乱数をかける
            for (_str, count) in (&mut word_list).iter_mut() {
                let rand: u32 = rng.gen();
                //u32で0~1倍する
                *count = ((*count) as u64 * rand as u64 / std::u32::MAX as u64) as u32;
            }

            //最大のものを取得
            let mut max_index = 0;
            let mut max_value: u32 = 0;
            for (i, (_str, value_this)) in word_list.iter().enumerate() {
                if max_value < *value_this {
                    max_value = *value_this;
                    max_index = i;
                }
            }

            let (next_word, _x) = word_list.get_mut(max_index).unwrap();
            //add word to text
            generated_text.push(next_word.to_string());
                
            //end of sentence
            if *next_word == "。" {
                break;
            }
            
        }
        
        //output text
        println!("{:?}", generated_text);
    }
    

}
