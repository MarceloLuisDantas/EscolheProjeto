use std::io::prelude::*;
use std::io::{ErrorKind, BufReader};
use std::fs::File;

/// Atualiza um arquivo de texto e cria um backup
pub fn edita_arquivo(path: &str, linhas: Vec<String>) {
    // Carrega arquivo antigo para fazer o backup
    let arquivo_antigo = BufReader::new(carrega_arquivo(path)); 

    // Separa o pah para pegar apenas o nome do arquivo e crira o backup
    let mut partes_path = Vec::new();
    for i in path.split("/") {
        partes_path.push(i.to_string());
    }
    let l = partes_path.len() - 1;
    partes_path[l] = format!(".backup-{}", partes_path[l]);    
    
    // Cria o arquivo de backup 
    let mut backup = match File::create(partes_path.join("/")) {
        Ok(file) => file,
        Err(err) => {
            panic!("Erro ao tentar criar arquivo | Error {}", err)
        } 
    };

    // Escreve no arquivo de backup as informações do antigo save
    for linha in arquivo_antigo.lines() {
        match linha {
            Ok(linha) => {
                let _ = backup.write(format!("{}\n", linha).as_bytes());
            },
            Err(err) => {
                panic!("Erro ao tentar ler arquivo | Error {}", err)
            }
        } 
    }

    // Cria o novo arquivo com as informações atualizadas
    match File::create(path) {
        Ok(mut file) => {
            for linha in linhas.iter() {
                let _ = file.write(format!("{}\n", linha).as_bytes());
            }
        },
        Err(err) => {
            panic!("Erro ao tentar criar novo arquivo para atualizar o arquivo original | Error {}", err)
        } 
    };

}

/// Carrega as informações dos projetos salvos
/// [nome do projeto] | [path do projeto] | [estado do projeto]
pub fn carrega_arquivo(path: &str) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match File::create(path) {
                Ok(_) => { },
                Err(err) => {
                    panic!("Erro ao tentar criar arquivo | Error {}", err)
                } 
            }
            match File::open(path) {
                Ok(file) => file,
                Err(err) => {
                    panic!("Erro ao tentar abrir arquivo criado | {}", err)
                }
            }
        },
        Err(err) => {
            panic!("Erro inesperado ao tentar abrir arquivo | Error {}", err)
        }
    }
}