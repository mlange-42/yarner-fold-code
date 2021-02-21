use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;
use yarner_lib::Node::Text;
use yarner_lib::{Node, TextBlock};

pub static CLEAN_LINK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9]").unwrap());

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            1
        }
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let (config, mut documents) = yarner_lib::parse_input(std::io::stdin())?;

    for (_path, doc) in documents.iter_mut() {
        let mut idx = 0;

        while idx < doc.nodes.len() {
            if let Node::Code(block) = &doc.nodes[idx] {
                let name = block.name.clone().unwrap_or_else(|| "unnamed".to_string());

                doc.nodes.insert(
                    idx,
                    Text(TextBlock {
                        text: vec![
                            format!(
                                "<details><summary>{}{}</summary>",
                                format_anchor(&name),
                                name
                            ),
                            String::new(),
                        ],
                    }),
                );
                idx += 1;

                doc.nodes.insert(
                    idx + 1,
                    Text(TextBlock {
                        text: vec!["</details>".to_string()],
                    }),
                );
            }
            idx += 1;
        }
    }

    let out_json = yarner_lib::to_json(&config, &documents)?;
    println!("{}", out_json);
    Ok(())
}

fn block_link(name: &str) -> String {
    format!(
        "yarner-block-{}",
        &CLEAN_LINK_REGEX.replace_all(&name.to_lowercase(), "-")
    )
}

fn format_anchor(name: &str) -> String {
    let block_link = block_link(name);
    format!("<a name=\"{}\" id=\"{}\"></a>", block_link, block_link)
}
