# OpenAI API Rust Client
This is a Rust library for working with the OpenAI API. It provides methods for generating text, listing available models, getting embeddings for text, and training custom models.

## Usage

To use the library, first create an instance of the `OpenAI` struct, passing in your API key:

```rust
use openai::OpenAI;
let api_key = "my_api_key";
let openai = OpenAI::new(api_key.to_string());
```

You can then call methods on the `OpenAI` instance to interact with the OpenAI API. For example, to generate text:

```rust
let model = "davinci";
let prompt = "Hello, my name is";
let max_tokens = 5;
let response: CompletionResponse = openai.generate(model, prompt, max_tokens).await.unwrap();
println!("{:?}", response);
```

To list available models:

```rust
let response: ModelsResponse = openai.list_models().await.unwrap();
println!("{:?}", response);
```

To get embeddings for text:

```rust
let model = "davinci";
let texts = ["Hello", "World"];
let response: EmbeddingsResponse = openai.get_embeddings(model, &texts).await.unwrap();
println!("{:?}", response);
```

To train a custom model:

```rust
let model = "davinci";
let training_data = ["This is the first example.", "This is the second example."];
let validation_data = ["This is the validation example."];
let name = "My training";
let model_id = None;
let options = None;
let response: TrainingResponse = openai.create_training(model, &training_data, &validation_data, name, model_id, options).await.unwrap();
println!("{:?}", response);
```

## Development

To run the tests:

```
cargo test
```

## License

This library is licensed under the MIT license. See the `LICENSE` file for more details.