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
    println!("{:?}", parsed);

    let article: Article = Article {
        article: String::from("foo"),
        author: String::from("Pedro"),
        paragraph: vec![Paragraph {
            name: String::from("hello"),
        }],
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("{:?}", json);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(&raw_json).unwrap();
    return parsed;
}
