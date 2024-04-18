////////////////////////////////////////////////////////////////////////
//
// 1. 每个Domain(Entity)单独一个文件夹
// 2. 每个Domain由两部分组成:
//    - model: 定义Schema
//    - repository: 实际的数据库底层操作
//
//////////////////////////////////////////////////////////////////////

pub(crate) mod user_service; // TODO

use std::sync::Arc;
use tracing::info;

use crate::services::user_service::{DynUserService, UserService};
use database::Database; // TODO

#[derive(Clone)]
pub struct Services {
    pub user: DynUserService,
}

impl Services {
    pub fn new(db: Database) -> Self {
        let repository = Arc::new(db);

        let user = Arc::new(UserService::new(repository.clone())) as DynUserService;

        info!("🧠 initializing services...");

        Self { user }
    }
}
