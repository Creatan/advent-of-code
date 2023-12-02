use regex::Regex;
fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    println!("{}", output);
}

fn process(input: &str) -> usize {
    return input.lines().map(|line|{
        let re = Regex::new(r"[^0-9]").unwrap();
        let decimals = re.replace_all(line, "");
        let mut chars = decimals.chars();
        let first = chars.nth(0).unwrap();
        let last = chars.nth_back(0);
        if let None = last {
            return format!("{first}{first}").parse::<usize>().unwrap();
        }
        let last_value = last.unwrap();
        return format!("{first}{last_value}").parse::<usize>().unwrap();
    }).sum();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let result = process("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
); 
        assert_eq!(result, 142)
    }
}
