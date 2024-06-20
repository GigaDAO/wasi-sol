use leptos::*;
use solana_client_wasm::WasmClient as RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::{fmt, ops::Deref, rc::Rc, sync::Arc};

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
pub struct ConnectionContext(Rc<ConnectionContextState>);

impl Deref for ConnectionContext {
    type Target = Rc<ConnectionContextState>;

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
    let (endpoint, _set_endpoint) = create_signal(endpoint);
    let connection_state = create_memo(move |_| {
        Rc::new(ConnectionContextState {
            connection: RpcClient::new_with_commitment(
                &endpoint.get(),
                CommitmentConfig::confirmed(),
            )
            .into(),
        })
    });

    view! {
        <Provider<ConnectionContext> value={ConnectionContext(connection_state.get_untracked())}>
           {children()}
        </Provider<ConnectionContext>>
    }
}

pub fn use_connection() -> ConnectionContext {
    use_context::<ConnectionContext>().expect("No ConnectionContext found")
}
