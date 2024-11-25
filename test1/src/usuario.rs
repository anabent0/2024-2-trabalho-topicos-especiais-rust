use chrono::NaiveDate;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Pessoa {
    pub id: Option<i32>,
    pub nome: String,
    pub data_nascimento: NaiveDate,
    pub email: String,
}

impl Pessoa {
    pub fn new(nome: String, data_nascimento: NaiveDate, email: String) -> Result<Self, String> {
        if nome.is_empty() {
            return Err("Nome não pode ser vazio.".to_string());
        }
        if email.is_empty() {
            return Err("Email não pode ser vazio.".to_string());
        }

        Ok(Self {
            id: None,
            nome,
            data_nascimento,
            email,
        })
    }

    pub fn save(&mut self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO pessoas (nome, data_nascimento, email) VALUES (?1, ?2, ?3)",
            (
                &self.nome,
                &self.data_nascimento.to_string(),
                &self.email,
            ),
        )?;
        self.id = Some(conn.last_insert_rowid() as i32);
        Ok(())
    }

    pub fn atualizar(&self, conn: &Connection) -> Result<()> {
        if let Some(id) = self.id {
            conn.execute(
                "UPDATE pessoas SET nome = ?1, data_nascimento = ?2, email = ?3 WHERE id = ?4",
                (
                    &self.nome,
                    &self.data_nascimento.to_string(),
                    &self.email,
                    id,
                ),
            )?;
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    pub fn remover(self, conn: &Connection) -> Result<()> {
        if let Some(id) = self.id {
            conn.execute("DELETE FROM pessoas WHERE id = ?1", [id])?;
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }
}

pub fn listar_pessoas(conn: &Connection) -> Result<Vec<Pessoa>> {
    let mut stmt = conn.prepare("SELECT id, nome, data_nascimento, email FROM pessoas")?;
    let pessoas = stmt
        .query_map([], |row| {
            Ok(Pessoa {
                id: row.get(0)?,
                nome: row.get(1)?,
                data_nascimento: NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d")
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                email: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(pessoas)
}
