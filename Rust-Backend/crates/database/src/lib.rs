////////////////////////////////////////////////////////////////////////
//
// 1. 每个Domain(Entity)单独一个文件夹
// 2. 每个Domain由两部分组成:
//    - model: 定义Schema
//    - repository: 实际的数据库底层操作
//
//////////////////////////////////////////////////////////////////////



use std::sync::Arc;
use tracing::info;
use mongodb::{Client, Collection}; // 源码中集成了mongodb，因此数据是直接存储在这个程序中的(此处的是driver还是mongodb本身?)
use utils::{AppConfig, AppResult, CargoEnv};

pub mod user;          //TODO: 数据库操作层中定义了user这个Domain(Entity)
use user::model::User; // TODO: 

#[derive(Clone, Debug)]
pub struct Database {
    pub users: Collection<User>, // TODO: 构建一个内置多个"集合"的Database
}

impl Database {
    pub async fn new(config: Arc<AppConfig>) -> AppResult<Self> {
        let client = Client::with_uri_str(&config.mongo_uri).await?;

        // let db = client.database(&config.mongo_db);

        let db = match &config.cargo_env {
            CargoEnv::Development => {
                client.database(&config.mongo_db_test)
            }
            CargoEnv::Production => {
                client.database(&config.mongo_db)
            }
        };
        let users = db.collection("User"); // TODO: 创建一个User集合


        // info!("🧱 database connected.");


        info!("🧱 database({:#}) connected.", match &config.cargo_env {
            CargoEnv::Development => {
                &config.mongo_db_test
            }
            CargoEnv::Production => {
                &config.mongo_db
            }
        });


        Ok(Database { users }) // TODO: 构建一个内置多个"集合"的Database.
    }
}

// impl Database {
//     pub async fn new(config: Arc<AppConfig>) -> AppResult<Self> {
//         match Database::connect(&config.mongo_uri).await {
//             Ok(client) => {
//                 let db = client.database(&config.mongo_db);
//                 let users = db.collection("User");

//                 info!("🧱 database conne123cted.");
//                 Ok(Database { users })
//             },
//             Err(err) => Err(err.into()), // 将 MongoError 转换成 AppError
//         }
//     }

//     async fn connect(uri: &str) -> Result<Client, MongoError> {
//         // 这里使用 `await` 异步等待连接结果
//         Client::with_uri_str(uri).await // TODO: 为什么mongodb中Client::with_uri_str连接失败与成功，没有区别
//     }
// }
