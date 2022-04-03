use std::io::prelude::*;
use std::io::{ErrorKind, BufReader};
use std::fs::File;

pub fn edita_arquivo(path: &str, linhas: Vec<String>) {
    let arquivo_antigo = BufReader::new(carrega_arquivo(path));

    let mut partes_path = Vec::new();
    for i in path.split("/") {
        partes_path.push(i.to_string());
    }
    let l = partes_path.len() - 1;
    partes_path[l] = format!(".backup{}", partes_path[l]);    
    
    let mut backup = match File::create(partes_path.join("/")) {
        Ok(file) => file,
        Err(err) => {
            panic!("Erro ao tentar criar arquivo | Error {}", err)
        } 
    };

    for line in arquivo_antigo.lines() {
        match line {
            Ok(line) => {
                let _ = backup.write(line.as_bytes());
                let _ = backup.write("\n".as_bytes());
            },
            Err(err) => {
                panic!("Erro ao tentar ler arquivo | Error {}", err)
            }
        } 
    }

    match File::create(path) {
        Ok(mut file) => {
            for linha in linhas.iter() {
                let _ = file.write(linha.as_bytes());
                let _ = file.write("\n".as_bytes());
            }
        },
        Err(err) => {
            panic!("Erro ao tentar criar novo arquivo para atualizar os projetos | Error {}", err)
        } 
    };

}
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