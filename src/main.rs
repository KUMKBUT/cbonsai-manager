use std::process::Command;
use std::io::{self, Write};
use std::fs::OpenOptions;
use rand::Rng;

struct BonsaiHunter {
    file_option: OpenOptions,
    max_seed: u32,
}

impl BonsaiHunter {
    fn new(max: u32) -> Self {
        let mut opt = OpenOptions::new();
        opt.create(true).append(true);
        Self {
            file_option: opt,
            max_seed: max,
        }
    }
    fn save_seed(&self, seed: u32) {
        match self.file_option.open("favorite_seeds.txt") {
            Ok(mut file) => {
                let data = format!("{}\n", seed);
                if let Err(e) = file.write_all(data.as_bytes()) {
                    eprintln!("Error write: {}", e);
                } else {
                    println!("⭐ Seed {} saved!", seed);
                }
            }
            Err(e) => eprintln!("Cant open file: {}", e),
        }
    }
    fn gen_random_seed(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..self.max_seed)
    }
    fn run_cbonsai(&self, seed: u32) {
        let output = Command::new("cbonsai")
            .arg("-p")
            .arg("-s")
            .arg(seed.to_string())
            .output();

        match output {
            Ok(out) => {
                io::stdout().write_all(&out.stdout).unwrap();
            }
            Err(_) => {
                eprintln!("Error: Please make sure cbonsai is installed on your system.");
            }
        }
    }
    fn show_favorites(&self) {
        if let Ok(content) = std::fs::read_to_string("favorite_seeds.txt") {
            println!("--- Showing your favorite collection ---");
        
            for line in content.lines() {
                if let Ok(seed) = line.trim().parse::<u32>() {
                    println!("Displaying seed: {}", seed);
                    self.run_cbonsai(seed);                
                
                    println!("Press Enter to see next favorite...");
                    let mut tmp = String::new();
                    io::stdin().read_line(&mut tmp).unwrap();
                }
            }
        } else {
            eprintln!("No favorites found yet!");
        }
    }
}

fn main() {
    println!("--- Let's start the hunt for beautiful bonsai ---");
    let hunter = BonsaiHunter::new(100000);

    loop {
        let seed = hunter.gen_random_seed();
        hunter.run_cbonsai(seed);

        println!("------------------------------------------");
        println!("Seed: {} | [Enter] Next, [s] Save, [f] Show favorite [q] Quit", seed);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "s" => hunter.save_seed(seed),
            "f" => hunter.show_favorites(),
            "q" => break,
            _ => continue,
        }
    }
    println!("\nThe hunt is over. Did you like the seed? Run it: cbonsai -p -s NUMBER");
}
