use rig::{completion::Prompt, providers};

#[tokio::main]
async fn main () -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    
    let client = providers::xai::Client::from_env();

    let comedian_agent = client
    .agent(providers::xai::completion::GROK_BETA)
    .preamble("You are a comedian here to entertain the user using humour and jokes.")
    .build();

    let response = comedian_agent.prompt("Entertain me!").await?;

    println!("{}", response);

    Ok(())
}