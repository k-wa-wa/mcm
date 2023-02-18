use super::{ContentsFormat, FrontMatter};

use std::collections::HashMap;

use regex::Regex;

pub struct Zenn {}
impl ContentsFormat for Zenn {
    fn new() -> Self {
        return Self {};
    }

    fn get_articles_dirname(&self) -> &str {
        return "";
    }

    fn get_images_dirname(&self) -> &str {
        return "images";
    }

    fn get_front_matter_delimiter(&self) -> &str {
        return "---";
    }

    fn is_local_image(&self, image_link: String) -> bool {
        return image_link.starts_with(&format!("/images"));
    }

    fn parse_front_matter(&self, front_matter_str: String) -> FrontMatter {
        let mut regex_map = HashMap::new();
        regex_map.insert("title", r#"\ntitle: "(.*?)"\n"#);
        regex_map.insert("emoji", r#"\nemoji: "(.*?)"\n"#);
        regex_map.insert("type", r#"\ntype: "(.*?)"\n"#);
        regex_map.insert("topics", r"\ntopics: (.*?)\n");
        regex_map.insert("published", r"\npublished: (.*?)\n");
        regex_map.insert("published_at", r"\npublished_at: (.*?)\n");
        let mut front_matters: HashMap<String, String> = HashMap::new();
        for (k, reg) in &regex_map {
            match Regex::new(reg).unwrap().captures(&front_matter_str) {
                Some(caps) => {
                    if caps.len() >= 1 {
                        front_matters
                            .insert(k.to_string(), caps.get(1).unwrap().as_str().to_string());
                    } else {
                        front_matters.insert(k.to_string(), "".to_string());
                    }
                }
                None => (),
            }
        }
        return FrontMatter::new(front_matters);
    }

    fn format_front_matter(&self, front_matters: FrontMatter) -> String {
        todo!()
    }

    fn format_image_path(&self, filename: &str) -> String {
        todo!()
    }
}
