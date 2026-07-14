use std::path::Path;

use super::{
    artifact::qa_artifact, benchmark, build_identity::BuildIdentity, commands,
    environment::Environment, license_sandbox, markdown, markdown::Field, options::Options,
    output_helper, packaged_app, prepared_markdown, release_candidate,
};

pub(super) fn print_paths(options: &Options) -> Result<(), String> {
    print_basic_paths(options);
    let artifact = qa_artifact(options)?;
    let mut fields: Vec<Field> = Vec::new();
    let identity = BuildIdentity::current_for_artifact(artifact.as_deref())?;
    let app_build = identity.app_build();
    println!("manual QA App build: {app_build}");
    fields.push(("App build", app_build));
    print_artifact(&artifact, &mut fields);
    print_open_artifact(&artifact);
    print_sample_set(options, &mut fields);
    benchmark::print_plan(options);
    let environment = Environment::current(options)?;
    for line in environment.manual_qa_lines() {
        println!("{line}");
    }
    fields.extend(environment.manual_qa_fields());
    markdown::print_fields(&fields);
    packaged_app::print_rows();
    license_sandbox::print_plan();
    if let Some(path) = &artifact {
        release_candidate::print_rows(path, &options.output_dir)?;
    }
    if let Some(path) = &options.markdown_output {
        prepared_markdown::write(path, &fields, artifact.as_deref(), options)?;
        print_manual_check(path);
        output_helper::print_helper_commands(path, &options.output_dir);
    }
    Ok(())
}

pub(super) fn print_basic_paths(options: &Options) {
    println!("manual QA state backup: {}", options.state_dir.display());
    println!("manual QA output folder: {}", options.output_dir.display());
    println!("app state source: {}", options.app_state_dir.display());
}

pub(super) fn require_ready(options: &Options) -> Result<(), String> {
    let artifact = qa_artifact(options)?;
    BuildIdentity::current_for_artifact(artifact.as_deref()).map(|_| ())
}

fn print_artifact(artifact: &Option<std::path::PathBuf>, fields: &mut Vec<Field>) {
    if let Some(path) = artifact {
        println!("manual QA App artifact: {}", path.display());
        fields.push(("App artifact", path.display().to_string()));
    } else {
        println!("manual QA App artifact unavailable: pass --app-artifact <path>");
    }
}

fn print_sample_set(options: &Options, fields: &mut Vec<Field>) {
    println!("{}", sample_set_line(options));
    if let Some(sample_set) = &options.input_sample_set {
        fields.push(("Input sample set", sample_set.clone()));
    }
}

pub(super) fn sample_set_line(options: &Options) -> String {
    match &options.input_sample_set {
        Some(sample_set) => format!("manual QA Input sample set: {sample_set}"),
        None => {
            "manual QA Input sample set unavailable: pass --input-sample-set <text>".to_string()
        }
    }
}

pub(super) fn manual_check_line(path: &Path) -> String {
    format!(
        "manual QA Check command: {}",
        commands::manual_check_command(path)
    )
}

pub(super) fn open_artifact_line(path: &Path) -> Option<String> {
    if path.extension().and_then(|value| value.to_str()) != Some("dmg") {
        return None;
    }
    Some(format!(
        "manual QA Open artifact command: {}",
        commands::open_dmg_command(path)
    ))
}

fn print_open_artifact(artifact: &Option<std::path::PathBuf>) {
    if let Some(line) = artifact.as_deref().and_then(open_artifact_line) {
        println!("{line}");
    }
}

fn print_manual_check(path: &Path) {
    println!("{}", manual_check_line(path));
}
