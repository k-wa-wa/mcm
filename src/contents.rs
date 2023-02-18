use std::{collections::HashMap, fs, fs::File, io::Write, path::Path};

use comrak::{
    format_commonmark,
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use glob::glob;

pub mod hugo_robust;
use self::hugo_robust::HugoRobust;
pub mod zenn;
use self::zenn::Zenn;

use super::Format;

struct ContentsTransformer<Source: ContentsFormat, Target: ContentsFormat> {
    source: Source,
    target: Target,
    source_dirpath: String,
    target_dirpath: String,
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

impl<From: ContentsFormat, To: ContentsFormat> ContentsTransformer<From, To> {
    fn transform_body(&self, file_body: String) -> String {
        let arena = Arena::new();
        let mut options = ComrakOptions::default();
        options.extension.tasklist = true;
        options.extension.front_matter_delimiter =
            Some(self.source.get_front_matter_delimiter().to_string());

        let root = parse_document(&arena, &file_body, &options);

        iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
            &mut NodeValue::Text(ref mut text) => {
                let orig = std::mem::replace(text, vec![]);
                *text = String::from_utf8(orig)
                    .unwrap()
                    .replace("my", "your")
                    .as_bytes()
                    .to_vec();
            }
            &mut NodeValue::FrontMatter(ref mut text) => {
                let front_matter_str = String::from_utf8(text.to_vec()).unwrap();
                let front_matters = self.source.parse_front_matter(front_matter_str);
                *text = self
                    .target
                    .format_front_matter(front_matters)
                    .as_bytes()
                    .to_vec();
            }
            _ => (),
        });

        let mut out = vec![];
        format_commonmark(root, &options, &mut out).unwrap();

        return String::from_utf8(out).unwrap();
    }

    fn export_contents(&self) {
        // articles
        for source_filepath in glob(
            Path::new(&self.source_dirpath)
                .join(self.source.get_articles_dirname())
                .join("**/*.md")
                .to_str()
                .unwrap(),
        )
        .unwrap()
        .map(|e| e.unwrap())
        {
            let file_body = fs::read_to_string(&source_filepath).unwrap();
            let source_filename = source_filepath.file_name().unwrap().to_str().unwrap();

            let target_dirpath =
                Path::new(&self.target_dirpath).join(self.target.get_articles_dirname());
            fs::create_dir_all(&target_dirpath).unwrap();
            let mut file = File::create(target_dirpath.join(source_filename)).unwrap();
            file.write_all(self.transform_body(file_body).as_bytes())
                .unwrap();
        }
        // images
    }
}

trait ContentsFormat {
    fn new() -> Self;

    fn get_articles_dirname(&self) -> &str;
    fn get_images_dirname(&self) -> &str;
    fn get_front_matter_delimiter(&self) -> &str;

    fn parse_front_matter(&self, front_matter_str: String) -> FrontMatter;
    fn format_front_matter(&self, front_matters: FrontMatter) -> String;
}

struct FrontMatter {
    title: String,
    published_at: String,
    custom: HashMap<String, String>,
}

impl FrontMatter {
    fn new(mut map: HashMap<String, String>) -> Self {
        let mut title = String::new();
        let mut published_at = String::new();

        match map.get("title") {
            Some(_t) => {
                title = _t.to_string();
                map.remove("title");
            }
            None => (),
        }
        match map.get("published_at") {
            Some(_p) => {
                published_at = _p.to_string();
                map.remove("published_at");
            }
            None => (),
        }

        return Self {
            title,
            published_at,
            custom: map,
        };
    }
}

pub fn export_contents(
    source: Format,
    target: Format,
    source_dirpath: String,
    target_dirpath: String,
) {
    if source == Format::Zenn && target == Format::HugoRobust {
        let c = ContentsTransformer {
            source: Zenn::new(),
            target: HugoRobust::new(),
            source_dirpath,
            target_dirpath,
        };
        c.export_contents();
    } else {
        panic!("unimplemented source_to_target");
    }
}
