use std::env;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use time::{format_description, OffsetDateTime};
use walkdir::{DirEntry, WalkDir};

fn generate_index_content(root_path: &str) -> String {
    let mut index_content = String::from("# Index\n");

    for entry in WalkDir::new(root_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_markdown_file)
    {
        if let Ok(relative_path) = entry.path().strip_prefix(root_path) {
            let relative_path = relative_path.display().to_string();
            // TODO: Exception handling
            let last_changed = entry.metadata().unwrap().modified().unwrap();

            index_content.push_str(&format!(
                "- [{}]({})  <sub>Last update: {:?}</sub>  \n",
                relative_path,
                relative_path,
                format_date(last_changed)
            ));

            println!("📄 {} ", relative_path);
        }
    }

    index_content.push_str("\n\n *This content is auto-generated and will be overwritten so please don't make manual changes to it.* \n");

    index_content
}

fn format_date(time: SystemTime) -> String {
    let date_format =
        format_description::parse("[year]-[month]-[day]-[hour]:[minute]:[second]").unwrap();
    let offset_date_time: OffsetDateTime = time.into();

    offset_date_time.format(&date_format).unwrap()
}

fn is_markdown_file(entry: &DirEntry) -> bool {
    if let Some(extension) = entry.path().extension() {
        return extension.to_string_lossy().to_lowercase() == "md";
    }
    false
}

fn write_index(content: &str, index_file_path: &str) {
    let mut index_file = File::create(index_file_path).expect("Failed to create index file");
    index_file
        .write_all(content.as_bytes())
        .expect("Failed to write index content to file");
}

fn main() {
    let root_path = env::args().nth(1).unwrap_or(".".to_string());
    let index_file_name = env::args()
        .nth(2)
        .unwrap_or("index.md".to_string())
        .to_owned();

    let index_content = generate_index_content(&root_path);
    // Interesting: this line can't be above this ☝️ because this line 👇 'moves' out the data from root_path to index_file_path
    let index_file_path = root_path + "/" + index_file_name.as_str();

    write_index(&index_content, index_file_path.as_str());

    println!("✅  Index generated and written to: {}", index_file_path)
}
