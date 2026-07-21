use serde_json::Value;

const PATH: &str = "tests/fixtures/secure-share/native-accessibility-fixture.annotation.json";

pub(super) fn check() -> Result<(), String> {
    let text = std::fs::read_to_string(PATH).map_err(|error| error.to_string())?;
    let value: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    if value.pointer("/acceptance/automatic_scenario_delay_seconds") != Some(&Value::from(10)) {
        return Err("native fixture must wait ten seconds before an automatic action".to_string());
    }
    for scenario in ["baseline", "rapid-burst", "input-burst", "popover", "sheet"] {
        require(&value, scenario, "strict_black_publish", "required")?;
    }
    require(&value, "title", "detected_but_not_covered", "allowed")?;
    for scenario in [
        "hide",
        "resize",
        "move",
        "move-return",
        "focus-loss",
        "same-app-focus",
    ] {
        require(&value, scenario, "fail_closed", "forbidden")?;
    }
    Ok(())
}

fn require(
    value: &Value,
    scenario: &str,
    classification: &str,
    artifact: &str,
) -> Result<(), String> {
    let path = format!("/scenario_contracts/{scenario}");
    let row = value
        .pointer(&path)
        .ok_or_else(|| format!("missing fixture contract: {scenario}"))?;
    let actual = row.get("classification").and_then(Value::as_str);
    if actual != Some(classification) {
        return Err(format!(
            "fixture classification for {scenario} must be {classification}"
        ));
    }
    let actual = row.get("final_artifact").and_then(Value::as_str);
    (actual == Some(artifact))
        .then_some(())
        .ok_or_else(|| format!("fixture artifact rule for {scenario} must be {artifact}"))
}
