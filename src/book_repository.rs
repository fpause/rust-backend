use postgres::{Client, Error, NoTls};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub(crate) title: String,
    pub(crate) author: String,
}

pub struct BookRepository {}

impl BookRepository {
    pub(crate) fn add(&mut self, book: Book) -> Result<Book, Error> {
        let mut client = Client::connect("postgresql://postgres:12345@localhost/sparks-rust", NoTls)?;

        client.execute(
            "INSERT INTO books (title, author) VALUES ($1, $2)",
            &[&book.title, &book.author],
        )?;

        return Ok(book);
    }

    fn remove(&mut self, title: &str) -> Result<(), Error> {
        let mut client = Client::connect("postgresql://postgres:12345@localhost/sparks-rust", NoTls)?;

        client.execute(
            "DELETE FROM books where title = $1)",
            &[&title],
        )?;

        Ok(())
    }

    pub(crate) fn get(&self, title: &str) -> Result<Book, String> {
        let mut client_opt = Client::connect("postgresql://postgres:12345@localhost/sparks-rust", NoTls);

        if let Ok(mut client) = client_opt {
            let result = client.query(
                "SELECT title, author FROM books where title = $1)",
                &[&title],
            );
            if let Ok(rows) = result {
                if rows.len() == 0 {
                    return Err("Book not found".to_string());
                }
                let row_opt = rows.get(0);
                if let Some(row) = row_opt {
                    return Ok(Book {
                        title: row.get(0),
                        author: row.get(1),
                    });
                }
            }
            return Err("Error finding Book".to_string());
        } else {
            return Err(client_opt.err().unwrap().to_string());
        }
    }
}