use std::io::{stdin,stdout,Write};

const ALPHA: [char;26] = [
    'A','B','C','D','E','F','G','H','I','J','K','L','M',
    'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'
];

fn input_string() -> String {
    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("String not valid");
    input = input.trim().to_string();
    return input
}

fn string_to_i16(loop_txt: String) -> i16 {
    let mut string = String::new();
    while !string.parse::<i16>().is_ok() | string.is_empty() {
        print!("{}",loop_txt);
        string = input_string()
    }
    return string.parse::<i16>().unwrap()

}


pub fn caesar(txt:String,encrypt:bool) {
    let shift: i16 = string_to_i16(String::from("Enter shift: "));
    let mut buffer: Vec<char> = Vec::new();

    for chr in Vec::from(txt.to_uppercase()) {
        let mut num: i16 = chr as i16;
        if num >= 65 && num <= 90 {
            num = if encrypt {
                num - 65 + shift
            } else {
                num - 65 - shift
            };
            if num < 0 {num = 26 + num};
            num = num % 26;
            buffer.push(*ALPHA.get(num as usize).expect("out of range"))
        } else {
            buffer.push(chr as char)
        } 
    }; 
    let output: String = buffer.into_iter().collect();
    print!("{}",output)
}

pub fn affine(txt:String,encrypt:bool) {
    println!("Affine cipher uses two keys a and b it follows the formula: ax+b % 26 ");
    let times: i16 = string_to_i16(String::from("Enter a: "));
    let shift: i16 = string_to_i16(String::from("Enter b: "));

    let mut buffer: Vec<char> = Vec::new();
    let mut shift_alpha : Vec<char> = Vec::new();

    if !encrypt {
        for i in 0..26 {
            let c = ((i * times) + shift) % 26;
            shift_alpha.push(*ALPHA.get(c as usize).expect("Out of Range"))
        }
    }

    for chr in Vec::from(txt.to_uppercase()) {
        let mut num: i16 = chr as i16;
        if num >= 65 && num <= 90 {
            num = if encrypt {
                (num - 65) * times + shift
            } else {
                let shift_index = shift_alpha.iter().position(|&r| r == (chr as char)).unwrap();
                buffer.push(*ALPHA.get(shift_index as usize).expect("Not in bounds"));
                continue;
            };
            if num < 0 {num = 26 + num}
            num = num % 26;
            buffer.push(*ALPHA.get(num as usize).expect("Out of Range"))
        } else {
            buffer.push(chr as char)
        }
    }

    let output: String = buffer.into_iter().collect();
    print!("{}",output)

}

pub fn vigenere(txt:String,encrypt:bool) {
    print!("Enter keyword: ");
    let keyword: Vec<u8> = Vec::from(input_string().to_uppercase());
    let mut index: usize = 0;
    let mut buffer: Vec<char> = Vec::new();
    for chr in Vec::from(txt.to_uppercase()) {
        let mut num = chr as i16;
        if num >= 65 && num <=(90) {
            let mut key =  *keyword.get(index).expect("out of bounds") as i16;
            if key < 65 || key > 90 {
                panic!("Only letters are allowed in keyword")
            };
            num -= 65;
            key -= 65;
            if encrypt {
                num = (num + key) % 26;
            } else {
                num -= key;
                if num < 0 {num = 26 + num}
            }
            index = (index + 1) % (keyword.len());
            buffer.push(*ALPHA.get(num as usize).expect("Out of bounds"))
        } else {
            buffer.push(chr as char)
        }
    }
    let output: String = buffer.into_iter().collect();
    print!("{}",output)
}



