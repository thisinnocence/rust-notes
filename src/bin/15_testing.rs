fn clamp(v: i32, low: i32, high: i32) -> i32 {
    if v < low {
        low
    } else if v > high {
        high
    } else {
        v
    }
}

fn main() {
    println!("clamp(15, 0, 10)={}", clamp(15, 0, 10));
}

#[cfg(test)]
mod tests {
    use super::clamp;

    #[test]
    fn test_clamp_low() {
        assert_eq!(clamp(-1, 0, 10), 0);
    }

    #[test]
    fn test_clamp_mid() {
        assert_eq!(clamp(5, 0, 10), 5);
    }

    #[test]
    fn test_clamp_high() {
        assert_eq!(clamp(15, 0, 10), 10);
    }
}
