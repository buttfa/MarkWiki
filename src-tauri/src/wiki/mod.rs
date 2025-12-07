//! 知识库管理模块
//! 知识库管理模块
//!
//! 此模块提供了知识库的核心功能，包括知识库的创建、查询和管理。
//! 它定义了知识库的数据结构，并实现了获取知识库列表、文件结构等功能。

use crate::git;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// 知识库操作可能出现的错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Git 操作错误
    #[error("Git operation error: {0}")]
    Git(#[from] crate::git::Error),

    /// UTF-8 转换错误
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// 路径转换错误
    #[cfg(not(target_os = "android"))]
    #[error("Path conversion error: {0}")]
    Path(String),

    /// 知识库已存在错误
    #[error("Wiki already exists: {0}")]
    AlreadyExists(String),

    /// 知识库不存在错误
    #[error("Wiki not found: {0}")]
    NotFound(String),

    /// 获取知识库存储目录失败
    #[error("Failed to get wiki storage directory")]
    StorageDir,

    /// 构建文件树失败
    #[error("Failed to build file tree: {0}")]
    BuildFileTree(String),
}

/// 知识库项结构体，用于表示知识库的基本信息
///
/// 该结构体主要用于在用户界面中显示知识库列表，包含知识库的名称和是否连接到远程仓库的状态。
///
/// # 字段
/// * `name` - 知识库的名称，通常与存储目录名称一致
/// * `has_remote_repo` - 布尔值，表示该知识库是否配置了远程Git仓库
/// * `path` - 知识库的完整文件路径
#[derive(Debug, Serialize, Deserialize)]
pub struct Wiki {
    name: String,
    has_remote_repo: bool,
    pub path: String,
}

impl Wiki {
    /// 根据名称获取知识库实例
    ///
    /// 该函数尝试根据指定的名称从存储目录中加载知识库。
    /// 首先构建知识库的存储路径，然后检查该路径是否存在，
    /// 最后尝试打开Git仓库并返回知识库实例。
    ///
    /// # 参数
    /// * `name` - 知识库的名称
    ///
    /// # 返回值
    /// * `Result<Self, ()>` - 成功时返回 `Ok(Wiki)`，包含知识库实例
    /// * 失败时返回 `Err(())`，表示无法获取知识库实例（如路径不存在或不是Git仓库）
    pub fn from_name(name: &str) -> Result<Self, Error> {
        // 构建知识库的存储路径
        let path = Self::get_wiki_storage_dir()
            .map_err(|_| Error::StorageDir)?
            .join(name);
        if !path.exists() {
            return Err(Error::NotFound(format!(
                "知识库路径不存在: {}",
                path.display()
            )));
        }
        // 检查是否为 git 仓库
        let repo = git::Repository::open(&path)?;

        // 构建知识库实例
        let has_remote_repo = repo.has_remote_repo().unwrap_or(false);
        Ok(Wiki {
            name: name.to_string(),
            has_remote_repo,
            path: path.to_string_lossy().to_string(),
        })
    }

    /// 检查指定名称的知识库是否存在
    ///
    /// 该函数通过尝试调用`from_name`方法来检查指定名称的知识库是否存在。
    /// 如果`from_name`返回`Ok`，则表示知识库存在；否则表示不存在。
    ///
    /// # 参数
    /// * `name` - 要检查的知识库名称
    ///
    /// # 返回值
    /// * `bool` - 如果知识库存在返回`true`，否则返回`false`
    pub fn exists(name: &str) -> bool {
        Self::from_name(name).is_ok()
    }

