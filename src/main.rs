use postgres::{Client, Error, NoTls};
use rocket::{get, launch, post, routes};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use crate::book_repository::{Book, BookRepository};

mod book_repository;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct CustomError {
    message: String,
}

#[get("/books?<title>")]
fn get_book(title: &str) -> Option<Json<Book>> {
    let mut repo = BookRepository {};
    repo.add(Book {
        title: "The Great Gatsby".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
    });
    let response = repo.get(title);
    match response {
        Ok(book) => Some(Json(Book { title: book.title.clone(), author: book.author.clone() })),
        _ => None
    }
}

#[post("/books", data = "<book>")]
fn add_book(book: Json<Book>) -> (Status, Result<Json<Book>, Json<CustomError>>) {
    let mut repo = BookRepository {};
    match repo.add(book.0) {
        Ok(book) => (Status::Ok, Result::Ok(Json(Book { title: book.title.clone(), author: book.author.clone() }))),
        Err(msg) => (Status::InternalServerError, Result::Err(Json(CustomError { message: String::from(msg.to_string()) })))
    }
}

fn create_table_if_not_exists() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:12345@localhost/sparks-rust", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS books (
            title              VARCHAR(64) PRIMARY KEY,
            author             VARCHAR(64),
        )
    ")?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    if let Err(msg) = create_table_if_not_exists() {
        println!("Error creating table")
    }
    rocket::build().mount("/", routes![get_book, add_book])
}