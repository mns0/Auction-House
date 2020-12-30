use super::AuctionType;

pub struct AuctionHouse {
    num_bidders: u16,
    auction_item: String,
    _type: AuctionType,
    bid_floor: f32,
}

impl AuctionHouse {
    pub fn new(num_bidders: u16, auction_item: String, _type: AuctionType, bid_floor: f32) -> Self {
        Self {
            num_bidders,
            auction_item,
            _type,
            bid_floor,
        }
    }

    pub fn run(self) {
        unimplemented!()
    }
}
