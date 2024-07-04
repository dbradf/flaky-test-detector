use crossterm::style::Stylize;
use rayon::prelude::*;
use std::{process::Command, time::Instant};

use clap::Parser;

/// Run tests repeatedly to find flakes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command to execute tests.
    #[arg(short, long)]
    test_command: String,

    /// Number of times to repeat tests.
    #[arg(short, long, default_value_t = 10)]
    repeat: usize,

    /// Test files.
    test_files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let now = Instant::now();

    let results: Vec<Vec<TestStatus>> = args
        .test_files
        .par_iter()
        .map(|test_file| {
            println!("Running tests in file: {}", test_file.clone().yellow());
            let start_time = Instant::now();
            let result = execute_tests_n_times(&args.test_command, test_file, args.repeat);
            println!(
                "Finished running tests in file: {} in {} ms",
                test_file.clone().yellow(),
                start_time.elapsed().as_millis()
            );

            result
        })
        .collect();

    let mut any_failures = false;
    for result in results {
        let was_success = report_results(&result);
        if !was_success {
            any_failures = true;
        }
    }

    println!(
        "\nAnalyzed {} tests in {} ms",
        args.test_files.len(),
        now.elapsed().as_millis()
    );

    if any_failures {
        std::process::exit(1);
    }
}

enum TestResult {
    Passed,
    Failed { stdout: String, stderr: String },
}

struct TestStatus {
    test_file: String,
    duration: u128,
    result: TestResult,
}

fn execute_tests_n_times(test_command: &str, test_file: &str, repeat: usize) -> Vec<TestStatus> {
    (0..repeat)
        .map(|_| execute_tests(test_command, test_file))
        .collect()
}

fn execute_tests(test_command: &str, test_file: &str) -> TestStatus {
    let command = test_command.replace("{}", test_file);
    let split_command = command.split_whitespace().collect::<Vec<_>>();
    let binary = split_command[0];
    let args = split_command[1..].to_vec();

    let start = Instant::now();
    let cmd = Command::new(binary).args(args).output().unwrap();

    let duration = start.elapsed();

    TestStatus {
        test_file: test_file.to_string(),
        duration: duration.as_millis(),
        result: match cmd.status.success() {
            true => TestResult::Passed,
            false => TestResult::Failed {
                stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
                stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
            },
        },
    }
}

fn report_results(results: &[TestStatus]) -> bool {
    let test_name = &results[0].test_file;
    let avg_duration =
        results.iter().map(|result| result.duration).sum::<u128>() / results.len() as u128;
    let failure_details = results
        .iter()
        .filter_map(|result| match &result.result {
            TestResult::Failed { stdout, stderr } => Some((stdout, stderr)),
            _ => None,
        })
        .collect::<Vec<_>>();

    let failure_percentage = failure_details.len() * 100 / results.len();
    if failure_percentage > 0 {
        println!(
            "Test: {} [avg runtime: {} ms]",
            test_name.clone().red(),
            avg_duration
        );
        println!(
            "Fail percentage: {}",
            format!("{}%", failure_percentage).red()
        );
        false
    } else {
        println!(
            "Test: {} [avg runtime: {} ms]",
            test_name.clone().green(),
            avg_duration
        );
        println!(
            "Fail percentage: {}",
            format!("{}%", failure_percentage).green()
        );
        true
    }
}
