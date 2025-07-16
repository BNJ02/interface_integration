use subprocess::{Popen, PopenConfig, Redirection};
use std::io::Write;
use std::{thread, time::Duration};

fn main() -> subprocess::Result<()> {
    // Lancer le sous-processus avec stdin redirigé
    let mut p = Popen::create(
        &[
            "cargo",
            "run",
            "--manifest-path",
            "src/interface/Cargo.toml",
        ],
        PopenConfig {
            stdin: Redirection::Pipe,
            stdout: Redirection::Pipe,
            stderr: Redirection::Merge,
            ..Default::default()
        },
    )?;

    let stdin = p.stdin.as_mut().expect("Échec ouverture stdin");

    // Envoi toutes les 5 secondes
    loop {
        let message = "Hello from parent!\n";
        stdin.write_all(message.as_bytes()).expect("Échec write");
        stdin.flush().expect("Échec flush");

        println!("Message envoyé : {}", message.trim());

        thread::sleep(Duration::from_secs(5));
    }
}
