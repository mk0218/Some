#[derive(Debug, PartialEq)]
pub struct IllegalTokenError;

#[derive(Debug, PartialEq)]
pub enum Token {
    SEMICOLON,
    SGQUOTE,
    DBQUOTE,
    Keyword(Keyword),
    Operator(Operator),
    Bracket(Bracket),
    Identifier(String),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    LET,
    FUN,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Binary(BinaryOperator),
    Unary(UnaryOperator),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    ASSIGN,
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    NOT,
}

#[derive(Debug, PartialEq)]
pub enum Bracket {
    LCURLY,
    RCURLY,
    LROUND,
    RROUND,
    LSQUARE,
    RSQUARE,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Number(i32),
    // String(String),
}

type KW = Keyword;
type OP = Operator;
type BO = BinaryOperator;
type UO = UnaryOperator;
type BK = Bracket;
type LI = Literal;

impl Token {
    pub fn try_tokenize(s: &str) -> Result<Token, IllegalTokenError> {
        let token = match s {
            ";"     => Token::SEMICOLON,
            "'"     => Token::SGQUOTE,
            "\""    => Token::DBQUOTE,

            "let"   => Token::Keyword(KW::LET),
            "fun"   => Token::Keyword(KW::FUN),

            "="     => Token::Operator(OP::Binary(BO::ASSIGN)),
            "+"     => Token::Operator(OP::Binary(BO::PLUS)),
            "-"     => Token::Operator(OP::Binary(BO::MINUS)),
            "*"     => Token::Operator(OP::Binary(BO::MUL)),
            "/"     => Token::Operator(OP::Binary(BO::DIV)),
            "%"     => Token::Operator(OP::Binary(BO::MOD)),

            "not"   => Token::Operator(OP::Unary(UO::NOT)),

            "{"     => Token::Bracket(BK::LCURLY),
            "}"     => Token::Bracket(BK::RCURLY),
            "("     => Token::Bracket(BK::LROUND),
            ")"     => Token::Bracket(BK::RROUND),
            "["     => Token::Bracket(BK::LSQUARE),
            "]"     => Token::Bracket(BK::RSQUARE),

            "true"  => Token::Literal(LI::Bool(true)),
            "false" => Token::Literal(LI::Bool(false)),

            s => match s.chars().nth(0) {
                Some('A'..='Z') |
                Some('a'..='z') |
                Some('_')      => Self::try_identifier(s)?,
                Some('0'..='9') |
                Some('-')      => Self::try_number(s)?,
                _              => Err(IllegalTokenError)?,
            }
        };

       Ok(token)
    }

    fn try_identifier(s: &str) -> Result<Token, IllegalTokenError> {
        let mut id = String::new();

        for (i, c) in s.chars().enumerate() {
            id.push(match (i, c) {
                (0, '0'..='9')  => Err(IllegalTokenError),
                (_, '0'..='9') |
                (_, 'a'..='z') |
                (_, 'A'..='Z') |
                (_, '_') => Ok(c),
                _ => Err(IllegalTokenError),
            }?);
        }

        Ok(Token::Identifier(id))
    }

    fn try_number(s: &str) -> Result<Token, IllegalTokenError> {
        let mut number = Ok(0);

        for c in s.chars() {
            if let Ok(mut n) = number {
                number = match c {
                    '0'..='9' => {
                        n *= 10;
                        n += c.to_digit(10).unwrap();
                        Ok(n)
                    },
                    _ => {
                        Err(IllegalTokenError)
                    }
                };
            } else { break }
        }

        Ok(Token::Literal(LI::Number(number?.try_into().unwrap())))
    }
}

#[cfg(test)]
mod test_token {
    use super::*;

    #[test]
    fn test_try_identifier() {
        assert_eq!(
            Token::try_identifier("AaZz012"),
            Ok(Token::Identifier(String::from("AaZz012"))
        ));

        assert_eq!(
            Token::try_identifier("_AaZz012"),
            Ok(Token::Identifier(String::from("_AaZz012"))
        ));
        
        assert_eq!(
            Token::try_identifier("0abcd"),
            Err(IllegalTokenError)
        );
    }
    
    #[test]
    fn test_try_number() {
        assert_eq!(
            Token::try_number("432123"),
            Ok(Token::Literal(LI::Number(432123)))
        );

        assert_eq!(
            Token::try_number("012345"),
            Ok(Token::Literal(LI::Number(12345)))
        );

        assert_eq!(
            Token::try_number("01#213"),
            Err(IllegalTokenError)
        );
    }
}