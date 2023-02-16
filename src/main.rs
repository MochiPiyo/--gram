use lindera::tokenizer::Tokenizer;

use rand::Rng;



fn main() {

    //create tokenizer
    let tokenizer = Tokenizer::new().unwrap();

    //ファイル全体を一気に読み込み
    println!("start reading file");
    let string = std::fs::read_to_string("./text.txt").unwrap();

    //tokenize the text
    println!("start tokenizeing");
    let tokens = tokenizer.tokenize(&string).unwrap();

    //Ngram辞書の作成
    println!("start counting");
    //まず２個ずつのペアで集計
    use std::collections::HashMap;
    let mut counter: HashMap<(&str, &str), u32> = HashMap::new();
    for (token1, token2) in tokens[..tokens.len() - 1].iter().zip(tokens[1..].iter()) {
        let bi_gram = (token1.text, token2.text);
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
