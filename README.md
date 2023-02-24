# OpenAI Rust Client

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is a Rust crate for working with the OpenAI API. It provides a library for generating text, listing available models, getting embeddings for text, and training custom models, as well as a command-line interface (CLI) for interacting with the API.

## Library Usage

To use the library, add the following to your `Cargo.toml` file:

```toml

[dependencies]
openai = "0.1.0"

```

You can then use the `OpenAI` struct to interact with the OpenAI API. For example, to generate text:

```rust
use openai::OpenAI;

let api_key = "my_api_key";
let openai = OpenAI::new(api_key.to_string());
let model = "davinci";
let prompt = "Hello, my name is";
let max_tokens = 5;
let response: CompletionResponse = openai.generate(model, prompt, max_tokens).await.unwrap();
println!("{:?}", response);

```

See the documentation in the source code for more information about available methods.

## CLI Usage

To use the CLI, you'll need an OpenAI API key. You can set your API key as an environment variable like this:

```sh
export OPENAI_API_KEY=your_api_key
```

Alternatively, you can pass your API key as an argument to each command with the `--api-key` option.

To install the CLI, you'll need to have Rust installed. Then, run the following command:

```sh
cargo install --path .
```

This will build the CLI and install it to your system.

To generate text:

```sh
oai generate --model davinci --prompt "Hello, my name is" --max-tokens 5
```

To list available models:

```
oai models
```

See the documentation in the source code for more information about available commands.

## Development

To run the tests:
```sh
cargo test
```

To run the CLI locally:
```sh
cargo run -- generate --model davinci --prompt "Hello, my name is" --max-tokens 5
```

## License

This crate is licensed under the MIT license. See the `LICENSE` file for more details.