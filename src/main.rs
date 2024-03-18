use serde::{Deserialize, Serialize};

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
        "article": "how to work with json in rust",
        "author": "Pedro",
        "paragraph": [
            {
            "name": "Pudding macaroon bear claw candy croissant."
            },
            {
            "name": "dragée licorice ice cream jujubes fruitcake wafer chocolate bar pudding chupa chups."
            },
            {
            "name": "macaroon chocolate bar apple pie tootsie roll sugar plum cupcake bear claw bonbon muffin."
            }
        ]
        }
    "#;

    let parsed: Article = read_json_typed(json);
    println!("{:?}", parsed)
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(&raw_json).unwrap();
    return parsed;
}
