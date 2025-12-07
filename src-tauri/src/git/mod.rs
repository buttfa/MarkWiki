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

    /// 提交失败
    #[error("Failed to commit: {0}")]
    Commit(#[source] git2::Error),

    /// 获取状态失败
    #[error("Failed to get status: {0}")]
    Status(#[source] git2::Error),

    /// 获取配置失败
    #[error("Failed to get config: {0}")]
    Config(#[source] git2::Error),

    /// 设置配置失败
    #[error("Failed to set config: {0}")]
    SetConfig(#[source] git2::Error),

    /// 添加文件到暂存区失败
    #[error("Failed to add to index: {0}")]
    AddToIndex(#[source] git2::Error),

    /// 创建提交签名失败
    #[error("Failed to create signature: {0}")]
    Signature(#[source] git2::Error),

    /// 查找引用失败
    #[error("Failed to find reference: {0}")]
    FindReference(#[source] git2::Error),

    /// 获取远程失败
    #[error("Failed to get remote: {0}")]
    Remote(#[source] git2::Error),

    /// Fetch 操作失败
    #[error("Failed to fetch: {0}")]
    Fetch(#[source] git2::Error),

    /// 合并失败
    #[error("Failed to merge: {0}")]
    Merge(#[source] git2::Error),

    /// 推送失败
    #[error("Failed to push: {0}")]
    Push(#[source] git2::Error),

    /// 有未提交的修改
    #[error("There are uncommitted changes")]
    UncommittedChanges,

    /// 没有配置远程仓库
    #[error("No remote repository configured")]
    NoRemote,

    /// 合并冲突（暂时不需要处理，但先定义）
    #[error("Merge conflict detected")]
    MergeConflict,

    /// 空提交
    #[error("Nothing to commit")]
    NothingToCommit,
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
    /// * `username` - Git 用户名（可选）
    /// * `email` - Git 邮箱（可选）
    ///
    /// # 返回值
    /// * `Result<Self, Error>` - 成功时返回 `Ok(Repository)`，包含初始化的仓库对象
    /// * 失败时返回具体的错误信息
    pub fn init(path: &Path, username: Option<&str>, email: Option<&str>) -> Result<Self, Error> {
        let repo = git2::Repository::init(path).map_err(Error::InitRepository)?;
        let mut repo_instance = Self { repo };

        // 设置用户名和邮箱配置
        if let Some(name) = username {
            repo_instance.set_config("user.name", name)?;
        }
        if let Some(email) = email {
            repo_instance.set_config("user.email", email)?;
        }

        Ok(repo_instance)
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
    pub fn clone(
        url: &str,
        path: &Path,
        username: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<Self, Error> {
        #[cfg(not(target_os = "android"))]
        {
            let mut callbacks = git2::RemoteCallbacks::new();

            // 设置身份验证回调
            if let Some(user) = username {
                callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                    if let Some(pass) = password {
                        git2::Cred::userpass_plaintext(user, pass)
                    } else {
                        git2::Cred::ssh_key_from_agent(user)
                    }
                });
            }

            // 创建远程配置
            let mut fetch_opts = git2::FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);

            // 创建构建器并设置fetch选项
            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options(fetch_opts);

            let repo = builder.clone(url, path).map_err(Error::CloneRepository)?;
            let mut repo_instance = Self { repo };

            // 设置用户名和邮箱配置
            if let Some(name) = username {
                repo_instance.set_config("user.name", name)?;
            }
            if let Some(email) = email {
                repo_instance.set_config("user.email", email)?;
            }

            Ok(repo_instance)
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

            // 设置身份验证回调
            if let Some(user) = username {
                callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                    if let Some(pass) = password {
                        git2::Cred::userpass_plaintext(user, pass)
                    } else {
                        git2::Cred::ssh_key_from_agent(user)
                    }
                });
            }

            // 创建远程配置
            let mut fetch_opts = git2::FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);

            // 创建构建器并设置fetch选项
            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options(fetch_opts);

            let repo = builder.clone(url, path).map_err(Error::CloneRepository)?;
            let mut repo_instance = Self { repo };

            // 设置用户名和邮箱配置
            if let Some(name) = username {
                repo_instance.set_config("user.name", name)?;
            }
            if let Some(email) = email {
                repo_instance.set_config("user.email", email)?;
            }

            Ok(repo_instance)
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

    /// 检查是否有未提交的修改
    pub fn has_uncommitted_changes(&self) -> Result<bool, Error> {
        let statuses = self.repo.statuses(None).map_err(Error::Status)?;
        Ok(!statuses.is_empty())
    }

    /// 获取用户配置（用户名和邮箱）
    pub fn get_user_config(&self) -> Result<(String, String), Error> {
        let config = self.repo.config().map_err(Error::Config)?;

        let name = config
            .get_string("user.name")
            .unwrap_or_else(|_| "MarkWiki User".to_string());

        let email = config
            .get_string("user.email")
            .unwrap_or_else(|_| "user@markwiki.app".to_string());

        Ok((name, email))
    }

    /// 设置用户配置（用户名和邮箱）
    pub fn set_user_config(&self, name: &str, email: &str) -> Result<(), Error> {
        let mut config = self.repo.config().map_err(Error::Config)?;

        config
            .set_str("user.name", name)
            .map_err(Error::SetConfig)?;
        config
            .set_str("user.email", email)
            .map_err(Error::SetConfig)?;

        Ok(())
    }

    /// 设置单个 Git 配置项
    ///
    /// # 参数
    /// * `key` - 配置项的键
    /// * `value` - 配置项的值
    ///
    /// # 返回值
    /// * `Result<(), Error>` - 成功时返回 `Ok(())`
    /// * 失败时返回具体的错误信息
    pub fn set_config(&self, key: &str, value: &str) -> Result<(), Error> {
        let mut config = self.repo.config().map_err(Error::Config)?;

        config.set_str(key, value).map_err(Error::SetConfig)?;

        Ok(())
    }

    /// 添加所有修改到暂存区
    pub fn add_all(&mut self) -> Result<(), Error> {
        let mut index = self.repo.index().map_err(Error::AddToIndex)?;
        index
            .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
            .map_err(Error::AddToIndex)?;
        index.write().map_err(Error::AddToIndex)?;
        Ok(())
    }

    /// 提交暂存区的修改
    pub fn commit(&self, message: &str) -> Result<(), Error> {
        // 检查是否有暂存的内容
        let mut index = self.repo.index().map_err(Error::AddToIndex)?;
        if index.is_empty() {
            return Err(Error::NothingToCommit);
        }

        // 获取用户配置
        let (name, email) = self.get_user_config()?;
        let signature = git2::Signature::now(&name, &email).map_err(Error::Signature)?;

        // 查找HEAD（可能是第一次提交）
        let tree_id = index.write_tree().map_err(Error::AddToIndex)?;
        let tree = self.repo.find_tree(tree_id).map_err(Error::AddToIndex)?;

        let parent_commit = if let Ok(head) = self.repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                Some(commit)
            } else {
                None
            }
        } else {
            None
        };

        // 创建提交
        match parent_commit {
            Some(parent) => {
                self.repo
                    .commit(
                        Some("HEAD"),
                        &signature,
                        &signature,
                        message,
                        &tree,
                        &[&parent],
                    )
                    .map_err(Error::Commit)?;
            }
            None => {
                self.repo
                    .commit(Some("HEAD"), &signature, &signature, message, &tree, &[])
                    .map_err(Error::Commit)?;
            }
        }

        Ok(())
    }

    /// 从远程仓库获取更新
    pub fn fetch(&self) -> Result<(), Error> {
        let remote_name = "origin";
        let mut remote = self
            .repo
            .find_remote(remote_name)
            .map_err(|_| Error::NoRemote)?;

        // 设置fetch选项
        let mut callbacks = git2::RemoteCallbacks::new();

        // 尝试加载保存的凭据
        if let Ok(config) = crate::config::AppConfig::load() {
            let username = config.git_credentials.username.clone();
            let password = config.git_credentials.password.clone();

            callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                git2::Cred::userpass_plaintext(&username, password.as_deref().unwrap_or(""))
            });
        }

        #[cfg(target_os = "android")]
        {
            // 在Android平台上忽略证书验证错误
            callbacks
                .certificate_check(|_cert, _host| Ok(git2::CertificateCheckStatus::CertificateOk));
        }

        let mut fetch_opts = git2::FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        // 执行fetch
        remote
            .fetch(&["master"], Some(&mut fetch_opts), None)
            .map_err(Error::Fetch)?;

        Ok(())
    }

    /// 合并远程分支到本地分支（无冲突版本）
    pub fn merge(&self) -> Result<bool, Error> {
        // 获取远程分支引用
        let fetch_head = self
            .repo
            .find_reference("FETCH_HEAD")
            .map_err(Error::FindReference)?;
        let fetch_commit = self
            .repo
            .reference_to_annotated_commit(&fetch_head)
            .map_err(Error::FindReference)?;

        // 分析合并
        let analysis = self
            .repo
            .merge_analysis(&[&fetch_commit])
            .map_err(Error::Merge)?;

        // 处理不同的合并情况
        if analysis.0.is_up_to_date() {
            // 已经是最新，无需合并
            return Ok(false);
        } else if analysis.0.is_fast_forward() {
            // 快速前进合并
            let mut head = self
                .repo
                .find_reference("refs/heads/master")
                .map_err(Error::FindReference)?;
            head.set_target(fetch_commit.id(), "Fast-forward")
                .map_err(Error::Merge)?;
            self.repo
                .set_head("refs/heads/master")
                .map_err(Error::Merge)?;
            self.repo
                .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                .map_err(Error::Merge)?;
            return Ok(true);
        } else if analysis.0.is_normal() {
            // 正常合并（无冲突情况）
            // 暂时不支持有冲突的合并
            return Err(Error::MergeConflict);
        }

        Ok(false)
    }

    /// 推送到远程仓库
    pub fn push(&self) -> Result<(), Error> {
        let remote_name = "origin";
        let mut remote = self
            .repo
            .find_remote(remote_name)
            .map_err(|_| Error::NoRemote)?;

        // 设置推送选项
        let mut callbacks = git2::RemoteCallbacks::new();

        // 尝试加载保存的凭据
        if let Ok(config) = crate::config::AppConfig::load() {
            let username = config.git_credentials.username.clone();
            let password = config.git_credentials.password.clone();

            callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                git2::Cred::userpass_plaintext(&username, password.as_deref().unwrap_or(""))
            });
        }

        #[cfg(target_os = "android")]
        {
            // 在Android平台上忽略证书验证错误
            callbacks
                .certificate_check(|_cert, _host| Ok(git2::CertificateCheckStatus::CertificateOk));
        }

        let mut push_opts = git2::PushOptions::new();
        push_opts.remote_callbacks(callbacks);

        // 执行推送
        remote
            .push(
                &["refs/heads/master:refs/heads/master"],
                Some(&mut push_opts),
            )
            .map_err(Error::Push)?;

        Ok(())
    }
    /// 检查是否有名为 "origin" 的远程仓库
    pub fn has_remote(&self) -> Result<bool, Error> {
        match self.repo.find_remote("origin") {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.code() == git2::ErrorCode::NotFound {
                    Ok(false)
                } else {
                    Err(Error::Remote(e)) // 转换为 Error 类型
                }
            }
        }
    }

    /// 完整的同步流程（无冲突情况下）
    pub fn sync(&self) -> Result<(), Error> {
        // 1. 检查是否有远程仓库
        if !self.has_remote()? {
            // 现在可以使用 ? 操作符
            return Err(Error::NoRemote);
        }

        // 2. 获取远程更新
        self.fetch()?;

        // 3. 合并远程分支（无冲突情况）
        match self.merge() {
            Ok(_) => (),
            Err(Error::MergeConflict) => {
                // 检测到冲突，同步失败
                return Err(Error::MergeConflict);
            }
            Err(e) => return Err(e),
        }

        // 4. 推送本地修改
        self.push()?;

        Ok(())
    }
}
