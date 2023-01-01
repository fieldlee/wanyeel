use axum::{middleware::from_extractor, Router};
use crate::controllers::init_need_admin_router;
use crate::middleware::auth_api;
//admin
pub fn routers() -> Router {
    need_auth_routers()
}
//需要权限认证的路由
pub fn need_auth_routers() -> Router {
    Router::new()
        .merge(init_need_admin_router())
        .layer(from_extractor::<auth_api::Auth>())
}
