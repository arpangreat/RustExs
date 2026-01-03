use traits_demo::Summary;
pub struct NewsAritcle {
    pub headlines: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsAritcle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headlines, self.author, self.location)
    }
}
