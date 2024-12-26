// use rig::providers::gemini;
fn main() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY").unwrap();
    
    println!("Hello, Gemini!{:#?}", api_key);
}
