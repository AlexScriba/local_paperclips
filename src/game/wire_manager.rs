
#[derive(Debug)]
pub struct WireManager{
    wire: u64,
    increase_amount: u64,
}

impl WireManager {
    pub fn amnt_wire(&self) -> u64 {
        self.wire
    }

    pub fn buy_wire(&mut self) {
        self.wire += self.increase_amount;
    }

    pub fn use_wire(&mut self, amount: u64) -> u64 {
        if amount > self.wire {
            let remaining = self.wire;
            self.wire = 0;
            return remaining;
        }

        self.wire -= amount;
        amount
    }

    pub fn buy_cost(&self) -> f64 {
        20.0
    }
}

impl Default for WireManager {
    fn default() -> Self {
        WireManager {
            wire: 1000,
            increase_amount: 1000,
        }
    }
}
