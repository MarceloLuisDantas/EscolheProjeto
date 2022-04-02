#![allow(non_snake_case)]

#[path="./estruturas/projeto.rs"]
mod projeto;

fn main() {
    let mut p: projeto::Projetos = Default::default() ;
    p.carrega_projetos();
    for i in p.lista.iter() {
        i.show();
    }
}