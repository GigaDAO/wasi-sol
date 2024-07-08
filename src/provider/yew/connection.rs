use std::{fmt, ops::Deref, rc::Rc, sync::Arc};

use yew::prelude::*;

use solana_client_wasm::WasmClient as RpcClient;

use solana_sdk::commitment_config::CommitmentConfig;

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

#[function_component]
pub fn ConnectionProvider(props: &Props) -> Html {
    let connection_state = use_memo(props.endpoint.clone(), |endpoint| ConnectionContextState {
        connection: RpcClient::new_with_commitment(
            &endpoint.clone(),
            CommitmentConfig::confirmed(),
        )
        .into(),
    });

    html! {
        <ContextProvider<ConnectionContext> context={ConnectionContext(connection_state)}>
            { props.children.clone() }
        </ContextProvider<ConnectionContext>>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    pub endpoint: String,
}

#[hook]
pub fn use_connection() -> ConnectionContext {
    use_context::<ConnectionContext>().expect("No ConnectionContext found")
}
