//! Git 仓库操作模块
//!
//! 此模块封装了 Git 仓库的基本操作。
//! 它基于 `git2` 库提供的功能，为上层应用提供更简洁的 Git 操作接口。

use git2;
use std::path::Path;

/// Git 仓库的封装结构体
///
/// 提供对 Git 仓库的各种操作，隐藏底层 `git2::Repository` 的复杂性。
pub struct Repository {
    /// 内部使用的 git2 仓库对象
    repo: git2::Repository,
}

impl Repository {
    /// 打开已存在的 Git 仓库
    ///
    /// # 参数
    /// - `path`: Git 仓库的路径
    ///
    /// # 返回值
    /// - 成功: 返回封装后的 `Repository` 对象
    /// - 失败: 返回包含错误信息的 `String`
    pub fn open(path: &Path) -> Result<Self, String> {
        Ok(Self {
            repo: git2::Repository::open(path).map_err(|e| format!("打开Git仓库失败: {}", e))?,
        })
    }

    /// 在指定路径初始化新的 Git 仓库
    ///
    /// # 参数
    /// - `path`: 要初始化 Git 仓库的路径
    ///
    /// # 返回值
    /// - 成功: 返回封装后的 `Repository` 对象
    /// - 失败: 返回包含错误信息的 `String`
    pub fn init(path: &Path) -> Result<Self, String> {
        Ok(Self {
            repo: git2::Repository::init(path).map_err(|e| format!("初始化Git仓库失败: {}", e))?,
        })
    }

    /// 从远程 URL 克隆 Git 仓库到本地路径
    ///
    /// # 参数
    /// - `url`: 远程 Git 仓库的 URL
    /// - `path`: 本地存储仓库的路径
    ///
    /// # 返回值
    /// - 成功: 返回封装后的 `Repository` 对象
    /// - 失败: 返回包含错误信息的 `String`
    pub fn clone(url: &str, path: &Path) -> Result<Self, String> {
        Ok(Self {
            repo: git2::Repository::clone(url, &path)
                .map_err(|e| format!("克隆Git仓库失败: {}", e))?,
        })
    }

    /// 检查仓库是否配置了远程仓库
    ///
    /// # 返回值
    /// - 成功: 返回布尔值，表示是否存在远程仓库配置
    /// - 失败: 返回包含错误信息的 `String`
    pub fn has_remote_repo(&self) -> Result<bool, String> {
        Ok(self
            .repo
            .remotes()
            .map_err(|e| format!("获取远程仓库信息失败: {}", e))?
            .len()
            > 0)
    }
}
