mod options;
mod state;

use options::Options;
use state::{backup_state, restore_state, RESET_FILES};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let options = Options::parse(args)?;
    if options.restore_state {
        let restored = restore_state(&options)?;
        print_paths(&options);
        for file in restored {
            println!("restored: {file}");
        }
        return Ok(());
    }
    let copied = backup_state(&options)?;
    print_paths(&options);
    for file in copied {
        println!("copied: {file}");
    }
    if options.reset_trial {
        println!("trial state reset: {}", RESET_FILES.join(", "));
    }
    Ok(())
}

fn print_paths(options: &Options) {
    println!("manual QA state backup: {}", options.state_dir.display());
    println!("manual QA output folder: {}", options.output_dir.display());
    println!("app state source: {}", options.app_state_dir.display());
}

#[cfg(test)]
mod tests;
