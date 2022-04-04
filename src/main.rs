#![allow(non_snake_case)]

#[path="./estruturas/projeto.rs"]
mod projeto;

fn main() {
    let mut p: projeto::Projetos = Default::default() ;
    p.carrega_projetos();

    println!("Antes ----------------- ");
    for i in p.todos_projetos().iter() {
        i.show();
    }
    println!();

    p.adiciona_projeto("Teste Numero 10", "./teste10", true);
    p.adiciona_projeto("Teste Numero 11", "./teste11", true);
    p.adiciona_projeto("Teste Numero 12", "./teste12", true);
    p.remove_projeto(7);
    
    println!("Depois ----------------- ");
    for i in p.todos_projetos().iter() {
        i.show();
    }

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