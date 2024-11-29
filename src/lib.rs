/// Constants for Persian number representations
const YEKAN: [&str; 9] = ["یک", "دو", "سه", "چهار", "پنج", "شش", "هفت", "هشت", "نه"];
const DAHGAN: [&str; 8] = ["بیست", "سی", "چهل", "پنجاه", "شصت", "هفتاد", "هشتاد", "نود"];
const SADGAN: [&str; 9] = [
    "یکصد",
    "دویست",
    "سیصد",
    "چهارصد",
    "پانصد",
    "ششصد",
    "هفتصد",
    "هشتصد",
    "نهصد",
];
const DAH: [&str; 10] = [
    "ده",
    "یازده",
    "دوازده",
    "سیزده",
    "چهارده",
    "پانزده",
    "شانزده",
    "هفده",
    "هیجده",
    "نوزده",
];

/// Converts a number to its Persian representation
///
/// # Arguments
///
/// * `num` - The number to convert
/// * `level` - The current level of recursion for formatting
/// * `is_ordinal` - Whether to add an ordinal suffix
///
/// # Returns
///
/// A `String` containing the Persian representation of the number
pub fn num_to_persian(num: i64, level: &mut i32, is_ordinal: bool) -> String {
    if num == 0 {
        if *level == 0 {
            return String::from("صفر");
        } else {
            return String::new();
        }
    }

    if num < 0 {
        return format!("منفی {}", num_to_persian(-num, level, is_ordinal));
    }

    let mut result = String::new();
    let mut tmp1 = *level + 1;

    if *level > 0 {
        result.push_str(" و ");
        *level -= 1;
    }

    if num < 10 {
        result.push_str(YEKAN[(num - 1) as usize]);
    } else if num < 20 {
        result.push_str(DAH[(num - 10) as usize]);
    } else if num < 100 {
        result.push_str(DAHGAN[(num / 10 - 2) as usize]);
        result.push_str(&num_to_persian(num % 10, &mut tmp1, is_ordinal));
    } else if num < 1000 {
        result.push_str(SADGAN[(num / 100 - 1) as usize]);
        result.push_str(&num_to_persian(num % 100, &mut tmp1, is_ordinal));
    } else if num < 1000000 {
        result.push_str(&num_to_persian(num / 1000, level, is_ordinal));
        result.push_str(" هزار");
        result.push_str(&num_to_persian(num % 1000, &mut tmp1, is_ordinal));
    } else if num < 1000000000 {
        result.push_str(&num_to_persian(num / 1000000, level, is_ordinal));
        result.push_str(" میلیون");
        result.push_str(&num_to_persian(num % 1000000, &mut tmp1, is_ordinal));
    } else if num < 1000000000000 {
        result.push_str(&num_to_persian(num / 1000000000, level, is_ordinal));
        result.push_str(" میلیارد");
        result.push_str(&num_to_persian(num % 1000000000, &mut tmp1, is_ordinal));
    } else if num < 1000000000000000 {
        result.push_str(&num_to_persian(num / 1000000000000, level, is_ordinal));
        result.push_str(" تریلیارد");
        result.push_str(&num_to_persian(num % 1000000000000, &mut tmp1, is_ordinal));
    }

    if is_ordinal {
        result.push_str(&add_ordinal_suffix(&result));
    }

    result
}

/// Adds an ordinal suffix to a Persian number string
///
/// # Arguments
///
/// * `number` - The Persian number string
///
/// # Returns
///
/// A `String` with the appropriate ordinal suffix
pub fn add_ordinal_suffix(number: &str) -> String {
    if number.ends_with("ی") {
        return format!("{} اُم", number);
    } else if number.ends_with("سه") {
        return format!("{}سوم", &number[0..number.len() - 2]);
    }
    format!("{}م", number)
}

