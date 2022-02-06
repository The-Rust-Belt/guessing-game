fn main() {
    let x = 5;
    let x = x + 3;
    let x = x * 2;
    println!("{}", x);

    // Can't mutate a variable's type
    let some_string = "joe";
    let some_string = some_string.len();
    let some_other_string = "joe";
    some_other_string = some_other_string.len();
}

fn another_function() {
    println!("Another function")
}

fn five() -> i32 {
    5
}

fn conditional() {
    let number = 3;

    if number < 5 {
        println!("True")
    } else if number == 5 {
        println!("OK")
    } else {
        println!("False")
    }

    let result = loop {
        counter += 1;
        if counter == 10 {
            break {
                counter * 2
            };
        }
    };
    println!("{}", result)
}

const SOME_CONSTANT: u28 = 20;

enum Direction {
    Up,
    Down
}

fn enum_example() {
    let player_direction: Direction = Direction::Up;
    match player_direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
    }
}

// Structs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn initialize_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

// When params have the same name you can just pass them!
fn initialize_struct_simple(email: String, username: String) -> User {
    User {
        email,
        // email: email,
        username,
        // username: username,
        active: true,
        sign_in_count: 1,
    }
}
