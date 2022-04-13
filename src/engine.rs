use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::stdout;
use std::str::FromStr;

use crate::client::Client;
use crate::transaction::{Transaction, TransactionType};

fn process_record(
    headers: &csv::StringRecord,
    row: &csv::StringRecord,
    clients: &mut HashMap<u16, Client>,
    transactions: &mut HashMap<u32, Transaction>,
) -> Result<(), Box<dyn Error>> {
    let mut tx = row.deserialize::<Transaction>(Some(&headers))?;

    if let Some(amount) = tx.amount {
        tx.amount = Some((amount * 10000.0).round() / 10000.0);
    }

    let client = clients
        .entry(tx.client_id)
        .or_insert(Client::new(tx.client_id));

    match TransactionType::from_str(&tx.kind)? {
        TransactionType::Deposit => {
            client.deposit(&tx);
            transactions.insert(tx.id, tx);
        }
        TransactionType::Withdrawal => {
            client.withdrawal(&tx);
            transactions.insert(tx.id, tx);
        }
        TransactionType::Dispute => {
            client.dispute(transactions.get(&tx.id));
        }
        TransactionType::Resolve => {
            client.resolve(transactions.get(&tx.id));
        }
        TransactionType::Chargeback => {
            client.chargeback(transactions.get(&tx.id));
        }
    }

    Ok(())
}

pub fn process_transactions(csv_file: &str) -> Result<(), Box<dyn Error>> {
    let mut clients: HashMap<u16, Client> = HashMap::new();
    let mut transactions: HashMap<u32, Transaction> = HashMap::new();

    let file_handle = File::open(csv_file)?;
    let mut csv_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(file_handle);

    let headers = csv_reader.headers()?.clone();
    let mut row = csv::StringRecord::new();

    while csv_reader.read_record(&mut row)? {
        process_record(&headers, &row, &mut clients, &mut transactions)?;
    }

    let mut csv_writer = csv::WriterBuilder::new().from_writer(stdout());
    for client in clients.values() {
        csv_writer.serialize(client)?;
    }

    Ok(())
}
