use std::io::{BufRead, BufReader};
mod arquivo;

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Projeto {
    id: u32,
    nome: String,
    path: String,
    finalizado: bool,
}

impl Projeto {
    /// Mostra as informações do projetos
    /// 
    /// `[nome] | [caminho] | [estado]`
    pub fn show(&self) {
        println!("Nome: {} | Path: {} | Finalizado: {} | ID: {}", 
                 self.nome, self.path, self.finalizado(), self.id);
    }
    fn finalizado(&self) -> String {
        if self.finalizado {
            "finalizado".to_string()
        } else {
            "inacabados".to_string()
        }
    }
    fn alterar_estados(&mut self) {
        self.finalizado = !self.finalizado;
    }
}

/// Estrutra responsavel por armazenar e manipular os projetos e suas informações.
/// Ao ser iniciada se deve chamar o metodo `carrega_projetos()` para que as informações
/// dos projetos sejám carregadas do arquivo de save
#[derive(Default)]
pub struct Projetos {
    pub projetos: Vec<Projeto>,
}

impl Projetos {
    fn len(&self) -> usize {
        self.projetos.len()
    }

    // Atualiza o arquivo de save
    fn atualiza_arquivo(&self) {
        let mut linhas = Vec::new();
        for i in self.projetos.iter() {
            let linha = format!("{} | {} | {} | {}", i.id, i.nome, i.path, i.finalizado());
            linhas.push(linha);
        }
        arquivo::edita_arquivo("./projetos.txt", linhas);
    }

    // reajusta todos os id dos projetos apos remover 
    fn reajusta_id_pos_remover(&mut self) {
        for i in 0..self.len() {
            self.projetos[i].id = (i + 1) as u32;
        }
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
                    if valores.len() == 4 {
                        let p = Projeto {
                            id: valores[0].trim().parse::<u32>().unwrap(),
                            nome: valores[1].trim().to_string(),
                            path: valores[2].trim().to_string(),
                            finalizado: valores[3].trim() == "finalizado",
                        };
                        self.projetos.push(p);
                    }
                },
                Err(err) => {
                    panic!("Erro ao tentar ler arquivo | Error {}", err)
                }
            } 
        }
        self.reajusta_id_pos_remover();
    }

    pub fn show_finalizados(&self) {
        println!("Finalizados -------");
        self.projetos.iter().for_each( |p| {
            if p.finalizado { p.show() }
        });
    }
    
    pub fn show_inacabados(&self) {
        println!("Inacabados -------");
        self.projetos.iter().for_each( |p| {
            if !p.finalizado { p.show() }
        });
    }

    pub fn show_projetos(&self) {
        self.show_finalizados();
        println!();
        self.show_inacabados();
    }

    pub fn adiciona_projeto(&mut self, nome: &str, path: &str, fina: bool) {
        let new_projeto = Projeto {
            id: (self.len() + 1) as u32,
            nome: nome.to_string(),
            path: path.to_string(),
            finalizado: fina,
        };
        self.projetos.push(new_projeto);
        self.atualiza_arquivo();
    }

    pub fn remove_projetos(&mut self, ids: Vec<u32>) {
        for id in ids.iter() {
            let mut indice: usize = 0;
            for (i, p) in self.projetos.iter().enumerate() {
                if p.id == *id {
                    indice = i;
                    break
                }
            }
            self.projetos.remove(indice);
        }
        self.reajusta_id_pos_remover();
        self.atualiza_arquivo();
    }

    /// Função responsavle por fazer a transgerencia de um projeto de uma lista
    /// para a outra, e logo em seguida fazer a atualização do arquivo de save
    pub fn altera_estado_projeto(&mut self, id: u32) {
        let lista = self.projetos.clone();
        for (i, p) in lista.iter().enumerate() {
            if p.id == id {
                self.projetos[i].alterar_estados();
            }
        }
        self.reajusta_id_pos_remover();
        self.atualiza_arquivo();
    }
}
