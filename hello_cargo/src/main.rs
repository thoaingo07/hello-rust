mod authentication;
mod car_factory;
mod text_processing;

use regex::Regex;
use text_processing::letters::count_letters;

use text_processing::numbers::count_numbers;
fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = count_letters(text);
    let number_of_numbers = count_numbers(text);
    (number_of_letters, number_of_numbers)
}
fn main() {

    let user = authentication::User::new("jeremy", "super-secret");
    car_factory::build_car();

    println!("The username is: {}", user.get_username());
    println!("The password is: {}", user.get_password_hash());
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));

    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));

}