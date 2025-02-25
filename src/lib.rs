#[derive(Debug, PartialEq)]
pub enum LexRankError {
    NegativeValue,
}

pub fn complement_reciprocal_unity_translate(x: f64) -> Result<f64, LexRankError> {
    if x < 0.0 {
        return Err(LexRankError::NegativeValue);
    }
    Ok(1.0 - 1.0 / (x + 1.0))
}

pub fn tonyk_lex_rank(
    values: Vec<f64>,
    monotone_function: Option<fn(f64) -> Result<f64, LexRankError>>,
) -> Result<f64, LexRankError> {
    let func = monotone_function.unwrap_or(complement_reciprocal_unity_translate);

    // Check if any value in `values` is negative
    if values.iter().any(|&v| v < 0.0) {
        return Err(LexRankError::NegativeValue);
    }

    let mut x = func(values[0])?; // Unwrap the result
    let mut w: f64;

    for value in values.iter().skip(1) {
        w = func(x + 1.0)? - x; // Unwrap the result
        x = x + w * func(*value)?; // Unwrap the result
    }

    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complement_reciprocal_unity_translate() {
        assert_eq!(complement_reciprocal_unity_translate(0.0), Ok(0.5));
        assert_eq!(
            complement_reciprocal_unity_translate(1.0),
            Ok(0.3333333333333333)
        );
        assert_eq!(complement_reciprocal_unity_translate(2.0), Ok(0.25));
        assert_eq!(
            complement_reciprocal_unity_translate(-1.0),
            Err(LexRankError::NegativeValue)
        );
    }

    #[test]
    fn test_tonyk_lex_rank_with_default_function() {
        let values = vec![1.0, 2.0, 3.0];
        // x = 1 - 1 / 2 = 1 / 2
        // w =
        let result = tonyk_lex_rank(values, None);
        assert_eq!(result, Ok(1.0));

        let values_single = vec![1.0];
        let result_single = tonyk_lex_rank(values_single, None);
        assert_eq!(result_single, Ok(0.5));

        let values_empty: Vec<f64> = Vec::new();
        let result_empty = tonyk_lex_rank(values_empty, None);
        assert!(result_empty.is_err(), "Expected error for empty vector");

        let values_negative = vec![-1.0, 2.0, 3.0];
        let result_negative = tonyk_lex_rank(values_negative, None);
        assert_eq!(result_negative, Err(LexRankError::NegativeValue));
    }

    #[test]
    fn test_tonyk_lex_rank_with_custom_function() {
        let custom_function = |x: f64| -> Result<f64, LexRankError> {
            if x < 0.0 {
                Err(LexRankError::NegativeValue)
            } else {
                Ok(x * 2.0)
            }
        };

        let values = vec![1.0, 2.0, 3.0];
        let result = tonyk_lex_rank(values, Some(custom_function));
        assert_eq!(result, Ok(6.0));

        let values_single = vec![1.0];
        let result_single = tonyk_lex_rank(values_single, Some(custom_function));
        assert_eq!(result_single, Ok(2.0));

        let values_empty: Vec<f64> = Vec::new();
        let result_empty = tonyk_lex_rank(values_empty, Some(custom_function));
        assert!(result_empty.is_err(), "Expected error for empty vector");

        let values_negative = vec![-1.0, 2.0, 3.0];
        let result_negative = tonyk_lex_rank(values_negative, Some(custom_function));
        assert_eq!(result_negative, Err(LexRankError::NegativeValue));
    }
}
