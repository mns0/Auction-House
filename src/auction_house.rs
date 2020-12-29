use super::AuctionType;


pub struct AuctionHouse {
    bidders: u16,
    auction_item: String,
    type: AuctionType
}



impl AuctionHouse {

    pub fn new(bidders: u16, auction_item: String, type: AuctionType) -> Self {
        Self { bidders , auction_item, type }
    }


    pub fn run(self, mut handler: impl Handler) {
        unimplemented!()
    }




}