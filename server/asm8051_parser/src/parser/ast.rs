use crate::{lexer::{tokens::{Instruction, Register}, Position}, lexer_old::Number};

pub enum TopLevel {
    Line(Line)
}

pub trait Positioned {
    fn position(&self) -> Position;
}

pub enum Argument {
    Register(Register),
    FlagOrBit(String),
    Label(String),
    Address(Number),
    Number(Number),
}


pub struct InstructionCall {
    pub instruction: Instruction,
    pub arguments: Vec::<Argument>,
    position: crate::lexer::Position,
}

impl Positioned for InstructionCall {
    fn position(&self) -> Position {
        self.position
    }
}

pub struct Label {
    pub value: String,
    pub has_delimiter: bool,
    position: Position,
}
impl Positioned for Label {
    fn position(&self) -> Position {
        self.position
    }
}

pub struct Comment {
    pub value: String,
    position: Position,
}
impl Positioned for Comment {
    fn position(&self) -> Position {
        self.position
    }
}

pub struct Line {
    label: Option<Comment>,
    instruction: Option<InstructionCall>,
    comment: Option<Comment>,
    position: Position,
}

impl Positioned for Line {
    fn position(&self) -> Position {
        self.position
    }
}
