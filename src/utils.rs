mod utils

pub fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada");
    return String::from(buffer.trim())
}

pub fn pause() {
    let _ = input();
}

pub fn clear_scream() {
    let mut ls = Command::new("clear");
    ls.status().expect("process failed to execute");
}

pub fn logo() {
    println!(" ---- Projetos ---- ");
    println!(" ---- Qual projeto você desejá abrir? ");
}