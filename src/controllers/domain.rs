#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::domains::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub domain: Option<String>,
    pub status: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.domain = Set(self.domain.clone());
        item.status = Set(self.status.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let _current_user = crate::models::users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    format::json(
        Entity::find()
            .filter(crate::models::_entities::domains::Column::UsersId.eq(_current_user.id))
            .all(&ctx.db)
            .await?,
    )
}

#[debug_handler]
pub async fn add(
    respond_to: RespondTo,
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let _current_user = crate::models::users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if let Some(domain) = params.domain.clone() {
        let existing_domain = Entity::find_by_domain(&domain, &ctx.db).await?;
        let max_domain = 5;
        let has_max_domain =
            Entity::check_max_domain_by_user(&ctx.db, _current_user.id, max_domain).await?;
        if existing_domain.is_some() {
            return match respond_to {
                RespondTo::Html => format::html("Domain already exists"),
                RespondTo::Json => format::json("Domain already exists"),
                RespondTo::Xml => format::json("Domain already exists"),
                RespondTo::None => format::json("Domain already exists"),
                RespondTo::Other(_) => format::json("Domain already exists"),
            };
        } else if !has_max_domain {
            return match respond_to {
                RespondTo::Html => format::html("Max domain reached"),
                RespondTo::Json => format::json("Max domain reached"),
                RespondTo::Xml => format::json("Max domain reached"),
                RespondTo::None => format::json("Max domain reached"),
                RespondTo::Other(_) => format::json("Max domain reached"),
            };
        }
    }

    let mut item = ActiveModel {
        domain: Set(params.domain.clone()),
        status: Set(params.status.clone()),
        users_id: Set(_current_user.id),
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/domains/")
        .add("/", get(list))
        .add("/", post(add))
        .add(":id", get(get_one))
        .add(":id", delete(remove))
        .add(":id", put(update))
        .add(":id", patch(update))
}
