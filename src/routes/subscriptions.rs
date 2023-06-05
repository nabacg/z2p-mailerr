use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<SubscriptionForm>,
    connection: web::Data<PgPool>,
) -> HttpResponse {
    log::info!(
        "Adding '{}' '{}' as a new subscriber",
        form.email,
        form.name
    );
    log::info!("Saving new subscriber details in DB");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscriber saved in DB.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute INSERT INTO subscriptions query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
