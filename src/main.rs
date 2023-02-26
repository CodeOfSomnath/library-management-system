mod parser;
mod prog;
use parser::*; // Here a simple user input parser
use prog::*; // program main structure is here
// This is for checking which method is performed
enum Out {
    Get(Book),
    Err,
    Add,
}
// This function will continually run the create, update command for my library
fn run(lib: &mut Library, cmd: &String) -> Out {
    let args = get_args();
    let book = Book::new(args.name, args.author);
    if cmd == "1" {
        lib.add(book);
        return Out::Add;
    } else if cmd == "2" {
        return Out::Get(lib.get(book));
    } else {
        return Out::Err;
    }
}
// Here the user available command prompt
fn show() {
    println!("Enter following command: ");
    println!("\t1. Add \n\t2. Get\n\t3. Exit\n\t4. Show\n\t5. Size\n\t6. Show Commands");
}
fn main() {
    let mut lib = Library::new();
    show();
    loop {
        let ex = input(">>> ");
        if ex == "3" {
            break;
        } else if ex == "4" {
            println!("{:#?}", lib);
        } else if ex == "5" {
            println!("Library have {} Books", lib.size());
        } else if ex == "6" {
            show();
        } else if ex == "1" || ex == "2" {
            let out = run(&mut lib, &ex);
            match out {
                Out::Get(b) => println!("Giving the book {} by {}", b.name, b.author),
                Out::Add => println!("Adding sucessful.."),
                Out::Err => println!("Something wrong...."),
            }
        } else {
            println!("Invalid option: '{}'", ex);
            show();
        }
    }
    // Outro
    println!("Thank you for using our Application");
}
