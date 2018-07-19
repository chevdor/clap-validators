/// Ensure valid `u64` or `f64 > 0`
pub fn is_positive(v: String) -> Result<(), String> {
    if v.parse::<u64>().is_ok() {
        return Ok(());
    } else {
        if let Ok(f) = v.parse::<f64>() {
            if f > 0_f64 {
                return Ok(())
            }
        }
    }
    Err(format!("{} isn't a positive number", &*v))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_positive_true() {
        assert!(is_positive("42".to_string()).is_ok());
    }

    #[test]
    fn test_is_positive_false() {
        assert!(is_positive("-42".to_string()).is_err());
    }
 }