use std::io::{self, Write};
use tiny_loop::Agent;

pub async fn run_cli_loop(mut agent: Agent) {
    println!("Chatbot started. Type 'quit' to exit.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        match agent.chat(input).await {
            Ok(response) => println!("\n{}\n", response),
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }
}
