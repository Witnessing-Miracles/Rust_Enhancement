enum TransactionType {
    Transfer,
    Mint,
    Burn,
    Stake,
}

enum ContractEvent {
    ContractDeployed,
    ContractTerminated,
    TokenTransfer {
        from: String,
        to: String,
        amount: u64,
    },
    OracleUpdate {
        price: f64,
    },
}

enum TransactionStatus {
    Pending,
    Confirmed(u32),
    Failed(String),
}

impl TransactionStatus {
    fn display_status(&self) {
        match self {
            TransactionStatus::Pending => println!("Transaction is currently pending."),
            TransactionStatus::Confirmed(block) => {
                println!("Transaction confirmed in block: {}", block)
            }
            TransactionStatus::Failed(error) => println!("Transaction failed, due to: {}", error),
        }
    }
}

enum ContractLifeCycl {
    Initialization,
    Active { participants: u32 },
    Paused,
    Terminated,
}

impl ContractLifeCycl {
    fn display_status(&self) {
        match self {
            ContractLifeCycl::Initialization => println!("Smart Contract is being initialized."),
            ContractLifeCycl::Active { participants } => {
                println!("Contract is active with {} participants.", participants)
            }
            ContractLifeCycl::Paused => println!("Contract is currently paused."),
            ContractLifeCycl::Terminated => println!("Contract is currently Terminated."),
        }
    }
}

pub fn demo() {
    let tx_type = TransactionType::Transfer;
    match tx_type {
        TransactionType::Transfer => println!("Processing a token transfer."),
        TransactionType::Mint => println!("Minting new tokens."),
        TransactionType::Burn => println!("Burning tokens from supply."),
        TransactionType::Stake => println!("Staking tokens for reward."),
        //_ => println!(""),
    }

    let transfer_event = ContractEvent::TokenTransfer {
        from: String::from("0x24680..."),
        to: String::from("0x13579..."),
        amount: 1000,
    };

    let oracle_event = ContractEvent::OracleUpdate { price: 111.11 };

    match oracle_event {
        ContractEvent::ContractDeployed => println!("Smart Contract Deployed."),
        ContractEvent::ContractTerminated => println!("Contract terminated."),
        ContractEvent::TokenTransfer { from, to, amount } => {
            println!(
                "
        Transfer of {} tokens from {} to {}.",
                amount, from, to
            )
        }
        ContractEvent::OracleUpdate { price } => println!("Oracle updated price to: ${}.", price),

        _ => println!("Unhandled Events."),
    }

    let tx1 = TransactionStatus::Pending;
    let tx2 = TransactionStatus::Confirmed(11111 as u32);
    let tx3 = TransactionStatus::Failed("Invalid Signature.".to_string());

    tx1.display_status();
    tx2.display_status();
    tx3.display_status();

    let contract = ContractLifeCycl::Active { participants: 100 };
    let paused_contract = ContractLifeCycl::Paused;
    let terminated_contract = ContractLifeCycl::Terminated;

    println!("\n----------------------------------------------\n");
    contract.display_status();
    paused_contract.display_status();
    terminated_contract.display_status();
}
