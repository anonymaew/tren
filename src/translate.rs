use crate::chunk::TOK_SEP;
use anyhow::{Result, anyhow};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
    },
};

pub async fn task(src: Vec<String>) -> Result<Vec<String>> {
    let client = Client::default();
    let system_prompt = std::fs::read_to_string(std::path::PathBuf::from("./tren-sys-prompt.txt"))?;

    let mut results = Vec::with_capacity(src.len());
    for mipc in src {
        // simple case
        if mipc.trim().len() == 0 {
            results.push(mipc.clone());
            continue;
        }

        let mut attempts = 1u8;
        loop {
            let request = CreateChatCompletionRequestArgs::default()
                .model("gpt-5-nano")
                .messages(vec![
                    ChatCompletionRequestSystemMessage::from(system_prompt.clone()).into(),
                    ChatCompletionRequestUserMessage::from(mipc.clone()).into(),
                ])
                .n(1)
                .build()?;

            let response = client
                .chat()
                .create(request)
                .await?
                .choices
                .first()
                .ok_or(anyhow!("no choices?"))?
                .message
                .content
                .clone()
                .ok_or(anyhow!("content error"))?;

            // special token check
            let src_tok_count = mipc.chars().filter(|c| *c == TOK_SEP).count();
            let tar_tok_count = response.chars().filter(|c| *c == TOK_SEP).count();
            if src_tok_count != tar_tok_count {
                attempts += 1;
                continue;
            }

            println!(
                "--- Source {}---
{}
--- Target ---
{}
",
                if attempts > 1 {
                    format!("(attempt {attempts})")
                } else {
                    "".to_string()
                },
                mipc,
                response
            );

            // some check login

            results.push(response);

            break;
        }
    }

    Ok(results)
}
