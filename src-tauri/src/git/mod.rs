//! Git 仓库操作模块
//!
//! 此模块封装了 Git 仓库的基本操作。
//! 它基于 `git2` 库提供的功能，为上层应用提供更简洁的 Git 操作接口。

use std::path::Path;
use thiserror::Error;

/// Git 操作可能出现的错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// Git 仓库打开失败
    #[error("Failed to open repository: {0}")]
    OpenRepository(#[source] git2::Error),

    /// Git 仓库初始化失败
    #[error("Failed to initialize repository: {0}")]
    InitRepository(#[source] git2::Error),

    /// Git 仓库克隆失败
    #[error("Failed to clone repository: {0}")]
    CloneRepository(#[source] git2::Error),

    /// 获取远程仓库信息失败
    #[error("Failed to get remotes: {0}")]
    GetRemotes(#[source] git2::Error),
}

/// Git 仓库的封装结构体
///
/// 提供对 Git 仓库的各种操作，隐藏底层 `git2::Repository` 的复杂性。
pub struct Repository {
    /// 内部使用的 git2 仓库对象
    repo: git2::Repository,
}

impl Repository {
    /// 打开一个已存在的 Git 仓库
    ///
    /// # 参数
    /// * `path` - Git 仓库的路径
    ///
    /// # 返回值
    /// * `Result<Self, Error>` - 成功时返回 `Ok(Repository)`，包含打开的仓库对象
    /// * 失败时返回具体的错误信息
    pub fn open(path: &Path) -> Result<Self, Error> {
        let repo = git2::Repository::open(path).map_err(Error::OpenRepository)?;
        Ok(Self { repo })
    }

    /// 在指定路径初始化一个新的 Git 仓库
    ///
    /// # 参数
    /// * `path` - 要初始化仓库的路径
    ///
    /// # 返回值
    /// * `Result<Self, Error>` - 成功时返回 `Ok(Repository)`，包含初始化的仓库对象
    /// * 失败时返回具体的错误信息
    pub fn init(path: &Path) -> Result<Self, Error> {
        let repo = git2::Repository::init(path).map_err(Error::InitRepository)?;
        Ok(Self { repo })
    }

    /// 从远程 URL 克隆 Git 仓库到指定路径
    ///
    /// # 参数
    /// * `url` - 远程仓库的 URL
    /// * `path` - 本地克隆的目标路径
    ///
    /// # 返回值
    /// * `Result<Self, Error>` - 成功时返回 `Ok(Repository)`，包含克隆的仓库对象
    /// * 失败时返回具体的错误信息
    pub fn clone(url: &str, path: &Path) -> Result<Self, Error> {
        #[cfg(not(target_os = "android"))]
        {
            let repo = git2::Repository::clone(url, path).map_err(Error::CloneRepository)?;
            Ok(Self { repo })
        }

        #[cfg(target_os = "android")]
        {
            // 使用带有SSL证书验证配置的方法
            let mut callbacks = git2::RemoteCallbacks::new();

            // 配置证书验证回调函数，在Android平台上忽略证书验证错误
            callbacks.certificate_check(|_cert, _host| {
                // 该回调函数以及 CertificateOk 表示证书验证直接通过
                Ok(git2::CertificateCheckStatus::CertificateOk)
            });

            // 创建远程配置
            let mut fetch_opts = git2::FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);

            // 创建构建器并设置fetch选项
            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options(fetch_opts);

            let repo = builder.clone(url, path).map_err(Error::CloneRepository)?;
            Ok(Self { repo })
        }
    }

    /// 检查仓库是否配置了远程仓库
    ///
    /// # 返回值
    /// * `Result<bool, Error>` - 成功时返回 `Ok(bool)`，`true` 表示配置了远程仓库，`false` 表示未配置
    /// * 失败时返回具体的错误信息
    pub fn has_remote_repo(&self) -> Result<bool, Error> {
        let remotes = self.repo.remotes().map_err(Error::GetRemotes)?;
        Ok(!remotes.is_empty())
    }
}
