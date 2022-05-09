// trait Summary has no default implementation
pub trait Summary {
    // describe the behaviors of the types that implement this trait
    fn summarize(&self) -> String;
}

// trait Ad has default implementation
pub trait Ad {
    fn advertisement(&self) -> String {
        format!("Check out Apple's new watch!")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Still need to specify NewsArticle implement Ad trait
// implement Ad for NewsArticle can be empty since there is a default implementation
impl Ad for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// fill the ad function to overwrite the default implementation
impl Ad for Tweet {
    fn advertisement(&self) -> String {
        format!("Switch to Ad exclusive from Tweet!")
    }
}