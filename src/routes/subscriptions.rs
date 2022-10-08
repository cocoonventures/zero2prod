#[allow(unused)]
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use chrono::Utc;
use entities::*;
use sea_orm::ActiveValue::*;
use sea_orm::{ActiveModelTrait, ConnectOptions};
use sea_orm::{Database, DatabaseConnection};
use tracing::*; // Instrument; // use log::{error, info};
use uuid::Uuid;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<ConnectOptions>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = info_span!(
        "Adding new subscriber",
        %request_id, subscriber_name = %form.name, subscriber_email = %form.email
    );
    let _request_span_guard = request_span.enter();

    let opts = db_pool.get_ref().clone();
    let db = Database::connect(opts).await.expect(
        format!(
            "[r-uuid:{}] Problem getting db connections from pool options.",
            request_id
        )
        .as_str(),
    );
    let user = user::ActiveModel {
        name: Set(form.name.to_owned()),
        email: Set(form.email.to_owned()),
        ..Default::default()
    };
    let user_span = tracing::info_span!(
        "Adding user to database",
        %request_id, subscriber_name = %form.name, subscriber_email = %form.email
    );
    let user = user
        .insert(&db)
        .instrument(user_span)
        .await
        .expect(format!("[r-uuid:{}] Problem inserting user into db", request_id).as_str());

    let sub_span = tracing::info_span!("Adding user's associated subscription", %request_id);
    let subscription = subscription::ActiveModel {
        user_id: Set(user.id),
        subscribed_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        ..Default::default()
    };
    match subscription.insert(&db).instrument(sub_span).await {
        Ok(_) => {
            tracing::info!(
                "[r-uuid:{}] User subscription saved successfully",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "[r-uuid:{}] Failed to save subscription, {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
