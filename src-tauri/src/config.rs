//! 配置管理模块
//!
//! 该模块负责处理应用程序的配置，包括保存和读取用户的Git凭据。

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// 配置操作可能出现的错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// JSON 序列化/反序列化错误
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// 配置文件不存在错误
    #[error("Config file not found")]
    ConfigNotFound,
}

/// Git 凭据配置
#[derive(Debug, Serialize, Deserialize)]
pub struct GitCredentials {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
}

/// 应用程序配置
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub git_credentials: GitCredentials,
}

impl AppConfig {
    /// 获取配置文件路径
    pub fn get_config_path() -> Result<PathBuf, Error> {
        // Android 平台
        #[cfg(target_os = "android")]
        {
            use crate::wiki::Wiki;

            let path = Wiki::get_wiki_storage_dir()
                .map_err(|_| {
                    Error::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "无法获取存储目录",
                    ))
                })?
                .join("config.json");
            Ok(path)
        }

        // Linux/Windows 平台
        #[cfg(not(target_os = "android"))]
        {
            use crate::wiki::Wiki;

            let path = Wiki::get_wiki_storage_dir()
                .map_err(|_| {
                    Error::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "无法获取存储目录",
                    ))
                })?
                .join("config.json");
            Ok(path)
        }
    }

    /// 从文件加载配置
    pub fn load() -> Result<Self, Error> {
        let config_path = Self::get_config_path()?;
        if !config_path.exists() {
            return Err(Error::ConfigNotFound);
        }

        let config_content = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&config_content)?)
    }

    /// 将配置保存到文件
    pub fn save(&self) -> Result<(), Error> {
        let config_path = Self::get_config_path()?;
        let config_content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_content)?;
        Ok(())
    }

    /// 创建新的配置实例
    pub fn new(username: &str, email: &str, password: Option<&str>) -> Self {
        Self {
            git_credentials: GitCredentials {
                username: username.to_string(),
                email: email.to_string(),
                password: password.map(|p| p.to_string()),
            },
        }
    }
}
