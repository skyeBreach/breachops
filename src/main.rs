use std::process::ExitCode;

use anyhow::Result;

fn main() -> ExitCode {
    let res_code = match run() {
        Ok(res_code) => res_code,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    };

    res_code
}

fn run() -> Result<ExitCode> {
    Ok(ExitCode::SUCCESS)
}
