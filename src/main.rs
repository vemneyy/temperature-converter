use std::io;

fn main() {
    println!("Введите режим конвертации:\n1 - Фаренгейт -> Цельсий\n2 - Цельсий -> Фаренгейт");


    let mode: u8 = loop {
        let mut mode= String::new();

        io::stdin()
        .read_line(&mut mode)
        .expect("Error");

        match mode.trim().parse() {
            Ok(temp) if temp == 1 || temp == 2 => break temp,
            _ => println!("Некоректный режим конвертации"),
        }
    };

    println!("Введите значение для конвертации: ");
    let mut first_var= String::new();

    io::stdin()
        .read_line(&mut first_var)
        .expect("Error");

    let first_var: f32 = loop{ match first_var.trim().parse() {
        Ok(temp) => break temp,
        Err(_) => {
            println!("Try again!");
            continue;
        },
    }
    };

    if mode == 1{
        let second_var: f32 = (first_var - 32.0) * 5.0 / 9.0;
        println!("{}°F = {}°C", first_var, second_var);
    } else {
        let second_var: f32 = (first_var * 9.0/5.0) + 32.0;
        println!("{}°C = {}°F", first_var, second_var);
    }

    println!("Нажмите Enter, чтобы выйти...");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit).expect("Error");
}