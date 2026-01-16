# Tren: Translation Engine

> **A lightweight machine-translation pipeline that works on rich-text documents using LLMs**

<!-- TODO: some picture examples -->

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [Build manually](#build-manually)
- [Configuration](#configuration)
- [Usage](#usage)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## Features

- Translate documents while **keeping all the rich text format intact**.
- **High quality translation** from previous context.
- **Bring your own LLMs**: compatible with OpenAI, Anthropic, Ollama, etc.;
  Anything with [OpenAI API structure](https://github.com/openai/openai-openapi).

## Prerequisites

- [Pandoc](https://pandoc.org/): ≥ v3.0
- Your LLM provider

## Installation

<!-- TODO: when published, add ways to install -->

### Build manually

```bash
# Clone the repository
git clone https://git.napatsc.com/ns/tren.git
cd tren

# Build in release mode
cargo build --release
```

## Configuration

| Setting | How to set | Require/Default |
|---------|------------|---------|
| `OPENAI_API_KEY` | Environment variable | Required |
| `OPENAI_API_BASE` | Environment variable | `https://api.openai.com/v1` |

Create a `.env` file or export the variables in your shell:

```bash
export OPENAI_API_KEY="sk-xxxxxx"
export OPENAI_API_BASE="https://api.openai.com/v1"
```

<!-- TODO: ways to set system prompt -->

## Usage

<!-- TODO: when CLI completed, add this -->

## Testing

```bash
cargo test
```

Feel free to add more unit tests for the translation logic or for error handling.

---

## Contributing

Contributions are welcome! Please open an issue or pull request.

1. Fork the repo.
2. Create a feature branch (`git checkout -b feature/awesome`).
3. Run `cargo test` to ensure existing tests pass.
4. Commit and push.
5. Open a pull request.

---

## License

MIT - see the [LICENSE](LICENSE) file.

---

## Contact

- Maintainer: [Napat Srichan](https://napatsc.com)
- Repository: <https://git.napatsc.com/ns/tren>, [GitHub mirror](https://github.com/anonymaew/tren).
