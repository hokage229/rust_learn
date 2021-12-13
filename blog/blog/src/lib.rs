// use std::cmp::Ordering;
//
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}

////OOP
//
// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State>;
// }
//
// struct Draft {}
//
// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview { count_to_publish: 0 })
//     }
//
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }
//
// struct PendingReview {
//     count_to_publish: u32,
// }
//
// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//
//     fn approve(mut self:  Box<Self>) -> Box<dyn State> {
//         self.count_to_publish += 1;
//         match self.count_to_publish.cmp(&2) {
//             Ordering::Less => {
//                 self
//             }
//             Ordering::Equal => {
//                 Box::new(Published {})
//             }
//             Ordering::Greater => {
//                 Box::new(Published {})
//             }
//         }
//     }
//
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Draft {})
//     }
// }
//
// struct Published {}
//
// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
//
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }
//
// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }
//
// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }
//
//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }
//
//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }
//
//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }
//
//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }
//
//     pub fn reject(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.reject())
//         }
//     }
// }
//
//
