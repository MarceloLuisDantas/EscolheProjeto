#![allow(non_snake_case)]
use ncurses::*;

const DESSELECIONADO: i16 = 0; 
const SELECIONADO: i16 = 1;

// Estrutura que guarda os projetos
#[path = "./estruturas/projeto.rs"]
mod projeto;

fn lista_projetos(projetos: Vec<(String, bool)>) {
    let mut quit = false;
    let mut item_selecioado = 0; // Cursos para destacar o item selecionado
    
    while !quit {
        for (row, (item, x)) in projetos.iter().enumerate() {
            // Verifica se esta selecionado
            let pair = if item_selecioado == row { SELECIONADO } 
                       else { DESSELECIONADO };
            
            attron(COLOR_PAIR(pair)); // Seta o estilo de formatação
            
            // Move para a posição correta e adiciona o projeto ao buffer
            mv((row + 4) as i32, 3); 

            // O ultimo elemento da lista é a opção de adicionar
            // um novo projeto, e a formação dele é difesrente
            if row < projetos.len() - 1 { 
                if !x {
                    addstr(&format!(" {} - [ ] {} ", row + 1, item)); 
                } else {
                    addstr(&format!(" {} - [X] {} ", row + 1, item)); 
                }
            } else {                                 
                addstr(&format!("\n {}", item));     
            }                                     

            attroff(COLOR_PAIR(pair)); // Desabilida a formatação
        }
        refresh();

        let key = getch(); 
        match key as u8 as char { 
            'q' => quit = true,
            // A por algum motivo tem o mesmo codigo de Arrow Up
            'A' => if item_selecioado >= 1 { 
                item_selecioado -= 1 
            },
            // B por algum motivo tem o mesmo codigo de Arrow Down
            'B' => if item_selecioado < projetos.len() - 1 { 
                item_selecioado += 1
            }, 
            'x' => { 
                 
            },
            x => { println!("{} - {}",x, key) },
        }
    }
}

fn main() {
    // Setup ===========================================
    initscr();                                     // Inicia o terminal
    start_color();                                 // Ativa formatação de cores 
    noecho();                                      // Desativa o echo
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Desativa o cursos

    init_pair(DESSELECIONADO, COLOR_WHITE, COLOR_BLACK); // Seta a formação para o item desselecionado 
    init_pair(SELECIONADO, COLOR_BLACK, COLOR_WHITE);    // Seta a formação para o item selecionado 

    // Seta as primeiras 4 linhas que n serão alteradass
    addstr("\n");
    addstr(" --------- Projetos ----------- \n");
    addstr(" Qual projeto você desejá abrir? \n");
    addstr("\n");
    // Setup ===========================================

    let mut inacamados = projeto::carrega_projetos("./inacamados.txt");
    inacamados.push(("Adicionar novo projeto".to_string(), false));
    let mut feitos = projeto::carrega_projetos("./feitos.txt");

    lista_projetos(inacamados);

    endwin();
}