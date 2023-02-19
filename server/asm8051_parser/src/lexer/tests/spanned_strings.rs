use ropey::Rope;
use test_case::test_case;
use crate::lexer::{
    initial::{SpannedString, InitialTokenType, self},
    Position,
    tests::spanned_string
};
use std::ops::Range;

#[test_case(
    "abc cde   dcr xyz",
    vec![ 
        SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
        SpannedString::from_str("cde", Position::new(4..6, 0, 4..6), InitialTokenType::Other),
        SpannedString::from_str("   ", Position::new(7..9, 0, 7..9), InitialTokenType::WhiteSpace),
        SpannedString::from_str("dcr", Position::new(10..12, 0, 10..12), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(13..13, 0, 13..13), InitialTokenType::WhiteSpace),
        SpannedString::from_str("xyz", Position::new(14..16, 0, 14..16), InitialTokenType::Other),
        ]
        ; "simple one line source" )]
#[test_case(
    r#"abc cde
 dcr xyz"#,
    vec![ 
        SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
        SpannedString::from_str("cde", Position::new(4..6, 0, 4..6), InitialTokenType::Other),
        SpannedString::from_str("\n",  Position::new(7..7, 0, 7..7), InitialTokenType::NewLine),

        SpannedString::from_str(" ",   Position::new(8..8, 1, 0..0), InitialTokenType::WhiteSpace),
        SpannedString::from_str("dcr", Position::new(9..11, 1, 1..3), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(12..12, 1, 4..4), InitialTokenType::WhiteSpace),
        SpannedString::from_str("xyz", Position::new(13..15, 1, 5..7), InitialTokenType::Other),
    ]
    ; "simple two line source" )]
#[test_case(
    "abc cde ;xyz",
    vec![
        SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
        SpannedString::from_str("cde", Position::new(4..6, 0, 4..6), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(7..7, 0, 7..7), InitialTokenType::WhiteSpace),
        SpannedString::from_str(";",   Position::new(8..8, 0, 8..8), InitialTokenType::Control),
        SpannedString::from_str("xyz", Position::new(9..11, 0, 9..11), InitialTokenType::Other),
    ]
    ; "simple one line source with a comment" )]
#[test_case(
r#"abc ;cde
 dcr ; xyz"#,
    vec![ 
        SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
        SpannedString::from_str(";",   Position::new(4..4, 0, 4..4), InitialTokenType::Control),
        SpannedString::from_str("cde", Position::new(5..7, 0, 5..7), InitialTokenType::Other),
        SpannedString::from_str("\n",  Position::new(8..8, 0, 8..8), InitialTokenType::NewLine),
        
        SpannedString::from_str(" ",   Position::new(9..9, 1, 0..0), InitialTokenType::WhiteSpace),
        SpannedString::from_str("dcr", Position::new(10..12, 1, 1..3), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(13..13, 1, 4..4), InitialTokenType::WhiteSpace),
        SpannedString::from_str(";",   Position::new(14..14, 1, 5..5), InitialTokenType::Control),
        SpannedString::from_str(" ",   Position::new(15..15, 1, 6..6), InitialTokenType::WhiteSpace),
        SpannedString::from_str("xyz", Position::new(16..18, 1, 7..9), InitialTokenType::Other),
    ]
    ; "simple two line source with comments" )]
#[test_case(
r#"    LJMP       START             
    ORG       100H
