mod module_01_data_types_examples;
mod module_02_control_flows_examples;
mod module_03_functions_closure_examples;
mod module_04_data_structures_examples;
mod module_05_ownership_examples;
mod module_06_struct_examples;
mod module_07_enum_examples;
mod module_08_option_example;
mod module_09_result_example;
mod module_10_option_result_combination;
mod module_11_trait_example;
mod module_12_concurrency_example;

#[tokio::main]
async fn main() {
    // module_01_data_types_examples::demo();
    // module_02_control_flows_examples::demo();
    // module_03_functions_closure_examples::demo();
    // module_04_data_structures_examples::demo();
    // module_06_struct_examples::demo();
    // module_07_enum_examples::demo();
    // module_09_result_example::demo();
    // module_10_option_result_combination::demo();
    // module_11_trait_example::demo();
    module_12_concurrency_example::demo().await;

    println!("\n\n\n");
}
