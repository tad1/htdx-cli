use clap::{Parser, Subcommand};
use indicium::simple::Indexable;
use indicium::simple::SearchIndex;
use std::fs;
use std::path::Path;

#[derive(Parser)]
struct Cli {
    filename: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(hide = true)]
    Complete { query: String },
}

const HTDX_PATH: &str = "/usr/share/htdx";

struct HtdxEntry {
    title: String,
    body: String,
}

impl Indexable for HtdxEntry {
    fn strings(&self) -> Vec<String> {
        vec![self.title.clone().replace("_", " "), self.body.clone()]
    }
}

fn load_dir(path: &str) -> Result<Vec<HtdxEntry>, Box<dyn std::error::Error>> {
    let mut htdx_entries: Vec<HtdxEntry> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(file_name) = path.file_name() {
                    htdx_entries.push(HtdxEntry {
                        title: file_name.to_string_lossy().into_owned(),
                        body: content,
                    });
                }
            }
        }
    }
    return Ok(htdx_entries);
}

fn query_htdx(query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if let Ok(entries) = load_dir(HTDX_PATH) {
        let mut search_index: SearchIndex<usize> = SearchIndex::default();
        entries
            .iter()
            .enumerate()
            .for_each(|(index, element)| search_index.insert(&index, element));

        let result_keys: Vec<&usize> = search_index.search(query);
        return Ok(result_keys
            .into_iter()
            .map(|key| entries.get(*key).unwrap().title.clone())
            .collect());
    }
    Ok(Vec::new())
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        None => {
            if let Some(filename) = cli.filename {
                let path = Path::new(HTDX_PATH).join(&filename);
                if path.exists() {
                    if let Ok(content) = fs::read_to_string(&path) {
                        println!("{}", content);
                    }
                } else {
                    println!(
                        "File {} not found, use htdx <query> + TAB for search",
                        &filename
                    );
                }
            } else {
                println!("Usage: htdx <query> + TAB for search")
            }
        }
        Some(Commands::Complete { query }) => {
            if let Ok(results) = query_htdx(&query) {
                for res in results {
                    println!("{}", res);
                }
            }
        }
    }
}
