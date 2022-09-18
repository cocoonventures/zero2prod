//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::subscription::Entity")]
    Subscription,
}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         panic!("No RelationDef")
//     }
// }

impl ActiveModelBehavior for ActiveModel {}