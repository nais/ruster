
/// Some data struct
struct Example {
    value: usize,
}

/// Imperative iteration, Go-like syntax.
fn extract_member(values: Vec<Example>) -> Vec<usize> {
    let mut my_values = vec![];
    for i in values {
        my_values.push(i.value);
    }
    my_values
}

/// Functional syntax.
fn extract_member_functional(values: Vec<Example>) -> Vec<usize> {
    values.iter().map(|i| i.value).collect()
}

