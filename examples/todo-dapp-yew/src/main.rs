mod checkbox;
mod landing;
mod local_storage;
mod task;
mod tasks;

use crate::landing::Landing;
use crate::task::Task;
use crate::task::TaskComponent;
use crate::tasks::TaskForm;
use yew::prelude::*;

use crate::local_storage::use_storage;
use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::Wallet,
    forms::yew::login::LoginForm,
    provider::yew::{
        connection::{use_connection, ConnectionProvider},
        wallet::{use_wallet, WalletProvider},
    },
    pubkey::Pubkey,
    spawn_local, system_instruction,
    transaction::Transaction,
};

use std::str::FromStr;

#[function_component]
pub fn App() -> Html {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        Wallet::Phantom.into(),
        Wallet::Solflare.into(),
        Wallet::Backpack.into(),
    ];

    html! {
        <ConnectionProvider {endpoint}>
            <WalletProvider {wallets}>
                <TasksApp />
            </WalletProvider>
        </ConnectionProvider>
    }
}

#[function_component]
fn TasksApp() -> Html {
    let (tasks, set_tasks) = use_storage::<Vec<Task>>("tasks", vec![]);
    let (connected, set_connected) = use_storage::<bool>("connected", false);

    let number_complete = tasks.iter().filter(|task| task.done).count();
    let number_total = tasks.len();

    let add_task = {
        let set_tasks = set_tasks.clone();
        let tasks = (*tasks).clone();
        Callback::from(move |name: String| {
            let mut new_tasks = tasks.clone();
            new_tasks.push(Task { name, done: false });
            set_tasks.emit(new_tasks.to_vec());
        })
    };

    let remove_task = {
        let set_tasks = set_tasks.clone();
        let tasks = (*tasks).clone();
        Callback::from(move |index: usize| {
            let mut new_tasks = tasks.clone();
            new_tasks.remove(index);
            set_tasks.emit(new_tasks.to_vec());
        })
    };

    let update_task_done = {
        let set_tasks = set_tasks.clone();
        let tasks = (*tasks).clone();
        Callback::from(move |(index, done): (usize, bool)| {
            let mut new_tasks = tasks.clone();
            if let Some(task) = new_tasks.get_mut(index) {
                task.done = done;
            }
            set_tasks.emit(new_tasks.to_vec());
        })
    };

    let rename_task = {
        let set_tasks = set_tasks.clone();
        let tasks = (*tasks).clone();
        Callback::from(move |(index, name): (usize, String)| {
            let mut new_tasks = tasks.clone();
            if let Some(task) = new_tasks.get_mut(index) {
                task.name = name;
            }
            set_tasks.emit(new_tasks.to_vec());
        })
    };

    use_effect_with(*connected, move |connected| {
        set_connected.emit(*connected);
    });

    html! {
        <main>
            <h1>{ "Yew Wasi Sol Todo Dapp" }</h1>
            <Landing connected={connected.clone()} />
            if *connected {
                <h2>{ format!("{}/{} Complete", number_complete, number_total) }</h2>
                <TaskForm on_add={add_task} />
                { for tasks.iter().enumerate().map(|(index, task)| html! {
                    <TaskComponent
                        task={task.clone()}
                        on_rename={rename_task.reform(move |name| (index, name))}
                        on_trash={remove_task.reform(move |_| index)}
                        on_toggle={update_task_done.reform(move |done| (index, done))}
                    />
                }) }
            }
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
