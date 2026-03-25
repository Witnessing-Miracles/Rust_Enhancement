pub fn option_basics_example() {
    let user_balance: Option<u64> = Some(1000);
    let user_address = Some(
        "0xabc123..."
    );
    let no_value: Option<u64> = None;

    match user_balance {
        Some(balance) => println!("User has a balance of {} tokens.", balance),
        None => println!("No balance found for this account."),
    }

    match user_address {
        Some(address) => println!("Found blockchain address {}.", address),
        None => println!("No balance found for this account."),
    }
}

pub fn if_let_example() {
    let user_balance: Option<u64> = Some(1000);

    if let Some(balance) = user_balance {
        println!("Using `if let`: User has a balance of {} tokens.", balance);
    } else {
        println!("No balance found for this user.");
    }
}

pub fn unwrap_example() {
    let user_balance = Some(1000);  // 这里是 None 的时候，使用 unwrap 会触发 panic
    let balance = user_balance.unwrap();
    println!("Unwrapped balance: {}", balance);
}

pub fn unwrap_or_example() {
    let no_balance: Option<i32> = None;
    let balance = no_balance.unwrap_or(0);
    println!("Unwrapped balance: {}", balance);
}

pub fn map_example() {
    let some_fee = Some(1000);
    let result = some_fee.map(|fee| fee * 2);
    println!("Doubled transaction fee: {:#?}", result);
}

pub fn demo()
{
    option_basics_example();
    if_let_example();
    unwrap_example();
    unwrap_or_example();
    map_example();
}