START:
    MOV    R0,#30H             ;address"#,
    vec![
        //line 0
        spanned_string("    ", 0..3, 0, 0..3, InitialTokenType::WhiteSpace),
        spanned_string("LJMP", 4..7, 0, 4..7, InitialTokenType::Other),
        spanned_string("       ", 8..14, 0, 8..14, InitialTokenType::WhiteSpace),
        spanned_string("START", 15..19, 0, 15..19, InitialTokenType::Other),
        spanned_string("             ", 20..32, 0, 20..32, InitialTokenType::WhiteSpace),
        spanned_string("\n", 33..33, 0, 33..33, InitialTokenType::NewLine),

        //line 1
        spanned_string("    ", 34..37, 1, 0..3, InitialTokenType::WhiteSpace),
        spanned_string("ORG", 38..40, 1, 4..6, InitialTokenType::Other),
        spanned_string("       ", 41..47, 1, 7..13, InitialTokenType::WhiteSpace),
        spanned_string("100H", 48..51, 1, 14..17, InitialTokenType::Other),
        spanned_string("\n", 52..52, 1, 18..18, InitialTokenType::NewLine),

        //line 2
        spanned_string("START", 53..57, 2, 0..4, InitialTokenType::Other),
        spanned_string(":", 58..58, 2, 5..5, InitialTokenType::Control),
        spanned_string("\n", 59..59, 2, 6..6, InitialTokenType::NewLine),

        //line 3
        spanned_string("    ", 60..63, 3, 0..3, InitialTokenType::WhiteSpace),
        spanned_string("MOV", 64..66, 3, 4..6, InitialTokenType::Other),
        spanned_string("    ", 67..70, 3, 7..10, InitialTokenType::WhiteSpace),
        spanned_string("R0", 71..72, 3, 11..12, InitialTokenType::Other),
        spanned_string(",", 73..73, 3, 13..13, InitialTokenType::Control),
        spanned_string("#", 74..74, 3, 14..14, InitialTokenType::Control),
        spanned_string("30H", 75..77, 3, 15..17, InitialTokenType::Other),
        spanned_string("             ", 78..90, 3, 18..30, InitialTokenType::WhiteSpace),
        spanned_string(";", 91..91, 3, 31..31, InitialTokenType::Control),
        spanned_string("address", 92..98, 3, 32..38, InitialTokenType::Other),
    ]
    ; "some more complex code")]
fn get_spanned_strings_for(source: &str, expected_strings: Vec<SpannedString>) {
    
    let rope = Rope::from_str(source);
    let actual_strings = initial::get_spanned_strings(rope);
    for item_index in 0..(actual_strings.len()) {
        let expected = &expected_strings[item_index];
        let actual = &actual_strings[item_index];

        assert_eq!(expected.content, actual.content, "item {}, content is different", item_index);
        assert_eq!(expected.position, actual.position, "item {}, position is different", item_index);
        assert_eq!(expected.initial_type, actual.initial_type, "item {}, initiall_type is different", item_index);
    }
}



