use chrono::{DateTime, Utc};
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use serde_xml_rs::from_str;

type GenericResult<T> = Result<T, Box<std::error::Error>>;

#[derive(Debug, Deserialize)]
struct Link {
    title: Option<String>,
    href: String,
}

#[derive(Debug, Deserialize)]
pub struct Author {
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
    author: Vec<Author>,
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
    pub author: Vec<Author>,
    pub pdf: Option<String>,
}

impl Into<SearchResultItem> for ArXivEntry {
    fn into(self) -> SearchResultItem {
        // Duplicated field is not working at 06/04/2019 in serde-xml-rs.
        // So I make pdf link from id as a workaround.
        // I'm sorry if there are entries without pdf.
        let pdf_address = self.id.replacen("abs", "pdf", 1);
        SearchResultItem {
            id: self.id,
            updated: self.updated,
            published: self.published,
            title: self.title,
            summary: self.summary,
            author: self.author,
            pdf: Some(pdf_address),
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
        let url = format!("https://export.arxiv.org/api/query?{}", query.as_ref());
        let text = reqwest::Client::new()
            .get(url.as_str())
            .header(
                USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:66.0) Gecko/20100101 Firefox/66.0",
            )
            .send()?
            .text()?;
        Ok(from_str(text.as_str())?)
    }
}

pub fn search(query: impl AsRef<str>) -> GenericResult<Vec<SearchResultItem>> {
    Ok(Feed::get(query)?.into())
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use super::*;

    #[test]
    fn it_works() {
        let results = search("search_query=all:electron&start=0&max_results=10").expect("search failed..");
        for item in results {
            let target_url = item.pdf.unwrap();
            let res = reqwest::get(target_url.as_str()).expect("pdf download failed.");
            if res.status() != 200 {
                panic!("pdf link {} not works..", target_url);
            }
            sleep(Duration::from_secs(3));
        }
    }
}
