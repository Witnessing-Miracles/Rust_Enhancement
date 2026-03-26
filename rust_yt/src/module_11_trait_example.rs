pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Account {
    pub username: String,
    pub balance: u64,
}

impl Describable for Account {
    fn describe(&self) -> String {
        format!(
            "Account: {}, Balance: {}",
            self.username, self.balance
        )
    }
}

pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub sender: String,
    pub receiver: String,
}

impl Describable for Transaction {
    fn describe(&self) -> String {
        format!("Transaction ID: {}, Amount: {}, Sender: {}, Receiver: {}",
        self.id, self.amount, self.sender, self.receiver)
    }
}

pub fn print_description<T: Describable>(trait_item: &T) {
    println!("Description: {}", trait_item.describe());
}

pub trait summarizable {
    fn summary(&self) -> String {
        String::from("Further details hidden ...")  // default
    }
}

impl summarizable for Account {
    fn summary(&self) -> String {
        format!(
            "User: {}, Balance: {} tokens",
            self.username, self.balance
        )
    }
}

impl summarizable for Transaction {}

pub fn multiple_trait_example<T: Describable + summarizable>(trait_item: &T) {
    println!("Full Description: {}", trait_item.describe());
    println!("Summary: {}", trait_item.summary());
}

pub fn impl_trait_example(trait_item: impl Describable) {
    println!("Implement Trait Description: {}", trait_item.describe());
}

pub fn demo()
{
    let account = Account {
        username: String::from("Alice"),
        balance: 5000,
    };

    let transaction = Transaction {
        id: String::from("tx123456..."),
        amount: 150,
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
    };

    //print_description(&account);
    //print_description(&transaction);

    //println!("Account Summary: {}", account.summary());
    //println!("Transaction Summary: {}", transaction.summary());

    multiple_trait_example(&account);
    multiple_trait_example(&transaction);
     println!("\n------------------------------------------------------------\n");

    impl_trait_example(account);
}