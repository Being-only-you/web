use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser, Tag, Event};
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

pub fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::all());

    let parser = Parser::new_ext(&content, options).map(|event| {
        match &event {
            Event::Start(tag) => match tag {
                Tag::HtmlBlock => println!("HtmlBlock"),
                Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                } => println!(
                    "Heading heading_level: {} fragment identifier: {:?} classes: {:?} attrs: {:?}",
                    level, id, classes, attrs
                ),
                Tag::Paragraph => println!("Paragraph"),
                Tag::List(ordered_list_first_item_number) => println!(
                    "List ordered_list_first_item_number: {:?}",
                    ordered_list_first_item_number
                ),
                Tag::DefinitionList => println!("Definition list"),
                Tag::DefinitionListTitle => println!("Definition title (definition list item)"),
                Tag::DefinitionListDefinition => println!("Definition (definition list item)"),
                Tag::Item => println!("Item (this is a list item)"),
                Tag::Emphasis => println!("Emphasis (this is a span tag)"),
                Tag::Strong => println!("Strong (this is a span tag)"),
                Tag::Strikethrough => println!("Strikethrough (this is a span tag)"),
                Tag::BlockQuote(kind) => println!("BlockQuote ({:?})", kind),
                Tag::CodeBlock(code_block_kind) => {
                    println!("CodeBlock code_block_kind: {:?}", code_block_kind)
                }
                Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                } => println!(
                    "Link link_type: {:?} url: {} title: {} id: {}",
                    link_type, dest_url, title, id
                ),
                Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                } => println!(
                    "Image link_type: {:?} url: {} title: {} id: {}",
                    link_type, dest_url, title, id
                ),
                Tag::Table(column_text_alignment_list) => println!(
                    "Table column_text_alignment_list: {:?}",
                    column_text_alignment_list
                ),
                Tag::TableHead => println!("TableHead (contains TableRow tags"),
                Tag::TableRow => println!("TableRow (contains TableCell tags)"),
                Tag::TableCell => println!("TableCell (contains inline tags)"),
                Tag::FootnoteDefinition(label) => println!("FootnoteDefinition label: {}", label),
                Tag::MetadataBlock(kind) => println!("MetadataBlock: {:?}", kind),
            },
            _ => (),
        };
        event
    });

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
