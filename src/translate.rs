use crate::chunk::{TOK_SEP, TaskType, Tasks};
use crate::cli::Job;
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
use minijinja::render;

async fn chat(client: &Client<OpenAIConfig>, job: Job, payload: String) -> Result<String> {
    // simple case
    if payload.trim().len() == 0 {
        return Ok(payload.clone());
    }

    let mut attempts = 1u8;
    loop {
        let request = CreateChatCompletionRequestArgs::default()
            .model(job.llm.model.clone())
            .messages(vec![
                ChatCompletionRequestSystemMessage::from(job.system.clone()).into(),
                ChatCompletionRequestUserMessage::from(job.user.clone()).into(),
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
        let src_tok_count = payload.chars().filter(|c| *c == TOK_SEP).count();
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
                format!("{} tokens", tokens.prompt_tokens)
            } else {
                "".to_string()
            },
            payload,
            {
                let mut res: Vec<String> = vec![];
                if let Some(ref tokens) = usage {
                    res.push(format!("{} tokens", tokens.completion_tokens));
                }
                if attempts > 1 {
                    res.push(format!("{attempts} attempts"));
                }
                res
            }
            .join(", "),
            answer
        );

        return Ok(answer);
    }
}

use crate::chunk::{AST, pandoc_ast::PandocAST};

pub async fn process_job(job: &Job) -> Result<()> {
    let mut ast = PandocAST::default();
    ast.import(&job.input)?;

    let micps = ast.to_mipcs();

    let client = Client::<OpenAIConfig>::with_config(
        OpenAIConfig::default().with_api_base(job.llm.url.clone()),
    );

    let special_tokens = vec!["𐑣"];

    let task_stream = async |src: Vec<String>, task_type: TaskType| -> Result<Vec<String>> {
        let parallel = job.parallel;
        let mut processings = stream::iter(src.clone())
            .enumerate()
            .map(|(i, mipc)| {
                let client = client.clone();
                let back_chunks = match task_type {
                    TaskType::Main => 32,
                    TaskType::Side => 0,
                };
                let previous_chunks = &src[i.saturating_sub(back_chunks)..i];
                let mut new_args = job.clone();
                new_args.system = render!(&job.system,
                    source_language => job.src,
                    target_language => job.tar,
                    special_tokens => special_tokens);
                new_args.user = render!(&job.user,
                    previous_chunks => previous_chunks,
                    source_text => mipc);
                async move { (i, chat(&client, new_args.clone(), mipc).await) }
            })
            .buffer_unordered(parallel)
            .collect::<Vec<_>>()
            .await;
        processings.sort_by_key(|item| item.0);
        processings
            .into_iter()
            .map(|item| item.1)
            .collect::<Result<Vec<_>>>()
    };

    let result = Tasks {
        main: task_stream.clone()(micps.main.into(), TaskType::Main)
            .await?
            .into(),
        sides: task_stream.clone()(micps.sides.into(), TaskType::Side)
            .await?
            .into(),
    };

    ast.apply_mipcs(result)?;

    ast.export(&job.output)?;

    Ok(())
}
