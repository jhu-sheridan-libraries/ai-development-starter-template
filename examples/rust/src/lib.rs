//! Minimal example crate for the libraries-ai-starter Rust reference path.
//!
//! Replace this with your actual project code once you've promoted this
//! example into `src/` (see `TEMPLATE_SETUP.md` at the repository root).

/// Returns a friendly greeting.
///
/// # Errors
///
/// Returns an error if `name` is empty.
pub fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("name must not be empty".to_string());
    }
    Ok(format!("Hello, {name}!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_expected_message() {
        assert_eq!(greet("Sheridan").unwrap(), "Hello, Sheridan!");
    }

    #[test]
    fn greet_rejects_empty_name() {
        assert!(greet("").is_err());
    }
}
