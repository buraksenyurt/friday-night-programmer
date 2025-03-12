/*
    Hedef : Select Id,Name From Products Where categoryId=1;

    Lexer'dan beklenen token çıktısı

    [Keyword(SELECT), Identifier(Id), Comma, Identifier(Name), Keyword(FROM),
    Identifier(Products), Keyword(WHERE), Identifier(categoryId), Assign, Number(1), Semicolon]
*/

pub enum Token{
    Keyword(Reserved),
    Identifier(String), // Tablo, field adları gibi
    Number(f64),
    Assign, // = operatörü
    Comma, // , sembolü
    Semicolon, // ; sembolü
}

pub enum Reserved {
    Select,
    From,
    Where
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
