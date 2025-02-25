pub fn complement_reciprocal_unity_translate(x: f64) -> f64 {
    1.0 - 1.0 / (x + 1.0)
}

pub fn tonyk_lex_rank(values: Vec<f64>, monotone_function: Option<fn(f64) -> f64>) -> f64 {
    let func = monotone_function.unwrap_or(complement_reciprocal_unity_translate);

    let mut x: f64 = func(values[0]);
    let mut w: f64;

    for value in values.iter().skip(1) {
        w = func(x + 1.0) - x; // Update the weight `w`
        x = x + w * func(*value); // Update x directly with the weight `w`
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Finish writing test suite.

    #[test]
    fn it_works() {
        assert_eq!(5, 4); // FIX: This is just a placeholder that fails on purpose.
    }
}
