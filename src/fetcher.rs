use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use rusqlite::{params, Connection};
use std::time::Duration;

pub fn atualizar_feed(conn: &Connection, feed_id: i64, url: &str) -> Result<usize, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(8))
        .redirect(Policy::limited(5))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .map_err(|e| e.to_string())?;

    let resposta = client
        .get(url)
        .send()
        .map_err(|e| format!("Erro na rede: {}", e))?;

    if !resposta.status().is_success() {
        return Err(format!("Status HTTP invalido: {}", resposta.status()));
    }

    let conteudo = resposta.bytes().map_err(|e| e.to_string())?;

    let feed_parsed = feed_rs::parser::parse(&conteudo[..])
        .map_err(|e| format!("Erro ao ler XML/RSS: {}", e))?;

    let mut novos_artigos = 0;

    for entry in feed_parsed.entries {
        let titulo = entry
            .title
            .map(|t| t.content)
            .unwrap_or_else(|| "Sem titulo".to_string());

        let link = entry
            .links
            .first()
            .map(|l| l.href.clone())
            .unwrap_or_default();

        let guid = if !entry.id.is_empty() {
            entry.id
        } else {
            link.clone()
        };

        let res = conn.execute(
            "INSERT OR IGNORE INTO artigos (feed_id, guid, titulo, link) VALUES (?1, ?2, ?3, ?4)",
            params![feed_id, guid, titulo, link],
        );

        if let Ok(linhas) = res {
            if linhas > 0 {
                novos_artigos += 1;
            }
        }
    }

    Ok(novos_artigos)
}