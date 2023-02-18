use super::{ContentsFormat, FrontMatter};

pub struct HugoRobust {}
impl ContentsFormat for HugoRobust {
    fn new() -> Self {
        Self {}
    }

    fn get_articles_dirname(&self) -> &str {
        return "content";
    }

    fn get_images_dirname(&self) -> &str {
        return "assets";
    }

    fn get_front_matter_delimiter(&self) -> &str {
        return "+++";
    }

    fn format_front_matter(&self, front_matters: FrontMatter) -> String {
        return format!(
            "
+++
date = \"{date}\"
title = \"{title}\"
thumbnail = \"\"
+++
",
            date = front_matters.published_at,
            title = front_matters.title
        );
    }

    fn parse_front_matter(&self, front_matter_str: String) -> FrontMatter {
        todo!()
    }
}
