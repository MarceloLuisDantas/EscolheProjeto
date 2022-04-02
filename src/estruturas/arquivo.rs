use std::fs::File;
use std::io::{ErrorKind};

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