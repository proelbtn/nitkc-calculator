#[derive(Debug)]
enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Semicolon,
    Openparen,
    Closeparen,
    Number(u32)
}

fn lexer(s: &str) -> Vec<Token> {
    let buf = s.trim().as_bytes();
    let mut vec: Vec<Token> = Vec::new();

    let mut i = 0;
    while i < buf.len() {
        match buf[i] as char {
            '+' => vec.push(Token::Plus),
            '-' => vec.push(Token::Minus),
            '*' => vec.push(Token::Asterisk),
            '/' => vec.push(Token::Slash),
            ';' => vec.push(Token::Semicolon),
            '(' => vec.push(Token::Openparen),
            ')' => vec.push(Token::Closeparen),
            '0' ... '9' => {
                let mut val: Vec<u8> = Vec::new();
                while i < buf.len() {
                    if !(('0' as u8) <= buf[i] && buf[i] <= ('9' as u8)) { break }
                    val.push(buf[i]);
                    i += 1;
                }
                i -= 1;

                let valstr = String::from_utf8(val).unwrap();
                vec.push(Token::Number(valstr.parse::<u32>().unwrap()))
            }
            _ => panic!("unexpected!")
        }
        i += 1;
    } 

    return vec;
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    
    let tokens = lexer(&s);

    for token in tokens {
        println!("{:?}", token);
    }
}
