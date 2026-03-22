fn primitive_data_types() {
    let token_supply: u128 = 1_000_000_000_000_000_000;
    let block_number: i64 = -0123456789;

    println!("Token Supply (u128): {}", token_supply);
    println!("Block Number (i64): {}", block_number);

    let token_price: f32 = 3.14;
    let transaction_fee: f64 = 0.000001;
    println!("Token Price (f32): {}", token_price);
    println!("Transaction Fee (f64): {}", transaction_fee);

    let is_tx_valid: bool = true;
    println!("Is the transaction valid: {}", is_tx_valid);

    let token_symbol: char = 'T';
    println!("Token Symbol: {}", token_symbol);

    let wallet_address: &str = "0x1234567890ABCEDF";
    let contract_name: String = String::from("DEX");

    println!("Wallet Address: {}", wallet_address);
    println!("Contract Name: {}", contract_name);

    let gas_price: f64 = 0.000000012;
    let gas_used: f64 = 21000.0;
    println!("Gas Price = {}, Gas Used = {}", gas_price, gas_used);
    println!("Total Gas Fee: {:.8}", gas_price * gas_used);
}

pub fn logical_operations() {
    let is_staking: bool = true;
    let has_sufficient_balance: bool = false;

    println!(
        "Can perform Staking: {}",
        is_staking && has_sufficient_balance
    );
    println!("Negating Staking Status: !is_staking = {}", !is_staking);
}

pub fn variable_shadowing_and_conversion() {
    let account_balance: i32 = 500;
    println!("Initial Account Balance: {}", account_balance);

    let account_balance = account_balance + 100;
    println!("Updated Account Balance: {}", account_balance);

    let gas_fee: f64 = 3.0025;
    let gas_fee_int: i32 = gas_fee as i32;

    println!(
        "Gas fee (f64): {}, Converted to lamports: {}",
        gas_fee, gas_fee_int
    );

    let block_height: i32 = 128550;
    let block_height_string: String = block_height.to_string();
    println!(
        "Block Height: {}, Converted to string: {}",
        block_height, block_height_string
    );
}

pub fn mutability_example() {
    let token_supply: i32 = 1_000_000;
    // token_supply = 2_000_000;
    let mut user_balance: i32 = 500;

    println!("Before transactions: User Balance = {}", user_balance);

    user_balance = token_supply;
    println!("Before transactions: User Balance = {}", user_balance);
}

pub fn tuple_destruction_example() {
    let transaction_info: (&str, i64, f64) = ("Transfer", 200, 0.002);

    let (_, tx_amount, tx_fee) = transaction_info;

    println!(
        "Transaction Type: {}, Amount: {}, Fee: {}",
        transaction_info.0, tx_amount, tx_fee
    );
}

pub fn demo() {
    primitive_data_types();

    println!("\n");
    logical_operations();

    println!("\n");
    variable_shadowing_and_conversion();

    println!("\n");
    mutability_example();

    println!("\n");
    tuple_destruction_example();
}
