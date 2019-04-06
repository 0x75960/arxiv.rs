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

#[derive(Clone)]
pub struct QueryBuilder {
    search_query: Vec<String>,
    start: usize,
    max_result: usize,
}

impl QueryBuilder {
    pub fn new() -> Self {
        QueryBuilder {
            search_query: vec![],
            start: 0,
            max_result: 10,
        }
    }

    pub fn add_search_query(mut self, query: impl AsRef<str>) -> Self {
        self.search_query.push(query.as_ref().to_owned());
        self
    }

    pub fn set_start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }

    pub fn set_max_result(mut self, max_result: usize) -> Self {
        self.max_result = max_result;
        self
    }

    fn query(&self) -> String {
        format!("search_query={}&start={}&max_results={}", self.search_query.join("+"), self.start, self.max_result)
    }

    pub fn search(&self) -> GenericResult<Vec<SearchResultItem>> {
        Ok(Feed::get(self.query())?.into())
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use super::*;

    #[test]
    fn query_works() {
        let q = QueryBuilder::new()
            .add_search_query("all:electron")
            .set_start(1)
            .set_max_result(11)
            .query();

        assert_eq!("search_query=all:electron&start=1&max_results=11", q);

        let q = QueryBuilder::new()
            .add_search_query("cat:cs.CR")
            .add_search_query(r#""machine learning""#)
            .query();

        assert_eq!(r#"search_query=cat:cs.CR+"machine learning"&start=0&max_results=10"#, q);
    }

    #[test]
    fn search_works() {
        let results = QueryBuilder::new()
            .add_search_query("cat:cs.CR")
            .add_search_query(r#""machine learning""#)
            .set_start(0)
            .set_max_result(3)
            .search()
            .expect("failed to get feed");

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
