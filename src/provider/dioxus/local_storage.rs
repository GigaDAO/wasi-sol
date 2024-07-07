use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

pub fn use_local_storage(key: String, initial_value: String) -> (String, Signal<String>) {
    LocalStorage::set(&key, &initial_value)
        .ok()
        .expect("Set LocalStorage");
    let stored_value = LocalStorage::get(&key).unwrap_or(initial_value.clone());
    let state = use_signal(|| stored_value.clone());

    use_effect(move || {
        LocalStorage::set(&key, &*state())
            .ok()
            .expect("Set LocalStorage");
    });

    (state.to_string(), state)
}
