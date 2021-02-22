use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;
use yarner_lib::{Context, Node, TextBlock};

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
    let (context, mut documents) = yarner_lib::parse_input()?;

    check_version(&context);

    for (_path, doc) in documents.iter_mut() {
        let mut idx = 0;

        while idx < doc.nodes.len() {
            if let Node::Code(block) = &doc.nodes[idx] {
                let name = block.name.clone().unwrap_or_else(|| "unnamed".to_string());

                doc.nodes.insert(
                    idx,
                    Node::Text(TextBlock {
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
                    Node::Text(TextBlock {
                        text: vec!["</details>".to_string()],
                    }),
                );
            }
            idx += 1;
        }
    }

    yarner_lib::write_output(&documents)?;
    Ok(())
}

pub fn check_version(context: &Context) {
    if context.yarner_version != yarner_lib::YARNER_VERSION {
        eprintln!(
            "  Warning: The {} plugin was built against version {} of Yarner, \
                    but we're being called from version {}",
            context.name,
            yarner_lib::YARNER_VERSION,
            context.yarner_version
        )
    }
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
