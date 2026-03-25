#[derive(Debug)]

struct BlockchainUser {
    username: String,
    public_key: String,
    balance: u64,
    active: bool,
}

// 为结构体添加 trait
impl BlockchainUser {
    fn new_user(username: &str, public_key: &str, balance: u64) -> Self {
        Self {
            // Self 表示该结构体本身
            username: String::from(username),
            public_key: public_key.to_string(),
            balance,
            active: true,
        }
    }

    fn display_info(&self) {
        println!(
            "Name: {}, Public Key: {}, Balance: {}, Active: {}",
            self.username, self.public_key, self.balance, self.active
        )
    }

    fn update_public_key(&mut self, public_key: &str) {
        self.public_key = public_key.to_string();
    }

    fn deactivation(&mut self) {
        self.active = false;
    }

    fn add_balance(&mut self, amount: u64) {
        self.balance += amount;
        println!("Added {} tokens to user: {}", amount, self.username);
    }
}

#[derive(Debug)]
struct Transaction(u64, String);

struct Block {
    index: u32,
    miner: BlockchainUser,
    transaction_count: u32,
}

pub fn demo() {
    let mut user1 = BlockchainUser {
        username: "dave".to_string(),
        public_key: String::from("0x1233..."),
        balance: 100,
        active: true,
    };

    // println!("Blockchain User: {:#?}", user1);

    let mut user2 = BlockchainUser::new_user("Toni", "0x23688...", 200);
    user2.display_info();
    user2.update_public_key("0x666888...");
    user2.display_info();
    user2.deactivation();
    user2.display_info();

    let tx1 = Transaction(500, String::from("0x.111666..."));
    let tx2 = Transaction(600, String::from("0x.222888..."));

    println!("Transaction 1: {:#?}", tx1);
    println!("Transaction 2: {} tokens sent to {}", tx2.0, tx2.1);

    let minedBlock = Block {
        index: 1,
        miner: user1,
        transaction_count: 2,
    };

    println!(
        "Block index: {}, Miner: {}, Transactions: {}, Active: {}",
        minedBlock.index,
        minedBlock.miner.username,
        minedBlock.transaction_count,
        minedBlock.miner.active
    )
}
