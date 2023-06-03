use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{PgConnection, PgPool};
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute INSERT INTO subscriptions query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
