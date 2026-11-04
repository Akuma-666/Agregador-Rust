use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Feed {
    pub id: i64,
    pub url: String,
}

#[derive(Debug)]
pub struct Artigo {
    pub id: i64,
    pub titulo: String,
    pub link: String,
    pub lido: bool,
}

pub fn iniciar_banco(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS feeds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT UNIQUE NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS artigos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            feed_id INTEGER NOT NULL,
            guid TEXT UNIQUE NOT NULL,
            titulo TEXT NOT NULL,
            link TEXT NOT NULL,
            lido INTEGER DEFAULT 0,
            FOREIGN KEY (feed_id) REFERENCES feeds(id)
        )",
        [],
    )?;

    Ok(())
}

pub fn adicionar_feed(conn: &Connection, url: &str) -> Result<()> {
    conn.execute("INSERT INTO feeds (url) VALUES (?1)", params![url])?;
    Ok(())
}

pub fn listar_feeds(conn: &Connection) -> Result<Vec<Feed>> {
    let mut stmt = conn.prepare("SELECT id, url FROM feeds")?;
    let feeds = stmt
        .query_map([], |row| {
            Ok(Feed {
                id: row.get(0)?,
                url: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(feeds)
}

pub fn listar_artigos(conn: &Connection) -> Result<Vec<Artigo>> {
    let mut stmt = conn.prepare("SELECT id, titulo, link, lido FROM artigos ORDER BY id DESC")?;
    let artigos = stmt
        .query_map([], |row| {
            let lido_int: i32 = row.get(3)?;
            Ok(Artigo {
                id: row.get(0)?,
                titulo: row.get(1)?,
                link: row.get(2)?,
                lido: lido_int == 1,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(artigos)
}

pub fn marcar_como_lido(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("UPDATE artigos SET lido = 1 WHERE id = ?1", params![id])?;
    Ok(())
}