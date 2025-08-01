use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 知识库项结构体，用于表示知识库的基本信息。
///
/// # 字段
/// * `name` - 知识库名称
/// * `has_remote_repo` - 是否配置了远程仓库
#[derive(Debug, Serialize, Deserialize)]
pub struct Wiki {
    pub name: String,
    pub has_remote_repo: bool,
}

/// 文件节点结构体，用于表示文件系统中的文件或目录
///
/// # 字段
/// * `name` - 文件名或目录名
/// * `is_directory` - 是否为目录
/// * `path` - 文件或目录的路径
/// * `children` - 子节点列表（如果是目录）
#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    pub name: String,
    pub is_directory: bool,
    pub path: String,
    pub children: Option<Vec<FileNode>>,
}

/// 该函数用于获取指定知识库的文件结构。
///
/// # 参数
/// * `wiki_name` - 知识库名称
///
/// # 返回值
/// * `Result<FileNode, String>` - 成功时返回 `Ok(FileNode)`，失败时返回 `Err(String)`。其中 `FileNode` 表示知识库的文件结构，`String` 表示错误信息。
#[tauri::command]
pub fn get_wiki_file_structure(wiki_name: String) -> Result<FileNode, String> {
    // 获取 MarkWiki 可执行文件路径
    let exe_path = std::env::current_exe().map_err(|e| format!("获取可执行文件路径失败: {}", e))?;

    // 获取 MarkWiki 所在目录
    let exe_dir = exe_path.parent().ok_or("获取可执行文件目录失败")?;

    // 构建知识库目录路径
    let wiki_dir = exe_dir.join("Wikis").join(wiki_name);

    // 检查知识库目录是否存在
    if !wiki_dir.exists() {
        return Err(format!("知识库目录不存在于路径: {:?}", wiki_dir));
    }

    // 检查目录是否可读
    if !wiki_dir.is_dir() {
        return Err(format!("路径不是一个目录: {:?}", wiki_dir));
    }

    // 构建文件树并返回
    build_file_tree(&wiki_dir)
}

/// 该函数用于获取MarkWiki运行路径下的知识库列表。它会遍历Wiki目录下的所有文件夹，检查每个文件夹是否为Git仓库，并判断是否配置了远程仓库。
///
/// # 返回值
/// * `Result<Vec<Wiki>, String>` - 成功时返回 `Ok(Vec<Wiki>)`，失败时返回 `Err(String)`。其中 `Vec<Wiki>` 包含所有知识库的信息，`String` 表示错误信息。
#[tauri::command]
pub fn get_wiki_list() -> Result<Vec<Wiki>, String> {
    // 获取 MarkWiki 可执行文件路径
    let exe_path = std::env::current_exe().map_err(|e| format!("获取可执行文件路径失败: {}", e))?;

    // 获取 MarkWiki 所在目录
    let exe_dir = exe_path.parent().ok_or("获取可执行文件目录失败")?;

    // 构建 Wikis 目录路径
    let wiki_dir = exe_dir.join("Wikis");

    // 确保 Wikis 目录存在
    if !wiki_dir.exists() {
        std::fs::create_dir_all(&wiki_dir).map_err(|e| format!("创建知识库目录失败: {}", e))?;
    }

    // 读取 Wikis 目录内容
    let entries = std::fs::read_dir(&wiki_dir).map_err(|e| format!("读取知识库目录失败: {}", e))?;

    // 收集 Wiki 信息
    let mut wikis = Vec::new();

    // 遍历 Wikis 目录下的所有文件夹
    for entry in entries {
        let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
        let path = entry.path();

        // 跳过非目录
        if !path.is_dir() {
            continue;
        }

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // 判断是否为 git 仓库
            match git2::Repository::open(&path) {
                Ok(repo) => {
                    // 获取远程仓库信息
                    let remotes = repo
                        .remotes()
                        .map_err(|e| format!("获取远程仓库失败: {}", e))?;

                    // 写入 wikis 列表
                    wikis.push(Wiki {
                        name: name.to_string(),
                        has_remote_repo: remotes.len() > 0,
                    });
                }
                Err(_) => {}
            }
        }
    }

    Ok(wikis)
}

