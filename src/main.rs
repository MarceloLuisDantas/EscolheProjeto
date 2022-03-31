use std::process::Command;
use utils::{input, pause, logo, clear_scream};


fn main() {
    
    let mut projetos = vec![
        "Orogonize".to_string(),
        "EScolheProjeto".to_string(),
        "MicroRPG".to_string()
    ];

    loop {
        clear_scream();
        logo();

        for (count, i) in projetos.iter().enumerate() {
            println!(" {}º Projeto - {}", count + 1, i);
        }

        pause();
    }
}