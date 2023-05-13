#![no_main]
use libfuzzer_sys::fuzz_target;
use leptos_router::ParamsMap;
use arbitrary::Arbitrary;

#[derive(Debug, Arbitrary)]
enum Operation {
    Insert(String, String),
    Get(String),
    Remove(String)
}

fuzz_target!(|input: Vec<Operation>| {
    let mut map = ParamsMap::new();
    for op in input {
        match op {
            Operation::Insert(key, val) => { map.insert(key, val); },
            Operation::Get(key) => { map.get(&key); },
            Operation::Remove(key) => { map.remove(&key); },
        }
    }
});