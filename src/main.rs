mod account;
mod pretty_logger;
mod pretty_tui;

use account::Account;
use clap::Parser;
use log::{error, Level, LevelFilter};
use pretty_logger::{CliLogger, TuiLogger};
use pretty_tui::Tui;
use std::{error::Error, io, sync::Arc};
use tokio::task;
use tui::backend::CrosstermBackend;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "USERNAME")]
    username: Option<String>,

    #[arg(short, long, value_name = "MILEAGE")]
    mileage: Option<f64>,

    /// Print verbose messages.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let (level, filter) = match cli.verbose {
        0 => (Level::Info, LevelFilter::Info),
        1 => (Level::Debug, LevelFilter::Debug),
        _ => (Level::Trace, LevelFilter::Trace),
    };

    // TODO: Add the debug mode prompt
    if cli.username.is_some() {
        if cli.mileage.is_none() {
            error!("You must specify the mileage using `-m` when using the `-u`!");
            return Ok(());
        }
        let stdout = io::stdout();

        let logger = CliLogger::new(level, stdout);
        log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(filter))?;

        let username = cli.username.unwrap();
        let password = rpassword::prompt_password(format!("Password for {}: ", username))?;
        let mileage = cli.mileage.unwrap();
        let mut account = Account::new().unwrap();

        if let Err(e) = account.login(username, password).await {
            error!("Login failed! Message: {:?}", e);
            return Ok(());
        }
        if let Err(e) = account.upload_running(mileage, None).await {
            error!("Upload running failed! Message: {:?}", e);
        }
    } else {
        let stdout = io::stdout();

        let backend = CrosstermBackend::new(stdout);
        let logger = Arc::new(TuiLogger::new(level));
        let mut tui = Tui::new(backend, logger.clone())?;
        log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(filter))?;
        tui.welcome()?;
        let mut t = tui.main()?;
        loop {
            if t.is_none() {
                tui.quit()?;
                return Ok(());
            }

            let (username, password, percent) = t.clone().unwrap();
            let mileage = percent as f64 * 5. / 100.;
            let task = task::spawn(async move {
                let mut account = Account::new().unwrap();
                let username = username.clone();
                if let Err(e) = account.login(username, password).await {
                    error!("Login failed! Message: {:?}", e);
                    return;
                }
                if let Err(e) = account.upload_running(mileage, None).await {
                    error!("Upload running failed! Message: {:?}", e);
                }
            });

            t = tui.main()?;
            task.await?;
        }
    }

    Ok(())
}
