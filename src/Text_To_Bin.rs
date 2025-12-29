use std::io::{self, Write};

fn main() {
    println!();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        let input = input.trim();

        let hex_string = text_to_hex(input);
        println!(": {}", hex_string);

        let binary_string = text_to_binary(input);
        println!(": {}", binary_string);

        let diamond_string = binary_to_diamond(&binary_string);
        println!(": {}", diamond_string);
        println!();
    }
}

fn text_to_binary(text: &str) -> String {
    text.chars()
        .map(|c| {
            let code = c as u32;
            if code <= 0xFF {
                format!("{:08b}", code)
            } else if code <= 0xFFFF {
                format!("{:016b}", code)
            } else {
                format!("{:b}", code)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn text_to_hex(text: &str) -> String {
    text.chars()
        .map(|c| {
            let code = c as u32;
            if code <= 0xFF {
                format!("{:02X}", code)
            } else if code <= 0xFFFF {
                format!("{:04X}", code)
            } else {
                format!("{:X}", code)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn binary_to_diamond(binary: &str) -> String {
    binary.chars()
        .map(|c| match c {
            '0' => '◇',
            '1' => '◆',
            _ => c,
        })
        .collect()
}
