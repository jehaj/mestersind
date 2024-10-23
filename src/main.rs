use std::env;
use std::io::Write;
use std::iter::zip;
use std::thread::sleep;
use std::time::Duration;
use rand::seq::IndexedRandom;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Describes the colors that our Master mind game supports.
/// You can use Red, Brown, Yellow, Green, Black, White, Orange, Blue or None.
enum Color {
    Red,
    Brown,
    Yellow,
    Green,
    Black,
    White,
    Orange,
    Blue,
    None,
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
/// Used when determining which part of the guess has same color and place, same color but wrong
/// position or just plain wrong.
enum Hint {
    CorrectlyPlaced,
    CorrectColorButWrong,
}

/// Print the remaining tries. Goes from attempts to n (excluded).
fn print_rows_of_dots(attempts: isize, n: isize) {
    for i in attempts..n {
        let row = i + 1;
        println!("{row:2}: {}", "･".repeat(5));
    }
}

/// Given a string *input*, get the color. Requires the string to be a valid color. See [Color].
fn string_to_color(input: &str) -> Color {
    match input.to_lowercase().as_str() {
        "red" => Color::Red,
        "brown" => Color::Brown,
        "yellow" => Color::Yellow,
        "green" => Color::Green,
        "black" => Color::Black,
        "white" => Color::White,
        "orange" => Color::Orange,
        "blue" => Color::Blue,
        "none" => Color::None,
        _ => panic!("Invalid color input") // precondition not fulfilled
    }
}

/// Print a circle with the given color ([Color]).
fn print_circle(color: &Color) {
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

/// Clears the terminal by calling OS programs. *cls* for windows and *clear* for unix.
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

/// checks if the guess and code are the same.
fn is_correct(guess: &Vec<Color>, code: &Vec<Color>) -> bool {
    if guess.len() != code.len() { return false; }
    let zip = zip(guess, code);
    zip.fold(true, |acc, (guess, code)| { guess == code && acc })
}

/// Asks the user to press enter. Uses [std::io::stdio::read_line] to block and wait for the user.
fn press_to_continue() {
    println!("Press enter to continue...");
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
}

/// Given a string of colors, get the vector of [Color] colors.
/// Requires the string to contain valid colors seperated by whitespace.
/// Uses [string_to_color] to map the string to a [Color] color.
fn get_colors(input: &String) -> Vec<Color> {
    input.split_whitespace().map(string_to_color).collect()
}



/// If guess contains five valid colors (see [Color]) seperated by spaces then it will return true,
/// otherwise false.
fn validate_guess(guess: &String) -> bool {
    const COLOR_SET: [&str; 9] = ["red", "brown", "yellow", "green", "black", "white", "orange",
                                  "blue", "none"];
    let colors = guess.split(" ");
    let colors = colors.collect::<Vec<&str>>();
    let size = colors.len();
    if size != 5 { return false; }
    colors.into_iter().fold(true,
                            |acc, color| COLOR_SET.contains(&color) && acc)
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    println!("Welcome to Mastermind! Let us get started.");
    sleep(Duration::new(0, 750_000_000));
    clear_terminal();

    let code = get_code();
    println!("I have found my code. It is your job to guess it!");
    // print_guess(&code); println!();

    let n = 12;
    let mut attempts = 0;
    let mut history: Vec<(Vec<Color>, Vec<Hint>)> = vec!();
    print_rows_of_dots(attempts, n);
    println!("The colors are: Red, Brown, Yellow, Green, Black, White, Orange, Blue, None");
    println!("You guess by writing the colors you want to guess seperated by spaces. A guess");
    println!("for Red, Brown, Yellow, Green and Black would look like:");
    println!("> Red Brown Yellow Green Black");
    println!("You will get hints: a '+' if you guessed a color correctly in the right place,");
    println!("or a '-' if you guessed a color, but it is in the wrong place.");
    while attempts < n {
        if attempts > 0 {
            clear_terminal();
            print_history(&history);
            print_rows_of_dots(attempts, n);
        }
        println!();
        println!("You have {} tries remaining. Try and guess.", n - attempts);
        print!("> ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        if !validate_guess(&input) {
            println!("Sorry, your guess was not valid. Please try again.");
            continue;
        }
        let guess: Vec<Color> = get_colors(&input);
        print!("You guessed: ");
        print_guess(&guess);
        attempts += 1;
        if is_correct(&guess, &code) {
            println!(", which is correct! Congratulations! It took {attempts} attempt{}.",
                     pluralize(&attempts));
            break;
        }
        println!(". Unfortunately that is not correct.");
        let hints: Vec<Hint> = get_hints(&guess, &code);
        press_to_continue();
        history.push((guess, hints));
    }
}

fn get_code() -> Vec<Color> {
    let color_choices = [
        Color::Red, Color::Brown, Color::Yellow, Color::Green, Color::Black, Color::White,
        Color::Orange, Color::Blue, Color::None];
    let mut colors = vec!();
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let &color = color_choices.choose(&mut rng).unwrap();
        colors.push(color);
    }
    colors
    // vec!(Color::Red, Color::Black, Color::None, Color::Red, Color::Green)
}

// Get hints based on guess and the correct code.
fn get_hints(guess: &Vec<Color>, code: &Vec<Color>) -> Vec<Hint> {
    let mut hints = vec!();
    let mut used = [false; 5];
    // CorrectColorButWrong can shadow CorrectlyPlaced, therefore
    // we must first find all the CorrectlyPlaced before looking at
    // CorrectColorButWrong.
    for (i, color) in guess.iter().enumerate() {
        if *color == code[i] {
            hints.push(Hint::CorrectlyPlaced);
            used[i] = true;
        }
    }
    for (i, color) in guess.iter().enumerate() {
        if *color == code[i] { continue }
        zip(code, used)
            .enumerate()
            .filter(|(_, (code, used))| color == *code && !used)
            .next().map(|(i, (_, _))| {
                used[i] = true;
                hints.push(Hint::CorrectColorButWrong)
            });
    }
    hints.sort();
    hints
}

fn print_guess(guess: &Vec<Color>) {
    for color in guess {
        print_circle(color);
    }
}

fn print_history(guesses: &Vec<(Vec<Color>, Vec<Hint>)>) {
    for (i, (guess, hints)) in guesses.iter().enumerate() {
        let row = i + 1;
        print!("{row:2}: ");
        print_guess(guess);
        print!(" ");
        print_hints(hints);
        println!();
    }
}

fn print_hints(hints: &Vec<Hint>) {
    for hint in hints {
        print!("{}", match hint {
            Hint::CorrectlyPlaced => "+",
            Hint::CorrectColorButWrong => "-",
        });
    }
}

fn pluralize(attempts: &isize) -> &str {
    if *attempts == 1 { "" } else { "s" }
}
