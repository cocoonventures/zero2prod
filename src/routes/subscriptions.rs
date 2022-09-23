#[allow(unused)]
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use chrono::Utc;
use entities::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::*;
use sea_orm::DatabaseConnection;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(
    _form: web::Form<FormData>,
    _db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let user = user::ActiveModel {
        name: Set(_form.name.to_owned()),
        email: Set(_form.email.to_owned()),
        ..Default::default()
    };
    let user = user
        .insert(_db.get_ref())
        .await
        .expect("Problem inserting user into db");

    let subscription = subscription::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(user.id),
        subscribed_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        ..Default::default()
    };
    let subscription = subscription
        .insert(_db.get_ref())
        .await
        .expect("Problem inserting subscription for userinto db");

    HttpResponse::Ok().finish()
}
