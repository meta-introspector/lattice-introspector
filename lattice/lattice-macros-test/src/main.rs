use lattice_macros::LatticePointDerive; // Changed to derive macro import

#[derive(LatticePointDerive)] // Apply the derive macro
struct MyTestStruct {
    field1: u32,
    field2: String,
}

#[derive(LatticePointDerive)]
enum MyTestEnum {
    Variant1,
    Variant2(u32),
    Variant3 { name: String, value: bool },
}

#[derive(LatticePointDerive)]
struct MyUnitTestStruct;

fn main() {
    let _my_struct = MyTestStruct {
        field1: 42,
        field2: "hello".to_string(),
    };
    println!("Macro applied to MyTestStruct. Check build output for generated code details.");
}
