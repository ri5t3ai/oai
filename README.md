

# OpenAI Rust API

![build main](https://github.com/ri5t3ai/oai/actions/workflows/ci.yml/badge.svg?branch=main) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) <img src="https://img.shields.io/badge/Contributions-welcome-blueviolet" />         <img src="https://img.shields.io/badge/Maintained%3F-yes-green.svg" /></a>

The OpenAI Rust API client follows a trait-based approach to building a client, which provides several advantages. The client defines a `OpenAI` struct that contains a client field, which is a `reqwest::Client` instance for handling HTTP requests. Methods in the OpenAI struct are defined using traits, which allows the client to provide different implementations for different types of requests.

For example, the `generate` method uses the `DeserializeOwned` trait to deserialize the response into the specified type. The use of traits provides a flexible and extensible design that makes it easy to add new functionality and integrate it with existing code. Additionally, this approach can make the code more modular and easier to maintain, as different components can be separated into their own traits and implemented independently. Overall, the trait-based approach used by the OpenAI Rust API client helps to provide a flexible and maintainable solution for interfacing with the OpenAI API.

##  Usage

To use the OpenAI API, you'll first need to sign up for an API key on the OpenAI website. Once you have an API key, you can create a new instance of the OpenAI struct, passing your API key as a parameter to the new function:

```rust
use openai::OpenAI;

let api_key = "YOUR_API_KEY_HERE";
let openai = OpenAI::new(api_key);

```
### Generating Text

You can generate text using the generate function, which takes three parameters: the name of the model you want to use, the prompt (i.e., the starting text for the generated text), and the maximum number of tokens to generate. Here's an example:

```rust
let prompt = "Once upon a time";
let max_tokens = 100;
let response: GenerateResponse = openai.generate("davinci", prompt, max_tokens).await.unwrap();

```
### Listing Models

You can list the models available to your account using the list_models function:

```rust
let models_response: ModelsResponse = openai.list_models().await.unwrap();

```

### Embeddings

You can get embeddings for one or more texts using the get_embeddings function:

```rust
let texts = &["Hello, world!", "How are you?"];
let embeddings_response: EmbeddingsResponse = openai.get_embeddings("davinci", texts).await.unwrap();

```

### Training

You can create a new training run using the create_training function:

```rust
let training_data = &["Example training data"];
let validation_data = &["Example validation data"];
let name = "My training run";
let model_id = None;
let options = None;

let training_response: TrainingResponse = openai.create_training("davinci", training_data, validation_data, name, model_id, options).await.unwrap();

```
You can get the status of a training run using the check_training_status function:

```rust
let training_id = "EXAMPLE_TRAINING_ID";
let status_response: TrainingStatusResponse = openai.check_training_status("davinci", training_id).await.unwrap();

```

You can get the logs for a training run using the get_training_logs function:

```rust
let training_id = "EXAMPLE_TRAINING_ID";
let logs_response: String = openai.get_training_logs("davinci", training_id).await.unwrap();

```
## Contributing

Thank you for considering contributing to this project! To ensure that the project remains maintainable and high-quality, please follow these guidelines when contributing:

### Issue tracking

Before starting work on a new feature or bug fix, please create an issue in the issue tracker describing the problem or feature request. This helps to ensure that the work is necessary and that it aligns with the goals of the project. Please include a clear and concise description of the issue or feature request, as well as any relevant code or error messages.

### Pull requests

When submitting a pull request, please ensure that your code follows the established coding conventions and that it is well-documented. In addition, please include test cases to ensure that the code is correct and robust.

Please also ensure that your code has been reviewed by at least one other developer before submitting a pull request. This helps to ensure that the code is of high quality and that it meets the needs of the project.
### Code conventions

When writing code for this project, please follow these conventions:

- Use four spaces for indentation
- Use camelCase for variable and function names
- Use snake_case for file names and module names
- Use Rustfmt to automatically format code

### Documentation

Please ensure that any new features or changes are well-documented in the README and other relevant documentation files. This helps to ensure that the project is easy to understand and use for other developers.

### Code of conduct

Please follow the project's code of conduct when contributing. This includes being respectful to other contributors and avoiding any discriminatory or abusive behavior. If you encounter any issues or concerns, please report them to the project maintainers.

## License

This crate is licensed under the MIT license. See the `LICENSE` file for more details.
