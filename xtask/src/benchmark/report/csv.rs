use super::BenchmarkRow;

pub(super) fn from_rows(rows: &[BenchmarkRow]) -> String {
    let mut output = String::from(
        "backend,input,output,original_bytes,output_bytes,duration_s,elapsed_s,compression_ratio,saved_percent,throughput_mib_s,speed_ratio\n",
    );
    for row in rows {
        output.push_str(&format!(
            "{},{},{},{},{},{},{:.3},{:.6},{:.6},{:.3},{}\n",
            cell(&row.backend),
            cell(&row.input),
            cell(&row.output),
            row.original_bytes,
            row.output_bytes,
            optional_f64(row.duration.map(|duration| duration.as_secs_f64())),
            row.elapsed.as_secs_f64(),
            row.compression_ratio(),
            row.saved_percent(),
            row.throughput_mib_s(),
            optional_f64(row.speed_ratio())
        ));
    }
    output
}

fn optional_f64(value: Option<f64>) -> String {
    value.map_or_else(String::new, |value| format!("{value:.3}"))
}

fn cell(value: &str) -> String {
    if !value.contains([',', '"', '\n']) {
        return value.to_string();
    }
    format!("\"{}\"", value.replace('"', "\"\""))
}
