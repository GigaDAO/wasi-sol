use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[hook]
pub fn use_local_storage(key: String, initial_value: String) -> (String, Callback<String>) {
    LocalStorage::set(&key, &initial_value).ok();
    let stored_value: String = LocalStorage::get(key.clone()).unwrap_or(initial_value.clone());

    let stored_value = use_state(|| stored_value);
    let data = (*stored_value).clone();

    let set_value = {
        let key = key.clone();
        Callback::from(move |value: String| {
            stored_value.set(value.clone());
            LocalStorage::set(&key, &value).ok();
        })
    };

    (data.clone(), set_value)
}
