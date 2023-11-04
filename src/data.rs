use chrono::FixedOffset;
use markdown::{to_html_with_options, CompileOptions, Constructs, Options, ParseOptions};
use perseus::ReactiveState;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{DirEntry, File};
use std::io;
use std::io::Read;
use std::path::Path;

pub fn get_blog_directories() -> Vec<String> {
    let listing: io::Result<Vec<DirEntry>> = Path::new("content/blog")
        .read_dir()
        .expect("cannot read blog directory")
        .collect(); // honestly just writing it this way for nifty collect trick example
    listing
        .expect("cannot read blog entry")
        .into_iter()
        .filter_map(|d| {
            if d.file_type().unwrap().is_dir() {
                Some(d.file_name().into_string().unwrap())
            } else {
                None
            }
        })
        .collect()
}

#[derive(Serialize, Deserialize, Clone, ReactiveState, PartialEq)]
#[rx(alias = "PostRx")]
pub struct Post {
    pub title: String,
    pub date: chrono::DateTime<FixedOffset>,
    pub description: Option<String>,
    pub html: String,
    pub path: String,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: chrono::DateTime<FixedOffset>,
    pub description: Option<String>,
    pub image: Option<String>,
}

pub fn get_front_matter(contents: &str) -> FrontMatter {
    if let Some(("", rest)) = contents.split_once("---") {
        // Parse front matter.
        if let Some((front_matter_str, _body_str)) = rest.split_once("---") {
            return serde_yaml::from_str(front_matter_str).expect("cannot parse front matter");
        }
    }
    panic!("front matter missing");
}

pub fn get_post_for_path(path: &String) -> Post {
    let mut file = File::open(Path::new("content/blog").join(path).join("index.md"))
        .expect("cannot open blog md");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read blog md");

    let front_matter = get_front_matter(&contents);

    let html = to_html_with_options(
        &contents,
        &Options {
            parse: ParseOptions {
                constructs: Constructs {
                    frontmatter: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..Default::default()
            },
        },
    )
    .expect("cannot render post html");

    let image = front_matter.image.or_else(|| {
        let re = Regex::new(r#"src="([^"]+)""#).unwrap();
        re.captures(&html).map(|img| img[1].to_string())
    });

    // whatever convert it twice. Should probably just manually pull the yaml instead.
    Post {
        path: path.clone(),
        title: front_matter.title,
        date: front_matter.date,
        description: front_matter.description,
        html,
        image,
    }
}
