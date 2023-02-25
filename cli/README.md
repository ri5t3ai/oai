# OpenAI Rust CLI

This is a Rust command-line interface (CLI) that uses the OpenAI API. It provides commands for generating text, listing available models, getting embeddings for text, and training custom models.

## Installation

To install the CLI, you'll need to have Rust installed. Then, run the following command:

```sh
cargo install --path .
```

This will build the CLI and install it to your system.

## Usage

To use the CLI, you'll need an OpenAI API key. You can set your API key as an environment variable like this:

```sh
export OPENAI_API_KEY=your_api_key
```

Alternatively, you can pass your API key as an argument to each command with the `--api-key` option.

To generate text:

```sh
oai generate --model davinci --prompt "Hello, my name is" --max-tokens 5
```

To list available models:

```sh
oai models
```

To get embeddings for text:

```sh
oai embeddings --model davinci --text "Hello" --text "World"
```

To train a custom model:

```sh
oai train --model davinci --training-data "This is the first example." --training-data "This is the second example." --validation-data "This is the validation example." --name "My training"
```

## Development

To run the tests:

```
cargo test
```

To run the CLI locally:

```
cargo run -- generate --model davinci --prompt "Hello, my name is" --max-tokens 5
```

## License

This CLI is licensed under the MIT license. See the `LICENSE` file for more details.