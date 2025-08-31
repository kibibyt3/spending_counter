use std::fs::read_to_string;

use clap::Parser;
use regex::Regex;
use anyhow::{bail, Result};

#[derive(Parser)]
#[clap(name="Spending Counter", author="Liv Haze")]
struct Cli {
    pattern: String,
    path: String,
}

const PRICE_RE: &'static str = r"-\$\d+.\d{2}";

fn main() {
    let cli = Cli::parse();

    let contents = match read_to_string(cli.path.clone()) {
        Err(why) => panic!("Cannot read {}: {}", cli.path, why),
        Ok(s) => s,
    };
    let re = match Regex::new(&cli.pattern) {
        Err(why) =>
            panic!("Cannot parse regular expression {}: {}", cli.pattern, why),
        Ok(re) => re,
    };

    let entry_prices = match parse(&contents) {
        Err(why) => panic!("Cannot parse file {}: {}", cli.path, why),
        Ok(prices) => prices,
    };
    let price_sum = sum_matches(&entry_prices, &re);

    println!("The price for all matches of {} is: {:.2}", cli.pattern, price_sum);
}

fn parse(s: &str) -> Result<Vec<(String, f64)>> {
    let mut prices = Vec::new();
    let price_re = Regex::new(PRICE_RE).unwrap();

    for line in s.split('\n') {
        let matches: Vec<_> = price_re.find_iter(line).map(|needle| needle).collect();
        if matches.len() > 1 {
            bail!("the following line contains more than one price:\n{line}");
        } else if matches.len() == 1 {
            let range = matches[0].range();
            prices.push((
                line.to_string().to_ascii_lowercase(),
                line[(range.start + 2)..range.end].parse().unwrap(),
            ));
        }
    }

    Ok(prices)
}

fn sum_matches(entries: &Vec<(String, f64)>, re: &Regex) -> f64 {
    let mut sum = 0.0;
    for (entry, price) in entries {
        if re.is_match(&entry) {
            sum += price;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let s = "abcd -$32.30 $55\n\
            ehea 48629$3.3. -$11.20\n\
            -$3.00 ejejee";
        let expected = vec![
            ("abcd -$32.30 $55", 32.30),
            ("ehea 48629$3.3. -$11.20", 11.20),
            ("-$3.00 ejejee", 3.00),
        ];
        let actual = super::parse(s).unwrap();
        
        for i in 0..expected.len() {
            assert_eq!(expected[i].0, actual[i].0);
            assert!((expected[i].1 - actual[i].1).abs() < 0.01,
                "Expected: {}\nActual: {}", expected[i].1, actual[i].1,
            );
        }

        assert!(super::parse("-$33.20, -$31.30").is_err());
    }

    #[test]
    fn sum_matches() {
        let entries = vec![
            ("te st".to_string(), 1.0),
            ("test".to_string(), 2.0),
            ("test test test".to_string(), 4.0),
            ("tester".to_string(), 8.0),
            ("tesst".to_string(), 16.0),
            ("tast".to_string(), 32.0),
        ];
        let re = Regex::new("t[ae]st").unwrap();
        let actual = super::sum_matches(&entries, &re);
        let expected = 2.0 + 4.0 + 8.0 + 32.0;
        assert!((actual - expected).abs() < 0.01,
            "Actual: {actual}\nExpected: {expected}"
        );
    }

    #[test]
    fn regex() {
        let re = Regex::new(PRICE_RE).unwrap();
        assert!(re.is_match("-$332.01"));
        assert!(re.is_match("-$3.01"));
        assert!(re.is_match("-$33.01"));
    }
}
