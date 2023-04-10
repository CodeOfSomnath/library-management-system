use std::vec::Vec;

#[derive(Debug)]
pub struct Book {
    pub name: String,
    pub author: String,
}

#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>,
}

// Here a new function is add
impl Book {
    pub fn new(name: String, author: String) -> Self {
        Book { name, author }
    }
}

impl Library {
    // This function return a new instance
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }
    // This function return how many books in a library
    pub fn size(&self) -> u32 {
        self.books.len().try_into().unwrap()
    }
    // This function add a book to Library
    pub fn add(&mut self, book: Book) {
        if book.name != "" || book.author != "" {
            self.books.push(book);
        } else {
            println!("Either name or author something is missing.\nNothing to be add..")
        }
    }
    // This function take a book name and author and return the book from library
    pub fn get(&mut self, book: Book) -> Book {
        let mut count = 0;
        for b in &self.books {
            if book.name != b.name || book.author != b.author {
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
    // Removes unused memory of vector struct
    pub fn fit(&mut self) {
        self.books.shrink_to_fit();
    }
}