    /// 创建本地知识库
    ///
    /// 该函数创建一个新的本地知识库，包括创建存储目录和初始化Git仓库。
    /// 如果指定名称的知识库已经存在，则返回错误。
    ///
    /// # 参数
    /// * `name` - 要创建的知识库名称
    ///
    /// # 返回值
    /// * `Result<Self, Error>` - 成功时返回 `Ok(Wiki)`，包含新创建的知识库实例
    /// * 失败时返回 `Err(Error)`，表示无法创建知识库（如路径已存在或创建目录失败）
    pub fn create_local_wiki(name: &str, username: &str, email: &str) -> Result<Self, Error> {
        // 构造知识库的存储路径
        let path = Self::get_wiki_storage_dir()
            .map_err(|_| Error::StorageDir)?
            .join(name);
        if path.exists() {
            return Err(Error::AlreadyExists(format!(
                "知识库已存在: {}",
                path.display()
            )));
        }

        // 创建知识库目录
        fs::create_dir_all(&path)?;
        // 初始化 git 仓库，设置用户名和邮箱
        git::Repository::init(&path, Some(username), Some(email))?;
        Ok(Wiki {
            name: name.to_string(),
            has_remote_repo: false,
            path: path.to_string_lossy().to_string(),
        })
    }

    /// 从远程Git仓库创建知识库
    ///
    /// 该函数通过克隆远程Git仓库来创建新的知识库。
    /// 如果指定名称的知识库已经存在，则返回错误。
    /// 成功克隆后，会将`has_remote_repo`设置为`true`。
    ///
    /// # 参数
    /// * `name` - 要创建的知识库名称
    /// * `url` - 远程Git仓库的URL
    ///
    /// # 返回值
    /// * `Result<Self, Error>` - 成功时返回 `Ok(Wiki)`，包含新创建的知识库实例
    /// * 失败时返回 `Err(Error)`，表示无法创建知识库（如路径已存在或克隆失败）
    pub fn create_remote_wiki(
        name: &str,
        url: &str,
        username: &str,
        email: &str,
        password: Option<&str>,
    ) -> Result<Self, Error> {
        // 构造知识库的存储路径
        let path = Self::get_wiki_storage_dir()
            .map_err(|_| Error::StorageDir)?
            .join(name);
        if path.exists() {
            return Err(Error::AlreadyExists(format!(
                "知识库已存在: {}",
                path.display()
            )));
        }
        // 克隆远程仓库，设置用户名、邮箱和密码
        git::Repository::clone(url, &path, Some(username), Some(email), password)?;
        Ok(Wiki {
            name: name.to_string(),
            has_remote_repo: true,
            path: path.to_string_lossy().to_string(),
        })
    }

    /// 获取所有知识库的统一存储目录
    ///
    /// 该函数根据当前运行平台，返回MarkWiki应用存储知识库的统一目录路径。
    /// 如果目录不存在，会自动创建该目录。
    ///
    /// # 返回值
    /// * `Result<PathBuf, Error>` - 成功时返回 `Ok(PathBuf)`，包含知识库的统一存储目录
    /// * 失败时返回 `Err(Error)`
    ///
    /// # 平台差异
    /// * Android平台：使用临时目录下的markwiki/wiki目录
    /// * Linux/Windows平台：使用可执行文件所在目录下的wiki目录
    pub fn get_wiki_storage_dir() -> Result<PathBuf, Error> {
        // Android 平台
        #[cfg(target_os = "android")]
        {
            let wiki_dir = std::env::temp_dir().join("markwiki").join("wiki");
            // 确保 wiki 目录存在
            if !wiki_dir.exists() {
                std::fs::create_dir_all(&wiki_dir)?;
            }
            Ok(wiki_dir)
        }

        // Linux/Windows 平台
        #[cfg(not(target_os = "android"))]
        {
            // 获取 MarkWiki 可执行文件路径
            let exe_path = std::env::current_exe()?;

            // 获取 MarkWiki 所在目录
            let exe_dir = exe_path
                .parent()
                .ok_or(Error::Path("无法获取可执行文件所在目录".to_string()))?;

            // 构建知识库目录路径
            let wiki_dir = exe_dir.join("wiki");

            // 确保 wiki 目录存在
            if !wiki_dir.exists() {
                std::fs::create_dir_all(&wiki_dir)?;
            }
            Ok(wiki_dir)
        }
    }
}

