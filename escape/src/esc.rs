#[derive(Debug, PartialEq, Clone)]
pub enum Esc {
    Code(EscCode),
    Unspecified { intermediate: Option<u8>, control: u8 },
}

#[derive(Debug, PartialEq, Clone)]
pub enum EscCode {

}

impl Esc {
    pub fn parse(intermediate: Option<u8>, control: u8) -> Self {
        Esc::Unspecified { intermediate, control }
    }
}
