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
