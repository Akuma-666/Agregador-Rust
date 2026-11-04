mod db;
mod fetcher;

use rusqlite::Connection;
use std::io::{self, Write};

fn ler_entrada(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    let conn = Connection::open("meu_rss.db").expect("Erro ao abrir banco SQLite");
    db::iniciar_banco(&conn).expect("Erro ao criar tabelas");

    println!("=== Agregador de RSS Simples ===");

    loop {
        println!("\n--- MENU ---");
        println!("1. Cadastrar novo feed");
        println!("2. Listar feeds cadastrados");
        println!("3. Atualizar feeds (buscar noticias)");
        println!("4. Ver artigos salvos");
        println!("5. Marcar artigo como lido");
        println!("0. Sair");

        let opcao = ler_entrada("Escolha uma opcao: ");

        match opcao.as_str() {
            "1" => {
                let url = ler_entrada("Digite a URL do feed RSS: ");
                match db::adicionar_feed(&conn, &url) {
                    Ok(_) => println!("Feed cadastrado com sucesso!"),
                    Err(e) => println!("Erro ao cadastrar feed: {}", e),
                }
            }
            "2" => {
                println!("\n-- Feeds Cadastrados --");
                match db::listar_feeds(&conn) {
                    Ok(feeds) => {
                        for f in feeds {
                            println!("[ID {}] {}", f.id, f.url);
                        }
                    }
                    Err(e) => println!("Erro ao buscar feeds: {}", e),
                }
            }
            "3" => {
                println!("\nBuscando atualizacoes...");
                if let Ok(feeds) = db::listar_feeds(&conn) {
                    for f in feeds {
                        print!("Atualizando {}... ", f.url);
                        match fetcher::atualizar_feed(&conn, f.id, &f.url) {
                            Ok(novos) => println!("OK! ({} novos artigos)", novos),
                            Err(e) => println!("Falhou: {}", e),
                        }
                    }
                }
            }
            "4" => {
                println!("\n-- Artigos Salvos --");
                if let Ok(artigos) = db::listar_artigos(&conn) {
                    for a in artigos {
                        let status = if a.lido { "[LIDO]" } else { "[NAO LIDO]" };
                        println!("{} ID {}: {} - {}", status, a.id, a.titulo, a.link);
                    }
                }
            }
            "5" => {
                let id_str = ler_entrada("Digite o ID do artigo lido: ");
                if let Ok(id) = id_str.parse::<i64>() {
                    let _ = db::marcar_como_lido(&conn, id);
                    println!("Artigo atualizado!");
                } else {
                    println!("ID invalido.");
                }
            }
            "0" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opcao invalida!"),
        }
    }
}