trait Summarizable {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}
impl Summarizable for Article {
    fn summarize(&self) -> String {
        format!("title: {} - content{}", self.title, self.content)
    }
}

struct Articles<S: Summarizable> {
    articles: Vec<S>,
}

impl Articles<Article> {
    fn add(&mut self, article: Article) {
        self.articles.push(article);
    }

    fn summarize_all(&self) {
        for article in &self.articles {
            println!("{}", article.summarize());
        }
    }
}

fn main() {
    let article = Article{ title: "test_title".to_string(), content: "test_content".to_string() };

    let mut  articles = Articles{ articles: vec![] };
    articles.add(article);

    articles.summarize_all();
}