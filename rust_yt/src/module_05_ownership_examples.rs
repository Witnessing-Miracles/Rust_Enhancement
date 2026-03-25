// 所有权引入的一个概念是：既然这里存在你需要的值，就没必要再分配内存复制或者创建一个新的类似对象，减少不必要的
// 内存分配和操作，提高效率 降低复杂度 避免内存loss

pub fn ownership_example() {
    let token_owner: String = String::from("Alice");
    let new_owner: String = token_owner; // 在其他编程语言里面（比如 Js）new_owner 是 token_owner 的一个复制
    let new_owner = String::from("Alice"); // 这里和上面那一行不一样，如果上面的内容存储在 slot5, 这里或许是 slot6

    println!("\nNew token owner: {}", new_owner);

    // borrow of moved value: `token_owner`, 所有权已经 move 出去了。解决方法可以用 clone;
    // 也可以用 &token_owner 但是会改变 new_owner 类型为 &String，接下来那一句就不能再用，不然类型不匹配报错
    // println!("\nOld token owner: {}", token_owner);
}

pub fn cloning_example() {
    let transaction_id = String::from("tx123");
    let transaction_id_copy = transaction_id.clone();
    println!("Original Transaction ID: {}", transaction_id);
    println!("Cloned Transaction ID: {}", transaction_id_copy);
}

pub fn copy_trait_example() {
    let token_amount = 100/*.to_string() String 没有实现 copy trait，而 copy trait 有管理所有权的方式*/;
    let token_copy = token_amount;

    println!(
        "\nOriginal Token amount: {}, Copied token amount: {}",
        token_amount, token_copy
    );
}

pub fn borrowing_example() {
    let contract_state = String::from("Contract Active");
    print_borrow(&contract_state);
    let new_contract = &contract_state;
    println!("\nAfter borrowing, contract state: {}", contract_state);
}

pub fn print_borrow(contract: &String) {
    println!("\nBorrowed Contract State: {}", contract);
}

pub fn mutable_borrowing_example() {
    let mut contract_state_mutable = String::from("Contract Pending");
    modify_state(&mut contract_state_mutable);
    println!(
        "\nAfter modifying, Contract State: {}",
        contract_state_mutable
    );
}

pub fn modify_state(contract: &mut String) {
    contract.push_str(" and now active!");
}

pub fn dangling_reference_example() {
    let invalid_refer = invalid_borrow();
    println!("{}", invalid_refer);
}

fn invalid_borrow() -> String /*&String*/ {
    // 这里千万不能返回 String 引用，这是一种悬垂引用。
    // 返回的东西没有所有权，返回的时候应该把所有权移交给需要赋值的对象。
    // 不返回带所有权的变量，该变量在函数执行完后内存会被回收，如果返回引用把它赋值给别的变量会出错
    let s = String::from("I will be employed.");
    //&s
    s
}

pub fn demo() {
    // ownership_example();
    // cloning_example();
    // copy_trait_example();
    // borrowing_example();
    // mutable_borrowing_example();
    dangling_reference_example();
}
