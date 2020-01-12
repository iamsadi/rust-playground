extern crate news_contract;
extern crate postgres;

use news_contract::News;
use postgres::{Client,NoTls};

pub fn connect() -> postgres::Client {
  Client::connect("postgres://postgres:docker@172.17.0.2:5432/postgres", NoTls).unwrap()
}

pub fn list_news() -> Option<Vec<News>> {
  //let mut client = connect();
  let mut vec_news = Vec::new();
  /*
  for row in &client.query("SELECT * FROM news", &[]).unwrap() {
    let news = News {
        id: row.get(0),
        desc: row.get(1),
        url: row.get(2),
    };
    vec_news.push(news);
  }*/
  vec_news.push(News {
    id: String::from("1234"),
    desc: String::from("google"),
    url: String::from("google.com")
  });
  return Some(vec_news);
}