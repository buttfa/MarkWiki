use crate::git;
use crate::wiki::build_file_tree;
use crate::wiki::get_wiki_storage_dir;
use crate::wiki::FileNode;
use crate::wiki::Wiki;

/// 获取指定知识库的文件结构
///
/// 该函数会递归遍历指定知识库的目录结构，构建完整的文件树结构并返回。
///
/// # 参数
/// * `wiki_name` - 知识库名称，用于确定要查询的目标知识库
///
/// # 返回值
/// * `Result<FileNode, String>` - 成功时返回 `Ok(FileNode)`，包含知识库的完整文件结构
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn get_wiki_file_structure(wiki_name: String) -> Result<FileNode, String> {
    // 构建目标知识库的存储目录
    let target_wiki_dir = get_wiki_storage_dir()?.join(wiki_name);

    // 构建文件树并返回
    build_file_tree(&target_wiki_dir)
}

/// 获取MarkWiki运行路径下的所有知识库列表
///
/// 该函数会遍历Wiki目录下的所有文件夹，检查每个文件夹是否为Git仓库，并判断是否配置了远程仓库。
/// 主要用于在应用程序界面中显示所有可用的知识库。
///
/// # 返回值
/// * `Result<Vec<Wiki>, String>` - 成功时返回 `Ok(Vec<Wiki>)`，包含所有知识库的基本信息
/// * 失败时返回 `Err(String)`，包含具体错误信息
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
            // 尝试作为 git 仓库打开
            if let Ok(repo) = git::Repository::open(&path) {
                wikis.push(Wiki {
                    name: name.to_string(),
                    // 检查是否配置了远程仓库
                    has_remote_repo: repo.has_remote_repo().unwrap_or(false),
                });
            }
        }
    }

    Ok(wikis)
}

/// 创建本地知识库
///
/// 该函数会在本地创建一个新的知识库目录，并在其中初始化Git仓库。
/// 如果创建过程中出现错误，会自动清理已创建的目录。
///
/// # 参数
/// * `wiki_name` - 要创建的知识库名称
///
/// # 返回值
/// * `Result<String, String>` - 成功时返回 `Ok(String)`，包含知识库的完整路径
/// * 失败时返回 `Err(String)`，包含具体错误信息
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
    match git::Repository::init(&target_wiki_dir) {
        Ok(_) => Ok(target_wiki_dir.to_string_lossy().to_string()),
        Err(_) => {
            // 删除已创建的目录
            std::fs::remove_dir_all(&target_wiki_dir)
                .map_err(|e| format!("删除已创建知识库目录失败: {}", e))?;
            // 返回错误信息
            Err(format!("初始化Git仓库失败"))
        }
    }
}

/// 从远程URL创建知识库
///
/// 该函数会从指定的远程Git仓库URL克隆内容，并在本地创建对应的知识库。
/// 知识库名称会从URL中自动提取。如果克隆过程中出现错误，会自动清理已创建的目录。
///
/// # 参数
/// * `remote_url` - 远程Git仓库的URL
///
/// # 返回值
/// * `Result<String, String>` - 成功时返回 `Ok(String)`，包含克隆的知识库的完整路径
/// * 失败时返回 `Err(String)`，包含具体错误信息
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
    match git::Repository::clone(remote_url, &target_wiki_dir) {
        Ok(_) => Ok(target_wiki_dir.to_string_lossy().to_string()),
        Err(_) => {
            // 删除已创建的目录
            std::fs::remove_dir_all(&target_wiki_dir)
                .map_err(|e| format!("删除已创建知识库目录失败: {}", e))?;
            // 返回错误信息
            Err(format!("克隆仓库失败"))
        }
    }
}
