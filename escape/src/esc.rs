#[derive(Debug, PartialEq, Clone)]
pub enum Esc {
    Code(EscCode),
    Unspecified { intermediate: Option<u8>, control: u8 },
}

#[derive(Debug, PartialEq, Clone)]
pub enum EscCode {
    SelectCharacterSet(u8),
    DecDoubleWidthLine,
    DecDoubleHeightTopHalfLine,
    DecDoubleHeightBottomHalfLine,
    DecNormalKeyPad,
    DecApplicationKeyPad,
    AsciiCharacterSetG0, 
}

impl Esc {
    pub fn parse(intermediate: Option<u8>, control: u8) -> Self {
        if let Some(inter) = intermediate {
            match inter {
                b'(' => return Esc::Code(EscCode::SelectCharacterSet(control)),
                _ => {}
            }
        }

        match control {
            _ => Esc::Unspecified { intermediate, control },
        }
    }
}
