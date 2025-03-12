/*
    Örnek Expression     : SELECT Id, Name, CategoryName, FROM Products JOIN Category
                          ON Products.CategoryId = Category.CategoryId
                          WHERE CategoryId = 1;

    Lexer'dan beklenen token çıktısı

    [
    Keyword(SELECT), Identifier(Id), Comma, Identifier(Name), Comma, Identifier(CategoryName),
    Keyword(FROM), Identifier(Products), Keyword(JOIN), Identifier(Category),
    Keyword(ON), Identifier(Products), Dot, Identifier(CategoryId), Assign,
    Identifier(Category), Dot, Identifier(CategoryId)
    Keyword(WHERE), Identifier(CategoryId), Assign, Number(1), Semicolon
    ]
*/

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Reserved),
    Identifier(String), // Tablo, field adları gibi
    Number(f64),
    Equation,  // = operatörü
    Comma,     // , sembolü
    Semicolon, // ; sembolü
    Dot,       // . sembolü
}

#[derive(Debug, PartialEq)]
pub enum Reserved {
    Select,
    From,
    Join,
    On,
    Where,
}

pub struct Sqlexer {
    input: Vec<char>,
    pos: usize,
}

impl Sqlexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn get_word(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_alphanumeric() {
            self.pos += 1;
        }
        let word: String = self.input[start..self.pos].iter().collect();

        let upper_word = word.to_uppercase();
        match upper_word.as_str() {
            "SELECT" => Token::Keyword(Reserved::Select),
            "FROM" => Token::Keyword(Reserved::From),
            "WHERE" => Token::Keyword(Reserved::Where),
            "JOIN" => Token::Keyword(Reserved::Join),
            "ON" => Token::Keyword(Reserved::On),
            _ => Token::Identifier(word),
        }
    }

    fn get_number(&mut self) -> Token {
        let start_index = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let num_str: String = self.input[start_index..self.pos].iter().collect();
        Token::Number(num_str.parse().unwrap())
    }

    pub fn next(&mut self) -> Option<Token> {
        while self.pos < self.input.len() {
            let curr_c = self.input[self.pos];

            match curr_c {
                '=' => {
                    self.pos += 1;
                    return Some(Token::Equation);
                }
                ',' => {
                    self.pos += 1;
                    return Some(Token::Comma);
                }
                ';' => {
                    self.pos += 1;
                    return Some(Token::Semicolon);
                }
                '.' => {
                    self.pos += 1;
                    return Some(Token::Dot);
                }
                'A'..='Z' | 'a'..='z' => return Some(self.get_word()),
                '0'..='9' => return Some(self.get_number()),
                ' ' | '\t' | '\n' => self.pos += 1,
                _ => panic!("Unknown character, '{}'", curr_c),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sql_query() {
        let expression = "SELECT Id, Name, CategoryName FROM Products JOIN Category
                          ON Products.CategoryId = Category.CategoryId
                          WHERE CategoryId = 1;";
        let mut sql_lexer = Sqlexer::new(expression);

        let mut actual = Vec::new();
        while let Some(token) = sql_lexer.next() {
            actual.push(token);
        }
        let expected = vec![
            Token::Keyword(Reserved::Select),
            Token::Identifier(String::from("Id")),
            Token::Comma,
            Token::Identifier(String::from("Name")),
            Token::Comma,
            Token::Identifier(String::from("CategoryName")),
            Token::Keyword(Reserved::From),
            Token::Identifier(String::from("Products")),
            Token::Keyword(Reserved::Join),
            Token::Identifier(String::from("Category")),
            Token::Keyword(Reserved::On),
            Token::Identifier(String::from("Products")),
            Token::Dot,
            Token::Identifier(String::from("CategoryId")),
            Token::Equation,
            Token::Identifier(String::from("Category")),
            Token::Dot,
            Token::Identifier(String::from("CategoryId")),
            Token::Keyword(Reserved::Where),
            Token::Identifier(String::from("CategoryId")),
            Token::Equation,
            Token::Number(1.0),
            Token::Semicolon,
        ];
        assert_eq!(actual, expected);
    }
}
