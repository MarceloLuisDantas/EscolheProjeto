use std::io::{BufRead, BufReader};
mod arquivo;

pub fn carrega_projetos(path: &str) -> Vec<Vec<String>> {
    let mut inacabados = Vec::new();
    let arquivo = BufReader::new(arquivo::carrega_arquivo(path));
    for line in arquivo.lines() {
        match line {
            Ok(line) => inacabados.push(vec![line, "0".to_string()]),
            Err(err) => {
                panic!("Erro ao tentar ler arquivo | Error {}", err)
            }
        }            
    }
    return inacabados
}

