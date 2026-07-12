// Tauri 命令处理

use std::collections::HashSet;
use std::fs;
use tauri::State;

use crate::db;
use crate::db::DbState;
use crate::models::*;

// ============ 提示词命令 ============

#[tauri::command]
pub fn get_prompts(
    state: State<DbState>,
    category_id: Option<String>,
    search: Option<String>,
    tag: Option<String>,
) -> Result<Vec<Prompt>, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::get_prompts(
        &conn,
        category_id.as_deref(),
        search.as_deref(),
        tag.as_deref(),
    )
}

#[tauri::command]
pub fn create_prompt(state: State<DbState>, input: CreatePromptInput) -> Result<Prompt, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::create_prompt(&conn, &input)
}

#[tauri::command]
pub fn update_prompt(state: State<DbState>, input: UpdatePromptInput) -> Result<Prompt, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::update_prompt(&conn, &input)
}

#[tauri::command]
pub fn delete_prompt(state: State<DbState>, id: String) -> Result<(), String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::delete_prompt(&conn, &id)
}

#[tauri::command]
pub fn set_prompt_pinned(
    state: State<DbState>,
    id: String,
    pinned: bool,
) -> Result<Prompt, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::set_prompt_pinned(&conn, &id, pinned)
}

#[tauri::command]
pub fn record_prompt_use(state: State<DbState>, id: String) -> Result<Prompt, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::record_prompt_use(&conn, &id)
}

#[tauri::command]
pub fn get_all_tags(state: State<DbState>) -> Result<Vec<String>, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::get_all_tags(&conn)
}

// ============ 分类命令 ============

#[tauri::command]
pub fn get_categories(state: State<DbState>) -> Result<Vec<CategoryWithCount>, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::get_categories(&conn)
}

#[tauri::command]
pub fn create_category(
    state: State<DbState>,
    input: CreateCategoryInput,
) -> Result<Category, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::create_category(&conn, &input)
}

#[tauri::command]
pub fn update_category(
    state: State<DbState>,
    input: UpdateCategoryInput,
) -> Result<Category, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::update_category(&conn, &input)
}

#[tauri::command]
pub fn delete_category(state: State<DbState>, id: String) -> Result<(), String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::delete_category(&conn, &id)
}

#[tauri::command]
pub fn get_uncategorized_count(state: State<DbState>) -> Result<i32, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::get_uncategorized_count(&conn)
}

// ============ 导入导出命令 ============

#[tauri::command]
pub fn export_to_json(state: State<DbState>, path: String) -> Result<String, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    let data = db::export_all_data(&conn)?;
    let json = serde_json::to_string_pretty(&data).map_err(|e| format!("序列化失败: {}", e))?;
    fs::write(&path, &json).map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(path)
}

#[tauri::command]
pub fn export_to_markdown(state: State<DbState>, path: String) -> Result<String, String> {
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    let data = db::export_all_data(&conn)?;
    let category_ids: HashSet<&str> = data.categories.iter().map(|cat| cat.id.as_str()).collect();

    let mut md = String::new();
    md.push_str("# Prompt Manager 导出\n\n");
    md.push_str(&format!("导出时间: {}\n\n", data.exported_at));
    md.push_str("---\n\n");

    // 按分类分组
    for cat in &data.categories {
        md.push_str(&format!("## {}\n\n", cat.name));
        let prompts: Vec<&Prompt> = data
            .prompts
            .iter()
            .filter(|p| p.category_id.as_deref() == Some(cat.id.as_str()))
            .collect();
        if prompts.is_empty() {
            md.push_str("暂无提示词\n\n");
        }
        for prompt in prompts {
            md.push_str(&format!("### {}\n\n", prompt.title));
            md.push_str(&format!("{}\n\n", prompt.content));
            if !prompt.tags.is_empty() {
                md.push_str(&format!(
                    "标签: {}\n\n",
                    prompt
                        .tags
                        .iter()
                        .map(|t| format!("`{}`", t))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
            md.push_str("---\n\n");
        }
    }

    // 未分类和已丢失分类的提示词
    let uncategorized: Vec<&Prompt> = data
        .prompts
        .iter()
        .filter(|p| match p.category_id.as_deref() {
            None => true,
            Some(category_id) => !category_ids.contains(category_id),
        })
        .collect();
    if !uncategorized.is_empty() {
        md.push_str("## 未分类\n\n");
        for prompt in uncategorized {
            md.push_str(&format!("### {}\n\n", prompt.title));
            md.push_str(&format!("{}\n\n", prompt.content));
            if !prompt.tags.is_empty() {
                md.push_str(&format!(
                    "标签: {}\n\n",
                    prompt
                        .tags
                        .iter()
                        .map(|t| format!("`{}`", t))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
            md.push_str("---\n\n");
        }
    }

    fs::write(&path, &md).map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(path)
}

#[tauri::command]
pub fn preview_import_json(state: State<DbState>, path: String) -> Result<ImportPreview, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("读取文件失败: {}", e))?;
    let data: ExportData =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    db::preview_import_data(&conn, &data)
}

#[tauri::command]
pub fn import_from_json(state: State<DbState>, path: String) -> Result<String, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("读取文件失败: {}", e))?;
    let data: ExportData =
        serde_json::from_str(&content).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let mut conn = state
        .0
        .lock()
        .map_err(|e| format!("获取数据库锁失败: {}", e))?;
    let (cats, prompts) = db::import_data(&mut conn, &data)?;

    Ok(format!("导入完成: {} 个分类, {} 条提示词", cats, prompts))
}
