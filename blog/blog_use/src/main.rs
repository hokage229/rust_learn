use blog::Post;

fn main() {
    ////OOP
    // let mut post = Post::new();
    //
    // post.add_text("I ate a salad for lunch today");
    // post.approve();
    // post.approve();
    // assert_eq!("", post.content());
    //
    //
    // post.request_review();
    // assert_eq!("", post.content());
    //
    // post.reject();
    // post.request_review();
    //
    // post.approve();
    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
