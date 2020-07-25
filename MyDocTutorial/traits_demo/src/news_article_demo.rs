pub fn run() {
    let article = NewsArticle {
        headline: String::from("India has won FIFA World Cup!!"),
        location: String::from("Kolkata, West Bengal, India"),
        author: String::from("Swastik Acharyya"),
        content: String::from(
            "India has successfully won FIFA World Cup in my dreams.
            Hope They will going many things later."
        ),
    };

    println!("New article available!, {}", article.summarize());
}
