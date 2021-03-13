use std::io;


fn credit_checksum(number_check: &mut [u32; 16]) {
    let test: u32 = number_check[15];
    number_check[15] = 0;

    for index in (0..15).step_by(2) {
        let mut number = number_check[index] * 2;
        
        if number > 9 {
            number = 1 + (number - 10);
        }

        number_check[index] = number;
    }

    println!("{:?}", number_check);

    let mut total: u32 = number_check.iter().sum();
    total += test;

    if total % 10 == 0 {
        println!("This is a valid card number.")
    } else {
        println!("This card number is invalid.")
    }   
}


fn number_vectorizer(card_digits: &String) {
    let mut card_number: [u32; 16] = [0; 16];
    let mut index = 0;

    for digit in card_digits.chars() {
        card_number[index] = digit.to_digit(10).expect("Non number on credit card");
        index += 1;
    }

    credit_checksum(&mut card_number);
}


fn main() {
    println!("Welcome to my credit card verifier. Please input number to check.");

    let mut card_number = String::new();

    io::stdin()
        .read_line(&mut card_number)
        .expect("Failed to read the input");
    
    if card_number.ends_with('\n') {
        card_number.pop();
        if card_number.ends_with('\r') {
            card_number.pop();
        }
    }

    if card_number.len() != 16 {
        panic!("Card number is not the correct length")
    }
    
    println!("Now checking the validity of the credit card number.");

    number_vectorizer(&card_number);
}
