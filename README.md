# Liv's Spending Counter
## Description
This is a small tool I made to quickly parse through bank transaction logs to track specific spending. It will find all matches of a given regular expression in a transaction log and find the sum of the prices of all instances of that pattern.
The program assumes that transactions are newline-separated and that each dollar-value is preceded by `-$`. For instance, the line `BUSINESS.INC -$34.90 $9000.00` will be treated as if $34.90 were spent at BUSINESS.INC.
## Usage
```bash
git clone https://github.com/kibibyt3/spending_counter.git
cargo run -- <PATTERN> <PATH>
```
where <PATTERN> is a valid regular expression. All transactions are automatically made lowercase, so given a `file.txt` containing `BUSINESS.INC $3.00` must be parsed via e.g. `cargo run -- "business" file.txt`, rather than `cargo run -- "BUSINESS" file.txt`.
