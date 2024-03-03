use std::collections::HashMap;

pub fn get_command_map() -> HashMap<&'static str, &'static str> {
    let commands_map: HashMap<&str, &str> = [
        ("começa", "init"),
        ("estado", "status"),
        ("ajuda", "--help"),
        ("puxa", "pull"),
        ("empurra", "push"),
        ("adiciona", "add"),
        ("remoto", "remote"),
        ("x9", "reflog"),
        ("histórico", "log"),
        ("troca", "switch"),
        ("esconde", "stash"),
        ("configura", "config"),
        ("reseta", "reset")
    ].iter().cloned().collect();
    commands_map
}

