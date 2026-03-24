use std::collections::{HashMap, btree_map::Keys};

pub fn array_example() {
    let block_hashes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 123, 456, 7890];
    println!("Block Hashes Array: {:?}", block_hashes);
    println!("First Block hash: {}", block_hashes[0]);
    println!("Last block hash: {}", block_hashes[block_hashes.len() - 1]);

    let slice = &block_hashes[1..=4];
    println!("Sliced Block Hashes: {:?}", slice);
}

pub fn vector_example() {
    let mut transaction_ids: Vec<&str> = vec!["tx1", "tx2", "tx3"];
    transaction_ids.push("tx4");
    transaction_ids.push("tx5");

    println!("After adding transactions: {:?}", transaction_ids);

    transaction_ids.pop();
    println!("After adding transactions: {:?}", transaction_ids);

    for tx in &transaction_ids {
        println!("Transaction ID: {}", tx);
    }

    if let Some(first_tx) = transaction_ids.get(0) {
        println!("First transaction ID: {}", first_tx);
    }
}

pub fn tuple_example() {
    let user_info = ("Alice", 30, 2.5);
    println!("UserinfoTuple: {:#?}", user_info);

    println!("Username: {}", user_info.0);
    println!("Age: {}", user_info.1);
    println!("Token Balances: {}", user_info.2);

    let (name, age, balance) = user_info;
    println!(
        "Destructured Info: Name = {}, Age = {}, Token Balance = {}",
        name, age, balance
    );
}

pub fn hash_map_example() {
    let mut balances: HashMap<&str, u32> = HashMap::new();
    balances.insert("token address1", 100);
    balances.insert("token address2", 150);
    balances.insert("token address3", 180);

    balances.insert("token address3", 280);

    println!("User balances hashmap: {:#?}", balances);

    match balances.get("token address1") {
        Some(value) => println!("Token Addr1's balance is: {}", value),
        None => println!("No Address 1."),
    }

    for (user, balance) in &balances {
        println!("{}'s Balance: {}", user, balance);
    }

    balances
        .entry("token address2")
        .and_modify(|balance| *balance -= 25);

    println!("Updated Alice's Balance: {:#?}", balances);

    balances.remove("token address1");
    println!("After Removing User: {:#?}", balances);
}

pub fn nested_data_structures_example() {
    let mut user_transactions: HashMap<&str, Vec<&str>> = HashMap::new();
    user_transactions.insert("token_address1", vec!["tx1", "tx2", "tx3"]);
    user_transactions.insert("token address2", vec!["tx4", "tx5"]);

    println!("User Transaction HashMap: {:#?}", user_transactions);

    if let Some(transactions) = user_transactions.get("token address2") {
        println!("User's Transaction: {:?}", transactions);
    }

    if let Some(transactions) = user_transactions.get_mut("token_address1") {
        transactions[0] = "tx_four";
        transactions.push("tx6");
        println!("Mutable User's Transaction: {:?}", transactions);
    }
}

pub fn demo() {
    array_example();
    vector_example();
    tuple_example();
    hash_map_example();
    nested_data_structures_example();
}
