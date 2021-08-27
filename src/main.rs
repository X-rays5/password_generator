use std::io::Write;

fn set_clipboard(value: String) {
    cli_clipboard::set_contents(value.to_owned()).unwrap();
    assert_eq!(cli_clipboard::get_contents().unwrap(), value);
}

fn read_line() -> String {
    let mut length = String::new();
    std::io::stdin()
        .read_line(&mut length)
        .expect("Failed to read input");

    length.trim().parse().unwrap()
}

fn use_type(enable_type: &str) -> bool {
    print!("Use {} (y\\n): ", enable_type);
    std::io::stdout().flush().expect("Failed to flush stdout");
    let enable = read_line();

    match enable == "y" || enable == "yes"{
        true => true,
        false => false,
    }
}

fn main() {
    println!("Password generator");
    print!("Input length of password: ");
    std::io::stdout().flush().expect("Failed to flush stdout");

    let passlength: usize = match read_line().trim().parse() {
        Ok(val) => val,
        Err(_) => 0,
    };

    let passwordgen = passwords::PasswordGenerator{
        length: passlength,
        numbers: use_type("numbers"),
        lowercase_letters: use_type("lowercase characters"),
        uppercase_letters: use_type("uppercase characters"),
        symbols: use_type("symbols"),
        strict: true,
    };

    loop {
        let password = match passwordgen.generate_one() {
            Ok(val) => val,
            Err(val) => val.parse().unwrap(),
        };
        println!("Password is: {}", password);
        set_clipboard(password);
        println!("Copied to clipboard");

        println!("\nPress enter to generate a new password");
        read_line();

        clearscreen::clear();
    }
}
