// https://www.hackerrank.com/challenges/staircase/problem
pub fn staircase(n: i32) -> Vec<String> {
    let mut result = Vec::new();

    for i in 1..=n {
        let spaces = (n - i) as usize;
        let hashes = i as usize;

        let line = format!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
        result.push(line);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_4() {
        let expected = vec![
            "   #".to_string(),
            "  ##".to_string(),
            " ###".to_string(),
            "####".to_string(),
        ];

        assert_eq!(staircase(4), expected);
    }

    #[test]
    fn test_staircase_1() {
        let expected = vec!["#".to_string()];
        assert_eq!(staircase(1), expected);
    }
}
