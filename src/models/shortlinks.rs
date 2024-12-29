use super::_entities::shortlinks::{ActiveModel, Column, Entity, Model};

use sea_orm::entity::prelude::*;
use sea_orm::{Order, QueryOrder, QuerySelect};

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl Entity {
    pub async fn find_by_shortlink(
        shortlink: &str,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Shortlink.eq(shortlink))
            .one(db)
            .await
    }

    pub async fn get_all_shortlinks(
        db: &DatabaseConnection,
        limit: i32,
        offset: i32,
        order_by: Option<&str>,
        search_query: Option<&str>,
        filter_by_clicks: Option<i32>,
        users_id: Option<i32>,
    ) -> Result<Option<Model>, DbErr> {
        let mut query = Entity::find();
        if let Some(users_id) = users_id {
            query = query.inner_join(crate::models::_entities::users::Entity);
            query = query.filter(crate::models::_entities::users::Column::Id.eq(users_id));
        }

        query = query.limit(limit as u64);

        query = query.offset(offset as u64);

        if let Some(_order_by) = order_by {
            query = query.order_by(Column::Shortlink, Order::Asc);
        }

        if let Some(search_query) = search_query {
            query = query.filter(Column::Shortlink.contains(search_query.to_lowercase()));
        }

        if let Some(filter_by_clicks) = filter_by_clicks {
            query = query.filter(Column::UsersId.eq(users_id));
            query = query.filter(Column::Clicks.eq(filter_by_clicks));
        }

        query.one(db).await
    }
}
