use std::path::Path;

use super::{
    bad_input_line, clean_draft_line, fill_benchmark_line, fill_benchmark_threshold_line,
    fill_check_line, fill_local_proof_line, fill_release_gates_line, pending_benchmark_line,
    pending_distribution_line, pending_license_line, pending_line, pending_local_proof_line,
    pending_packaged_app_line, ready_all_line, ready_distribution_line, ready_license_line,
    ready_local_proof_line,
};

pub(super) fn print_helper_commands(path: &Path, csv_path: &Path) {
    println!("{}", fill_release_gates_line(path));
    println!("{}", fill_local_proof_line(path, csv_path));
    println!("{}", ready_local_proof_line(path, csv_path));
    println!("{}", ready_all_line(path, csv_path));
    println!("{}", ready_license_line(path));
    println!("{}", ready_distribution_line(path));
    println!("{}", bad_input_line());
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
