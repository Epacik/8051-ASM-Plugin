use ropey::Rope;
use super::Position;

pub(super) struct SpannedString {
    pub content: String,
    pub position: Position,
    pub initial_type: InitialTokenType,
}

impl std::fmt::Debug for SpannedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpannedString").field("content", &self.content).field("position", &self.position).field("initial_type", &self.initial_type).finish()
    }
}

impl SpannedString {
    pub fn new(string: String, position: Position, initial_type: InitialTokenType) -> SpannedString {
        SpannedString { content: string, position, initial_type }
    }

    pub fn from_str(string: &str, position: Position, initial_type: InitialTokenType) -> SpannedString {
        SpannedString { content: String::from(string), position, initial_type }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone, Copy)]
pub(super) enum InitialTokenType {
    #[default]
    None,
    NewLine,
    WhiteSpace,
    Control,
    Other,
}

fn is_control_char(c: char) -> bool {
    match c {
        ';' | ':' => true,
        '"' | '\'' => true,
        '(' | ')' => true,
        ',' => true,
        '#' => true,
        '@' => true,
        '+' | '-' | '*' | '/' | '%' => true,
        _ => false,
    }
}

pub(super) fn is_newline(c: char) -> bool {
    match c {
        '\n' | '\r' | '\x0B' | '\x0C' | '\u{0085}' | '\u{2028}' | '\u{2029}' => true,
        _ => false
    }
}

pub(super) fn is_whitespace(c: char, match_newline: bool) -> bool {
    if match_newline {
        c.is_whitespace() || is_newline(c)
    }
    else {
        c.is_whitespace() && !is_newline(c)
    }
}

pub(super) fn get_initial_char_type(c: char) -> InitialTokenType {
    if is_newline(c) {
        InitialTokenType::NewLine
    }
    else if is_control_char(c) {
        InitialTokenType::Control
    }
    else if is_whitespace(c, false){
        InitialTokenType::WhiteSpace
    }
    else {
        InitialTokenType::Other
    }
}

pub(super) fn get_spanned_strings(source: Rope) -> Vec<SpannedString> {

    let mut spans = Vec::<SpannedString>::new();

    // let's  go trough all of the lines
    for line_idx in 0..(source.len_lines()) {
        
        //if, for some reason, we ventured outside of the bounds of provided source, let's just break out 
        let line = match source.get_line(line_idx) {
            Some(l) => l,
            None => break,
        };
        
        // index of a first char of a current line in a source text
        let line_char_idx = source.line_to_char(line_idx);
        
        let mut buf = Vec::<(char, InitialTokenType, usize)>::new();
        
        let push_buf_to_spans = |buf: &Vec<(char, InitialTokenType, usize)>, spans: &mut  Vec<SpannedString>,| {
            let start_column = buf.first().unwrap().2;
            let end_column = buf.last().unwrap().2;
            
            let out_str = buf.iter().map(|x| x.0).collect::<String>();
            
            let position = Position::new(
                (line_char_idx+start_column)..(line_char_idx+end_column), 
                line_idx, 
                start_column..end_column);

            let tp = match buf.first() {
                Some(x) => x.1,
                None => InitialTokenType::None
            };


            spans.push(SpannedString::new(out_str, position, tp));
        };

        //we're going trough the line
        let chars = line.chars().collect::<Vec<char>>();
        for char_idx in 0..(line.len_chars()) {
            let ch = chars[char_idx];
            
            let buf_len = buf.len();
    
            let char_type = get_initial_char_type(ch);
            let last_type = if buf_len > 0 { &buf.last().unwrap().1 } else { &InitialTokenType::None };

            // if type has changed (or it is a Control character) push it to spans and reset buffer
            if (char_type != *last_type || char_type == InitialTokenType::Control) && buf_len > 0 {
                push_buf_to_spans(&buf, &mut spans);
                buf = Vec::new();
            }

            buf.push((ch, char_type, char_idx));
        }

        if buf.len() > 0 {
            push_buf_to_spans(&buf, &mut spans);
            buf = Vec::new();
        }

    }

    spans
}

pub(super) fn split_spanned_strings_into_lines(strings: Vec<SpannedString>) -> Vec<Vec<SpannedString>> {
    let mut result = Vec::<Vec::<SpannedString>>::new();
    let mut line = Vec::<SpannedString>::new();

    for string in strings 
    {
        let last_line_index = match line.last() 
        {
            Some(it) => (*it).position.line,
            None => 0,
        };

        let current_line_index = string.position.line;

        if current_line_index > last_line_index {
            result.push(line);
            line = Vec::<SpannedString>::new();
        }

        line.push(string);

    }

    if line.len() > 0 {
        result.push(line);
    }

    result
}