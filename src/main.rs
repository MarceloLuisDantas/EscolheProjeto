#![allow(non_snake_case)]

#[path="./estruturas/projeto.rs"]
mod projeto;

use std::io;

fn input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u32>().unwrap()
}

fn main() {
    let mut p: projeto::Projetos = Default::default() ;
    p.carrega_projetos();

    
    // p.adiciona_projeto("Teste Numero 11", "./teste11", true);
    // p.adiciona_projeto("Teste Numero 12", "./teste12", true);
    // p.remove_projetos(vec![1, 4, 7]);
    
    println!();
    p.show_projetos();
    
    println!("Qual projeto deseja finalizar [ID]: ");
    p.altera_estado_projeto(input());

    println!();
    p.show_projetos();
    

    // println!("Projetos inacabados: ");
    // for (indice, projeto) in p.inacabados.iter().enumerate() {
    //     print!("{} - ", indice + 1);
    //     projeto.show();
    // }
    // println!();

    // println!("Projetos finalizados: ");
    // for (indice, projeto) in p.finalizado.iter().enumerate() {
    //     print!("{} - ", indice + 1);
    //     projeto.show();
    // }
    // println!();

    // p.adiciona_projetos("Teste Numero 0", "./teste0.rs", false);
    // println!(" -------------------------------- ");
    
    // println!("Projetos inacabados: ");
    // for (indice, projeto) in p.inacabados.iter().enumerate() {
    //     print!("{} - ", indice + 1);
    //     projeto.show();
    // }
    // println!();

    // println!("Projetos finalizados: ");
    // for (indice, projeto) in p.finalizado.iter().enumerate() {
    //     print!("{} - ", indice + 1);
    //     projeto.show();
    // }
    // println!();


}