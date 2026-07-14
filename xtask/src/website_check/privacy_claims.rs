use std::path::Path;

const DISALLOWED_PHRASES: &[&str] = &[
    "guaranteed metadata removal",
    "guarantees metadata removal",
    "guarantees all metadata removal",
    "removes all metadata",
    "strips all metadata",
    "deletes all metadata",
    "erases all metadata",
    "metadata-free output",
    "metadata free output",
];

pub(super) fn check(path: &Path, text: &str, errors: &mut Vec<String>) {
    let lower = text.to_ascii_lowercase();
    for phrase in DISALLOWED_PHRASES {
        if lower.contains(phrase) {
            errors.push(format!(
                "{} contains unsupported metadata-removal claim: {phrase}",
                path.display()
            ));
        }
    }
}
