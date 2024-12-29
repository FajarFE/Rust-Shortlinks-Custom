#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, extract::Query};
use loco_rs::prelude::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::models::_entities::shortlinks::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub shortlink: Option<String>,
    pub url: Option<String>,
    pub clicks: Option<i32>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.shortlink = Set(self.shortlink.clone());
        item.url = Set(self.url.clone());
        item.clicks = Set(self.clicks.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn generate_shortlink() -> String {
    let mut rng = rand::thread_rng();
    let shortlink: String = (0..6)
        .map(|_| *CHARSET.choose(&mut rng).unwrap() as char)
        .collect();
    shortlink
}

#[derive(Deserialize)]
pub struct ListParams {
    limit: Option<i32>,
    offset: Option<i32>,
    order_by: Option<String>,
    search_query: Option<String>,
    filter_by_clicks: Option<i32>,
    users_id: Option<i32>,
}

#[debug_handler]

pub async fn list(query: Query<ListParams>, State(ctx): State<AppContext>) -> Result<Response> {
    // Set default values if parameters are not provided
    let limit = query.limit.unwrap_or(10); // Default to 10
    let offset = query.offset.unwrap_or(0); // Default to 0
    let order_by = query.order_by.as_deref();
    let search_query = query.search_query.as_deref();
    let filter_by_clicks = query.filter_by_clicks;
    let users_id = query.users_id;

    // Call the function that retrieves shortlinks
    match Entity::get_all_shortlinks(
        &ctx.db,
        limit,
        offset,
        order_by,
        search_query,
        filter_by_clicks,
        users_id,
    )
    .await
    {
        Ok(Some(shortlinks)) => format::json(shortlinks),
        Ok(None) => format::json(()),
        Err(err) => {
            tracing::error!("Error retrieving shortlinks: {}", err);
            format::json(())
        }
    }
}

#[debug_handler]
pub async fn add(
    respond_to: RespondTo,
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let _current_user = crate::models::users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if params.url.is_none() {
        return match respond_to {
            RespondTo::Html => format::html("URL is required"),
            RespondTo::Json => format::json("URL is required"),
            RespondTo::Xml => format::json("URL is required"),
            RespondTo::None => format::json("URL is required"),
            RespondTo::Other(_) => format::json("URL is required"),
        };
    }

    let mut item = ActiveModel {
        url: Set(params.url.clone()),
        clicks: Set(params.clicks.clone()),
        users_id: Set(_current_user.id),
        ..Default::default()
    };

    let shortlink = generate_shortlink();
    if params.shortlink.is_some() {
        let existing_shortlink = Entity::find_by_shortlink(&shortlink, &ctx.db).await?;
        if existing_shortlink.is_some() {
            return match respond_to {
                RespondTo::Html => format::html("Shortlink already exists"),
                RespondTo::Json => format::json("Shortlink already exists"),
                RespondTo::Xml => format::json("Shortlink already exists"),
                RespondTo::None => format::json("Shortlink already exists"),
                RespondTo::Other(_) => format::json("Shortlink already exists"),
            };
        } else {
            item.shortlink = Set(Some(shortlink));
        }
        item.shortlink = Set(params.shortlink.clone());
    } else {
        return match respond_to {
            RespondTo::Html => format::html("Shortlink is required"),
            RespondTo::Json => format::json("Shortlink is required"),
            RespondTo::Xml => format::json("Shortlink is required"),
            RespondTo::None => format::json("Shortlink is required"),
            RespondTo::Other(_) => format::json("Shortlink is required"),
        };
    }

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
        .prefix("api/shortlinks/")
        .add("/", get(list))
        .add("/", post(add))
        .add(":id", get(get_one))
        .add(":id", delete(remove))
        .add(":id", put(update))
        .add(":id", patch(update))
}
