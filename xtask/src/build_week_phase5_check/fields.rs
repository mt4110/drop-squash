use serde_json::Value;

pub(super) fn required_bool(value: &Value, pointer: &str, expected: bool) -> Result<(), String> {
    match value.pointer(pointer).and_then(Value::as_bool) {
        Some(actual) if actual == expected => Ok(()),
        actual => Err(format!("{pointer} must be {expected}, found {actual:?}")),
    }
}

pub(super) fn required_string(value: &Value, pointer: &str, expected: &str) -> Result<(), String> {
    match value.pointer(pointer).and_then(Value::as_str) {
        Some(actual) if actual == expected => Ok(()),
        actual => Err(format!("{pointer} must be {expected:?}, found {actual:?}")),
    }
}

pub(super) fn positive_u64(value: &Value, pointer: &str) -> Result<(), String> {
    match value.pointer(pointer).and_then(Value::as_u64) {
        Some(actual) if actual > 0 => Ok(()),
        actual => Err(format!("{pointer} must be positive, found {actual:?}")),
    }
}

pub(super) fn equal_u64(value: &Value, left: &str, right: &str) -> Result<(), String> {
    let left_value = value.pointer(left).and_then(Value::as_u64);
    let right_value = value.pointer(right).and_then(Value::as_u64);
    if left_value == right_value && left_value.is_some() {
        Ok(())
    } else {
        Err(format!(
            "{left} and {right} must match, found {left_value:?} and {right_value:?}"
        ))
    }
}

pub(super) fn required_array_item(
    value: &Value,
    pointer: &str,
    expected: &str,
) -> Result<(), String> {
    let found = value
        .pointer(pointer)
        .and_then(Value::as_array)
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(expected)));
    found
        .then_some(())
        .ok_or_else(|| format!("{pointer} must include {expected:?}"))
}
