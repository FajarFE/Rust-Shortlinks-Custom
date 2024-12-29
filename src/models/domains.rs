use super::_entities::domains::Column;
use super::_entities::domains::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type Domains = Entity;

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    /// Fungsi untuk mencari domain berdasarkan nama
    pub async fn find_by_domain(
        domain: &str,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Domain.eq(domain))
            .one(db)
            .await
    }

    pub async fn check_max_domain_by_user(
        db: &DatabaseConnection,
        user_id: i32,
        max_domain: i32,
    ) -> Result<bool, DbErr> {
        // Menghitung jumlah domain yang dimiliki oleh pengguna
        let count = Entity::find()
            .filter(Column::UsersId.eq(user_id)) // Filter berdasarkan user_id
            .count(db) // Menghitung jumlah entitas
            .await?;

        // Memeriksa apakah jumlah domain yang dimiliki lebih dari max_domain
        Ok(count < max_domain as u64) // Memeriksa apakah jumlah domain yang dimiliki lebih dari max_domain
    }
}