#[test_case(
    vec![
        //line 0
        spanned_string("    ", 0..3, 0, 0..3, InitialTokenType::WhiteSpace),
        spanned_string("LJMP", 4..7, 0, 4..7, InitialTokenType::Other),
        spanned_string("       ", 8..14, 0, 8..14, InitialTokenType::WhiteSpace),
        spanned_string("START", 15..19, 0, 15..19, InitialTokenType::Other),
        spanned_string("             ", 20..32, 0, 20..32, InitialTokenType::WhiteSpace),
        spanned_string("\n", 33..33, 0, 33..33, InitialTokenType::NewLine),

        //line 1
        spanned_string("    ", 34..37, 1, 0..3, InitialTokenType::WhiteSpace),
        spanned_string("ORG", 38..40, 1, 4..6, InitialTokenType::Other),
        spanned_string("       ", 41..47, 1, 7..13, InitialTokenType::WhiteSpace),
        spanned_string("100H", 48..51, 1, 14..17, InitialTokenType::Other),
        spanned_string("\n", 52..52, 1, 18..18, InitialTokenType::NewLine),

        //line 2
        spanned_string("START", 53..57, 2, 0..4, InitialTokenType::Other),
        spanned_string(":", 58..58, 2, 5..5, InitialTokenType::Control),
        spanned_string("\n", 59..59, 2, 6..6, InitialTokenType::NewLine),

        //line 3
        spanned_string("    ", 60..63, 3, 0..3, InitialTokenType::WhiteSpace),
        spanned_string("MOV", 64..66, 3, 4..6, InitialTokenType::Other),
        spanned_string("    ", 67..70, 3, 7..10, InitialTokenType::WhiteSpace),
        spanned_string("R0", 71..72, 3, 11..12, InitialTokenType::Other),
        spanned_string(",", 73..73, 3, 13..13, InitialTokenType::Control),
        spanned_string("#", 74..74, 3, 14..14, InitialTokenType::Control),
        spanned_string("30H", 75..77, 3, 15..17, InitialTokenType::Other),
        spanned_string("             ", 78..90, 3, 18..30, InitialTokenType::WhiteSpace),
        spanned_string(";", 91..91, 3, 31..31, InitialTokenType::Control),
        spanned_string("address", 92..98, 3, 32..38, InitialTokenType::Other),
    ],
    vec![
        vec![
            //line 0
            spanned_string("    ", 0..3, 0, 0..3, InitialTokenType::WhiteSpace),
            spanned_string("LJMP", 4..7, 0, 4..7, InitialTokenType::Other),
            spanned_string("       ", 8..14, 0, 8..14, InitialTokenType::WhiteSpace),
            spanned_string("START", 15..19, 0, 15..19, InitialTokenType::Other),
            spanned_string("             ", 20..32, 0, 20..32, InitialTokenType::WhiteSpace),
            spanned_string("\n", 33..33, 0, 33..33, InitialTokenType::NewLine),
        ],
        vec![
            //line 1
            spanned_string("    ", 34..37, 1, 0..3, InitialTokenType::WhiteSpace),
            spanned_string("ORG", 38..40, 1, 4..6, InitialTokenType::Other),
            spanned_string("       ", 41..47, 1, 7..13, InitialTokenType::WhiteSpace),
            spanned_string("100H", 48..51, 1, 14..17, InitialTokenType::Other),
            spanned_string("\n", 52..52, 1, 18..18, InitialTokenType::NewLine),
        ],
        vec![
            //line 2
            spanned_string("START", 53..57, 2, 0..4, InitialTokenType::Other),
            spanned_string(":", 58..58, 2, 5..5, InitialTokenType::Control),
            spanned_string("\n", 59..59, 2, 6..6, InitialTokenType::NewLine),
        ],
        vec![
            //line 3
            spanned_string("    ", 60..63, 3, 0..3, InitialTokenType::WhiteSpace),
            spanned_string("MOV", 64..66, 3, 4..6, InitialTokenType::Other),
            spanned_string("    ", 67..70, 3, 7..10, InitialTokenType::WhiteSpace),
            spanned_string("R0", 71..72, 3, 11..12, InitialTokenType::Other),
            spanned_string(",", 73..73, 3, 13..13, InitialTokenType::Control),
            spanned_string("#", 74..74, 3, 14..14, InitialTokenType::Control),
            spanned_string("30H", 75..77, 3, 15..17, InitialTokenType::Other),
            spanned_string("             ", 78..90, 3, 18..30, InitialTokenType::WhiteSpace),
            spanned_string(";", 91..91, 3, 31..31, InitialTokenType::Control),
            spanned_string("address", 92..98, 3, 32..38, InitialTokenType::Other),
        ],
    ]
; "1")]
#[test_case(
    vec![ 
        SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
        SpannedString::from_str(";",   Position::new(4..4, 0, 4..4), InitialTokenType::Control),
        SpannedString::from_str("cde", Position::new(5..7, 0, 5..7), InitialTokenType::Other),
        SpannedString::from_str("\n",  Position::new(8..8, 0, 8..8), InitialTokenType::NewLine),
        
        SpannedString::from_str(" ",   Position::new(9..9, 1, 0..0), InitialTokenType::WhiteSpace),
        SpannedString::from_str("dcr", Position::new(10..12, 1, 1..3), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(13..13, 1, 4..4), InitialTokenType::WhiteSpace),
        SpannedString::from_str(";",   Position::new(14..14, 1, 5..5), InitialTokenType::Control),
        SpannedString::from_str(" ",   Position::new(15..15, 1, 6..6), InitialTokenType::WhiteSpace),
        SpannedString::from_str("xyz", Position::new(16..18, 1, 7..9), InitialTokenType::Other),
    ],
    vec![ 
        vec![
            SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
            SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
            SpannedString::from_str(";",   Position::new(4..4, 0, 4..4), InitialTokenType::Control),
            SpannedString::from_str("cde", Position::new(5..7, 0, 5..7), InitialTokenType::Other),
            SpannedString::from_str("\n",  Position::new(8..8, 0, 8..8), InitialTokenType::NewLine),
        ],
        vec![

            SpannedString::from_str(" ",   Position::new(9..9, 1, 0..0), InitialTokenType::WhiteSpace),
            SpannedString::from_str("dcr", Position::new(10..12, 1, 1..3), InitialTokenType::Other),
            SpannedString::from_str(" ",   Position::new(13..13, 1, 4..4), InitialTokenType::WhiteSpace),
            SpannedString::from_str(";",   Position::new(14..14, 1, 5..5), InitialTokenType::Control),
            SpannedString::from_str(" ",   Position::new(15..15, 1, 6..6), InitialTokenType::WhiteSpace),
            SpannedString::from_str("xyz", Position::new(16..18, 1, 7..9), InitialTokenType::Other),
        ]
    ]