/// 文件节点结构体，用于表示文件系统中的文件或目录
///
/// 该结构体用于构建完整的文件树，支持递归表示目录结构，便于在用户界面中展示知识库的文件组织结构。
/// 支持序列化和反序列化，可用于与前端JavaScript代码进行数据交互。
///
/// # 字段
/// * `name` - 文件名或目录名
/// * `is_directory` - 布尔值，表示是否为目录
/// * `path` - 文件或目录的完整路径字符串
/// * `children` - 可选的子节点列表，如果是目录则包含子文件和子目录，否则为None
#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    name: String,
    is_directory: bool,
    path: String,
    children: Option<Vec<FileNode>>,
}

/// 递归构建文件系统的节点树
///
/// 该函数从指定路径开始，递归遍历文件系统，构建一个完整的文件树结构表示。
/// 返回的FileNode结构体包含了完整的目录层次结构，可用于在用户界面中显示文件浏览器。
///
/// # 参数
/// * `path` - 要构建文件树的根路径，可以是文件或目录
///
/// # 返回值
/// * `Result<FileNode, Error>` - 成功时返回 `Ok(FileNode)`，包含构建的文件节点树
/// * 失败时返回 `Err(Error)`，包含具体错误信息
///
/// # 实现细节
/// * 对于目录，会递归遍历其所有子项并构建子节点树
/// * 子节点会按名称排序，目录排在文件前面
/// * 对于文件，不会包含子节点信息
pub fn build_file_tree(path: &Path) -> Result<FileNode, Error> {
    // 获取文件或目录的元数据
    let metadata = fs::metadata(path)
        .map_err(|e| Error::BuildFileTree(format!("获取文件元数据失败: {:?}: {}", path, e)))?;

    // 提取并转换文件或目录名称为字符串
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| Error::BuildFileTree(format!("获取文件名失败: {:?}", path)))?
        .to_string();

    // 转换路径为字符串表示
    let path_str = path
        .to_str()
        .ok_or_else(|| Error::BuildFileTree(format!("将路径转换为字符串失败: {:?}", path)))?
        .to_string();

    // 如果是目录，递归构建子节点树
    if metadata.is_dir() {
        let mut children = Vec::new();
        let entries = fs::read_dir(path)
            .map_err(|e| Error::BuildFileTree(format!("读取目录失败: {:?}: {}", path, e)))?;

        // 遍历目录中的所有条目
        for entry in entries {
            let entry =
                entry.map_err(|e| Error::BuildFileTree(format!("读取目录条目失败: {}", e)))?;
            let child_path = entry.path();
            // 递归构建子节点
            children.push(build_file_tree(&child_path)?);
        }

        // 按名称排序，目录在前，文件在后
        children.sort_by(|a, b| {
            if a.is_directory && !b.is_directory {
                std::cmp::Ordering::Less
            } else if !a.is_directory && b.is_directory {
                std::cmp::Ordering::Greater
            } else {
                a.name.cmp(&b.name)
            }
        });

        // 返回目录节点，包含子节点信息
        Ok(FileNode {
            name,
            is_directory: true,
            path: path_str,
            children: Some(children),
        })
    } else {
        // 返回文件节点，不包含子节点
        Ok(FileNode {
            name,
            is_directory: false,
            path: path_str,
            children: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 `get_wiki_storage_dir` 函数的基本功能
    ///
    /// # 测试目标：
    /// * 验证函数是否能正确返回知识库的统一存储目录
    /// * 验证知识库的统一存储目录名称为 "wiki"
    /// * 验证该目录存在且父目录包含 markwiki 应用程序
    #[test]
    fn test_get_wiki_storage_dir() {
        // 调用函数获取知识库目录
        let wiki_dir = Wiki::get_wiki_storage_dir().unwrap();

        // 验证知识库目录名称为 "wiki"
        assert_eq!(wiki_dir.file_name().unwrap().to_str().unwrap(), "wiki");

        // 验证知识库目录存在且是一个目录
        assert!(wiki_dir.exists());
        assert!(wiki_dir.is_dir());

        // 验证父目录下存在 markwiki 应用程序
        #[cfg(target_os = "windows")]
        let markwiki_exe = wiki_dir.parent().unwrap().join("markwiki.exe");
        #[cfg(target_os = "linux")]
        let markwiki_exe = wiki_dir.parent().unwrap().join("markwiki");
        assert!(markwiki_exe.exists());

        // 验证 markwiki 应用程序是一个可执行文件
        assert!(markwiki_exe.is_file());
    }

    /// 测试 `build_file_tree` 函数的基本功能
    ///
    /// # 测试目标：
    /// * 验证函数是否能正确递归构建文件夹节点树
    /// * 验证构建的文件夹节点树与实际文件系统结构一致
    #[test]
    fn test_build_file_tree() {
        // 创建临时目录用于测试
        let temp_dir = tempfile::TempDir::new().unwrap();
        let test_path = temp_dir.path();

        // 创建测试目录结构
        // temp_dir/
        // ├── file1.txt
        // ├── subdir1/
        // │   ├── file2.txt
        // │   └── file3.md
        // └── subdir2/
        //     └── file4.txt

        let subdir1 = test_path.join("subdir1");
        let subdir2 = test_path.join("subdir2");
        fs::create_dir(&subdir1).unwrap();
        fs::create_dir(&subdir2).unwrap();

        fs::write(test_path.join("file1.txt"), "content1").unwrap();
        fs::write(subdir1.join("file2.txt"), "content2").unwrap();
        fs::write(subdir1.join("file3.md"), "content3").unwrap();
        fs::write(subdir2.join("file4.txt"), "content4").unwrap();

        // 调用 build_file_tree 函数
        let root_node = build_file_tree(test_path).unwrap();

        // 验证根节点名称是临时目录的实际名称
        let expected_name = test_path.file_name().unwrap().to_str().unwrap();
        assert_eq!(root_node.name, expected_name);
        assert!(root_node.is_directory);
        assert!(root_node.children.is_some());

        let children = root_node.children.unwrap();

        // 验证子节点数量（应该有3个：file1.txt, subdir1, subdir2）
        assert_eq!(children.len(), 3);

        // 验证排序：目录在前，文件在后
        assert!(children[0].is_directory); // subdir1
        assert!(children[1].is_directory); // subdir2
        assert!(!children[2].is_directory); // file1.txt

        // 验证子目录1
        let subdir1_node = &children[0];
        assert_eq!(subdir1_node.name, "subdir1");
        assert!(subdir1_node.is_directory);
        assert!(subdir1_node.children.is_some());

        let subdir1_children = subdir1_node.children.as_ref().unwrap();
        assert_eq!(subdir1_children.len(), 2);

        // 验证子目录1的文件（按名称排序）
        assert_eq!(subdir1_children[0].name, "file2.txt");
        assert!(!subdir1_children[0].is_directory);
        assert_eq!(subdir1_children[1].name, "file3.md");
        assert!(!subdir1_children[1].is_directory);

        // 验证子目录2
        let subdir2_node = &children[1];
        assert_eq!(subdir2_node.name, "subdir2");
        assert!(subdir2_node.is_directory);
        assert!(subdir2_node.children.is_some());

        let subdir2_children = subdir2_node.children.as_ref().unwrap();
        assert_eq!(subdir2_children.len(), 1);

        // 验证子目录2的文件
        assert_eq!(subdir2_children[0].name, "file4.txt");
        assert!(!subdir2_children[0].is_directory);

        // 验证根目录下的文件
        let file1_node = &children[2];
        assert_eq!(file1_node.name, "file1.txt");
        assert!(!file1_node.is_directory);
        assert!(file1_node.children.is_none());

        // 验证路径正确性
        assert!(root_node.path.ends_with(expected_name));
        assert!(subdir1_node.path.ends_with("subdir1"));
        assert!(subdir2_node.path.ends_with("subdir2"));
        assert!(file1_node.path.ends_with("file1.txt"));
    }
}
