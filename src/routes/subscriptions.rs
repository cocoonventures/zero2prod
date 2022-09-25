#[allow(unused)]
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use chrono::Utc;
use entities::*;
use sea_orm::ActiveValue::*;
use sea_orm::{ActiveModelTrait, ConnectOptions};
use sea_orm::{Database, DatabaseConnection};
use std::ops::Deref;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(
    form: web::Form<FormData>,
    db_pool: web::Data<ConnectOptions>,
) -> impl Responder {
    let opts = db_pool.get_ref().clone();
    let db = Database::connect(opts)
        .await
        .expect("Problem getting db connections from pool opts.");
    let user = user::ActiveModel {
        name: Set(form.name.to_owned()),
        email: Set(form.email.to_owned()),
        ..Default::default()
    };
    let user = user
        .insert(&db)
        .await
        .expect("Problem inserting user into db");

    let subscription = subscription::ActiveModel {
        user_id: Set(user.id),
        subscribed_at: Set(Utc::now().with_timezone(&FixedOffset::east(0))),
        ..Default::default()
    };
    match subscription.insert(&db).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to save subscription, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
    // .expect("Problem inserting subscription for userinto db");
}
