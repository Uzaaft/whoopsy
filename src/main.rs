use std::env;
use whoopsy::{OAuthConfig, Result, Scope, WhoopClient};

#[tokio::main]
async fn main() -> Result<()> {
    // Example 1: Using direct access token
    if let Ok(access_token) = env::var("WHOOP_ACCESS_TOKEN") {
        let client = WhoopClient::new(access_token);

        // Get user profile
        match client.get_profile_basic().await {
            Ok(profile) => {
                println!("User: {} {}", profile.first_name, profile.last_name);
                println!("Email: {}", profile.email);
            }
            Err(e) => eprintln!("Error fetching profile: {}", e),
        }

        // Get recent cycles
        match client.get_cycle_collection(None).await {
            Ok(cycles) => {
                if let Some(records) = cycles.records {
                    println!("\nRecent cycles:");
                    for cycle in records.iter().take(5) {
                        println!("  Cycle {}: {}", cycle.id, cycle.start);
                        if let Some(score) = &cycle.score {
                            println!("    Strain: {:.2}", score.strain);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error fetching cycles: {}", e),
        }
    }

    // Example 2: Using OAuth flow (uncomment to use)
    /*
    let client_id = env::var("WHOOP_CLIENT_ID").expect("WHOOP_CLIENT_ID not set");
    let client_secret = env::var("WHOOP_CLIENT_SECRET").expect("WHOOP_CLIENT_SECRET not set");
    let redirect_uri = env::var("WHOOP_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:8080/callback".to_string());

    let config = OAuthConfig::new(client_id, client_secret, redirect_uri)
        .with_scope(Scope::ReadProfile)
        .with_scope(Scope::ReadCycles)
        .with_scope(Scope::ReadSleep)
        .with_scope(Scope::ReadRecovery)
        .with_scope(Scope::ReadWorkout);

    // Print authorization URL
    println!("Visit this URL to authorize: {}", config.get_authorization_url());

    // After user authorizes, exchange the code for a token
    // let code = "AUTHORIZATION_CODE_FROM_CALLBACK";
    // let client = WhoopClient::from_authorization_code(config, code.to_string()).await?;
    */

    Ok(())
}
