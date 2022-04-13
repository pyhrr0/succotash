use std::collections::HashSet;

use serde::Serialize;

use crate::transaction::Transaction;

#[derive(Serialize)]
pub struct Client {
    pub id: u16,
    pub available_funds: f32,
    pub held_funds: f32,
    pub total_funds: f32,
    pub locked: bool,
    #[serde(skip_serializing)]
    disputes: HashSet<u32>,
}

impl Client {
    pub fn new(id: u16) -> Self {
        Client {
            id: id,
            total_funds: 0.0,
            available_funds: 0.0,
            held_funds: 0.0,
            locked: false,
            disputes: HashSet::<u32>::new(),
        }
    }

    pub fn deposit(&mut self, tx: &Transaction) {
        let deposit = tx.amount.unwrap();

        self.total_funds += deposit;
        self.available_funds += deposit;
    }

    pub fn withdrawal(&mut self, tx: &Transaction) {
        let withdrawal = tx.amount.unwrap();

        if self.available_funds >= withdrawal {
            self.total_funds -= withdrawal;
            self.available_funds -= withdrawal;
        }
    }

    pub fn dispute(&mut self, tx: Option<&Transaction>) {
        if let Some(tx) = tx {
            let dispute = tx.amount.unwrap();

            self.available_funds -= dispute;
            self.held_funds += dispute;
            self.disputes.insert(tx.id);
        }
    }

    pub fn resolve(&mut self, tx: Option<&Transaction>) {
        if let Some(tx) = tx {
            let amount = tx.amount.unwrap();

            self.available_funds += amount;
            self.held_funds -= amount;
            self.disputes.remove(&tx.id);
        }
    }

    pub fn chargeback(&mut self, tx: Option<&Transaction>) {
        if let Some(tx) = tx {
            let amount = tx.amount.unwrap();

            self.total_funds -= amount;
            self.held_funds -= amount;
            self.locked = true;
        }
    }
}
