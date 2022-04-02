// Estrutura que guarda os projetos
#[path = "./estruturas/projeto.rs"]
mod projeto;

fn new_projeto() -> projeto::Projetos {
    let p: projeto::Projetos = Default::default();  
    projeto.carrega_projetos();
    return p
}

mod test {

}