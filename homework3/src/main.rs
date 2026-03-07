use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro

    let mut file = File::create(filename).expect("Can't open file");

    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year).expect("Can't open file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader

    let file = File::open(filename).expect("Can't open file");
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Can't read line");
        let parts: Vec<&str> = line.splitn(3, ',').collect();

        if parts.len() == 3 {
            books.push(Book {
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: parts[2].parse::<u16>().expect("Invalid year"),
            });
        }
    }
    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "Don Quixote".to_string(), author: "Miguel de Cervantes".to_string(), year: 1605 },
        Book { title: "Alice's Adventures in Wonderland".to_string(), author: "Lewis Carroll".to_string(), year: 1865 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}