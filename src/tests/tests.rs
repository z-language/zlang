mod tokenizer_tests {
    use crate::tokenizer::{Token, Tokenizer, Type};

    #[test]
    fn test_strings_normal() {
        let mut tokenizer = Tokenizer::new();

        // Its important to note that here I'm escaping the rust's quotes, and not sending the escapes to the parser.
        let test_case = String::from("\"test\"");
        let expected = Token {
            line: 1,
            pos: 0,
            value: "test".to_owned(),
            t_type: Type::String,
        };

        let got = tokenizer.tokenize(test_case);
        let token = got.get(0).unwrap().clone();
        assert_eq!(got.len(), 1); // only one token expected
        assert_eq!(expected, token);
    }

    #[test]
    fn test_string_escapes() {
        let mut tokenizer = Tokenizer::new();

        let test_case = String::from("\"\\\"\\\"test\\\"\\\"\\\\\"");
        let expected = Token {
            line: 1,
            pos: 0,
            value: "\"\"test\"\"\\".to_owned(),
            t_type: Type::String,
        };

        let got = tokenizer.tokenize(test_case);
        let token = got.get(0).unwrap().clone();
        assert_eq!(got.len(), 1); // only one token expected
        assert_eq!(expected, token);
    }

    #[test]
    fn test_variables() {
        use crate::tests::cases::get_tokenizer_variable_case;
        let mut tokenizer = Tokenizer::new();

        let test_case = String::from(
            "var ime = \"luka\"
var mut starost = 13
var mut assignLater
assignLater = 14",
        );

        let got = tokenizer.tokenize(test_case);
        let expected = get_tokenizer_variable_case();
        assert_eq!(expected, got);
    }
}

mod parser_tests {
    #[test]
    // testing the test
    fn test_test() {
        assert_eq!(1, 1);
    }
}
