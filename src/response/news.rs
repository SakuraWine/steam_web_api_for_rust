use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct News {
    pub appnews: AppNews
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AppNews {
    pub appid: i32,
    pub newsitems: Vec<NewsItem>,
    pub count: i32
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NewsItem {
    pub gid: String,
    pub title: String,
    pub url: String,
    pub is_external_url: bool,
    pub author: String,
    pub contents: String,
    pub feedlabel: String,
    pub date: i32,
    pub feedname: String,
    pub feed_type: i16,
    pub appid: i32,
    tags: Option<Vec<String>>
}