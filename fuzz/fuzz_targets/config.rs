#![no_main]
use libfuzzer_sys::fuzz_target;
use leptos_config::get_config_from_str;

fuzz_target!(|value: &str| {
    let _ = get_config_from_str(value);
});