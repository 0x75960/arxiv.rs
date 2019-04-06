use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_xml_rs::from_str;

type GenericResult<T> = Result<T, Box<std::error::Error>>;

#[derive(Debug, Deserialize)]
struct Link {
    title: Option<String>,
    href: String,
}

#[derive(Debug, Deserialize)]
pub struct ArXivAuthor {
    pub name: String,
    pub affiliation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ArXivEntry {
    id: String,
    updated: DateTime<Utc>,
    published: DateTime<Utc>,
    title: String,
    summary: String,
    author: Vec<ArXivAuthor>,
    // track: https://github.com/RReverser/serde-xml-rs/issues/55
    //link: Vec<Link>,
}

#[derive(Debug, Deserialize)]
pub struct Feed {
    entry: Vec<ArXivEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResultItem {
    pub id: String,
    pub updated: DateTime<Utc>,
    pub published: DateTime<Utc>,
    pub title: String,
    pub summary: String,
    pub author: Vec<ArXivAuthor>,
    pub pdf: Option<String>,
}

impl Into<SearchResultItem> for ArXivEntry {
    fn into(self) -> SearchResultItem {
        SearchResultItem {
            id: self.id,
            updated: self.updated,
            published: self.published,
            title: self.title,
            summary: self.summary,
            author: self.author,
            pdf: None,
            /*
            pdf: self.link.into_iter()
                .find(|x| x.title == Some("pdf".to_string()))
                .and_then(|x| Some(x.href)),
                */
        }
    }
}

impl Into<Vec<SearchResultItem>> for Feed {
    fn into(self) -> Vec<SearchResultItem> {
        self.entry.into_iter().map(|x| x.into()).collect()
    }
}

impl Feed {
    fn get(query: impl AsRef<str>) -> GenericResult<Self> {
        let url = format!("https://export.arxiv.org/api/query?search_query={}", query.as_ref());
        let text = reqwest::get(url.as_str())?
            .text()?;
        Ok(from_str(text.as_str())?)
    }
}

pub fn search(query: impl AsRef<str>) -> GenericResult<Vec<SearchResultItem>> {
    Ok(Feed::get(query)?.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        search("all:electron&start=0&max_results=10").expect("search failed..");
    }
}
