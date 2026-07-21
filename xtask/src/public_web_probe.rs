use std::process::Command;

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- public-web-probe";
const OWNER_ONLY: &str = "https://dropsquash-app.system-obj-gg.chatgpt.site/release-status";
const CANONICAL: &str = "https://dropsquash.app/release-status";
const PRICING: &str = "https://dropsquash.app/pricing";
const REFUND: &str = "https://dropsquash.app/refund";
const APEX_TARGETS: &str = "162.159.143.30 172.66.3.26";
const OPENAI_VERIFY: &str = "openai-site-verification=1B5z4jz2Z2ifY2MsN3UggSHPMaSsaGwrOgttmLnIw6g";
const CF_VERIFY: &str = "c70e75c8-8887-4b4a-a390-72fd4a8400fd";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Err(USAGE.to_string());
    }
    let dns_a = probe_dns("dropsquash.app", "A");
    let owner_only = run_command("curl", &["-I", "-sS", OWNER_ONLY]);
    let canonical = run_command("curl", &["-I", "-sS", CANONICAL]);
    let pricing = run_command("curl", &["-I", "-sS", PRICING]);
    let refund = run_command("curl", &["-I", "-sS", REFUND]);
    print_block("public web probe date", &crate::current_date::display());
    print_block("dns A", &dns_a);
    print_block("expected dns A", APEX_TARGETS);
    print_block(
        "dns TXT verification",
        &probe_dns("_openai-site-verification.dropsquash.app", "TXT"),
    );
    print_block("expected TXT verification", OPENAI_VERIFY);
    print_block(
        "dns TXT custom-hostname",
        &probe_dns("_cf-custom-hostname.dropsquash.app", "TXT"),
    );
    print_block("expected TXT custom-hostname", CF_VERIFY);
    print_block("owner-only HTTP", &owner_only);
    print_block("canonical HTTP", &canonical);
    print_block("pricing HTTP", &pricing);
    print_block("refund HTTP", &refund);
    print_block("owner-only status", owner_only_status(&owner_only));
    print_block("expected domain status", "active");
    print_block(
        "canonical status",
        canonical_status(&dns_a, &[&canonical, &pricing, &refund]),
    );
    print_block(
        "blocker state",
        blocker_state(&dns_a, &[&canonical, &pricing, &refund]),
    );
    Ok(())
}

fn print_block(label: &str, value: &str) {
    println!("{label}: {}", summarize(value));
}

fn summarize(value: &str) -> String {
    let line = value
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with(";;") && !line.starts_with("Server:"));
    let Some(line) = line else {
        return "no output".to_string();
    };
    line.to_string()
}

fn probe_dns(name: &str, record: &str) -> String {
    let dig = run_command("dig", &["+short", name, record]);
    if summarize(&dig) != "no output" {
        return dig;
    }
    run_command("host", &["-t", record, name])
}

fn owner_only_status(owner_only: &str) -> &'static str {
    if summarize(owner_only).starts_with("HTTP/2 401") {
        "owner-only deploy alive"
    } else {
        "recheck owner-only deploy manually"
    }
}

fn canonical_status(dns_a: &str, pages: &[&str]) -> &'static str {
    let dns = summarize(dns_a);
    if dns.contains("NXDOMAIN")
        || pages
            .iter()
            .map(|page| summarize(page))
            .any(|http| http.contains("Could not resolve host"))
    {
        "canonical host not ready"
    } else {
        "canonical host may be ready; verify release-status, pricing, and refund pages"
    }
}

fn blocker_state(dns_a: &str, pages: &[&str]) -> &'static str {
    if canonical_status(dns_a, pages) == "canonical host not ready" {
        "keep public-web blockers open"
    } else {
        "check release-status, pricing, and refund production URLs before moving blockers"
    }
}

fn run_command(bin: &str, args: &[&str]) -> String {
    match Command::new(bin).args(args).output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).into_owned()
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stderr.trim().is_empty() {
                stdout.into_owned()
            } else {
                stderr.into_owned()
            }
        }
        Err(error) => format!("{bin} unavailable: {error}"),
    }
}
