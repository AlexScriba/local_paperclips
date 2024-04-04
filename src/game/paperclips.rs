use rand::{rngs::ThreadRng, Rng};

use super::{clipper::ClipperManager, wire_manager::WireManager};

#[derive(Debug)]
pub struct Game {
    money: f64,
    paperclips: u128,
    sell_price: f64,
    public_demand: f64,
    clippers: ClipperManager,
    wire_manager: WireManager,
    purchase_modifier: f64,
    rng: ThreadRng,
}

impl Game {
    pub fn tick(&mut self) {
        self.handle_buy_paperclips();
        self.calculate_public_demand();
        self.produce_clips();
    }

    pub fn num_paperclips(&self) -> u128 {
        self.paperclips
    }

    pub fn money(&self) -> f64 {
        self.money
    }

    pub fn sell_price(&self) -> f64 {
        self.sell_price
    }

    pub fn public_demand(&self) -> f64 {
        self.public_demand
    }

    pub fn num_clippers(&self) -> u32 {
        self.clippers.num_clippers()
    }

    pub fn clippers(&self) -> &ClipperManager {
        &self.clippers
    }

    pub fn wire_manager(&self) -> &WireManager {
        &self.wire_manager
    }

    pub fn make_paperclip(&mut self) {
        let amount = self.wire_manager.use_wire(1);
        self.paperclips += amount as u128;
    }

    pub fn change_sell_price(&mut self, val: f64){
        self.sell_price = (self.sell_price + val).max(0.01);
    }

    pub fn buy_wire(&mut self) {
        let cost = self.wire_manager.buy_cost();

        if cost > self.money {
            return
        }

        self.money -= cost;
        self.wire_manager.buy_wire();
    }

    pub fn buy_clipper(&mut self) {
        let price = self.clippers.buy_price();

        if price > self.money {
            return;
        }
        
        self.money -= price;
        self.clippers.increment();
    }

    fn produce_clips(&mut self) {
        let num_clips = self.clippers.produce_clips();
        let num_used = self.wire_manager.use_wire(num_clips as u64);
        self.paperclips += num_used as u128;
    }

    fn calculate_public_demand(&mut self) {
        let change = self.calc_demand(self.sell_price);
        self.public_demand = if change <= 0.0 { 0.0 } else { change }
    }

    fn handle_buy_paperclips(&mut self) {
        let buying = self.rng.gen_range(0.0..=1.0) <= self.public_demand();

        if !buying {
            return;
        }

        let purchase_amount = (self.paperclips as f64 * self.public_demand * 0.1).round() as u128;

        let purchase_amount = purchase_amount.max(1);

        if purchase_amount > self.paperclips {
            return;
        }

        self.paperclips -= purchase_amount;
        self.money += self.sell_price as f64 * purchase_amount as f64;
    }

    fn calc_demand(&self, val: f64) -> f64 {
        (10000.0 as f64).powf(-val + self.purchase_modifier)
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            money: 100.0,
            paperclips: 0,
            sell_price: 0.2,
            public_demand: 1.0,
            purchase_modifier: 0.1,
            rng: rand::thread_rng(),
            clippers: ClipperManager::new(),
            wire_manager: WireManager::default(),
        }
    }
}
