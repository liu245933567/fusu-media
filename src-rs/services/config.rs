use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::dto::AppConfig;

#[derive(Debug, Clone)]
pub struct GlobalData {
    
    pub config: AppConfig,
}

impl Default for GlobalData {
    fn default() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }
}

static GLOBAL_DATA: Lazy<RwLock<GlobalData>> = Lazy::new(|| RwLock::new(GlobalData::default()));

/// 初始化全局数据
pub async fn init_global_data() -> anyhow::Result<()> {
    let config_path = "./data/config.json";

    if !tokio::fs::metadata(config_path).await.is_ok() {
        println!("config file not found, writing default config");
        write_app_config_to_file(AppConfig::default()).await?;
    } else {
        let json_str = tokio::fs::read_to_string(config_path).await?;
        let config: AppConfig = serde_json::from_str(&json_str)?;
        GLOBAL_DATA.write().await.config = config;
    }
    Ok(())
}

/// 将配置写入本地文件
pub async fn write_app_config_to_file(config: AppConfig) -> anyhow::Result<()> {
    let config_path = "./data/config.json";
    // 先确保目录存在，否则 Windows 下会报“系统找不到指定的路径”
    if let Some(parent) = std::path::Path::new(config_path).parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let json = serde_json::to_string_pretty(&config)?;
    println!("writing config to file: {}", json);
    tokio::fs::write(config_path, json).await?;
    Ok(())
}

/// 获取当前应用配置（内存快照）
pub async fn get_app_config() -> AppConfig {
    GLOBAL_DATA.read().await.config.clone()
}

/// 覆盖应用配置（仅内存，不落盘）
pub async fn set_app_config(cfg: AppConfig) -> anyhow::Result<()> {
    write_app_config_to_file(cfg.clone()).await?;
    GLOBAL_DATA.write().await.config = cfg.clone();
    Ok(())
}
