use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PostStatus {
    Draft,
    Review,
    Published,
}

impl Default for PostStatus {
    fn default() -> Self {
        PostStatus::Draft
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct PostMetadata {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub status: PostStatus,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub cover: String,
}

#[derive(Clone, Debug)]
pub struct PostData {
    pub metadata: PostMetadata,
    pub content: String,
}

fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    html
}

#[derive(Debug, Default)]
pub struct StatusFilter {
    pub include: Option<Vec<PostStatus>>,
    pub exclude: Option<Vec<PostStatus>>,
    pub only: Option<PostStatus>,
}

pub fn get_all_posts(filter: StatusFilter) -> HashMap<String, PostData> {
    use include_dir::*;
    static POST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/posts");

    POST_DIR
        .files()
        .map(|post| -> (String, PostData) {
            let matter = Matter::<YAML>::new();
            let markdown = matter.parse(post.contents_utf8().unwrap());
            let front_matter: PostMetadata = markdown.data.unwrap().deserialize().unwrap();

            (
                post.path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                PostData {
                    metadata: front_matter,
                    content: markdown_to_html(markdown.content),
                },
            )
        })
        .filter(|(_id, post)| {
            let status = &post.metadata.status;

            let include_filter = filter.include.as_ref()
                .map_or(true, |include_list| include_list.contains(status));
            
            let exclude_filter = filter.exclude.as_ref()
                .map_or(true, |exclude_list| !exclude_list.contains(status));
            
            let only_filter = filter.only.as_ref()
                .map_or(true, |only_status| only_status == status);

            include_filter && exclude_filter && only_filter
        })
        .collect()
}


pub fn get_all_tags() -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();

    get_all_posts(StatusFilter { include: None, exclude: None, only: None }).values().for_each(|post| {
        post.metadata.tags.iter().for_each(|tag| {
            if !tags.contains(tag) {
                tags.push(tag.clone());
            }
        })
    });

    tags.sort();

    tags
}

pub fn get_posts_by_tag(tag: String) -> HashMap<String, PostData> {
    get_all_posts(StatusFilter { include: None, exclude: None, only: None })
        .into_iter()
        .filter(|(_id, post)| post.metadata.tags.contains(&tag))
        .collect()
}

pub fn get_post(id: String) -> PostData {
    get_all_posts(StatusFilter { include: None, exclude: None, only: None }).get(&id).cloned().unwrap()
}
