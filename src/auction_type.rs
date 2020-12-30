//use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

//Auction Status
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AuctionType {
    English,
    Dutch,
    FirstPrice,
    SecondPrice,
    AllPay,
}

//Verbal response to status code
impl AuctionType {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::English => "English Auction",
            Self::Dutch => "Dutch Auction",
            Self::FirstPrice => "Spawning a First Price, Sealed Bid Auction",
            Self::SecondPrice => "Spawning a Second Price, Sealed Bid Auction",
            Self::AllPay => "Spawning a Second Price, Sealed Bid Auction",
        }
    }
}

impl FromStr for AuctionType {
    type Err = ();

    fn from_str(input: &str) -> Result<AuctionType, Self::Err> {
        match input {
            "English" => Ok(AuctionType::English),
            "Dutch" => Ok(AuctionType::Dutch),
            "FirstPrice" => Ok(AuctionType::FirstPrice),
            "SecondPrice" => Ok(AuctionType::SecondPrice),
            "AllPay" => Ok(AuctionType::AllPay),
            _ => Err(()),
        }
    }
}

// //Fortamtted display for Status interface
// impl Display for AuctionType {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         write!(f, "{}", *self as str)
//     }
// }
