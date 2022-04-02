use std::io::{BufRead, BufReader};
mod arquivo;

pub struct Projeto {
    nome: String,
    path: String,
    finalizado: bool,
}

impl Projeto {
    pub fn show(&self) {
        println!("{} | {} | {}", self.nome, self.path, self.finalizado);
    }
    pub fn alterar_estados(&mut self) {
        self.finalizado = !self.finalizado;
    }
}

#[derive(Default)]
pub struct Projetos {
    pub lista: Vec<Projeto>,
}

impl Projetos {
    pub fn carrega_projetos(&mut self) {
        let arquivo = BufReader::new(arquivo::carrega_arquivo("./projetos.txt"));
        for line in arquivo.lines() {
            match line {
                Ok(line) => {
                    let mut valores = Vec::new();
                    for i in line.split("|") {
                        valores.push(i);
                    }             
                    let p = Projeto {
                        nome: valores[0].trim().to_string(),
                        path: valores[1].trim().to_string(),
                        finalizado: valores[2].trim() == "finalizado",
                    };
                    self.lista.push(p);
                },
                Err(err) => {
                    panic!("Erro ao tentar ler arquivo | Error {}", err)
                }
            } 
        }
    }
}
