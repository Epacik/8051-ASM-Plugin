use chumsky::{prelude::Simple, Error, Span};

use crate::extensions::EndsWithDigit;
use super::{
    PositionedToken, 
    keywords,
    initial::{
        SpannedString, 
        InitialTokenType
    },
    Position, 
    Token, 
    Trivia,
    Keyword,
    Number,
    };

pub(super) fn perform_analysis(lines: Vec<Vec<SpannedString>>) -> (Option<Vec<PositionedToken>>, Vec<Simple<String, Position>>) {
    let mut tokens = Vec::<PositionedToken>::new();
    let mut errors = Vec::<Simple<String, Position>>::new();
    for line in lines
    {
        let mut indexes = 0..line.len();

        while let Some(item_index) = indexes.next() {
            let item = &line[item_index];

            // a comment
            if item.initial_type == InitialTokenType::Control && item.content == ";" {
                // get the rest of the tokens in line
                let the_rest = &line[item_index..];

                // as well as the first and the last item in the rest of the line
                let first = the_rest.first();
                let last = the_rest.last();
                if first.is_none() || last.is_none() {
                    // that shouldn't ever happen, I think
                    continue;
                }

                // get the positions of first and last elements, and contents of the whole comment
                let position_first = &first.unwrap().position;
                let position_last = &last.unwrap().position;
                let content = the_rest.iter().map(|x| x.content.as_str()).collect::<String>();

                // push it to the list of tokens
                tokens.push(
                    PositionedToken::new(
                        Token::Trivia(Trivia::Comment(content)), 
                        Position::new(
                            position_first.range.start..position_last.range.end, 
                            position_first.line, 
                            position_first.columns.start..position_last.columns.end)));

                // skip everything else in the line, since it's a comment
                indexes.nth(usize::MAX);
            }

            // ASCII string
            else if item.initial_type == InitialTokenType::Control && (item.content == "\"" || item.content == "'") {
                // we shouldn't allow missmatched deliminers 
                let deliminer = &item.content;

                let the_rest = &line[item_index..];

                // let's find an end of a ASCII string
                let mut string_end_index = find_closing_deliminer(the_rest, deliminer);

                // if someone forgot to place a second " or ' we're just going to grab the whole line
                if string_end_index == 0 {
                    string_end_index = the_rest.len() - 1;

                    // and and inform the programmer using an error
                    errors.push(
                        Simple::unclosed_delimiter(
                            Position::new(
                                item.position.range.start..(item.position.range.start + string_end_index),
                                item.position.line, 
                                item.position.columns.start..(item.position.columns.start + string_end_index)), 
                            deliminer.clone(), 
                            Position::new(
                                item.position.range.start..(item.position.range.start + string_end_index),
                                item.position.line, 
                                item.position.columns.start..(item.position.columns.start + string_end_index)), 
                                deliminer.clone(), 
                            None));
                }

                // grab the content of the string, as well as start and end positions
                let string = &the_rest[..=string_end_index];
                let content = string.iter().map(|x| x.content.as_str()).collect::<String>();
                let position_first = &string.first().unwrap().position;
                let position_last = &string.last().unwrap().position;

                // push it to the vec of tokens
                tokens.push(
                    PositionedToken::new(
                        Token::String(content),
                        Position::new(
                            position_first.range.start..position_last.range.end, 
                            position_first.line,
                            position_first.range.start..position_last.range.end)));

                // and skip to the end of the string in line
                indexes.nth(string_end_index - 1);
            }
        
            // Labels
            else if item.initial_type == InitialTokenType::Other && item_index == 0 {

                // labels can, depending on the use case, have a colon immidiatelly after, or not have it at all
                let next = &line[1];
                let has_colon = next.initial_type == InitialTokenType::Control && next.content == ":";

                // based on existence of that colon we can either include it in currently created token, or not
                let (content, position) = if has_colon {

                    indexes.nth(0);

                    let pos = Position::new(
                        item.position.range.start..next.position.range.end, 
                        item.position.line, 
                        item.position.columns.start..next.position.columns.end);
                    (item.content.clone(), pos)
                }
                else {
                    (item.content.clone(), item.position.clone())
                };

                tokens.push(PositionedToken::new(Token::Label(content), position));
            }

            // number
            else if 
                item.initial_type == InitialTokenType::Control && 
                item_index > 0 && 
                item.content == "#" &&
                line.len() >= item_index + 1 {

                let next = &line[item_index + 1];
                let num = next.content.clone();

                let token = if num.ends_with("H") || num.ends_with("h") {
                    Token::Number(Number::Hexadecimal(num))
                }
                else if num.ends_with("O") || num.ends_with("o") {
                    Token::Number(Number::Octal(num))
                }
                else if num.ends_with("B") || num.ends_with("b") {
                    Token::Number(Number::Octal(num))
                }
                else if num.ends_with_digit() {
                    Token::Number(Number::Decimal(num))
                }
                else {
                    Token::Unknown(num)
                };

                let position = Position::new(
                    (item.position.range.start)..(next.position.range.end),
                    item.position.line,
                    (item.position.columns.start)..(next.position.columns.end));

                tokens.push(PositionedToken::new(token, position));
                indexes.next();
            }

            // whitespace
            else if item.initial_type == InitialTokenType::WhiteSpace {
                tokens.push(
                    PositionedToken::new(
                        Token::Trivia(Trivia::WhiteSpace(item.content.clone())),
                        item.position.clone()));
            }

            // newline
            else if item.initial_type == InitialTokenType::NewLine {
                tokens.push(
                    PositionedToken::new(
                        Token::Trivia(Trivia::NewLine(item.content.clone())),
                        item.position.clone()));
            }

            // others
            else if item.initial_type == InitialTokenType::Other && item_index > 0 {
                let content = item.content.clone();

                if keywords::is_keyword(&content) {

                    let token = string_to_keyword(content);
                
                    tokens.push(PositionedToken::new(Token::Keyword(token), item.position.clone()));
                }
                else {
                    tokens.push(PositionedToken::new(Token::Other(content), item.position.clone()));
                }
            }

        }
    }
    (Some(tokens), errors)
}

fn string_to_keyword(content: String) -> Keyword {
    if keywords::is_instruction(&content) 
    { 
        Keyword::Instruction(content)
    }
    else if keywords::is_directive(&content) 
    { 
        Keyword::Directive(content)
    }
    else if keywords::is_register(&content) 
    { 
        Keyword::Register(keywords::string_to_register(&content))
    }
    else if keywords::is_flag_or_bit(&content) 
    { 
        Keyword::FlagOrBit(content) 
    }
    else { panic!("Invalid keyword was provided") }
}

fn find_closing_deliminer(spanned_strings: &[SpannedString], deliminer: &String) -> usize {
   
    if spanned_strings.len() <= 1 {
        return 0;
    }

    for index in 1..spanned_strings.len() {
        let string = &spanned_strings[index];
        if string.initial_type == InitialTokenType::Control && string.content == *deliminer {
            
            return index;
        }
    }

    0
}