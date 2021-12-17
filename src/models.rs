use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ArticleData {
    pub article_title: String,
    pub url_link: String,
    pub domain_name: String,
}
