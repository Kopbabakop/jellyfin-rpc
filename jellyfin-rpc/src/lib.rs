pub mod prelude;
pub mod services;
pub use crate::core::error;
#[cfg(feature = "imgur")]
pub use crate::services::imgur;
use discord_rich_presence::DiscordIpc;
use discord_rich_presence::DiscordIpcClient;
use retry::retry_with_index;
pub mod core;
pub use core::rpc::setactivity;
#[cfg(test)]
mod tests;

#[cfg(not(feature = "cli"))]
pub fn connect(rich_presence_client: &mut DiscordIpcClient) {
    retry_with_index(
        retry::delay::Exponential::from_millis(1000),
        |_| match rich_presence_client.connect() {
            Ok(result) => retry::OperationResult::Ok(result),
            Err(_) => retry::OperationResult::Retry(()),
        },
    )
    .unwrap();
}

#[cfg(feature = "cli")]
pub fn connect(rich_presence_client: &mut DiscordIpcClient) {
    use colored::Colorize;
    println!(
        "{}",
        "------------------------------------------------------------------".bold()
    );
    retry_with_index(
        retry::delay::Exponential::from_millis(1000),
        |current_try| {
            println!(
                "{} {}{}",
                "Attempt".bold().truecolor(225, 69, 0),
                current_try.to_string().bold().truecolor(225, 69, 0),
                ": Trying to connect".bold().truecolor(225, 69, 0)
            );
            match rich_presence_client.connect() {
                Ok(result) => retry::OperationResult::Ok(result),
                Err(err) => {
                    eprintln!(
                        "{}\nError: {}",
                        "Failed to connect, retrying soon".red().bold(),
                        err
                    );
                    retry::OperationResult::Retry(())
                }
            }
        },
    )
    .unwrap();
}
