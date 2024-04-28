mod account;
mod pretty_logger;
mod pretty_tui;

use account::Account;
use clap::{ArgAction, CommandFactory, Parser};
use clap_complete::{generate, Shell};
use log::{debug, error, info, Level, LevelFilter};
use pretty_logger::{CliLogger, TuiLogger};
use pretty_tui::Tui;
use std::{error::Error, io, sync::Arc};
use tokio::{sync::Mutex, task};
use tui::backend::CrosstermBackend;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "USERNAME")]
    username: Option<String>,

    #[arg(short, long, value_name = "MILEAGE")]
    mileage: Option<f64>,

    /// Print verbose messages.
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    #[arg(long, value_name = "SHELL")]
    completion: Option<Shell>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let (level, filter) = match cli.verbose {
        0 => (Level::Info, LevelFilter::Info),
        1 => (Level::Debug, LevelFilter::Debug),
        _ => (Level::Trace, LevelFilter::Trace),
    };

    if let Some(generator) = cli.completion {
        let mut cmd = Cli::command();
        let name = cmd.get_name().to_string();
        generate(generator, &mut cmd, name, &mut io::stdout());

        return Ok(());
    }

    if cli.username.is_some() {
        if cli.mileage.is_none() {
            error!("You must specify the mileage using `-m` when using the `-u`!");
            return Ok(());
        }
        let stdout = io::stdout();

        let logger = CliLogger::new(level, stdout);
        log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(filter))?;

        debug!(
            "{} level is enabled.",
            match cli.verbose {
                0 => "Info",
                1 => "Debug",
                _ => "Trace",
            }
        );

        let username = cli.username.unwrap();
        let password = rpassword::prompt_password(format!("Password for {}: ", username))?;
        let mileage = cli.mileage.unwrap();
        let mut account = Account::new();

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

        debug!(
            "{} level is enabled.",
            match cli.verbose {
                0 => "Info",
                1 => "Debug",
                _ => "Trace",
            }
        );

        let account_arc = Arc::new(Mutex::new(Account::new()));
        tui.welcome()?;
        let mut t = tui.main()?;
        loop {
            if t.is_none() {
                tui.quit()?;
                return Ok(());
            }
            let account = account_arc.clone();

            let (username, password, percent) = t.clone().unwrap();
            let mileage = percent as f64 * 7. / 100.;
            let task = task::spawn(async move {
                let account_arc = account.clone();
                let mut account = account_arc.lock().await;
                let username = username.clone();
                if username.is_empty() {
                    error!("Username cannot be empty!");
                    return;
                }
                if password.is_empty() {
                    error!("Password cannot be empty!");
                    return;
                }
                if mileage == 0. {
                    error!("Mileage cannot be zero!");
                    return;
                }
                if let Err(e) = account.login(username, password).await {
                    error!("Login failed! Message: {:?}", e);
                    return;
                }
                info!("Will running for {} miles.", mileage);
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
