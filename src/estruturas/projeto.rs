use std::io::{BufRead, BufReader};
mod arquivo;

#[derive(Clone)]
pub struct Projeto {
    nome: String,
    path: String,
    finalizado: bool,
}

impl Projeto {
    /// Mostra as informações do projetos
    /// 
    /// `[nome] | [caminho] | [estado]`
    pub fn show(&self) {
        println!("Nome: {} | Path: {} | Finalizado: {}", 
                 self.nome, self.path, self.finalizado());
    }
    fn alterar_estados(&mut self) {
        self.finalizado = !self.finalizado;
    }
    fn finalizado(&self) -> String {
        if self.finalizado {
            "finalizado".to_string()
        } else {
            "inacamabdo".to_string()
        }
    }
}

/// Estrutra responsavel por armazenar e manipular os projetos e suas informações.
/// Ao ser iniciada se deve chamar o metodo `carrega_projetos()` para que as informações
/// dos projetos sejám carregadas do arquivo de save
#[derive(Default)]
pub struct Projetos {
    pub inacabados: Vec<Projeto>,
    pub finalizado: Vec<Projeto>,
}

impl Projetos {
    // Cria uma lista unica com todos os projetos
    fn todos_projetos(&self) -> Vec<Projeto> {
        let mut fina = self.finalizado.clone();
        fina.append(&mut self.inacabados.clone());
        return fina
    }
    // Atualiza o arquivo de save
    fn atualiza_arquivo(&self) {
        let mut linhas = Vec::new();
        for i in self.todos_projetos().iter() {
            let linha = format!("{} | {} | {}", i.nome, i.path, i.finalizado());
            linhas.push(linha);
        }
        arquivo::edita_arquivo("./projetos.txt", linhas);
    }

    /// Primeira função a ser chamada quando se inicia a estrutura, ela ira carregar
    /// todos os projetos salvos no arquivo de save e ira organizar dentro das listas
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
                    if p.finalizado {
                        self.finalizado.push(p);
                    } else {
                        self.inacabados.push(p);
                    }
                },
                Err(err) => {
                    panic!("Erro ao tentar ler arquivo | Error {}", err)
                }
            } 
        }
    }

    /// Função responsavle por fazer a transgerencia de um projeto de uma lista
    /// para a outra, e logo em seguida fazer a atualização do arquivo de save
    pub fn move_projeto(&mut self, projeto_indice: usize, lista_alvo: bool) {
        if lista_alvo {
            let mut p = self.finalizado[projeto_indice].clone();
            self.finalizado.remove(projeto_indice);
            p.alterar_estados();
            self.inacabados.push(p);
        } else {
            let mut p = self.inacabados[projeto_indice].clone();
            self.inacabados.remove(projeto_indice);
            p.alterar_estados();
            self.finalizado.push(p);
        }
        self.atualiza_arquivo();
    }
}
