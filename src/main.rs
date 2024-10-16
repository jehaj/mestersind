use std::env;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

enum Color {
    Red, Brown, Yellow, Green, Black, White, Orange, Blue, None
}

fn print_rows_of_dots(n: isize) {
    for i in 0..n {
        let row = i+1;
        println!("{row:2}: {}", "･".repeat(5));
    }
}

fn print_circle(color: Color) {
    let color = match color {
        Color::Green => "32",
        Color::Red => "31",
        Color::Brown => "38;2;210;105;30",
        Color::Yellow => "93",
        Color::Black => "90",
        Color::White => "37",
        Color::Orange => "33",
        Color::Blue => "94",
        Color::None => {
            print!("◯");
            return;
        }
    };
    let format = format!("\x1b[{color}m");
    print!("{}", format); // Set color
    print!("⬤");
    print!("\x1b[0m") // Reset color back to normal
}

fn clear_terminal() {
    let family = env::consts::FAMILY;
    let program = match family {
        "windows" => "cmd",
        "unix" => "sh",
        f => panic!("Does not support that family: {f}.")
    };
    let mut command = std::process::Command::new(program);
    match family {
        "windows" => command.args(&["/C", "cls"]),
        "unix" => command.arg("clear"),
        _ => panic!("Unknown family: {family}")
    };
    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}



fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    println!("Welcome to Mastermind! Let us get started.");
    sleep(Duration::new(0, 750_000_000));
    clear_terminal();

    let n = 12;
    print_rows_of_dots(n);
    let mut attempts = 0;
    println!("You guess by writing:");
    println!("The colors are: Red, Brown, Yellow, Green, Black, White, Orange, Blue, None");
    println!("You guess by writing the colors you want to guess seperated by spaces. A guess");
    println!("for Red, Brown, Yellow, Green and Black would look like:");
    println!(">Red Brown Yellow Green Black");
    while attempts < n {
        println!();
        println!("You have {} tries remaining. Try and guess.", n - attempts);
        print!(">");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        if !validate_guess(&input) {
            continue;
        }
    }
}

/// If guess contains five valid colors (see Color) seperated by spaces then it will return true,
/// otherwise false.
fn validate_guess(guess: &String) -> bool {
    true
}

