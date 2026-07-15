use std::path::Path;

use super::commands;

fn line(label: &str, command: String) -> String {
    format!("{label}: {command}")
}

pub(super) fn fill_release_gates_line(path: &Path) -> String {
    line(
        "manual QA Fill release gates command",
        commands::fill_release_gates_command(path),
    )
}

pub(super) fn fill_local_proof_line(path: &Path, csv_path: &Path) -> String {
    line(
        "manual QA Fill local proof command",
        commands::fill_local_proof_command(path, csv_path),
    )
}

pub(super) fn ready_local_proof_line(path: &Path, csv_path: &Path) -> String {
    line(
        "manual QA Ready local proof command",
        commands::ready_local_proof_command(path, csv_path),
    )
}

pub(super) fn ready_all_line(path: &Path, csv_path: &Path) -> String {
    line(
        "manual QA Ready all command",
        commands::ready_all_command(path, csv_path),
    )
}

pub(super) fn ready_license_line(path: &Path) -> String {
    line("manual QA Ready license command", commands::ready_license_command(path))
}

pub(super) fn ready_distribution_line(path: &Path) -> String {
    line(
        "manual QA Ready distribution command",
        commands::ready_distribution_command(path),
    )
}

pub(super) fn fill_benchmark_line(path: &Path, csv_path: &Path) -> String {
    line(
        "manual QA Fill benchmark command",
        commands::fill_benchmark_command(path, csv_path),
    )
}

pub(super) fn fill_benchmark_threshold_line(path: &Path, csv_path: &Path) -> String {
    line(
        "manual QA Fill benchmark threshold command",
        commands::fill_benchmark_threshold_command(path, csv_path)
    )
}

pub(super) fn clean_draft_line(path: &Path) -> String {
    line("manual QA Clean draft command", commands::clean_draft_command(path))
}

pub(super) fn fill_check_line(path: &Path) -> String {
    line("manual QA Fill check command", commands::fill_check_command(path))
}

pub(super) fn pending_line(path: &Path) -> String {
    line("manual QA Pending command", commands::pending_command(path))
}

pub(super) fn pending_packaged_app_line(path: &Path) -> String {
    line(
        "manual QA Packaged App pending command",
        commands::pending_packaged_app_command(path),
    )
}

pub(super) fn pending_local_proof_line(path: &Path) -> String {
    line(
        "manual QA Local proof pending command",
        commands::pending_local_proof_command(path),
    )
}

pub(super) fn pending_license_line(path: &Path) -> String {
    line(
        "manual QA License pending command",
        commands::pending_license_command(path),
    )
}

pub(super) fn pending_benchmark_line(path: &Path) -> String {
    line(
        "manual QA Benchmark pending command",
        commands::pending_benchmark_command(path),
    )
}

pub(super) fn pending_distribution_line(path: &Path) -> String {
    line(
        "manual QA Distribution pending command",
        commands::pending_distribution_command(path),
    )
}

pub(super) fn print_helper_commands(path: &Path, csv_path: &Path) {
    println!("{}", fill_release_gates_line(path));
    println!("{}", fill_local_proof_line(path, csv_path));
    println!("{}", ready_local_proof_line(path, csv_path));
    println!("{}", ready_all_line(path, csv_path));
    println!("{}", ready_license_line(path));
    println!("{}", ready_distribution_line(path));
    println!("{}", fill_benchmark_line(path, csv_path));
    println!("{}", fill_benchmark_threshold_line(path, csv_path));
    println!("{}", clean_draft_line(path));
    println!("{}", pending_line(path));
    println!("{}", pending_packaged_app_line(path));
    println!("{}", pending_local_proof_line(path));
    println!("{}", pending_license_line(path));
    println!("{}", pending_benchmark_line(path));
    println!("{}", pending_distribution_line(path));
    println!("{}", fill_check_line(path));
}
