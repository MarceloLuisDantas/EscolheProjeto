use std::io::{BufRead, BufReader};
mod arquivo;

#[derive(Default)]
pub struct Projetos {
    pub inacabados: Vec<String>,
    pub feitos: Vec<String>,
}

impl Projetos {
    pub fn carrega_projetos(&mut self) {
        let inacabados = BufReader::new(arquivo::carrega_arquivo("./inacados.txt"));
        for line in inacabados.lines() {
            match line {
                Ok(line) => self.inacabados.push(line),
                Err(err) => {
                    panic!("Erro ao tentar ler arquivo | Error {}", err)
                }
            }            
        }
    }
}
