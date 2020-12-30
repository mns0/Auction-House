use crate::agents::bidders;

use super::AuctionType;
use bidders::Bidder;

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
        //Spawn a set of bidders
        let bidder_vec: Vec<Bidder> = Vec::new();
        for i in 0..self.num_bidders {
            bidder_vec.push(Bidder::new(false, 0.0, 0.0, true, true))
        }

        unimplemented!()
    }
}
