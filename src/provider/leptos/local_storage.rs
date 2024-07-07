use gloo_storage::{LocalStorage, Storage};
use leptos::*;

pub fn use_local_storage(key: String, initial_value: String) -> (String, Callback<String>) {
    LocalStorage::set(&key, &initial_value).ok();
    let stored_value: String = LocalStorage::get(key.clone()).unwrap_or(initial_value.clone());

    let (stored_value, set_stored_value) = create_signal(stored_value);

    let set_value = {
        let key = key.clone();
        Callback::from(move |value: String| {
            set_stored_value.set(value.clone());
            LocalStorage::set(&key, &value).ok();
        })
    };

    (stored_value.get_untracked(), set_value)
}
