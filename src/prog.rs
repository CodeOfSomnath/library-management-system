//use arg::{get_args, Args};
use crate::Args;
use std::vec::Vec;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Book {
    pub name: String,
    pub author: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>,
}

#[allow(dead_code)]
impl Book {
    pub fn new(name: String, author: String) -> Self {
        Book {
            name: name,
            author: author,
        }
    }
    pub fn from(args: &Args) -> Self {
        let name: String = args.name.clone();
        let author: String = args.author.clone();
        Book {
            name: name,
            author: author,
        }
    }
}

#[allow(dead_code)]
impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }
    pub fn size(&self) -> u32 {
        self.books.len().try_into().unwrap()
    }
    pub fn add(&mut self, book: Book) {
        if book.name != "" || book.author != "" {
            self.books.push(book);
        } else {
            println!("Either name or author something is missing.\nNothing to be add..")
        }
    }
    pub fn get(&mut self, book: Book) -> Book {
        let mut count = 0;
        for b in &self.books {
            if book.name != b.name && book.author != b.author {
                count = count + 1;
            } else {
                break;
            }
        }
        if self.books.len() == count {
            println!("Book not Found...");
            Book {
                name: "None".to_string(),
                author: "None".to_string(),
            }
        } else {
            self.books.remove(count)
        }
    }
}
