use std::process::ExitCode;

use anyhow::Result;

fn main() -> ExitCode {
    match run() {
        Ok(res_code) => res_code,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {

    Ok(ExitCode::SUCCESS)
}
