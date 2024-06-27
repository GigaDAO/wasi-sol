# 📚 WASI SOL YEW Component Example

## 🛠️ Pre-requisites:

1. Install [`rustup`](https://www.rust-lang.org/tools/install):

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Install [`trunk`](https://trunkrs.dev/):

    ```bash
    cargo install --locked trunk
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
	cd wasi-sol/examples/yew
	```

1. Run the client:

	```sh
	trunk serve --port 3000
	```

Navigate to http://localhost:3000 to explore the landing page.
