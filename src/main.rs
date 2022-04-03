#![allow(non_snake_case)]

#[path="./estruturas/projeto.rs"]
mod projeto;

fn input() -> i32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().parse::<i32>().unwrap();
}

fn main() {
    let mut p: projeto::Projetos = Default::default() ;
    p.carrega_projetos();

    println!("Projetos inacabados: ");
    for (indice, projeto) in p.inacabados.iter().enumerate() {
        print!("{} - ", indice + 1);
        projeto.show();
    }
    println!();

    println!("Projetos finalizados: ");
    for (indice, projeto) in p.finalizado.iter().enumerate() {
        print!("{} - ", indice + 1);
        projeto.show();
    }
    println!();

    println!("Mover de para: [indice, lista[1, 2]]: ");
    let x = input() - 1;
    let y = input() == 2;
    p.move_projeto(x as usize, y);

    println!();
    println!("Projetos inacabados: ");
    for (indice, projeto) in p.inacabados.iter().enumerate() {
        print!("{} - ", indice + 1);
        projeto.show();
    }
    println!();

    println!("Projetos finalizados: ");
    for (indice, projeto) in p.finalizado.iter().enumerate() {
        print!("{} - ", indice + 1);
        projeto.show();
    }
    println!();
}