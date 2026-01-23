use crate::chunk::TOK_SEP;
use crate::cli::Args;
use anyhow::{Result, anyhow};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
    },
};
use futures::{StreamExt, stream};

async fn chat(client: &Client<OpenAIConfig>, args: &Args, user_prompt: &String) -> Result<String> {
    // simple case
    if user_prompt.trim().len() == 0 {
        return Ok(user_prompt.clone());
    }

    let mut attempts = 1u8;
    loop {
        let request = CreateChatCompletionRequestArgs::default()
            .model(args.model.clone())
            .messages(vec![
                ChatCompletionRequestSystemMessage::from(args.system.clone()).into(),
                ChatCompletionRequestUserMessage::from(user_prompt.clone()).into(),
            ])
            .n(1)
            .build()?;

        let response = client.chat().create(request).await?;

        let answer = response
            .choices
            .first()
            .ok_or(anyhow!("no choices?"))?
            .message
            .content
            .clone()
            .ok_or(anyhow!("content error"))?;

        // special token check
        let src_tok_count = user_prompt.chars().filter(|c| *c == TOK_SEP).count();
        let tar_tok_count = answer.chars().filter(|c| *c == TOK_SEP).count();
        if src_tok_count != tar_tok_count {
            attempts += 1;
            continue;
        }

        let usage = response.usage;

        println!(
            "--- Source {}---
{}
--- Target {}---
{}
",
            if let Some(ref tokens) = usage {
                format!("({} tokens) ", tokens.prompt_tokens)
            } else {
                "".to_string()
            },
            user_prompt,
            vec![
                if let Some(ref tokens) = usage {
                    format!("{} tokens", tokens.completion_tokens)
                } else {
                    "".to_string()
                },
                if attempts > 1 {
                    format!("{attempts} attempts")
                } else {
                    "".to_string()
                },
            ]
            .join(", "),
            answer
        );

        return Ok(answer);
    }
}

pub async fn task(src: Vec<String>, args: &Args) -> Result<Vec<String>> {
    let client = Client::<OpenAIConfig>::with_config(
        OpenAIConfig::default().with_api_base(
            std::env::var_os("OPENAI_API_BASE")
                .unwrap_or("https://api.openai.com/v1".into())
                .into_string()
                .unwrap(),
        ),
    );

    let mut processings = stream::iter(src)
        .enumerate()
        .map(|(i, mipc)| {
            let client = client.clone();
            async move { (i, chat(&client, &args, &mipc).await) }
        })
        .buffer_unordered(args.parallel)
        .collect::<Vec<_>>()
        .await;

    processings.sort_by_key(|item| item.0);
    let results = processings
        .into_iter()
        .map(|item| item.1)
        .collect::<Result<Vec<_>>>()?;

    Ok(results)
}
