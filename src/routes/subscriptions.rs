use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding new subscriber",
    skip(form, connection),
    fields(
        subscriber_email = %form.email,
        subscriber_name  = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<SubscriptionForm>,
    connection: web::Data<PgPool>,
) -> HttpResponse {
    // takes care of entering the query_span during sqlx query future execution
    match insert_subscriber(connection.get_ref(), &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument("Saving new subscriber details in DB", skip(pool, form))]
async fn insert_subscriber(pool: &PgPool, form: &SubscriptionForm) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute INSERT INTO subscriptions query: {:?}", e);
        e
    })?;

    tracing::info!("New subscriber saved in DB.");
    Ok(())
}
