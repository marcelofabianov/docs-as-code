use clap::{Parser, Subcommand};
use comfy_table::Table;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use termimad::MadSkin;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Frontmatter {
    title: Option<String>,
}

#[derive(Debug)]
struct Document {
    id: String,
    title: String,
    file_name: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    List {
        #[arg(default_value_t = String::from("."))]
        path: String,
    },
    View {
        id: String,
        #[arg(default_value_t = String::from("."))]
        path: String,
    },
}

fn parse_document(content: &str) -> (Option<Frontmatter>, &str) {
    if content.starts_with("---") {
        if let Some(end_pos) = content.get(4..).unwrap_or("").find("---") {
            let frontmatter_str = &content[4..end_pos + 4];
            let markdown_content = &content[end_pos + 4 + 3..];
            match serde_yaml::from_str(frontmatter_str) {
                Ok(frontmatter) => (Some(frontmatter), markdown_content.trim_start()),
                Err(_) => (None, content),
            }
        } else {
            (None, content)
        }
    } else {
        (None, content)
    }
}

fn extract_title<'a>(
    frontmatter: &'a Option<Frontmatter>,
    markdown_content: &'a str,
    file_path: &'a Path,
) -> String {
    if let Some(fm) = frontmatter {
        if let Some(title) = &fm.title {
            return title.clone();
        }
    }
    if let Some(line) = markdown_content.lines().next() {
        if line.starts_with("# ") {
            return line.trim_start_matches("# ").to_string();
        }
    }
    file_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

fn run_view_command(path_str: &str, id: &str) -> Result<(), String> {
    for entry in WalkDir::new(path_str).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    let current_id = path.file_stem().unwrap_or_default().to_string_lossy();
                    if current_id == id {
                        let content = fs::read_to_string(path)
                            .map_err(|e| format!("Failed to read file: {}", e))?;
                        let (_frontmatter, markdown_content) = parse_document(&content);
                        let skin = MadSkin::default();
                        skin.print_text(markdown_content);
                        return Ok(());
                    }
                }
            }
        }
    }
    Err(format!("Document with ID '{}' not found.", id))
}

fn main() {
    let args = Args::parse();

    let command = args.command.unwrap_or(Command::List {
        path: ".".to_string(),
    });

    let result = match command {
        Command::List { path } => {
            let mut documents: Vec<Document> = Vec::new();
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "md" {
                            if let Ok(content) = fs::read_to_string(path) {
                                let (frontmatter, markdown_content) = parse_document(&content);
                                let title = extract_title(&frontmatter, markdown_content, path);
                                let id = path
                                    .file_stem()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();
                                let file_name = path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();
                                documents.push(Document {
                                    id,
                                    title,
                                    file_name,
                                });
                            }
                        }
                    }
                }
            }
            let mut table = Table::new();
            table.set_header(vec!["ID", "Título", "Nome do Arquivo"]);
            for doc in documents {
                table.add_row(vec![doc.id, doc.title, doc.file_name]);
            }
            println!("{table}");
            Ok(())
        }
        Command::View { id, path } => run_view_command(&path, &id),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
