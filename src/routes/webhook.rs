use diesel::r2d2::{Pool, ConnectionManager};
use diesel::SqliteConnection;
use crate::models::{WebhookEvent, BotResponse};
use crate::llm;
use crate::db;

pub async fn handle_webhook(
    event: WebhookEvent,
    pool: Pool<ConnectionManager<SqliteConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("📥 Received user input: {:?}", event.user_input);

    if event.user_input.to_lowercase().contains("talk to agent") {
        return Ok(warp::reply::json(&BotResponse {
            response: "Connecting you to a human. One sec 👩‍💻📞".to_string(),
        }));
    }

    let genz_prompt = format!(
        "You're a chill Gen-Z customer support bot. Respond in a funny, friendly, casual tone to this user message: \"{}\". Keep it short, witty, and helpful.",
        event.user_input
    );

    let reply = match llm::ask_together_ai(&genz_prompt).await {
        Ok(resp) => resp,
        Err(_) => "Oops 😅 I’m bugging rn. Try again in a bit?".to_string(),
    };

    // Log to DB with correct arguments
    let _ = db::insert_log(&pool, &event.user_input, &reply);

    Ok(warp::reply::json(&BotResponse {
        response: reply,
    }))
}
