use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureDisposition {
    Retryable,
    Permanent,
}

pub fn retry_delay(attempt: u32, base: Duration, maximum: Duration) -> Duration {
    if attempt <= 1 || base.is_zero() {
        return base.min(maximum);
    }

    let shift = attempt.saturating_sub(1).min(31);
    let multiplier = 1_u32 << shift;
    base.saturating_mul(multiplier).min(maximum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_delay_grows_exponentially_and_is_bounded() {
        let base = Duration::from_millis(100);
        let maximum = Duration::from_secs(1);

        assert_eq!(retry_delay(1, base, maximum), base);
        assert_eq!(retry_delay(2, base, maximum), Duration::from_millis(200));
        assert_eq!(retry_delay(5, base, maximum), maximum);
        assert_eq!(retry_delay(100, base, maximum), maximum);
    }
}
