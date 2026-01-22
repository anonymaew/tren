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

async fn chat(
    client: &Client<OpenAIConfig>,
    model: &String,
    sys_prompt: &String,
    user_prompt: &String,
) -> Result<String> {
    // simple case
    if user_prompt.trim().len() == 0 {
        return Ok(user_prompt.clone());
    }

    let mut attempts = 1u8;
    loop {
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(vec![
                ChatCompletionRequestSystemMessage::from(sys_prompt.clone()).into(),
                ChatCompletionRequestUserMessage::from(user_prompt.clone()).into(),
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
        let src_tok_count = user_prompt.chars().filter(|c| *c == TOK_SEP).count();
        let tar_tok_count = response.chars().filter(|c| *c == TOK_SEP).count();
        if src_tok_count != tar_tok_count {
            attempts += 1;
            continue;
        }

        println!(
            "--- Source ---
{}
--- Target {}---
{}
",
            user_prompt,
            if attempts > 1 {
                format!("(attempt {attempts})")
            } else {
                "".to_string()
            },
            response
        );

        return Ok(response);
    }
}

use crate::cli::Args;
use futures::{StreamExt, stream};

pub async fn task(src: Vec<String>, args: &Args) -> Result<Vec<String>> {
    let client = Client::new();
    let system_prompt = std::fs::read_to_string(std::path::PathBuf::from("./tren-sys-prompt.txt"))?;

    let mut processings = stream::iter(src)
        .enumerate()
        .map(|(i, mipc)| {
            let client = client.clone();
            let model = args.model.clone();
            let system_prompt = system_prompt.clone();
            async move { (i, chat(&client, &model, &system_prompt, &mipc).await) }
        })
        .buffer_unordered(16)
        .collect::<Vec<_>>()
        .await;

    processings.sort_by_key(|item| item.0);
    let results = processings
        .into_iter()
        .map(|item| item.1)
        .collect::<Result<Vec<_>>>()?;

    Ok(results)
}
