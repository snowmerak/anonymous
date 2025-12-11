use anonymous::enumx;

fn main() {
    let value: enumx!(i32 | String) = 10.into();

    match value {
        enumx::Enum2::V1(v) => println!("Int: {}", v),
        enumx::Enum2::V2(s) => println!("String: {}", s),
    }

    let value2: enumx!(i32 | String) = enumx::Enum2::V2("hello".to_string());
    
    match value2 {
        enumx::Enum2::V1(v) => println!("Int: {}", v),
        enumx::Enum2::V2(s) => println!("String: {}", s),
    }
}
