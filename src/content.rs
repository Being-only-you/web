use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag};
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
    pub cover: Option<String>,
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
                Tag::HtmlBlock => Event::Html("<div class=\"prose prose-lg\">".into()),
                Tag::Heading { level, .. } => {
                    let heading_class = match level {
                        HeadingLevel::H1 => "text-4xl font-bold mb-6 mt-8",
                        HeadingLevel::H2 => "text-3xl font-semibold mb-5 mt-7",
                        HeadingLevel::H3 => "text-2xl font-medium mb-4 mt-6",
                        HeadingLevel::H4 => "text-xl font-medium mb-3 mt-5",
                        HeadingLevel::H5 => "text-lg font-medium mb-2 mt-4",
                        HeadingLevel::H6 => "text-base font-medium mb-2 mt-3",
                    };
                    Event::Html(format!("<h{} class=\"{}\">", *level as u8, heading_class).into())
                }
                Tag::Paragraph => Event::Html("<p class=\"text-base leading-relaxed mb-4\">".into()),
                Tag::List(Some(_)) => Event::Html("<ol class=\"list-decimal list-inside mb-4 pl-6\">".into()),
                Tag::List(None) => Event::Html("<ul class=\"list-disc list-inside mb-4 pl-6\">".into()),
                Tag::Item => Event::Html("<li class=\"mb-2\">".into()),
                Tag::Emphasis => Event::Html("<em class=\"italic\">".into()),
                Tag::Strong => Event::Html("<strong class=\"font-semibold\">".into()),
                Tag::Strikethrough => Event::Html("<s class=\"line-through\">".into()),
                Tag::BlockQuote(_) => Event::Html("<blockquote class=\"border-l-4 border-honest-300 pl-4 italic mb-4\">".into()),
                Tag::CodeBlock(_) => Event::Html("<pre class=\"bg-honest-200 p-4 rounded-lg mb-4 overflow-x-auto\"><code>".into()),
                Tag::Link { .. } => Event::Html("<a class=\"text-social-500 hover:underline\">".into()),
                Tag::Image { .. } => Event::Html("<img class=\"rounded-lg mb-4 max-w-full h-auto\" />".into()),  // Self-closing
                Tag::Table(_) => Event::Html("<table class=\"table-auto w-full text-left\">".into()),
                Tag::TableHead => Event::Html("<thead class=\"bg-honest-200\">".into()),
                Tag::TableRow => Event::Html("<tr class=\"border-b\">".into()),
                Tag::TableCell => Event::Html("<td class=\"px-4 py-2\">".into()),
                Tag::FootnoteDefinition(_) => Event::Html("<div class=\"footnote-definition mb-4\">".into()),
                _ => event,
            }
            _ => event,
        }
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

            // Simplify the filter checks for include, exclude, and only
            let include_filter = filter.include.as_ref().map_or(true, |include_list| include_list.contains(status));
            let exclude_filter = filter.exclude.as_ref().map_or(true, |exclude_list| !exclude_list.contains(status));
            let only_filter = filter.only.as_ref().map_or(true, |only_status| *status == *only_status);

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
