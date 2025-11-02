// This chapter is dedicated to the error handling, tests and documentation.
//
#![allow(clippy::manual_is_multiple_of)]

// RESULT
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `first_char(text: &str) -> Result<char, String>` that returns the first
// character of a string or an error message "Empty string" if the string is empty.

pub fn first_char(text: &str) -> Result<char, String> {
    text.chars().next().ok_or(String::from("Empty string"))
}

// ----- 2 --------------------------------------
// Write a function `read_numbers_from_str(line: &str) -> Result<Vec<i32>, String>` that reads a
// line of integers separated by whitespace and parses each integer as i32. In case the value cannot
// be parsed (if it is not an integer) return the `Err("Invalid number")` result.

pub fn read_numbers_from_str(line: &str) -> Result<Vec<i32>, String> {
    let mut ans = Vec::<i32>::new();
    for s in line.split_whitespace() {
        ans.push(s.parse::<i32>().or(Err("Invalid number"))?);
    }
    Ok(ans)
}

// OPTION
// ================================================================================================

// ----- 3 --------------------------------------
// You have a struct `UserProfile` with fields `username: String` and `email: Option<String>`.
//
// Implement a method `get_email_domain(&self) -> Option<String>` that:
// - If the email exists, extracts the domain (the part after @).
// - If the email is missing, returns `None`.

// IMPLEMENT HERE:
pub struct UserProfile {
    #[allow(dead_code)]
    username: String,
    email: Option<String>,
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>) -> Self {
        UserProfile { username, email }
    }

    pub fn get_email_domain(&self) -> Option<String> {
        if let Some(em) = &self.email {
            em.split_terminator('@').skip(1).last().map(String::from)
        } else {
            None
        }
    }
}

// WRITING TESTS
// ================================================================================================

// ----- 4 --------------------------------------
// Write unit tests for the `factorial(n: u32) -> u64` function.

fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;
    // IMPLEMENT HERE:
    #[test]
    fn test_factorial_base_cases() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 2 * 3);
        assert_eq!(factorial(4), 2 * 3 * 4);
        assert_eq!(factorial(5), 2 * 3 * 4 * 5);
        assert_eq!(factorial(6), 2 * 3 * 4 * 5 * 6);
        assert_eq!(factorial(7), 2 * 3 * 4 * 5 * 6 * 7);
        assert_eq!(factorial(8), 2 * 3 * 4 * 5 * 6 * 7 * 8);
        assert_eq!(factorial(9), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9);
        assert_eq!(factorial(10), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10);
        assert_eq!(factorial(11), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11);
        assert_eq!(factorial(12), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12);
        assert_eq!(factorial(13), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13);
        assert_eq!(factorial(14), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14);
        assert_eq!(factorial(15), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14 * 15);
        assert_eq!(factorial(16), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14 * 15 * 16);
        assert_eq!(
            factorial(17),
            2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14 * 15 * 16 * 17
        );
        assert_eq!(
            factorial(18),
            2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14 * 15 * 16 * 17 * 18
        );
        assert_eq!(
            factorial(19),
            2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14 * 15 * 16 * 17 * 18 * 19
        );
        assert_eq!(
            factorial(20),
            2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14 * 15 * 16 * 17 * 18 * 19 * 20
        );
    }

    #[test]
    #[should_panic]
    fn overflow() {
        factorial(21);
    }
}

// ----- 5 --------------------------------------
// Write unit tests for the `is_prime(n: u64) -> bool` function checking both prime and non-prime
// numbers.

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod prime_tests {
    use super::*;

    // IMPLEMENT HERE:
    #[test]
    fn basic_prime_tests() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(is_prime(11));
        assert!(!is_prime(12));
        assert!(is_prime(13));
        assert!(!is_prime(13 * 7));
        assert!(is_prime(1_000_000_000 + 7));
        assert!(!is_prime((1_000_000_000 + 7) * (1_000_000_000 + 7)));
        assert!(!is_prime(u64::MAX - 1));
        assert!(!is_prime(u64::MAX));
    }
}

// WRITING DOCS
// ================================================================================================

// ----- 6 --------------------------------------
// You have an implemented `TemperatureLog` struct below, which stores a city name and a list of
// daily temperature readings. This struct have a constructor, an `add_reading` method which just
// ads a new value to the `readings` vector and an `average` method which returns an average value
// of the readings of there are some.
//
// Your task is to add doc comments:
// - High-level purpose of the struct.
// - Inline docs for each field and method.
//
// In case you want something more than хор(5):
// - Additionally white the usage example for the `TemperatureLog` in the high-level docs.
// - For the `average` method additionally write an example of its usage.

/// A log that stores daily temperature readings for a specific city.
///
/// This struct tracks the name of the city and maintains a list of temperature readings collected
/// over time. It provides methods to create a new log, add new temperature readings, and calculate
/// the average temperature.
///
/// # Examples
///
/// ```
/// let mut log = TemperatureLog::new("San Francisco");
/// log.add_reading(68.5);
/// log.add_reading(70.2);
/// log.add_reading(66.8);
/// assert_eq!(log.average(), Some(68.5));
/// ```
#[allow(dead_code)]
pub struct TemperatureLog {
    /// The name of the city for which temperatures are recorded.
    pub city: String,

    /// A list of daily temperature readings in degrees.
    pub readings: Vec<f64>,
}

#[allow(dead_code)]
impl TemperatureLog {
    /// Creates a new `TemperatureLog` for the given city.
    ///
    /// The log starts with an empty list of readings.
    ///
    /// # Examples
    ///
    /// ```
    /// let log = TemperatureLog::new("New York");
    /// assert_eq!(log.city, "New York");
    /// assert!(log.readings.is_empty());
    /// ```
    pub fn new(city: &str) -> Self {
        Self {
            city: city.to_string(),
            readings: Vec::new(),
        }
    }

    /// Adds a new temperature reading to the log.
    ///
    /// # Arguments
    ///
    /// * `value` - The temperature reading to add (in degrees).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut log = TemperatureLog::new("London");
    /// log.add_reading(55.5);
    /// assert_eq!(log.readings.len(), 1);
    /// ```
    pub fn add_reading(&mut self, value: f64) {
        self.readings.push(value);
    }

    /// Calculates the average temperature from the logged readings.
    ///
    /// Returns `None` if there are no readings.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut log = TemperatureLog::new("Tokyo");
    /// assert_eq!(log.average(), None);
    ///
    /// log.add_reading(60.0);
    /// log.add_reading(70.0);
    /// assert_eq!(log.average(), Some(65.0));
    /// ```
    pub fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }
        let sum_of_readings: f64 = self.readings.iter().sum();
        Some(sum_of_readings / self.readings.len() as f64)
    }
}