; "2")]
#[test_case(
    vec![
        SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
        SpannedString::from_str("cde", Position::new(4..6, 0, 4..6), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(7..7, 0, 7..7), InitialTokenType::WhiteSpace),
        SpannedString::from_str(";",   Position::new(8..8, 0, 8..8), InitialTokenType::Control),
        SpannedString::from_str("xyz", Position::new(9..11, 0, 9..11), InitialTokenType::Other),
    ],
    vec![
        vec![
        SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
        SpannedString::from_str("cde", Position::new(4..6, 0, 4..6), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(7..7, 0, 7..7), InitialTokenType::WhiteSpace),
        SpannedString::from_str(";",   Position::new(8..8, 0, 8..8), InitialTokenType::Control),
        SpannedString::from_str("xyz", Position::new(9..11, 0, 9..11), InitialTokenType::Other),
        ]
    ]
; "3")]
#[test_case(
    vec![
        SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
        SpannedString::from_str("cde", Position::new(4..6, 0, 4..6), InitialTokenType::Other),
        SpannedString::from_str("   ", Position::new(7..9, 0, 7..9), InitialTokenType::WhiteSpace),
        SpannedString::from_str("dcr", Position::new(10..12, 0, 10..12), InitialTokenType::Other),
        SpannedString::from_str(" ",   Position::new(13..13, 0, 13..13), InitialTokenType::WhiteSpace),
        SpannedString::from_str("xyz", Position::new(14..16, 0, 14..16), InitialTokenType::Other),
    ],
    vec![
        vec![
            SpannedString::from_str("abc", Position::new(0..2, 0, 0..2), InitialTokenType::Other),
            SpannedString::from_str(" ",   Position::new(3..3, 0, 3..3), InitialTokenType::WhiteSpace),
            SpannedString::from_str("cde", Position::new(4..6, 0, 4..6), InitialTokenType::Other),
            SpannedString::from_str("   ", Position::new(7..9, 0, 7..9), InitialTokenType::WhiteSpace),
            SpannedString::from_str("dcr", Position::new(10..12, 0, 10..12), InitialTokenType::Other),
            SpannedString::from_str(" ",   Position::new(13..13, 0, 13..13), InitialTokenType::WhiteSpace),
            SpannedString::from_str("xyz", Position::new(14..16, 0, 14..16), InitialTokenType::Other),
        ]  
    ]
; "4")]
fn split_spanned_strings_into_lines(strings: Vec::<SpannedString>, expected: Vec::<Vec::<SpannedString>>){
    let actual = initial::split_spanned_strings_into_lines(strings);
    assert_eq!(expected.len(), actual.len(), "actual length is different from actual length");

    for line_number in 0..actual.len() {
        let expected_line = &expected[line_number];
        let actual_line = &actual[line_number];

        assert_eq!(expected_line.len(), actual_line.len(), "line {}, expected length different from actual length", line_number);

        for item_index in 0..actual_line.len() {
            let expected_item = &expected_line[item_index];
            let actual_item = &actual_line[item_index];

            assert_eq!(expected_item.content, actual_item.content, "line {}, item {} different contents", line_number, item_index);
            assert_eq!(expected_item.initial_type, actual_item.initial_type, "line {}, item {} different initial types", line_number, item_index);
            assert_eq!(expected_item.position, actual_item.position, "line {}, item {} different positions", line_number, item_index);
        }
    }
}