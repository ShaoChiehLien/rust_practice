// mod traits_lib means declaring a module under traits named traits_lib
// traits_lib would need to be organized like this traits/traits_lib.rs
mod traits_lib; 

// relative "use" start from traits_lib since its namespace is brought 
// into scope above
use traits_lib::{Summary, Ad, Tweet, NewsArticle}; 
// traits like Summary and Ad needs to be brought into scope so we 
// could call it, e.g. tweet.summarize()

fn implement_traits_on_type() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    // Summary has no default implementation
    println!("Summarize: {:?}", tweet.summarize());
    // advertisement has default implmentation but is overwrite by tweet
    println!("Ad: {:?}", tweet.advertisement());

    let news_article = NewsArticle {
        headline: String::from("johnny depp and amber heard"),
        location: String::from("new york"),
        author: String::from("jack"),
        content: String::from("jonny depp won the case!")
    };

    // Summary has no default implementation
    println!("Summarize: {:?}", news_article.summarize());
    // can still be called even though there is no implementation on advertisement
    println!("Ad: {:?}", news_article.advertisement());
}

/************** impl trait vs trait bound **************/
fn notify_with_impl_trait(item: &impl Summary) {
    println!("Breaking news!: {}", item.summarize());
}

fn notify_with_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news!: {}", item.summarize());
}

fn trait_as_parameter() {
    let news_article = NewsArticle {
        headline: String::from("johnny depp and amber heard lawsuit"),
        location: String::from("new york"),
        author: String::from("jack"),
        content: String::from("jonny depp won the case!")
    };

    println!("Use impl trait, a syntax sugar for trait bound");
    notify_with_impl_trait(&news_article);

    println!("Use trait bound, a complex version of impl trait");
    notify_with_trait_bound(&news_article);

    println!("The output is the same");

    // let int_value = 100;
    // notify(&int_value); // Error: trait is n ot implemented for i32
}
/************ End of impl trait vs trait bound ************/

/****** Multiple impl trait vs multiple trait bound ******/
fn notify_multiple_impl_trait(item: &(impl Summary + Ad)) {
    println!("Breaking news!: {}, sponsored by {}", 
                item.summarize(), item.advertisement());
}

fn notify_multiple_trait_bound<T: Summary + Ad>(item: &T) {
    println!("Breaking news!: {}, sponsored by {}", 
                item.summarize(), item.advertisement());
}

fn multiple_trait_bound() {
    let news_article = NewsArticle {
        headline: String::from("johnny depp and amber heard lawsuit"),
        location: String::from("new york"),
        author: String::from("jack"),
        content: String::from("jonny depp won the case!")
    };

    println!("Use impl trait that specify multiple traits that need to have");
    notify_multiple_impl_trait(&news_article);
    println!("Use trait bound that specify multiple traits that need to have");
    notify_multiple_trait_bound(&news_article);
    println!("The output is the same");
}
/*** End of multiple impl trait vs multiple trait bound ***/


/** Trait bound with where clauses vs without where clauses **/
fn combine_tweet_and_article_without_where<T: Ad + Summary, U: Ad + Summary>(t: &T, u: &U) -> String {
    format!("tweet: {} Ad: {}\narticle: {} Ad: {}",
             t.summarize(), t.advertisement(), u.summarize(), u.advertisement())
}

fn combine_tweet_and_article_with_where<T, U>(t: &T, u: &U) -> String 
    where T: Ad + Summary, 
          U: Ad + Summary
{
    format!("tweet: {} Ad: {}\narticle: {} Ad: {}",
             t.summarize(), t.advertisement(), u.summarize(), u.advertisement())
}

fn complex_trait_bound_with_where_clauses() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        headline: String::from("johnny depp and amber heard"),
        location: String::from("new york"),
        author: String::from("jack"),
        content: String::from("jonny depp won the case!")
    };

    println!("Not using where clauses for clearer traits bounds");
    println!("{}", combine_tweet_and_article_without_where(&tweet, &news_article));

    println!("Use where clauses for clearer traits bounds");
    println!("{}", combine_tweet_and_article_with_where(&tweet, &news_article));

    println!("The output is the same");
}
/** End of trait bound with where clauses vs without where clauses **/


/************** Return type with impl trait ****************/
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("johnny depp and amber heard"),
        location: String::from("new york"),
        author: String::from("jack"),
        content: String::from("jonny depp won the case!")
    }
}

fn return_types_that_impl_traits() {
    let article = returns_summarizable();
    println!("Summarize: {}", &article.summarize());
}
/********** End of return type with impl trait ************/

pub fn run() {
    println!("Implement traits on type: ");
    implement_traits_on_type();

    println!("\nTrait as a parameter ");
    trait_as_parameter();

    println!("\nMultiple trait bounds ");
    multiple_trait_bound();

    println!("\nUse where clauses for clearer trait bound");
    complex_trait_bound_with_where_clauses();

    println!("\nReturn type with impl trait: ");
    return_types_that_impl_traits();
}