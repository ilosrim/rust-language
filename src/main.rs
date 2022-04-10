use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    // GUESS NUMBER GAME

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..5);

    loop{
        println!("Please input your guess.");

        // rust-da o'zgaruvchilar default o'zgarmas bo'lib yaratiladi,
        // shuning uchun biz "mut" so'zi orqali kerakli ma'lumotlarni
        // o'zgaruvchanga aylantirib olamiz. Shu bilan birga havolalar (&) ham
        // o'zgarmas bo'lib yaratiladi, ularga ham tepadagi usulni ishlatamiz.

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // gueass nomli soya o'zgaruvchisini yaratamiz
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // println!("The secret number is: {}", secret_number);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("To Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    
}
