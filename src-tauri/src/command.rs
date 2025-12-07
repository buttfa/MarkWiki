use std::path::Path;

use crate::config::AppConfig;
use crate::wiki::build_file_tree;
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
    let target_wiki_dir = Wiki::from_name(&wiki_name)
        .map(|wiki| wiki.path)
        .map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 构建文件树并返回
    build_file_tree(Path::new(&target_wiki_dir)).map_err(|e| e.to_string())
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
    for entry in std::fs::read_dir(
        &Wiki::get_wiki_storage_dir()
            .map_err(|e| format!("无法打开所有知识库的统一存储目录: {}", e))?,
    )
    .map_err(|e| format!("读取知识库目录失败: {}", e))?
    {
        let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
        let path = entry.path();

        // 跳过非目录
        if !path.is_dir() {
            continue;
        }

        // 将文件夹名称转换为知识库名称
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        // 尝试从知识库名称创建 Wiki 实例
        if let Ok(wiki) = Wiki::from_name(name) {
            wikis.push(wiki);
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
/// * `username` - Git用户名
/// * `email` - Git邮箱
///
/// # 返回值
/// * `Result<Wiki, String>` - 成功时返回 `Ok(Wiki)`，包含新创建的知识库信息
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn create_local_wiki(
    wiki_name: &str,
    username: &str,
    email: &str,
) -> Result<Wiki, String> {
    // 检查知识库是否已存在
    if Wiki::exists(wiki_name) {
        return Err(format!("知识库 {} 已存在", wiki_name));
    }

    // 创建知识库
    let wiki = Wiki::create_local_wiki(wiki_name, username, email)
        .map_err(|e| format!("创建本地知识库失败: {}", e))?;

    // 保存用户配置到文件
    let config = AppConfig::new(username, email, None);
    config.save().map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(wiki)
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
/// * `Result<Wiki, String>` - 成功时返回 `Ok(Wiki)`，包含新创建的知识库信息
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn create_remote_wiki(
    remote_url: &str,
    username: &str,
    email: &str,
    password: &str,
) -> Result<Wiki, String> {
    // 从URL提取仓库名称
    let wiki_name = remote_url
        .split('/')
        .next_back()
        .and_then(|s| s.split('.').next())
        .ok_or("从URL提取仓库名称失败")?;

    // 检查知识库是否已存在
    if Wiki::exists(wiki_name) {
        return Err(format!("知识库 {} 已存在", wiki_name));
    }

    // 从远程URL创建知识库
    let wiki = Wiki::create_remote_wiki(
        wiki_name,
        remote_url,
        username,
        email,
        if password.is_empty() {
            None
        } else {
            Some(password)
        },
    )
    .map_err(|e| format!("从远程URL创建知识库失败: {}", e))?;

    // 保存用户配置到文件
    let config = AppConfig::new(
        username,
        email,
        if password.is_empty() {
            None
        } else {
            Some(password)
        },
    );
    config.save().map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(wiki)
}

/// 删除知识库
///
/// 该函数会删除指定名称的知识库，包括其所有内容。
/// 请谨慎使用此功能，删除操作是不可逆的。
///
/// # 参数
/// * `wiki_name` - 要删除的知识库名称
///
/// # 返回值
/// * `Result<(), String>` - 成功时返回 `Ok(())`
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn delete_wiki(wiki_name: &str) -> Result<(), String> {
    // 构建目标知识库的存储目录
    let target_wiki_dir = Wiki::get_wiki_storage_dir()
        .map_err(|_| "无法打开所有知识库的统一存储目录")?
        .join(wiki_name);

    // 检查知识库目录是否存在
    if !target_wiki_dir.exists() {
        return Err(format!("知识库不存在: {}", wiki_name));
    }

    // 检查是否为目录
    if !target_wiki_dir.is_dir() {
        return Err(format!("指定的路径不是一个目录: {:?}", target_wiki_dir));
    }

    // 删除知识库目录
    std::fs::remove_dir_all(&target_wiki_dir).map_err(|e| format!("删除知识库失败: {}", e))?;

    Ok(())
}

/// 创建文件
///
/// 该函数会在指定知识库的指定路径下创建一个新文件。
///
/// # 参数
/// * `wiki_name` - 知识库名称
/// * `file_name` - 文件名
/// * `parent_path` - 父目录路径，相对于知识库根目录
///
/// # 返回值
/// * `Result<(), String>` - 成功时返回 `Ok(())`
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn create_file(
    wiki_name: String,
    file_name: String,
    parent_path: String,
) -> Result<(), String> {
    // 验证文件名
    if !file_name.ends_with(".md") {
        return Err("文件名必须以.md结尾".to_string());
    }

    // 获取知识库路径
    let wiki =
        Wiki::from_name(&wiki_name).map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 构建文件的完整路径
    let mut full_path = std::path::PathBuf::from(&wiki.path);

    // 添加父目录路径
    if !parent_path.is_empty() && parent_path != "/" {
        full_path = full_path.join(&parent_path);
    }

    // 添加文件名
    full_path = full_path.join(&file_name);

    // 检查文件是否已存在
    if full_path.exists() {
        return Err(format!("文件已存在: {}", file_name));
    }

    // 确保父目录存在
    if let Some(parent_dir) = full_path.parent() {
        std::fs::create_dir_all(parent_dir).map_err(|e| format!("创建父目录失败: {}", e))?;
    }

    // 创建空文件
    std::fs::File::create(full_path).map_err(|e| format!("创建文件失败: {}", e))?;

    Ok(())
}

/// 创建文件夹
///
/// 该函数会在指定知识库的指定路径下创建一个新文件夹。
///
/// # 参数
/// * `wiki_name` - 知识库名称
/// * `folder_name` - 文件夹名
/// * `parent_path` - 父目录路径，相对于知识库根目录
///
/// # 返回值
/// * `Result<(), String>` - 成功时返回 `Ok(())`
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn create_folder(
    wiki_name: String,
    folder_name: String,
    parent_path: String,
) -> Result<(), String> {
    // 获取知识库路径
    let wiki =
        Wiki::from_name(&wiki_name).map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 构建文件夹的完整路径
    let mut full_path = std::path::PathBuf::from(&wiki.path);

    // 添加父目录路径
    if !parent_path.is_empty() && parent_path != "/" {
        full_path = full_path.join(&parent_path);
    }

    // 添加文件夹名
    full_path = full_path.join(&folder_name);

    // 检查文件夹是否已存在
    if full_path.exists() {
        return Err(format!("文件夹已存在: {}", folder_name));
    }

    // 确保父目录存在
    if let Some(parent_dir) = full_path.parent() {
        std::fs::create_dir_all(parent_dir).map_err(|e| format!("创建父目录失败: {}", e))?;
    }

    // 创建文件夹
    std::fs::create_dir(full_path).map_err(|e| format!("创建文件夹失败: {}", e))?;

    Ok(())
}
/// 读取文件内容
///
/// 该函数会读取指定知识库中指定路径的文件内容。
///
/// # 参数
/// * `wiki_name` - 知识库名称
/// * `file_path` - 文件路径，相对于知识库根目录
///
/// # 返回值
/// * `Result<String, String>` - 成功时返回 `Ok(String)`，包含文件内容
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn read_file(wiki_name: String, file_path: String) -> Result<String, String> {
    // 添加详细调试信息
    eprintln!(
        "开始读取文件 - wiki_name: {}, file_path: {}",
        wiki_name, file_path
    );

    // 获取知识库路径
    let wiki = Wiki::from_name(&wiki_name).map_err(|e| {
        let error_msg = format!("无法打开知识库 {}: {}", wiki_name, e);
        eprintln!("{}", error_msg);
        return error_msg;
    })?;

    eprintln!("知识库路径: {}", wiki.path);

    // 构建文件的完整路径
    let mut full_path = std::path::PathBuf::from(&wiki.path);

    // 添加文件路径，使用更健壮的路径处理
    if !file_path.is_empty() && file_path != "/" {
        // 标准化路径分隔符
        let normalized_path = file_path.replace('/', &std::path::MAIN_SEPARATOR.to_string());
        // 如果路径以分隔符开头，移除它
        let normalized_path = if normalized_path.starts_with(std::path::MAIN_SEPARATOR) {
            &normalized_path[1..]
        } else {
            &normalized_path
        };
        full_path = full_path.join(normalized_path);
    }

    // 打印完整路径（调试用，确认路径是否正确）
    eprintln!("读取文件完整路径: {:?}", full_path);

    // 检查文件是否存在
    if !full_path.exists() {
        let error_msg = format!("文件不存在: {}", full_path.display());
        eprintln!("{}", error_msg);
        return Err(error_msg);
    }

    // 检查是否为文件
    if !full_path.is_file() {
        let error_msg = format!("指定的路径不是一个文件: {}", full_path.display());
        eprintln!("{}", error_msg);
        return Err(error_msg);
    }

    // 读取文件内容
    std::fs::read_to_string(&full_path)
        .map_err(|e| {
            let error_msg = format!("读取文件失败 {}: {}", full_path.display(), e);
            eprintln!("{}", error_msg);
            return error_msg;
        })
        .map(|content| {
            // 输出文件内容长度（避免内容过长时输出过多）
            eprintln!("文件内容读取成功，长度: {} 字符", content.len());
            // 如果内容较短，可以输出前100个字符进行检查
            if content.len() <= 100 {
                eprintln!("文件内容: {}", content);
            } else {
                eprintln!("文件内容（前100字符）: {}", &content[0..100]);
            }
            // 返回文件内容
            content
        })
}

/// 写入文件
///
/// 该函数会将指定内容写入指定知识库中指定路径的文件。
///
/// # 参数
/// * `wiki_name` - 知识库名称
/// * `file_path` - 文件路径，相对于知识库根目录
///
/// # 返回值
/// * `Result<String, String>` - 成功时返回 `Ok(String)`，包含文件内容
/// * 失败时返回 `Err(String)`，包含具体错误信息

#[tauri::command]
pub async fn save_file(
    wiki_name: String,
    file_path: String,
    content: String,
) -> Result<(), String> {
    // 添加详细调试信息
    eprintln!(
        "开始保存文件 - wiki_name: {}, file_path: {}, 内容长度: {}",
        wiki_name,
        file_path,
        content.len()
    );

    // 获取知识库路径
    let wiki = Wiki::from_name(&wiki_name).map_err(|e| {
        let error_msg = format!("无法打开知识库 {}: {}", wiki_name, e);
        eprintln!("{}", error_msg);
        return error_msg;
    })?;

    eprintln!("知识库路径: {}", wiki.path);

    // 构建文件的完整路径（使用更健壮的路径处理）
    let mut full_path = std::path::PathBuf::from(&wiki.path);
    if !file_path.is_empty() && file_path != "/" {
        // 标准化路径分隔符
        let normalized_path = file_path.replace('/', &std::path::MAIN_SEPARATOR.to_string());
        // 如果路径以分隔符开头，移除它
        let normalized_path = if normalized_path.starts_with(std::path::MAIN_SEPARATOR) {
            &normalized_path[1..]
        } else {
            &normalized_path
        };
        full_path = full_path.join(normalized_path);
    }

    // 打印完整路径（调试用）
    eprintln!("保存文件完整路径: {:?}", full_path);

    // 确保父目录存在
    if let Some(parent_dir) = full_path.parent() {
        eprintln!("确保父目录存在: {:?}", parent_dir);
        std::fs::create_dir_all(parent_dir).map_err(|e| {
            let error_msg = format!("创建父目录失败 {}: {}", parent_dir.display(), e);
            eprintln!("{}", error_msg);
            return error_msg;
        })?;
    }

    // 写入文件内容
    std::fs::write(&full_path, content).map_err(|e| {
        let error_msg = format!("保存文件失败 {}: {}", full_path.display(), e);
        eprintln!("{}", error_msg);
        return error_msg;
    })
}
// 添加这行导入（放在文件顶部，其他导入之后）
use crate::git::Repository;
use std::path::PathBuf;

/// Git 同步操作
///
/// 执行完整的 Git 同步流程：获取远程更新 → 合并 → 推送
/// 只支持无冲突情况下的同步
///
/// # 参数
/// * `wiki_name` - 知识库名称
///
/// # 返回值
/// * `Result<(), String>` - 成功时返回 `Ok(())`
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn git_sync(wiki_name: String) -> Result<(), String> {
    // 获取知识库路径
    let wiki = crate::wiki::Wiki::from_name(&wiki_name)
        .map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 打开 Git 仓库
    let repo = Repository::open(&PathBuf::from(&wiki.path))
        .map_err(|e| format!("无法打开 Git 仓库: {}", e))?;

    // 执行同步
    repo.sync().map_err(|e| format!("同步失败: {}", e))
}

/// Git 提交并同步
///
/// 先提交本地修改，然后执行同步
///
/// # 参数
/// * `wiki_name` - 知识库名称
/// * `message` - 提交信息
///
/// # 返回值
/// * `Result<(), String>` - 成功时返回 `Ok(())`
/// * 失败时返回 `Err(String)`，包含具体错误信息
#[tauri::command]
pub async fn git_commit_and_sync(wiki_name: String, message: String) -> Result<(), String> {
    // 获取知识库路径
    let wiki = crate::wiki::Wiki::from_name(&wiki_name)
        .map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 打开 Git 仓库
    let mut repo = Repository::open(&PathBuf::from(&wiki.path))
        .map_err(|e| format!("无法打开 Git 仓库: {}", e))?;

    // 检查是否有未提交的修改
    if !repo
        .has_uncommitted_changes()
        .map_err(|e| format!("检查修改状态失败: {}", e))?
    {
        return Err("没有需要提交的修改".to_string());
    }

    // 添加所有修改到暂存区
    repo.add_all()
        .map_err(|e| format!("添加文件到暂存区失败: {}", e))?;

    // 提交
    repo.commit(&message)
        .map_err(|e| format!("提交失败: {}", e))?;

    // 执行同步
    repo.sync().map_err(|e| format!("同步失败: {}", e))
}

/// 检查是否有未提交的修改
///
/// # 参数
/// * `wiki_name` - 知识库名称
///
/// # 返回值
/// * `Result<bool, String>` - `true` 表示有未提交修改，`false` 表示没有
/// * 失败时返回具体错误信息
#[tauri::command]
pub async fn git_check_status(wiki_name: String) -> Result<bool, String> {
    // 获取知识库路径
    let wiki = crate::wiki::Wiki::from_name(&wiki_name)
        .map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 打开 Git 仓库
    let repo = Repository::open(&PathBuf::from(&wiki.path))
        .map_err(|e| format!("无法打开 Git 仓库: {}", e))?;

    repo.has_uncommitted_changes()
        .map_err(|e| format!("检查状态失败: {}", e))
}

/// 设置 Git 用户配置
///
/// # 参数
/// * `wiki_name` - 知识库名称
/// * `name` - 用户名
/// * `email` - 邮箱
///
/// # 返回值
/// * `Result<(), String>` - 成功时返回 `Ok(())`
/// * 失败时返回具体错误信息
#[tauri::command]
pub async fn git_set_user_config(
    wiki_name: String,
    name: String,
    email: String,
) -> Result<(), String> {
    // 获取知识库路径
    let wiki = crate::wiki::Wiki::from_name(&wiki_name)
        .map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 打开 Git 仓库
    let repo = Repository::open(&PathBuf::from(&wiki.path))
        .map_err(|e| format!("无法打开 Git 仓库: {}", e))?;

    repo.set_user_config(&name, &email)
        .map_err(|e| format!("设置用户配置失败: {}", e))
}

/// 获取 Git 用户配置
///
/// # 参数
/// * `wiki_name` - 知识库名称
///
/// # 返回值
/// * `Result<(String, String), String>` - 成功时返回 `(用户名, 邮箱)`
/// * 失败时返回具体错误信息
#[tauri::command]
pub async fn git_get_user_config(wiki_name: String) -> Result<(String, String), String> {
    // 获取知识库路径
    let wiki = crate::wiki::Wiki::from_name(&wiki_name)
        .map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 打开 Git 仓库
    let repo = Repository::open(&PathBuf::from(&wiki.path))
        .map_err(|e| format!("无法打开 Git 仓库: {}", e))?;

    repo.get_user_config()
        .map_err(|e| format!("获取用户配置失败: {}", e))
}

/// 设置知识库远程仓库
///
/// 该函数会为指定的知识库设置远程仓库信息，包括URL、用户名、邮箱和密码
///
/// # 参数
/// * `wiki_name` - 知识库名称
/// * `remote_url` - 远程仓库URL
/// * `username` - Git用户名
/// * `email` - Git邮箱
/// * `password` - Git密码
///
/// # 返回值
/// * `Result<(), String>` - 成功时返回 `Ok(())`
/// * 失败时返回具体错误信息
#[tauri::command]
pub async fn setup_remote_repo(
    wiki_name: String,
    remote_url: String,
    username: String,
    email: String,
    password: String,
) -> Result<(), String> {
    // 获取知识库路径
    let wiki = crate::wiki::Wiki::from_name(&wiki_name)
        .map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 打开 Git 仓库
    let repo = Repository::open(&PathBuf::from(&wiki.path))
        .map_err(|e| format!("无法打开 Git 仓库: {}", e))?;

    // 设置用户配置
    repo.set_user_config(&username, &email)
        .map_err(|e| format!("设置用户配置失败: {}", e))?;

    // 设置远程仓库
    repo.set_remote("origin", &remote_url)
        .map_err(|e| format!("设置远程仓库失败: {}", e))?;

    // 保存用户凭据到应用配置
    let config = crate::config::AppConfig::new(
        &username,
        &email,
        if password.is_empty() {
            None
        } else {
            Some(&password)
        },
    );
    config.save().map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

/// 获取知识库远程仓库配置信息
///
/// 该函数会获取指定知识库的远程仓库URL和用户配置信息
///
/// # 参数
/// * `wiki_name` - 知识库名称
///
/// # 返回值
/// * `Result<(Option<String>, String, String), String>` - 成功时返回 `(远程仓库URL(可能为None), 用户名, 邮箱)`
/// * 失败时返回具体错误信息
#[tauri::command]
pub async fn get_remote_repo_config(
    wiki_name: String,
) -> Result<(Option<String>, String, String), String> {
    // 获取知识库路径
    let wiki = crate::wiki::Wiki::from_name(&wiki_name)
        .map_err(|e| format!("无法打开知识库 {}: {}", wiki_name, e))?;

    // 打开 Git 仓库
    let repo = Repository::open(&PathBuf::from(&wiki.path))
        .map_err(|e| format!("无法打开 Git 仓库: {}", e))?;

    // 获取用户配置
    let (username, email) = repo
        .get_user_config()
        .map_err(|e| format!("获取用户配置失败: {}", e))?;

    // 获取远程仓库URL
    let remote_url = repo
        .get_remote_url("origin")
        .map_err(|e| format!("获取远程仓库URL失败: {}", e))?;

    Ok((remote_url, username, email))
}
