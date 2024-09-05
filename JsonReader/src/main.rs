use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
      "article": "How are you?",
      "author": "Baibhav",
      "paragraph": [
        {
          "name": "Naam kya ha apka?"
        },
        {
          "name": "nhi btaoga?"
        },
        {
          "name": "chlo mat btao?"
        }
      ]
    }
    "#;

    let parsed: Article = read_json_typed(json);

    
    println!("\n\n Parsed Article Struct: {:#?}", parsed);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).expect("Failed to parse JSON");
    parsed
}
