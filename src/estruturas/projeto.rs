use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

#[derive(Default)]
pub struct Projetos {
    pub inacabados: Vec<String>,
    pub feitos: Vec<String>,
}

// Verifica se os arquivos com os projetos existem
pub fn verifica_arquivos(path: String) {

}
// Caso os arquivos não existam eles seram craidos
pub fn cria_arquivos() {

}
// Depois de verificar se os arquivos existem
// esles seram lidos e enviados
pub fn le_arquivo() -> Vec<String> {
    let projetos: Vec<String> = Vec::new();



    return projetos
}

impl Projetos {
    pub fn carrega_projetos(&mut self) {

    }
}
