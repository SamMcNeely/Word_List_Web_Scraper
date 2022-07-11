use std::fs::{write, File};

fn main() {
    let url_list = [
        "https://www.vocabulary.com/lists/154147",
        "https://www.vocabulary.com/lists/154148",
        "https://www.vocabulary.com/lists/154149",
    ];

    let mut word_list: Vec<String> = Vec::new();

    for url in url_list {
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&response);

        let title_selector = scraper::Selector::parse("a.word").unwrap();

        let words = document.select(&title_selector).map(|x| {
            let inner_html = x.inner_html();

            let word: String = inner_html
                .split(' ')
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .into();

            word
        });

        let words_vec: Vec<String> = words.collect();

        words_vec.iter().for_each(|word| {
            if word.len() >= 4 {
                word_list.push(word.to_string());
            }
        });
    }

    let json = serde_json::to_string(&word_list).expect("Could Not Serialize Word List");

    File::create("word_list.json").expect("Could not create file word.json");
    write("word_list.json", json).expect("Could not write data to file");
}
