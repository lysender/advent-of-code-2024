use std::path::Path;
use std::time::Duration;
use std::{fs, path::PathBuf};

pub fn get_puzzle_input(name: &str) -> String {
    let file = format!("day{}.txt", name);
    let filename: PathBuf = Path::new("..").join("data").join(file);
    fs::read_to_string(filename).unwrap()
}

pub fn format_duration(duration: Duration) -> String {
    let micros = duration.as_micros();
    if micros < 10_000 {
        return format!("{}µs", micros);
    }
    let millis = duration.as_millis();
    if millis < 10_000 {
        return format!("{}ms", millis);
    }
    let seconds = duration.as_secs();
    format!("{}s", seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_micros() {
        let duration = Duration::new(0, 400_000);
        assert_eq!(format_duration(duration), "400µs".to_string());
    }

    #[test]
    fn test_duration_millis() {
        let duration = Duration::new(1, 400_000_000);
        assert_eq!(format_duration(duration), "1400ms".to_string());
    }
}
