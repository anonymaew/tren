# Tren: Translation Engine

> **A lightweight machine-translation pipeline that works on rich-text documents using LLMs**

<!-- TODO: some picture examples -->

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [Build manually](#build-manually)
- [Usage](#usage)
  - [Custom prompts](#custom-prompts)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Features

- Translate documents while **keeping all the rich text format intact**.
- **High quality translation** from previous context.
- **Bring your own LLMs**: compatible with OpenAI, Anthropic, Ollama, etc.;
  Anything with [OpenAI API structure](https://github.com/openai/openai-openapi).

## Prerequisites

- [Pandoc](https://pandoc.org/): ≥ v3.0
- Your LLM provider

## Installation

Make sure you already have [Cargo](https://doc.rust-lang.org/stable/cargo/) installed.

```bash
cargo install --git https://git.napatsc.com/ns/tren
```

## Usage

First, make sure you set up your environment variables: be creating a `.env` file or export the variables in your shell:

```bash
OPENAI_API_KEY="sk-xxxxxx"
# for a custom server other than OpenAI, change this below.
OPENAI_API_BASE="https://api.openai.com/v1"
```

| Key | Required/Default | Description |
|---------|------------|---------|
| `OPENAI_API_KEY` | **Required** | Your OpenAI API key; usually starts with `sk-` |
| `OPENAI_API_BASE`| `https://api.openai.com/v1` | Your LLM server endpoint. For custom LLM server other than OpenAI, change this value to the server URL |

Then, call the program:

```bash
tren --src English \
    --tar Spanish \
    -i some-document.md
```

Here are available CLI arguments:

| Flag | Required/Default | Description |
|------|-----------|---------------------|
| `--src` | **Yes** | Source language (e.g., `English`). |
| `--tar` | **Yes** | Target language (e.g., `Spanish`). |
| `-i`, `--input` | **Yes** | Path to the file that contains the text to translate. |
| `--inter-sheet` | `<INPUT>-inter.csv` (generated if omitted) | Path to a CSV file where intermediate translation results are stored for inspection/editing. Default to the input filename as `csv` with `-inter` suffix added. |
| `-o`, `--output` | `<INPUT>-translated.<EXT>` (same extension as input) | Path for the final translated file. Default to the input filename with `-translated` suffix added. |
| `--model` | `openai/gpt-oss-20b` | Hugging‑Face repository name of the LLM to use. |
| `--system` | Built‑in system prompt ([see below](#custom-prompts)) | System‑level prompt that sets the LLM’s role. |
| `--user` | Built‑in user prompt ([see below](#custom-prompts)) | User‑level prompt that supplies the actual translation request. |
| `-j`, `--parallel` | `1` | Maximum number of concurrent requests sent to the LLM. For a number larger than 1, please make sure your server supports batch inference; SGLang and vLLM are supported.  Ollama and llama.cpp are not. |
| `-h`, `--help` | - | Show command help |

### Custom prompts

<details>
    <summary>System prompt template</summary>
    
Default system prompt:

```jinja
You are an expert translator. Please translate {{ source_language }} into {{ target_language }}. The user will submit sentences or paragraphs with some contexts; please only translate the intended text into {{ target_language }}.

- If there are symbols {{ special_tokens | join(", ") }}, keep the symbol intact on the result text in the correct position.
- Do not give any alternative translation or including any previous context, notes or discussion.
```

To create a custom prompt, here are available variables for composing another one:

- `source_language`: Source language value from CLI
- `target_language`: Target language value from CLI
- `special_tokens`: List of special characters used to mark position of the
  source text, so the position is not lost on the target text.

</details>

<details>
    <summary>User prompt template</summary>

Default user prompt:

```jinja
{%- set previous_chunks = previous_chunks[-8:] -%}
{%- if previous_chunks -%}
Given the previous context:

{{ previous_chunks | join("\n\n") }}

Only translate the following text:

{% endif -%}
{{ source_text }}
```

To create a custom prompt, here are available variables for composing another one:

- `previous_chunks`: A list of 32 chunked texts before the source text. For
  example: `previous_chunks[-8:]` will obtain 8 text chunks before the text.
- `source_text`: The source text to be translated.

</details>

## Testing

```bash
cargo test
```

Feel free to add more unit tests for the translation logic or for error handling.

## Contributing

Contributions are welcome! Please open an issue or pull request.

1. Fork the repo.
2. Create a feature branch (`git checkout -b feature/awesome`).
3. Run `cargo test` to ensure existing tests pass.
4. Commit and push.
5. Open a pull request.

## License

MIT

## Contact

- Maintainer: [Napat Srichan](https://napatsc.com)
- Repository: <https://git.napatsc.com/ns/tren>, [GitHub mirror](https://github.com/anonymaew/tren).
