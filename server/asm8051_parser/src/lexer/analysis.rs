use crate::issues::{self, Issue};

use super::tokens::{
    PositionedToken,
    Token, 
    Trivia,
    Keyword,
    Number,
    ControlCharacter,
    Parenthesis,
    Arithmetic,
    Delimiter,
};

use super::{
    keywords,
    initial::{
        SpannedString, 
        InitialTokenType
    },
    Position,
    extensions::Digits,
};

pub(super) fn perform_analysis(lines: Vec<Vec<SpannedString>>) -> (Option<Vec<PositionedToken>>, Vec<Issue>) {
    let mut tokens = Vec::<PositionedToken>::new();
    let mut lexing_issues = Vec::<Issue>::new();
    for line in lines
    {
        if line.len() == 0 {
            continue;
        }

        let mut indices = 0..line.len();

        while let Some(item_index) = indices.next() {
            let item = &line[item_index];

            // a comment
            if item.initial_type == InitialTokenType::Control && item.content == ";" {

                // I need something like
                // [;][contents of comment][newline(optional)]
                // so I need to check if newline characters exist at the end (they may not if it's the last line)
                // next I need to get everything between [;] and newline or end of the line
                // get the rest of the tokens in line

                // start of the comment
                tokens.push(
                    PositionedToken::new(
                        Token::ControlCharacter(ControlCharacter::Delimiter(Delimiter::CommentStart)),
                        item.position.clone()));

                //if it's the last token in the line (that may happen if it's the last line of file and comment has nothing written yet)
                if item_index == line.len() - 1 {
                    //TODO: Warn/inform user about empty comment

                    lexing_issues.push(issues::empty_comment(item.position.clone()));
                }

                // second-last token in line, a/k/a empty comment in not last line
                else if item_index + 2 == line.len() {

                    // after we push a comment start, let's push a newline characters
                    let newline = &line[item_index + 1];
                    let newline_content = newline.content.clone();
                    let newline_position = newline.position.clone();
                    tokens.push(
                        PositionedToken::new(
                            Token::Trivia(
                                Trivia::NewLine(newline_content)), 
                                newline_position));
                    
                    lexing_issues.push(issues::empty_comment(item.position.clone()));
                }

                // everything else 
                else {
                    let last_item = line.last().unwrap();

                    //last line
                    let items = match last_item.initial_type {
                        InitialTokenType::NewLine => &line[(item_index + 1)..(line.len() - 1)],
                        _ => &line[(item_index + 1)..],
                    };

                    let content = items.iter().map(|x| x.content.as_str()).collect::<String>();
                    // as well as the first and the last item in the rest of the line
                    let first = items.first();
                    let last = items.last();
                    if first.is_none() || last.is_none() {
                        // that shouldn't ever happen, I think
                        continue;
                    }

                    let position_first = &first.unwrap().position;
                    let position_last = &last.unwrap().position;

                    // push it to the list of tokens
                    tokens.push(
                        PositionedToken::new(
                            Token::Trivia(Trivia::Comment(content)), 
                            Position::new(
                                position_first.range.start..position_last.range.end, 
                                position_first.line, 
                                position_first.columns.start..position_last.columns.end)));

                    // if we had a new line in the line, let's push it in
                    if last_item.initial_type == InitialTokenType::NewLine {

                        let newline_content = last_item.content.clone();
                        let newline_position = last_item.position.clone();
                        tokens.push(
                            PositionedToken::new(
                                Token::Trivia(
                                    Trivia::NewLine(newline_content)), 
                                    newline_position));
                    }

                }

                // skip everything else in the line, since it's a comment
                indices.nth(usize::MAX);
            }

            // ASCII string
            else if item.initial_type == InitialTokenType::Control && (item.content == "\"" || item.content == "'") {
                // a string can look like this:
                // "some text here", or 'some text here'
                // potential problems:
                // user can forget a second ' or "
                // user can make an empty string - what to do then? (it should be an error)

                // I need something like
                // ["][contents of string]["] OR ['][contents of string][']
                //
                // what can also happen: (this should be an error)
                // ["][contents of string][newline(optional)] OR ['][contents of string][newline(optional)]
                // OR ["][newline(optional)] OR ['][newline(optional)] -- this may be a warning or another error as well

                
                // we shouldn't allow missmatched deliminers 
                let delimiter = &item.content;
                let delimiter_token = match delimiter.as_str() {
                    "'" => Token::ControlCharacter(ControlCharacter::Delimiter(Delimiter::SingleQuote)),
                    "\"" => Token::ControlCharacter(ControlCharacter::Delimiter(Delimiter::DoubleQuote)),
                    _d => panic!("invalid delimiter while parsing ASCII string: '{}'", _d),
                };

                // start of the comment
                tokens.push(
                    PositionedToken::new(
                        delimiter_token.clone(),
                        item.position.clone()));

                // if it's the last token in the line (that may happen if it's the last line of file and comment has nothing written yet)
                // or second-last token in line, a/k/a empty comment in not last line

                // last item in the line! 
                if item_index == line.len() - 1 {
                    //TODO: Warn/inform user about empty comment

                    lexing_issues.push(issues::empty_string(item.position.clone()));

                    lexing_issues.push(issues::unclosed_string(
                        item.position.clone(), 
                        delimiter_token.clone(), 
                        None));
                    continue;
                }
                // second last item in the line
                else if item_index == line.len() - 2 {
                    let last = line.last().unwrap();

                    // we have an empty string and unclosed delimiter
                    if last.initial_type == InitialTokenType::NewLine {
                        lexing_issues.push(issues::empty_string(item.position.clone()));
                        lexing_issues.push(issues::unclosed_string(
                            item.position.clone(), 
                            delimiter_token.clone(), 
                            None));
                        continue;
                    }
                    // we have an empty string
                    else if last.initial_type == InitialTokenType::Control && last.content.as_str() == delimiter.as_str() {
                        lexing_issues.push(issues::empty_string(item.position.clone()));
                        indices.next();
                        continue;
                    }
                    // we can have an unclosed delimiter by itself, but we'll handle that later
                }

                let the_rest = &line[item_index..];

                // since we determined that we at least have something in the string
                // let's find an end of it
                let mut string_end_index = find_closing_deliminer(the_rest, delimiter);


                // string was empty
                // add an error and skip to the end delimiter
                if string_end_index == 1 {
                    let position = the_rest[1].position.clone();
                    lexing_issues.push(issues::empty_string(position));

                    // and skip to the end of the string in line
                    indices.next();
                    continue;
                }

                let is_unclosed = string_end_index == 0;

                // if someone forgot to place a second " or ' we're just going to grab the whole line
                // except a newline
                if is_unclosed {
                    // get token of 
                    let expected = delimiter_token.clone();

                    let last = the_rest.last().unwrap();

                    string_end_index = match last.initial_type {
                        InitialTokenType::NewLine => the_rest.len() - 1,
                        _ => the_rest.len(), 
                    };

                    let actual = &the_rest[string_end_index - 1];
                    let last_char = actual.content.clone().chars().last().unwrap_or_default().to_string();

                    let actual_token = if last.initial_type == InitialTokenType::NewLine 
                    {
                        Token::Trivia(Trivia::NewLine(last_char))
                    }
                    else {
                        Token::Other(last_char)
                    };

                    let span = Position::new(
                        item.position.range.start..(actual.position.range.end),
                        item.position.line, 
                        item.position.columns.start..(actual.position.columns.end));


                    lexing_issues.push(issues::unclosed_string(span, expected, Some(actual_token)));
                }

                // grab the content of the string, as well as start and end positions
                let string = &the_rest[1..string_end_index];
                let content = string.iter().map(|x| x.content.as_str()).collect::<String>();

                let (escaped_content, escape_issues) = escape_characters(content, item.position.clone());
                let position_first = &string.first().unwrap().position;
                let position_last = &string.last().unwrap().position;
                let string_position = Position::new(
                    position_first.range.start..position_last.range.end, 
                    position_first.line,
                    position_first.range.start..position_last.range.end);

                if !escaped_content.is_ascii() {
                    let non_ascii_characters = escaped_content
                        .clone()
                        .chars()
                        .filter(|x| !x.is_ascii())
                        .collect::<Vec<char>>();
                    lexing_issues.push(issues::not_an_ascii_string(string_position.clone(), non_ascii_characters));
                }

                // push it to the vec of tokens
                tokens.push(
                    PositionedToken::new(
                        Token::String(escaped_content),
                        string_position));

                // end delimiter if string is closed
                if !is_unclosed {

                    let position = the_rest[string_end_index].position.clone();
                    tokens.push(
                        PositionedToken::new(
                            delimiter_token.clone(),
                            position));

                    // and skip to the end of the string in line
                    indices.nth(string_end_index - 1);
                }
                else {
                    // and skip to the end of the string in line
                    indices.nth(string_end_index - 2);
                }
                

                // push escape issues 
                for issue in escape_issues {
                    lexing_issues.push(issue)
                }
            }
        
            // Labels
            else if item.initial_type == InitialTokenType::Other && item_index == 0 {

                tokens.push(PositionedToken::new(Token::Label(item.content.clone()), item.position.clone()));
            }

            // number
            else if 
                item.initial_type == InitialTokenType::Control && 
                item_index > 0 && 
                item.content == "#" &&
                line.len() >= item_index + 1 &&
                (&line[item_index + 1].content).is_number() {

                let next = &line[item_index + 1];
                let num = next.content.clone();

                let token = if num.is_hexadecimal() {
                    Token::Number(Number::Hexadecimal(num))
                }
                else if num.is_octal() {
                    Token::Number(Number::Octal(num))
                }
                else if num.is_binary() {
                    Token::Number(Number::Octal(num))
                }
                else if num.is_decimal() {
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
                indices.next();
            }

            // address
            else if
                item.initial_type == InitialTokenType::Other &&
                item_index > 0 &&
                item.content.starts_with_digit() &&
                item.content.is_number() {

                let content = item.content.clone();
                let token = if content.is_hexadecimal() {
                    Token::Address(Number::Hexadecimal(content))
                }
                else if content.is_octal(){
                    Token::Address(Number::Octal(content))
                }
                else if content.is_binary() {
                    Token::Address(Number::Octal(content))
                }
                else if content.is_decimal() {
                    Token::Address(Number::Decimal(content))
                }
                else {
                    Token::Unknown(content)
                };

                tokens.push(PositionedToken::new(token, item.position.clone()));
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

            // control characters
            else if item.initial_type == InitialTokenType::Control {
                let content = item.content.clone();
                let token = match content.as_str() {
                    "(" => Token::ControlCharacter(ControlCharacter::Parenthesis(Parenthesis::Open)),
                    ")" => Token::ControlCharacter(ControlCharacter::Parenthesis(Parenthesis::Close)),
                    "," => Token::ControlCharacter(ControlCharacter::ArgumentSeparator),
                    "." => Token::ControlCharacter(ControlCharacter::AddressingSeparator),
                    "@" => Token::ControlCharacter(ControlCharacter::AddressingModifier),
                    "#" => Token::ControlCharacter(ControlCharacter::ImmediateModifier),
                    
                    "+" => Token::ControlCharacter(ControlCharacter::Arithmetic(Arithmetic::Add)),
                    "-" => Token::ControlCharacter(ControlCharacter::Arithmetic(Arithmetic::Subtract)),
                    "*" => Token::ControlCharacter(ControlCharacter::Arithmetic(Arithmetic::Multiply)),
                    "/" => Token::ControlCharacter(ControlCharacter::Arithmetic(Arithmetic::Divide)),
                    "%" => Token::ControlCharacter(ControlCharacter::Arithmetic(Arithmetic::Modulo)),

                    ":" => Token::ControlCharacter(ControlCharacter::Delimiter(Delimiter::LabelEnd)),

                    _ => {
                        let token = Token::Unknown(content);
                        lexing_issues.push(issues::unknown_token(item.position.clone(), token.clone()));
                        token
                    },
                };

                tokens.push(PositionedToken::new(token, item.position.clone()));
            }

            // others
            else if item.initial_type == InitialTokenType::Other && item_index > 0 {
                let content = item.content.clone();

                let token = if keywords::is_keyword(&content) {
                    Token::Keyword(string_to_keyword(content))
                }
                else {
                    Token::Other(content)
                };

                tokens.push(PositionedToken::new(token, item.position.clone()))
            }

            // everything that was not catched is Unknown
            else {
                
                tokens.push(PositionedToken::new(Token::Unknown(item.content.clone()), item.position.clone()))
            }

        }
    }
    (Some(tokens), lexing_issues)
}

fn escape_characters(content: String, position: Position) -> (String, Vec<Issue>) {

    // TODO: Add errors (invalid escape sequence)
    let mut escaped = String::new();
    let mut issues = Vec::<Issue>::new();

    let line_num = position.line;
    let column = position.columns.start;
    let range_start = position.range.start;

    let chars = content.chars().collect::<Vec<char>>();

    let mut indices = 0..chars.len();

    while let Some(index) = indices.next() {
        let ch = chars[index];

        let ran = range_start + index;
        let col = column + index ;

        if ch != '\\' {
            escaped.push(ch);
            continue;
        }
        if index == chars.len() - 1 {
            issues.push(issues::empty_escape_sequence(Position::new(ran..ran, line_num, col..col)));
            break;
        }

        let next = chars[index + 1];

        // \xXX
        // 0123
        let ch = if next == 'x' && index + 3 < chars.len() {

            indices.nth(2);

            let hex: String = vec![ chars[index + 2], chars[index + 3] ].iter().collect();
            let hex_num = match u8::from_str_radix(hex.as_str(), 16) {
                Ok(h) => h as u32,
                Err(err) => {
                    issues.push(
                        issues::invalid_hex_escape_sequence(
                            Position::new(ran..(ran+2), line_num, col..(col+2)),
                            err.kind().clone()));
                    continue;
                },
            };

            
            match char::from_u32(hex_num) {
                Some(c) => c,
                None => {
                    issues.push(
                        issues::invalid_hex_escape_sequence(
                            Position::new(ran..(ran+2), line_num, col..(col+2)),
                            std::num::IntErrorKind::InvalidDigit ));
                    continue;
                },
            }
        }
        else if next != 'x' {

            indices.next();

            match next {
                'a'  => '\x07',
                'b'  => '\x08',
                'f'  => '\x0C',
                'n'  => '\n',
                't'  => '\t',
                'v'  => '\x0B',
                '?'  => '?',
                '\\' => '\\',
                '\'' => '\'',
                '\"' => '\"',
                seq => {
                    issues.push(
                        issues::invalid_escape_sequence(
                            Position::new(ran..(ran+2), line_num, col..(col+2)),
                            seq));
                    continue;
                },
            }
        }
        else { 
            issues.push(
                issues::invalid_escape_sequence(
                    Position::new(ran..(ran+2), line_num, col..(col+2)),
                    next));
            continue;
        };

        escaped.push(ch);

    }

    (escaped, issues)
}

fn string_to_keyword(content: String) -> Keyword {
    if keywords::is_instruction(&content) 
    { 
        Keyword::Instruction(keywords::string_to_instruction(content).unwrap())
    }
    else if keywords::is_directive(&content) 
    { 
        Keyword::Directive(keywords::string_to_directive(content).unwrap())
    }
    else if keywords::is_register(&content) 
    { 
        Keyword::Register(keywords::string_to_register(&content).unwrap())
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

    let mut indices = 1..spanned_strings.len();

    while let Some(index) = indices.next() {
        let string = &spanned_strings[index];
        if string.initial_type == InitialTokenType::Control && string.content == "\\" {
            indices.next();
            continue;
        }
        if string.initial_type == InitialTokenType::Control && string.content == *deliminer {
            return index;
        }
    }

    0
}