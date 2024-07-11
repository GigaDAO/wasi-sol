use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[hook]
pub fn use_storage<T: Clone + for<'de> serde::Deserialize<'de> + serde::Serialize + 'static>(
    key: &str,
    initial_value: T,
) -> (UseStateHandle<T>, Callback<T>) {
    let stored_value: T = LocalStorage::get(key).unwrap_or(initial_value);
    let connected = use_state(|| stored_value.clone());

    let set_value = {
        let connected = connected.clone();
        let key = key.to_string();
        Callback::from(move |value: T| {
            connected.set(value.clone());
            LocalStorage::set(&key, &value).expect("failed to set");
        })
    };

    (connected, set_value)
}
