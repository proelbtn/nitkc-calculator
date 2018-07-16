#[derive(Debug)]
enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Semicolon,
    Openparen,
    Closeparen,
    Number(u64)
}

#[derive(Debug)]
enum EElement {
    Plus,
    Minus,
    T(Box<ASTNode>)
}

#[derive(Debug)]
enum TElement {
    Asterisk,
    Slash,
    F(Box<ASTNode>)
}

#[derive(Debug)]
enum ASTNode {
    S(Box<ASTNode>), 
    E(Vec<Box<EElement>>),
    T(Vec<Box<TElement>>),
    F(Box<ASTNode>),
    N(u64),
}

impl ASTNode {
    fn show(&self) {
        match self {
            ASTNode::S(node) => {
                print!("(S ");
                node.show();
                println!(")");
            },
            ASTNode::E(vec) => {
                print!("(E");
                for elem in vec.iter() {
                    print!(" ");
                    match **elem {
                        EElement::Plus => print!("+"),
                        EElement::Minus => print!("-"),
                        EElement::T(ref node) => node.show(),
                    }
                }
                print!(")");
            },
            ASTNode::T(vec) => {
                print!("(T");
                for elem in vec.iter() {
                    print!(" ");
                    match **elem {
                        TElement::Asterisk => print!("*"),
                        TElement::Slash => print!("/"),
                        TElement::F(ref node) => node.show(),
                    }
                }
                print!(")");
            },
            ASTNode::F(node) => {
                print!("(F ");
                node.show();
                print!(")");
            },
            ASTNode::N(num) => {
                print!("(N {})", num)
            },
        }
    }

    fn eval(&self) -> f64 {
        match self {
            ASTNode::S(node) => node.eval(),
            ASTNode::E(vec) => {
                let mut i = 0;
                let mut val: f64 = 0.;

                match *vec[0] {
                    EElement::T(ref node) => {
                        val = node.eval();
                        i = 1;
                    }
                    _ => (),
                };

                while i < vec.len() {
                    match *vec[i] {
                        EElement::Plus => {
                            match *vec[i + 1] {
                                EElement::T(ref node) => {
                                    val = val + node.eval();
                                }
                                _ => panic!("unexpected")
                            }
                            i += 2;
                        },
                        EElement::Minus => {
                            match *vec[i + 1] {
                                EElement::T(ref node) => {
                                    val = val - node.eval();
                                }
                                _ => panic!("unexpected")
                            }
                            i += 2;
                        },
                        _ => panic!("unexpected"),
                    };
                }

                return val;
            },
            ASTNode::T(vec) => {
                let mut i = 0;
                let mut val: f64 = 0.;

                match *vec[0] {
                    TElement::F(ref node) => {
                        val = node.eval();
                        i = 1;
                    }
                    _ => panic!("unexpected"),
                };

                while i < vec.len() {
                    match *vec[i] {
                        TElement::Asterisk => {
                            match *vec[i + 1] {
                                TElement::F(ref node) => {
                                    val = val * node.eval();
                                }
                                _ => panic!("unexpected")
                            }
                            i += 2;
                        },
                        TElement::Slash => {
                            match *vec[i + 1] {
                                TElement::F(ref node) => {
                                    val = val / node.eval();
                                }
                                _ => panic!("unexpected")
                            }
                            i += 2;
                        },
                        _ => panic!("unexpected"),
                    };
                }

                return val;
            },
            ASTNode::F(node) => {
                return node.eval();
            },
            ASTNode::N(num) => *num as f64
        }
    }
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
                vec.push(Token::Number(valstr.parse::<u64>().unwrap()))
            }
            _ => panic!("unexpected")
        }
        i += 1;
    } 

    return vec;
}

fn read_number(tokens: &Vec<Token>, pos: usize) -> (Box<ASTNode>, usize) {
    match tokens[pos] {
        Token::Number(num) => (Box::new(ASTNode::N(num)), pos + 1),
        _ => panic!("unexpected")
    }
}

fn read_factor(tokens: &Vec<Token>, pos: usize) -> (Box<ASTNode>, usize) {
    match tokens[pos] {
        Token::Number(_) => {
            let (node, pos) = read_number(tokens, pos);
            (Box::new(ASTNode::F(node)), pos)
        },
        Token::Openparen => {
            let (node, pos) = read_expression(tokens, pos + 1);
            match tokens[pos] {
                Token::Closeparen => (Box::new(ASTNode::F(node)), pos + 1),
                _ => panic!("unexpected")
            }
        },
        _ => panic!("unexpected")
    }
}

fn read_term(tokens: &Vec<Token>, pos: usize) -> (Box<ASTNode>, usize) {
    let mut vec: Vec<Box<TElement>> = Vec::new();

    let (node, mut pos) = read_factor(tokens, pos);
    vec.push(Box::new(TElement::F(node)));

    loop {
        if pos < tokens.len() {
            match tokens[pos] {
                Token::Asterisk => {
                    vec.push(Box::new(TElement::Asterisk));
                    let (node, t) = read_factor(tokens, pos + 1);
                    vec.push(Box::new(TElement::F(node)));
                    pos = t;
                },
                Token::Slash => {
                    vec.push(Box::new(TElement::Slash));
                    let (node, t) = read_factor(tokens, pos + 1);
                    vec.push(Box::new(TElement::F(node)));
                    pos = t;
                },
                _ => break
            }
        }
        else { break }
    }

    return (Box::new(ASTNode::T(vec)), pos);
}

fn read_expression(tokens: &Vec<Token>, pos: usize) -> (Box<ASTNode>, usize) {
    let mut pos = pos;
    let mut vec: Vec<Box<EElement>> = Vec::new();

    if pos < tokens.len() {
        match tokens[pos] {
            Token::Plus => {
                vec.push(Box::new(EElement::Plus));
                pos = pos + 1;
            },
            Token::Minus => {
                vec.push(Box::new(EElement::Minus));
                pos = pos + 1;
            },
            _ => ()
        }
    }

    let (node, mut pos) = read_term(tokens, pos);
    vec.push(Box::new(EElement::T(node)));

    loop {
        if pos < tokens.len() {
            match tokens[pos] {
                Token::Plus => {
                    vec.push(Box::new(EElement::Plus));
                    let (node, t) = read_term(tokens, pos + 1);
                    vec.push(Box::new(EElement::T(node)));
                    pos = t;
                },
                Token::Minus => {
                    vec.push(Box::new(EElement::Minus));
                    let (node, t) = read_term(tokens, pos + 1);
                    vec.push(Box::new(EElement::T(node)));
                    pos = t;
                },
                _ => break
            }
        }
        else { break }
    }

    return (Box::new(ASTNode::E(vec)), pos);
}

fn read_statement(tokens: &Vec<Token>, pos: usize) -> (Box<ASTNode>, usize) {
    let (node, pos) = read_expression(tokens, pos);
    match tokens[pos] {
        Token::Semicolon => (Box::new(ASTNode::S(node)), pos + 1),
        _ => panic!("unexpected")
    }
}

fn parser(tokens: &Vec<Token>) -> Box<ASTNode> {
    return read_statement(tokens, 0).0;
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    
    let tokens = lexer(&s);
    let ast = parser(&tokens);
    ast.show();
    println!("ans : {}", ast.eval());
}