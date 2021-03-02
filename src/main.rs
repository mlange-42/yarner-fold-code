use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
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
    let mut data = yarner_lib::parse_input()?;
    let config = &data.context.config;

    check_version(&data.context);

    let min_lines = config
        .get("min-lines")
        .and_then(|s| s.as_integer())
        .unwrap_or(0);

    let languages: HashSet<String> = match config.get("languages") {
        None => HashSet::new(),
        Some(v) => v
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|l| l.as_str().unwrap_or_default().to_lowercase())
            })
            .ok_or("Can't parse array of languages")?
            .collect(),
    };

    let ignore_languages: HashSet<String> = match config.get("ignore-languages") {
        None => HashSet::new(),
        Some(v) => v
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|l| l.as_str().unwrap_or_default().to_lowercase())
            })
            .ok_or("Can't parse array of ignored languages")?
            .collect(),
    };

    for (_path, doc) in data.documents.iter_mut() {
        let mut idx = 0;

        while idx < doc.nodes.len() {
            if let Node::Code(block) = &doc.nodes[idx] {
                let lang = block
                    .language
                    .as_ref()
                    .map(|l| l.to_lowercase())
                    .unwrap_or_else(|| "".to_string());

                let num_lines = block.source.len() + if block.is_unnamed { 0 } else { 1 };

                if (languages.is_empty() || languages.contains(&lang))
                    && !ignore_languages.contains(&lang)
                    && num_lines as i64 >= min_lines
                {
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
            }
            idx += 1;
        }
    }

    yarner_lib::write_output(&data)?;
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
