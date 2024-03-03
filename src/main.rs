use std::process::Command;
use std::env;
mod lista_comandos;

fn main() {
    let commands_map = lista_comandos::get_command_map();
    // let flags_map = lista_comandos::get_flags_map();

    let args: Vec<String> = env::args().skip(1).collect();

    let git_command = match args.get(0) {
        Some(user_command) => match commands_map.get(user_command.as_str()) {
            Some(&git_command) => git_command,
            None => {
                eprintln!("Tradução não encontrada, tentando rodar git com o comando {} enviado.", user_command.as_str());
                user_command.as_str()
            }
        },
        None => {
            eprintln!("Nenhum argumento enviado.");
            return;
        }
    };

    let output = Command::new("git")
        .arg(git_command)
        .args(&args[1..])
        .output()
        .expect("Erro ao executar o comando.");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}
