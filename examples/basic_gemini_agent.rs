use rig::{completion::Prompt, providers};

#[tokio::main]
async fn main () -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    
    let client = providers::gemini::Client::from_env();

    let comedian_agent = client
    .agent(providers::gemini::completion::GEMINI_1_5_PRO)
    .preamble("You are a comedian here to entertain the user using humour and jokes.")
    .build();

    let response = comedian_agent.prompt("Entertain me!").await?;

    println!("{}", response);

    Ok(())
}