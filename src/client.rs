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

    pub fn withdraw(&mut self, tx: &Transaction) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initalization() {
        let client = Client::new(42);
        assert_eq!(client.id, 42);
        assert_eq!(client.available_funds, 0.0);
        assert_eq!(client.held_funds, 0.0);
        assert_eq!(client.total_funds, 0.0);
        assert_eq!(client.locked, false);
        assert_eq!(client.disputes.len(), 0);
    }

    #[test]
    fn deposit() {
        let mut client = Client::new(42);
        let tx = Transaction {
            id: 42,
            kind: "deposit".into(),
            amount: Some(1337.0),
            client_id: client.id,
        };
        client.deposit(&tx);

        assert_eq!(client.available_funds, 1337.0);
        assert_eq!(client.total_funds, 1337.0);
    }

    #[test]
    fn withdrawal() {
        let mut client = Client::new(42);
        client.available_funds = 1337.0;
        client.total_funds = 1337.0;
        let tx = Transaction {
            id: 42,
            kind: "withdrawal".into(),
            amount: Some(1337.0),
            client_id: client.id,
        };
        client.withdraw(&tx);

        assert_eq!(client.available_funds, 0.0);
        assert_eq!(client.total_funds, 0.0);
    }

    #[test]
    fn dispute() {
        let mut client = Client::new(42);
        client.available_funds = 1337.0;
        let tx = Transaction {
            id: 42,
            kind: "dispute".into(),
            amount: Some(1337.0),
            client_id: client.id,
        };
        client.dispute(Some(&tx));

        assert_eq!(client.available_funds, 0.0);
        assert_eq!(client.held_funds, 1337.0);
        assert_eq!(client.disputes.get(&tx.id), Some(&tx.id));
    }

    #[test]
    fn resolve() {
        let mut client = Client::new(42);
        client.available_funds = 0.0;
        client.held_funds = 1337.0;
        let tx = Transaction {
            id: 42,
            kind: "resolve".into(),
            amount: Some(1337.0),
            client_id: client.id,
        };
        client.disputes.insert(tx.id);
        client.resolve(Some(&tx));

        assert_eq!(client.available_funds, 1337.0);
        assert_eq!(client.held_funds, 0.0);
        assert_eq!(client.disputes.get(&tx.id), None);
    }

    #[test]
    fn chargeback() {
        let mut client = Client::new(42);
        client.total_funds = 1337.0;
        client.held_funds = 1337.0;
        assert_eq!(client.locked, false);

        let tx = Transaction {
            id: 42,
            kind: "chargeback".into(),
            amount: Some(1337.0),
            client_id: client.id,
        };
        client.chargeback(Some(&tx));

        assert_eq!(client.total_funds, 0.0);
        assert_eq!(client.held_funds, 0.0);
        assert_eq!(client.locked, true);
    }
}
