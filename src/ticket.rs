use std::fmt;
use std::io::Read;
use std::fs::File;
use std::vec::Vec;
use std::collections::HashMap;
use std::option::Option;

#[derive(Debug)]
pub struct Ticket {
    title: String,
    description: String,
    tags: Vec<String>,
    fields: HashMap<String, String>,
}

fn extract_all_fields(contents: &str) -> Option<Ticket> {
    if let Some(title_start) = contents.find("#") {
        if let Some(title_end) = contents[title_start..].find('\n') {
            let title = contents[title_start + 1..title_start + title_end].trim();

            if let Some(tags_start) = contents[title_start + title_end..].find('\n') {
                let tags_line = contents[title_start + title_end..title_start + title_end + tags_start].trim();
                let tags: Vec<String> = tags_line.split(',').map(|tag| tag.trim().to_string()).collect();

                let fields_start = title_start + title_end + tags_start + 1;
                if let Some(fields_end) = contents[fields_start..].find("\n\n") {
                    let fields_block = contents[fields_start..fields_start + fields_end].trim();
                    let fields: HashMap<String, String> = fields_block
                        .lines()
                        .filter_map(|line| {
                            let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
                            if parts.len() == 2 {
                                Some((parts[0].to_string(), parts[1].to_string()))
                            } else {
                                None
                            }
                        })
                        .collect();

                    let description_start = fields_start + fields_end + 2;
                    let description = contents[description_start..].trim().to_string();

                    return Some(Ticket {
                        title: title.to_string(),
                        tags,
                        fields,
                        description,
                    });
                }
            }
        }
    }
    None
}

impl Ticket {
    pub fn from_file(mut ticket_file: File) -> Result<Self,()> {
        let mut contents = String::new();
        let result: Self;
        match ticket_file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(err) => {
                println!("Ticket file could not be read: {err}");
                return Err(());
            }
        }
        let maybe_result = extract_all_fields(contents.as_str());
        match maybe_result {
            Some(some_result) => {
                result = some_result;
            },
            None => {
                println!("Error: invalid format of ticket file {:?}", ticket_file);
                return Err(());
            }
        }


        Ok(result)
    }
}


impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nTags: {}\nFields: {:#?}\nDescription: {}",
            self.title,
            self.tags.join(", "),
            self.fields,
            self.description
        )
    }
}

