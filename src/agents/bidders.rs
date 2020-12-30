//use super::status::Status;

pub struct Bidder {
    status: bool,
    value: f32,
    signal: f32,
    sealed: bool,
    strategy: bool,
}

impl Bidder {
    pub fn new(status: bool, value: f32, signal: f32, sealed: bool, strategy: bool) -> Self {
        Self {
            status,
            value,
            signal,
            sealed,
            strategy,
        }
    }

    pub fn evaluate_bid(self) {
        unimplemented!()
    }

    pub fn assess_bids(self) {
        unimplemented!()
    }

    pub fn submit_bid(self) {
        unimplemented!()
    }
}
