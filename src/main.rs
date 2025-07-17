use subprocess::{Popen, PopenConfig, Redirection};
use std::io::Write;
use std::{thread, time::Duration};

use serde::Serialize;

#[derive(Serialize)]
struct Task {
    name: String,
    freq_start: f64,
    freq_end: f64,
    time_start: f64,
    time_end: f64,
    amplifier: String, // Pour simplifier : représente Amplifier sous forme de String
}

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
            ..Default::default()
        },
    )?;

    let stdin = p.stdin.as_mut().expect("Échec ouverture stdin");

    // Liste des tâches à envoyer une par une
    let tasks = vec![
        Task {
            name: "Init capteurs".into(),
            freq_start: 100.0,
            freq_end: 300.0,
            time_start: 0.0,
            time_end: 300.0,
            amplifier: "A20_500".into(),
        },
        Task {
            name: "Transmission".into(),
            freq_start: 1000.0,
            freq_end: 2500.0,
            time_start: 300.0,
            time_end: 600.0,
            amplifier: "A1000_2500".into(),
        },
        Task {
            name: "Sleep mode".into(),
            freq_start: 5000.0,
            freq_end: 5500.0,
            time_start: 0.0,
            time_end: 1000.0,
            amplifier: "A2400_6000".into(),
        },
    ];

    let mut step = 0;

    loop {
        let task = &tasks[step % tasks.len()];  // Avance dans la liste cycliquement
        let json = serde_json::to_string(task).expect("Erreur sérialisation JSON");

        stdin.write_all(json.as_bytes()).expect("Échec write");
        stdin.write_all(b"\n").expect("Échec write newline");
        stdin.flush().expect("Échec flush");

        println!("Tâche envoyée : {}", task.name);

        step += 1;
        thread::sleep(Duration::from_secs(5));
    }
}
