mod db;
mod livro;
mod status;
mod usuario;

use db::init_db;
use livro::{listar_livros, Livro};
use usuario::{listar_pessoas, Pessoa};
use status::Status;
use chrono::NaiveDate;

fn main() -> rusqlite::Result<()> {
    let conn = init_db()?; // Inicializa o banco de dados

    // Criar um novo livro
    let mut livro = Livro::new(
        "Rust Programming".to_string(),
        NaiveDate::from_ymd_opt(2021, 9, 1).expect("expect valid data"),
        450,
        Status::NaoLido,
    ).expect("Dados inválidos");
    livro.save(&conn)?;

    // Listar livros
    let livros = listar_livros(&conn)?;
    for livro in livros {
        println!("{:?}", livro);
    }

    // Atualizar o livro
    livro.titulo = "Rust Programming - Updated".to_string();
    livro.atualizar(&conn)?;

    // Remover o livro
    livro.remover(&conn)?;

    Ok(())
}
