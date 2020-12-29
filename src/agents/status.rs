use std::fmt::{Display, Formatter, Result as FmtResult};

//Auction Status
#[derive(Copy, Clone, Debug)]
pub enum Status {
    Thinking = 100,
    SentBid = 200,
}

//Verbal response to status code
impl Status {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Thinking => "Thinking",
            Self::SentBid => "Bid has been set",
        }
    }
}

//Fortamtted display for Status interface
impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
