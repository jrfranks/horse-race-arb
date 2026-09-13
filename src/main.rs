//! CLI for the horse-race surebet scanner.
//!
//! Alerts + stake sizes only — never places bets.

use clap::{Parser, Subcommand};
use race_arb::adapters::{JsonFileAdapter, OddsAdapter, XpressbetStub};
use race_arb::scan::{run_demo, scan_adapters};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "race-arb",
    about = "Horse-race surebet / odds arbitrage scanner (alerts only; never places bets)",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the built-in MockBookA / MockBookB demo (intentional misprices → surebets).
    Demo,
    /// Scan odds from a local JSON file (and optional Xpressbet stub snapshot).
    Scan {
        /// Path to odds JSON (generic schema — see README / fixtures).
        #[arg(long)]
        odds: PathBuf,
        /// Optional local Xpressbet JSON snapshot (no live scrape).
        #[arg(long)]
        xpressbet: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Commands::Demo => match run_demo() {
            Ok(bets) => {
                print_results(&bets);
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("demo failed: {e}");
                ExitCode::FAILURE
            }
        },
        Commands::Scan { odds, xpressbet } => run_scan(odds, xpressbet),
    }
}

fn run_scan(odds: PathBuf, xpressbet: Option<PathBuf>) -> ExitCode {
    let json = JsonFileAdapter::new(&odds);
    match xpressbet {
        Some(path) => {
            let xb = XpressbetStub::from_local_json(path);
            let adapters: Vec<&dyn OddsAdapter> = vec![&json, &xb];
            finish_scan(scan_adapters(&adapters))
        }
        None => {
            let adapters: Vec<&dyn OddsAdapter> = vec![&json];
            finish_scan(scan_adapters(&adapters))
        }
    }
}

fn finish_scan(result: Result<Vec<race_arb::Surebet>, String>) -> ExitCode {
    match result {
        Ok(bets) => {
            print_results(&bets);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("scan failed: {e}");
            ExitCode::FAILURE
        }
    }
}

fn print_results(bets: &[race_arb::Surebet]) {
    if bets.is_empty() {
        println!("No surebets found.");
        println!("(Scanned markets: win, horse_vs_field. Place/Show stubbed.)");
        return;
    }
    println!("Found {} surebet(s):\n", bets.len());
    for (i, sb) in bets.iter().enumerate() {
        println!("─── #{} ───", i + 1);
        print!("{}", sb.format_alert());
        println!();
    }
}
