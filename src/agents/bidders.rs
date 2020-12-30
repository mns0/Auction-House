use super::Status


struct bidders {
    status: Status,
    value: f32,
    signal: f32
    sealed: bool
}


impl bidder {
    pub fn new(status: Status, value: f32, signal: f32, sealed: bool) -> Self {
        Self {status, value, signal, sealed }
    }

    pub fn evaluate_bid (self) {
        unimplemented!()
    }

    pub fn assess_bids (self) {
        unimplemented!()
    }


    pub fn submit_bid (self) {
        unimplemented!()
    }




}
