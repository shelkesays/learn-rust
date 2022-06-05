fn main() {
    let days: [String; 12] = [
        "first".to_string(), "second".to_string(), "third".to_string(), "fourth".to_string(), "fifth".to_string(), "sixth".to_string(), 
        "seventh".to_string(), "eight".to_string(), "ninth".to_string(), "tenth".to_string(), "11th".to_string(), "12th".to_string()
    ];

    let first_line: String = "On the !day day of Christmas".to_string();

    let second_line: String = "My true love sent to me".to_string();

    let next_lines: [String; 12] = [
        "!change partridge in a pear tree".to_string(),
        "Two turtle-doves".to_string(),
        "Three French hens".to_string(),
        "Four calling birds".to_string(),
        "Five golden rings (five golden rings)".to_string(),
        "Six geese a-laying".to_string(),
        "Seven swans a-swimming".to_string(),
        "Eight maids a-milking".to_string(),
        "Nine ladies dancing".to_string(),
        "Ten lords a-leaping".to_string(),
        "Eleven pipers piping".to_string(),
        "12 drummers drumming".to_string()
    ];

    for num in 0..days.len() {
        let result = str::replace(&first_line, "!day", &days[num]);

        println!("{}", result);
        println!("{}", second_line);

        for line in (0..num+1).rev() {
            let mut current = "".to_string();
            if num != 0 {
                current = str::replace(&next_lines[line], "!change", "And a");
            } else {
                current = str::replace(&next_lines[line], "!change", "A");
            }
            
            println!("{}", current);
        }
        println!("");
    }
    
}
