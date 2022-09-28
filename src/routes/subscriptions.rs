#[allow(unused)]
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use chrono::Utc;
use entities::*;
// use log::{error, info};
use sea_orm::ActiveValue::*;
use sea_orm::{ActiveModelTrait, ConnectOptions};
use sea_orm::{Database, DatabaseConnection};
use std::ops::Deref;
use tracing::*;
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
    info!(
        "[r-uuid:{}] Adding user[\"{}\" ({})] as a new subscriber",
        request_id, form.name, form.email
    );
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
    info!(
        "[r-uuid:{}] Adding user[\"{}\" {}]",
        request_id, form.name, form.email
    );
    let user = user
        .insert(&db)
        .await
        .expect(format!("[r-uuid:{}] Problem inserting user into db", request_id).as_str());

    info!("[r-uuid:{}] Adding user's subscription", request_id);
    let subscription = subscription::ActiveModel {
        user_id: Set(user.id),
        subscribed_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        ..Default::default()
    };
    match subscription.insert(&db).await {
        Ok(_) => {
            info!(
                "[r-uuid:{}] User subscription saved successfully",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!(
                "[r-uuid:{}] Failed to save subscription, {:?}",
                request_id, e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
