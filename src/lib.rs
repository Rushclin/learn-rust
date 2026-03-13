
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
}
pub struct SocialPost {
    pub title: String,
    pub headline: String,
    pub content: String,
    pub author: String,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String{
        format!("{} {} {}", self.title, self.headline, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}