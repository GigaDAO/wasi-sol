# 📚 WASI SOL Dioxus Component Example

## 🛠️ Pre-requisites:

1. Install [`rustup`](https://www.rust-lang.org/tools/install):

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.5/getting_started):

    ```bash
    cargo install dioxus-cli
    ```

1. Add Wasm target:

    ```bash
    rustup target add wasm32-unknown-unknown
    ```

## 🚀 Building and Running

1. Fork/Clone the GitHub repository.

	```bash
	git clone https://github.com/gigadao/wasi-sol
	```

1. Navigate to the application directory.

	```bash
	cd wasi-sol/examples/dioxus
	```

1. Run the client:

	```sh
	dx serve --port 3000
	```

Navigate to http://localhost:3000 to explore the landing page.
