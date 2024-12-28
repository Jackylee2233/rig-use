use rig::{completion::Prompt, providers};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main () -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    
    let client = providers::gemini::Client::from_env();

    let comedian_agent = client
    .agent(providers::gemini::completion::GEMINI_1_5_PRO)
    .preamble("You are a Rust export here to help answer questions about Rust.")
    // .preamble("You are a comedian here to entertain the user using humour and jokes.")
    .build();

    let response = comedian_agent.prompt("Tell me about the update of Rust language").await?;
    // let response = comedian_agent.prompt("Entertain me!").await?;

    let mut file = File::create("response.md")?;
    writeln!(file, "# Response\n\n{}", response)?;

    println!("{}", response);

    Ok(())
}