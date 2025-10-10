use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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
pub async fn get_wiki_file_structure(wiki_name: String) -> Result<FileNode, String> {
    // 构建目标知识库的存储目录
    let target_wiki_dir = get_wiki_storage_dir()?.join(wiki_name);

    // 构建文件树并返回
    build_file_tree(&target_wiki_dir)
}

/// 该函数用于获取MarkWiki运行路径下的知识库列表。它会遍历Wiki目录下的所有文件夹，检查每个文件夹是否为Git仓库，并判断是否配置了远程仓库。
///
/// # 返回值
/// * `Result<Vec<Wiki>, String>` - 成功时返回 `Ok(Vec<Wiki>)`，失败时返回 `Err(String)`。其中 `Vec<Wiki>` 包含所有知识库的信息，`String` 表示错误信息。
#[tauri::command]
pub async fn get_wiki_list() -> Result<Vec<Wiki>, String> {
    // 统计知识库信息
    let mut wikis = Vec::new();

    // 遍历 Wikis 目录下的所有文件夹
    for entry in std::fs::read_dir(&get_wiki_storage_dir()?)
        .map_err(|e| format!("读取知识库目录失败: {}", e))?
    {
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
pub async fn create_local_wiki(wiki_name: &str) -> Result<String, String> {
    // 构建目标知识库的存储目录
    let target_wiki_dir = get_wiki_storage_dir()?.join(wiki_name);

    // 检查知识库目录是否已存在
    if target_wiki_dir.exists() {
        return Err(format!("知识库目录已存在: {:?}", target_wiki_dir));
    }

    // 创建知识库目录
    std::fs::create_dir_all(&target_wiki_dir).map_err(|e| format!("创建知识库目录失败: {}", e))?;

    // 初始化 Git 仓库
    match git2::Repository::init(&target_wiki_dir) {
        Ok(_) => Ok(target_wiki_dir.to_string_lossy().to_string()),
        Err(e) => {
            // 删除已创建的目录
            std::fs::remove_dir_all(&target_wiki_dir)
                .map_err(|e| format!("删除已创建知识库目录失败: {}", e))?;
            // 返回错误信息
            Err(format!("初始化Git仓库失败: {}", e))
        }
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
pub async fn create_remote_wiki(remote_url: &str) -> Result<String, String> {
    // 从URL提取仓库名称
    let repo_name = remote_url
        .split('/')
        .last()
        .and_then(|s| s.split('.').next())
        .ok_or("从URL提取仓库名称失败")?;

    // 构建目标知识库的存储目录
    let target_wiki_dir = get_wiki_storage_dir()?.join(repo_name);

    // 检查知识库目录是否已存在
    if target_wiki_dir.exists() {
        return Err(format!("知识库目录已存在: {:?}", target_wiki_dir));
    } else {
        // 创建知识库目录
        std::fs::create_dir_all(&target_wiki_dir)
            .map_err(|e| format!("创建知识库目录失败: {}", e))?;
    }

    // 克隆远程仓库
    match git2::Repository::clone(remote_url, &target_wiki_dir) {
        Ok(_) => Ok(target_wiki_dir.to_string_lossy().to_string()),
        Err(e) => {
            // 删除已创建的目录
            std::fs::remove_dir_all(&target_wiki_dir)
                .map_err(|e| format!("删除已创建知识库目录失败: {}", e))?;
            // 返回错误信息
            Err(format!("克隆仓库失败: {}", e))
        }
    }
}

/// 删除知识库
///
/// # 参数
/// * `wiki_name` - 知识库名称
///
/// # 返回值
/// * `Result<String, String>` - 成功时返回 `Ok(String)` 包含成功信息，失败时返回 `Err(String)` 包含错误信息
#[tauri::command]
pub async fn delete_wiki(wiki_name: &str) -> Result<String, String> {
    // 构建目标知识库的存储目录
    let target_wiki_dir = get_wiki_storage_dir()?.join(wiki_name);

    // 检查知识库目录是否存在
    if !target_wiki_dir.exists() {
        return Err(format!("知识库不存在: {}", wiki_name));
    }

    // 检查是否为目录
    if !target_wiki_dir.is_dir() {
        return Err(format!("指定的路径不是一个目录: {:?}", target_wiki_dir));
    }

    // 删除知识库目录
    std::fs::remove_dir_all(&target_wiki_dir)
        .map_err(|e| format!("删除知识库失败: {}", e))?;

    Ok(format!("成功删除知识库: {}", wiki_name))
}

/// 递归构建文件夹节点树
///
/// # 参数
/// * `path` - 文件夹路径
///
/// # 返回值
/// * `Result<FileNode, String>` - 成功时返回 `Ok(FileNode)`，失败时返回 `Err(String)`。其中 `FileNode` 表示文件夹节点，`String` 表示错误信息。
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

/// 获取所有知识库的统一存储目录
///
/// # 返回值
/// * `Result<PathBuf, String>` - 成功时返回 `Ok(PathBuf)` 包含知识库的统一存储目录，失败时返回 `Err(String)` 包含错误信息
fn get_wiki_storage_dir() -> Result<PathBuf, String> {
    // Android 平台
    #[cfg(target_os = "android")]
    {
        let wiki_dir = std::env::temp_dir().join("markwiki").join("wiki");
        // 确保 wiki 目录存在
        if !wiki_dir.exists() {
            std::fs::create_dir_all(&wiki_dir).map_err(|e| format!("创建知识库目录失败: {}", e))?;
        }
        return Ok(wiki_dir);
    }

    // Linux/Windows 平台
    #[cfg(not(target_os = "android"))]
    {
        // 获取 MarkWiki 可执行文件路径
        let exe_path =
            std::env::current_exe().map_err(|e| format!("获取可执行文件路径失败: {}", e))?;

        // 获取 MarkWiki 所在目录
        let exe_dir = exe_path.parent().ok_or("获取可执行文件目录失败")?;

        // 构建知识库目录路径
        let wiki_dir = exe_dir.join("wiki");

        // 确保 wiki 目录存在
        if !wiki_dir.exists() {
            std::fs::create_dir_all(&wiki_dir).map_err(|e| format!("创建知识库目录失败: {}", e))?;
        }
        return Ok(wiki_dir);
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
        let wiki_dir = get_wiki_storage_dir().unwrap();

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