/// 创建本地知识库
///
/// # 参数
/// * `wiki_name` - 知识库名称
///
/// # 返回值
/// * `Result<String, String>` - 成功时返回 `Ok(String)` 包含知识库路径，失败时返回 `Err(String)` 包含错误信息
#[tauri::command]
pub fn create_local_wiki(wiki_name: &str) -> Result<String, String> {
    // 获取 MarkWiki 可执行文件路径
    let exe_path = std::env::current_exe().map_err(|e| format!("获取可执行文件路径失败: {}", e))?;

    // 获取 MarkWiki 所在目录
    let exe_dir = exe_path.parent().ok_or("获取可执行文件目录失败")?;

    // 构建知识库目录路径
    let wiki_dir = exe_dir.join("Wikis").join(wiki_name);

    // 检查知识库目录是否已存在
    if wiki_dir.exists() {
        return Err(format!("知识库目录已存在: {:?}", wiki_dir));
    }

    // 创建知识库目录
    std::fs::create_dir_all(&wiki_dir).map_err(|e| format!("创建知识库目录失败: {}", e))?;

    // 初始化 Git 仓库
    match git2::Repository::init(&wiki_dir) {
        Ok(_) => Ok(wiki_dir.to_string_lossy().to_string()),
        Err(e) => Err(format!("初始化Git仓库失败: {}", e)),
    }
}

/// 从远程URL创建知识库
///
/// # 参数
/// * `remote_url` - 远程仓库URL
///
/// # 返回值
/// * `Result<String, String>` - 成功时返回 `Ok(String)` 包含知识库路径，失败时返回 `Err(String)` 包含错误信息
#[tauri::command]
pub fn create_remote_wiki(remote_url: &str) -> Result<String, String> {
    // 获取 MarkWiki 可执行文件路径
    let exe_path = std::env::current_exe().map_err(|e| format!("获取可执行文件路径失败: {}", e))?;

    // 获取 MarkWiki 所在目录
    let exe_dir = exe_path.parent().ok_or("获取可执行文件目录失败")?;

    // 从URL提取仓库名称
    let repo_name = remote_url
        .split('/')
        .last()
        .and_then(|s| s.split('.').next())
        .ok_or("从URL提取仓库名称失败")?;

    // 构建知识库目录路径
    let wiki_dir = exe_dir.join("Wikis").join(repo_name);

    // 检查知识库目录是否已存在
    if wiki_dir.exists() {
        return Err(format!("知识库目录已存在: {:?}", wiki_dir));
    } else {
        // 创建知识库目录
        std::fs::create_dir_all(&wiki_dir).map_err(|e| format!("创建知识库目录失败: {}", e))?;
    }

    // 克隆远程仓库
    match git2::Repository::clone(remote_url, &wiki_dir) {
        Ok(_) => Ok(wiki_dir.to_string_lossy().to_string()),
        Err(e) => Err(format!("克隆仓库失败: {}", e)),
    }
}

/// 递归构建文件节点树
fn build_file_tree(path: &Path) -> Result<FileNode, String> {
    let metadata =
        fs::metadata(path).map_err(|e| format!("获取文件元数据失败: {:?}: {}", path, e))?;

    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("获取文件名失败: {:?}", path))?
        .to_string();

    let path_str = path
        .to_str()
        .ok_or_else(|| format!("将路径转换为字符串失败: {:?}", path))?
        .to_string();

    if metadata.is_dir() {
        let mut children = Vec::new();
        let entries = fs::read_dir(path).map_err(|e| format!("读取目录失败: {:?}: {}", path, e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
            let child_path = entry.path();
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

        Ok(FileNode {
            name,
            is_directory: true,
            path: path_str,
            children: Some(children),
        })
    } else {
        Ok(FileNode {
            name,
            is_directory: false,
            path: path_str,
            children: None,
        })
    }
}
