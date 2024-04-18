use anyhow::Context;
use axum::serve;
use std::sync::Arc;
use tokio::signal;
use tracing::info;

use utils::{AppConfig, logger::Logger, CargoEnv};
use database::Database;
use crate::{
    services::Services,
    router::AppRouter
};

pub struct ApplicationServer;

impl ApplicationServer {
    pub async fn serve(config: Arc<AppConfig>) -> anyhow::Result<()> {
        let _guard = Logger::new(config.cargo_env);

        let app_port = match &config.cargo_env {
            CargoEnv::Development => {
                config.app_port_test
            }
            CargoEnv::Production => {
                config.app_port
            }            
        };

        let address = format!("{}:{}", config.app_host, app_port);
        let tcp_listener = tokio::net::TcpListener::bind(address)
            .await
            .context("🔴 Failed to bind TCP listener")?;

        let local_addr = tcp_listener
            .local_addr()
            .context("🔴 Failed to get local address")?;

        // 构建一个内置了多种"集合"对应的底层数据库操作的Database
        let db = Database::new(config.clone()).await?;
        let services = Services::new(db);
        let router = AppRouter::new(services);

        info!("🟢 server:{{project-name}} has launched on {local_addr} 🚀");
        
        serve(tcp_listener, router)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .context("🔴 Failed to start server")?;

        Ok(())
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("🔴 Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("🔴 Failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        tracing::warn!("❌ Signal received, starting graceful shutdown...");
    }
}
