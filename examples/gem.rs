use std::path::Path;

use futures::stream::StreamExt;
use gem_rs::api::Models;
use gem_rs::client::GemSession;
use gem_rs::init_log;
use gem_rs::types::{Blob, Context, FileManager, HarmBlockThreshold, Role, Settings};

#[tokio::main]
async fn main() {
    init_log();
    test().await;
}

//TODO: Something with the API cause the cached files in cloud to change uri every time they are deleted
async fn test_clear_files() {
    let mut file_manager = FileManager::new();
    file_manager.fetch_list().await.unwrap();
    file_manager.clear_files().await;
}

// IMPORTANT:
// When 'timeout' is set, 'read_timeout' is ignored according to the reqwest docs.
// Use for non-streaming requests. otherwise, the stream will be closed after the timeout
// even if the server is still responding.

async fn test_stream() {
    let mut session = GemSession::Builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .read_timeout(std::time::Duration::from_secs(30))
        .model(Models::Gemini25FlashPreview0417)
        .context(Context::new())
        .build();

    let mut settings = Settings::new();
    settings.set_all_safety_settings(HarmBlockThreshold::BlockNone);
    settings.set_stream_max_json_size(16384);

    // While using a thinking model,
    // you may indicate to the user as "thinking" while
    // waiting for the first tokens, cause as far i know,
    // currently there's no way to get the thinking tokens
    // in the gemini APIs (if possible, PR!).
    println!("Thinking...");

    let stream_result = session
        .send_message_stream("Hello! tell me a long story", Role::User, &settings)
        .await;

    match stream_result {
        Ok(mut stream) => {
            while let Some(response) = stream.next().await {
                match response {
                    Ok(response) => {
                        println!(
                            "{}",
                            response.get_results().get(0).unwrap_or(&"".to_string())
                        );
                    }
                    Err(e) => {
                        println!("Error 1: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error 2: {:?}", e);
        }
    }
}

async fn test() {
    let mut session = GemSession::Builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(Some(std::time::Duration::from_secs(30)))
        .context(Context::new())
        .model(Models::Gemini25ProExp0325)
        .build();

    let mut settings = Settings::new();
    settings.set_all_safety_settings(HarmBlockThreshold::BlockNone);

    // Set the thinking budget to 4000
    // This isn't necessary as the model will decide whether to think or not
    settings.set_thinking_budget(4000);

    let response = session
        .send_message("Hello! What is your name?", Role::User, &settings)
        .await;

    match response {
        Ok(response) => {
            println!("Response: {:#?}", response.get_results());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

async fn test_blob() {
    let mut session = GemSession::Builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(Some(std::time::Duration::from_secs(30)))
        .model(Models::Gemini25ProPreview0325)
        .context(Context::new())
        .build();

    let mut settings = Settings::new();
    settings.set_all_safety_settings(HarmBlockThreshold::BlockNone);
    settings.set_system_instruction("Explain any file that I send to you, in a (UwU) style");
    settings.set_max_output_tokens(8192);
    settings.set_temperature(1.5);

    let blob = Blob::new(
        "image/png",
        include_bytes!("/Users/hades/Downloads/test.png"),
    );

    let response = session.send_blob(blob, Role::User, &settings).await;

    match response {
        Ok(response) => {
            println!("Response: {:#?}", response.get_results());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

// IMPORTANT:
// When 'timeout' is set, 'read_timeout' is ignored according to the reqwest docs.
// Use for non-streaming requests. otherwise, the stream will be closed after the timeout
// even if the server is still responding.

async fn test_blob_stream() {
    let mut session = GemSession::Builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .read_timeout(std::time::Duration::from_secs(30))
        .model(Models::Gemini25FlashPreview0417)
        .context(Context::new())
        .build();

    let mut settings = Settings::new();
    settings.set_all_safety_settings(HarmBlockThreshold::BlockNone);
    settings.set_system_instruction("Explain any file that I send to you, in a (UwU) style");
    settings.set_max_output_tokens(8192);
    settings.set_temperature(1.5);

    let blob = Blob::new(
        "image/png",
        include_bytes!("/Users/hades/Downloads/test.png"),
    );

    let stream_result = session.send_blob_stream(blob, Role::User, &settings).await;

    match stream_result {
        Ok(mut stream) => {
            while let Some(response) = stream.next().await {
                match response {
                    Ok(response) => {
                        println!(
                            "{}",
                            response.get_results().get(0).unwrap_or(&"".to_string())
                        );
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

async fn test_file() {
    let mut session = GemSession::Builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(Some(std::time::Duration::from_secs(30)))
        .model(Models::Gemini25FlashPreview0417)
        .context(Context::new())
        .build();

    let mut settings = Settings::new();
    settings.set_all_safety_settings(HarmBlockThreshold::BlockNone);
    settings.set_system_instruction("Summraize the PDFs that I send to you, in a (UwU) style");
    settings.set_max_output_tokens(8192);
    settings.set_temperature(1.5);

    let mut file_manager = FileManager::new();
    file_manager.fetch_list().await.unwrap();
    let data = file_manager
        .add_file(Path::new("C:/Users/0xhades/Downloads/9.pdf"))
        .await
        .unwrap();

    let response = session.send_file(data, Role::User, &settings).await;

    match response {
        Ok(response) => {
            println!("Response: {:#?}", response.get_results());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

// IMPORTANT:
// When 'timeout' is set, 'read_timeout' is ignored according to the reqwest docs.
// Use for non-streaming requests. otherwise, the stream will be closed after the timeout
// even if the server is still responding.

async fn test_file_stream() {
    let mut session = GemSession::Builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .read_timeout(std::time::Duration::from_secs(30))
        .model(Models::Gemini25FlashPreview0417)
        .context(Context::new())
        .build();

    let mut settings = Settings::new();
    settings.set_all_safety_settings(HarmBlockThreshold::BlockNone);
    settings.set_system_instruction("Summraize the PDFs that I send to you, in a (UwU) style");
    settings.set_max_output_tokens(8192);
    settings.set_temperature(1.5);

    let mut file_manager = FileManager::new();
    file_manager.fetch_list().await.unwrap();
    let data = file_manager
        .add_file(Path::new("C:/Users/0xhades/Downloads/9.pdf"))
        .await
        .unwrap();

    let stream_result = session.send_file_stream(data, Role::User, &settings).await;

    match stream_result {
        Ok(mut stream) => {
            while let Some(response) = stream.next().await {
                match response {
                    Ok(response) => {
                        println!(
                            "{}",
                            response.get_results().get(0).unwrap_or(&"".to_string())
                        );
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}