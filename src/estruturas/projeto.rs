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
            "inacamabdo".to_string()
        }
    }
    fn alterar_estados(&mut self) {
        self.finalizado = !self.finalizado;
    }
    fn reajusta_id(&mut self) {
        self.id -= 1 ;
    }
}

/// Estrutra responsavel por armazenar e manipular os projetos e suas informações.
/// Ao ser iniciada se deve chamar o metodo `carrega_projetos()` para que as informações
/// dos projetos sejám carregadas do arquivo de save
#[derive(Default)]
pub struct Projetos {
    pub inacabados: Vec<Projeto>,
    pub finalizado: Vec<Projeto>,
    len: u32,
}

impl Projetos {
    // Cria uma lista unica com todos os projetos
    pub fn todos_projetos(&self) -> Vec<Projeto> {
        let mut fina = self.finalizado.clone();
        fina.append(&mut self.inacabados.clone());
        fina.sort();
        return fina
    }

    // Atualiza o arquivo de save
    fn atualiza_arquivo(&self) {
        let mut linhas = Vec::new();
        for i in self.todos_projetos().iter() {
            let linha = format!("{} | {} | {} | {}", i.id, i.nome, i.path, i.finalizado());
            linhas.push(linha);
        }
        arquivo::edita_arquivo("./projetos.txt", linhas);
    }

    // reajusta todos os id dos projetos apos remover 
    fn reajusta_id_pos_remover(&mut self, ids_removidos: Vec<u32>) {
        for id_removido in ids_removidos {
            for i in self.finalizado.iter_mut() {
                if i.id >= id_removido {
                    i.reajusta_id();
                }
            }
            for i in self.inacabados.iter_mut() {
                if i.id >= id_removido {
                    i.reajusta_id();
                }
            }
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
                        if p.finalizado {
                            self.finalizado.push(p);
                        } else {
                            self.inacabados.push(p);
                        }
                        self.len += 1;
                    }
                },
                Err(err) => {
                    panic!("Erro ao tentar ler arquivo | Error {}", err)
                }
            } 
        }
    }

    /// Função que cria e adiciona um novo projeto e atualiza o arquivo
    pub fn adiciona_projeto(&mut self, nome: &str, path: &str, fina: bool) {
        let new_projeto = Projeto {
            id: self.len + 1,
            nome: nome.to_string(),
            path: path.to_string(),
            finalizado: fina,
        };
        if new_projeto.finalizado {
            self.finalizado.push(new_projeto);
        } else {
            self.inacabados.push(new_projeto);
        }
        self.atualiza_arquivo();
        self.len += 1;
    }

    pub fn remove_projetos(&mut self, ids: Vec<u32>) {
        for id in ids.iter() {
            let mut indice: i32 = -1;
            for (i, p) in self.finalizado.iter().enumerate() {
                if p.id == *id {
                    indice = i as i32;
                    break
                }
            }
            if indice == -1 {
                for (i, p) in self.inacabados.iter().enumerate() {
                    if p.id == *id {
                        indice = i as i32;
                        break;
                    }
                }   
                self.inacabados.remove(indice as usize);
            } else {
                self.finalizado.remove(indice as usize);
            }
        }
        self.reajusta_id_pos_remover(ids);
        self.atualiza_arquivo();
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
