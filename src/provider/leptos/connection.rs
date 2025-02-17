use leptos::{context::Provider, prelude::*};
use solana_client_wasm::WasmClient as RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::{fmt, ops::Deref, sync::Arc};

#[derive(Clone)]
pub struct ConnectionContextState {
    pub connection: Arc<RpcClient>,
}

impl PartialEq for ConnectionContextState {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.connection, &other.connection)
    }
}

impl fmt::Debug for ConnectionContextState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectionContextState")
            .field("commitment", &self.connection.commitment())
            .finish()
    }
}

#[derive(Clone, PartialEq)]
pub struct ConnectionContext(Arc<ConnectionContextState>);

impl Deref for ConnectionContext {
    type Target = Arc<ConnectionContextState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for ConnectionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectionContext")
            .field("commitment", &self.connection.commitment())
            .finish()
    }
}

#[component]
pub fn ConnectionProvider(children: Children, endpoint: &'static str) -> impl IntoView {
    let (endpoint, _set_endpoint) = signal(endpoint);
    let connection_state = Memo::new(move |_| {
        Arc::new(ConnectionContextState {
            connection: RpcClient::new_with_commitment(
                &endpoint.get(),
                CommitmentConfig::confirmed(),
            )
            .into(),
        })
    });

    view! {
        <Provider value={ConnectionContext(connection_state.get_untracked())}>
           {children()}
        </Provider>
    }
}

pub fn use_connection() -> ConnectionContext {
    use_context::<ConnectionContext>().expect("No ConnectionContext found")
}
