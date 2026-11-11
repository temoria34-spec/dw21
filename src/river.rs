/// Gives advice on how to cross a river based on depth.
///
/// ```
/// use dw21::river::crossing_advice;
///
/// assert_eq!(crossing_advice(2), "ford");
/// assert_eq!(crossing_advice(3), "caulk");
/// ```
///
/// ```
/// use dw21::river::crossing_advice;
///
/// assert_eq!(crossing_advice(5), "caulk");
/// assert_eq!(crossing_advice(6), "ferry");
/// ```
pub fn crossing_advice(depth_ft: u32) -> &'static str {
    match depth_ft {
        0..=2 => "ford",
        3..=5 => "caulk",
        _ => "ferry",
    }
}

/// Describes the food supply, in pounds. `None` means nobody counted it.
///
/// ```
/// use dw21::river::food_status;
///
/// assert_eq!(food_status(None), "no record");
/// assert_eq!(food_status(Some(0)), "out of food");
/// ```
///
/// ```
/// use dw21::river::food_status;
///
/// assert_eq!(food_status(Some(20)), "running low");
/// assert_eq!(food_status(Some(80)), "well stocked");
/// ```
pub fn food_status(pounds: Option<u32>) -> &'static str {
    match pounds {
        None => "no record",
        Some(0) => "out of food",
        Some(n) if n <= 50 => "running low",
        Some(_) => "well stocked",
    }
}
