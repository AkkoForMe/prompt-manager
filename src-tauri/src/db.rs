// 数据库操作模块

use rusqlite::{params, types::Type, Connection, OptionalExtension, Row};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::models::*;

/// 数据库状态（通过 Tauri 管理状态共享）
pub struct DbState(pub Mutex<Connection>);

/// 初始化数据库表结构和示例数据
pub fn init_db(conn: &Connection) -> Result<(), String> {
    // 创建表
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS prompts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            category_id TEXT,
            tags TEXT NOT NULL DEFAULT '[]',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            pinned INTEGER NOT NULL DEFAULT 0,
            last_used_at TEXT,
            use_count INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
        );",
    )
    .map_err(|e| format!("初始化数据库失败: {}", e))?;

    migrate_prompt_usage_columns(conn)?;

    // 检查是否已有数据，没有则插入示例数据
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM prompts", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    if count == 0 {
        insert_sample_data(conn)?;
    }

    repair_orphaned_prompts(conn)?;

    Ok(())
}

fn migrate_prompt_usage_columns(conn: &Connection) -> Result<(), String> {
    let mut stmt = conn
        .prepare("PRAGMA table_info(prompts)")
        .map_err(|e| format!("读取提示词表结构失败: {}", e))?;
    let columns: HashSet<String> = stmt
        .query_map([], |row| row.get(1))
        .map_err(|e| format!("读取提示词表结构失败: {}", e))?
        .collect::<Result<_, _>>()
        .map_err(|e| format!("解析提示词表结构失败: {}", e))?;

    if !columns.contains("pinned") {
        conn.execute(
            "ALTER TABLE prompts ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0",
            [],
        )
        .map_err(|e| format!("迁移置顶字段失败: {}", e))?;
    }
    if !columns.contains("last_used_at") {
        conn.execute("ALTER TABLE prompts ADD COLUMN last_used_at TEXT", [])
            .map_err(|e| format!("迁移最近使用字段失败: {}", e))?;
    }
    if !columns.contains("use_count") {
        conn.execute(
            "ALTER TABLE prompts ADD COLUMN use_count INTEGER NOT NULL DEFAULT 0",
            [],
        )
        .map_err(|e| format!("迁移使用次数字段失败: {}", e))?;
    }
    Ok(())
}

/// 插入示例数据
fn insert_sample_data(conn: &Connection) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();

    // 示例分类
    let categories = vec![
        ("cat-dev", "开发", 1),
        ("cat-work", "工作", 2),
        ("cat-translate", "翻译", 3),
        ("cat-writing", "写作", 4),
        ("cat-daily", "日常", 5),
    ];

    for (id, name, sort_order) in &categories {
        conn.execute(
            "INSERT INTO categories (id, name, sort_order, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![id, name, sort_order, now],
        )
        .map_err(|e| format!("插入示例分类失败: {}", e))?;
    }

    // 示例提示词
    let prompts = vec![
        (
            "prompt-1",
            "代码审查助手",
            "你是一个资深代码审查专家。请对以下代码进行详细审查，关注以下方面：\n\n1. 代码质量和可读性\n2. 潜在的 Bug 和安全漏洞\n3. 性能优化建议\n4. 最佳实践建议\n\n请用清晰的结构化格式输出你的审查意见，按严重程度（Critical / Warning / Info）分级。",
            Some("cat-dev"),
            vec!["开发", "review", "代码审查"],
        ),
        (
            "prompt-2",
            "周报生成器",
            "请根据我提供的本周工作内容，生成一份结构清晰的工作周报。要求：\n\n1. 本周完成的工作（按项目或模块分类）\n2. 遇到的困难和解决方案\n3. 下周工作计划\n4. 需要协调的事项\n\n语气正式专业，条理清晰，突出重点成果。",
            Some("cat-work"),
            vec!["工作", "周报", "效率"],
        ),
        (
            "prompt-3",
            "英文翻译助手",
            "你是一个专业的中英双语翻译专家。请将以下文本翻译为英文，要求：\n\n1. 准确传达原文含义\n2. 使用地道自然的英文表达\n3. 保持原文的语气和风格\n4. 如有专业术语，请使用通用翻译并在括号中标注原文\n\n翻译完成后，请简要说明翻译中的关键决策。",
            Some("cat-translate"),
            vec!["翻译", "英文", "中英"],
        ),
        (
            "prompt-4",
            "写作风格改写",
            "请将以下文本改写为更加专业、流畅的版本。保持核心意思不变，但优化：\n\n1. 用词精准度\n2. 句式多样性\n3. 逻辑连贯性\n4. 可读性\n\n请提供改写后的版本，并简要说明主要改动点。",
            Some("cat-writing"),
            vec!["写作", "润色", "改写"],
        ),
        (
            "prompt-5",
            "头脑风暴助手",
            "你是一个创意顾问。请围绕以下主题进行头脑风暴，提供：\n\n1. 至少 10 个创新想法\n2. 每个想法的简要描述\n3. 可行性评估（高/中/低）\n4. 实施的关键步骤\n\n请鼓励发散思维，不要过早否定任何想法。",
            Some("cat-daily"),
            vec!["创意", "头脑风暴", "日常"],
        ),
    ];

    for (id, title, content, category_id, tags) in &prompts {
        let tags_json = serde_json::to_string(tags).unwrap();
        conn.execute(
            "INSERT INTO prompts (id, title, content, category_id, tags, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, title, content, category_id, tags_json, now, now],
        )
        .map_err(|e| format!("插入示例提示词失败: {}", e))?;
    }

    Ok(())
}

fn parse_tags(tags_str: &str) -> Result<Vec<String>, rusqlite::Error> {
    serde_json::from_str(tags_str)
        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, Type::Text, Box::new(e)))
}

fn map_prompt_row(row: &Row<'_>) -> Result<Prompt, rusqlite::Error> {
    let tags_str: String = row.get(4)?;
    let tags = parse_tags(&tags_str)?;

    Ok(Prompt {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
        category_id: row.get(3)?,
        tags,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
        pinned: row.get(7)?,
        last_used_at: row.get(8)?,
        use_count: row.get(9)?,
    })
}

fn repair_orphaned_prompts(conn: &Connection) -> Result<(), String> {
    conn.execute(
        "UPDATE prompts
         SET category_id = NULL
         WHERE category_id IS NOT NULL
           AND category_id NOT IN (SELECT id FROM categories)",
        [],
    )
    .map_err(|e| format!("修复无效分类关联失败: {}", e))?;

    Ok(())
}

// ============ 提示词操作 ============

/// 获取所有提示词（支持分类筛选和搜索）
pub fn get_prompts(
    conn: &Connection,
    category_id: Option<&str>,
    search: Option<&str>,
    tag: Option<&str>,
) -> Result<Vec<Prompt>, String> {
    let mut sql = String::from(
        "SELECT id, title, content, category_id, tags, created_at, updated_at, pinned, last_used_at, use_count FROM prompts WHERE 1=1",
    );
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut param_idx = 1;

    // 分类筛选：None 表示"全部"，Some("uncategorized") 表示"未分类"，Some(id) 表示指定分类
    if let Some(cid) = category_id {
        if cid == "uncategorized" {
            sql.push_str(" AND category_id IS NULL");
        } else {
            sql.push_str(&format!(" AND category_id = ?{}", param_idx));
            param_values.push(Box::new(cid.to_string()));
            param_idx += 1;
        }
    }

    let parsed_search = parse_search_query(search.unwrap_or_default());
    let mut relevance_parts = Vec::new();
    for term in parsed_search.terms {
        let pattern = format!("%{}%", term);
        sql.push_str(&format!(
            " AND (title LIKE ?{} OR content LIKE ?{} OR tags LIKE ?{})",
            param_idx,
            param_idx + 1,
            param_idx + 2
        ));
        relevance_parts.push(format!(
            "CASE WHEN title LIKE ?{} THEN 100 ELSE 0 END + CASE WHEN tags LIKE ?{} THEN 40 ELSE 0 END + CASE WHEN content LIKE ?{} THEN 10 ELSE 0 END",
            param_idx,
            param_idx + 2,
            param_idx + 1
        ));
        param_values.push(Box::new(pattern.clone()));
        param_values.push(Box::new(pattern.clone()));
        param_values.push(Box::new(pattern));
        param_idx += 3;
    }

    for search_tag in parsed_search.tags {
        let tag_pattern = format!("%\"{}\"%", search_tag);
        sql.push_str(&format!(" AND tags LIKE ?{}", param_idx));
        param_values.push(Box::new(tag_pattern));
        param_idx += 1;
    }

    if let Some(category_name) = parsed_search.category_name {
        sql.push_str(&format!(
            " AND category_id IN (SELECT id FROM categories WHERE name = ?{} COLLATE NOCASE)",
            param_idx
        ));
        param_values.push(Box::new(category_name));
        param_idx += 1;
    }

    // 标签筛选
    if let Some(t) = tag {
        if !t.is_empty() {
            let tag_pattern = format!("%\"{}\"%", t);
            sql.push_str(&format!(" AND tags LIKE ?{}", param_idx));
            param_values.push(Box::new(tag_pattern));
        }
    }

    if relevance_parts.is_empty() {
        sql.push_str(" ORDER BY pinned DESC, updated_at DESC");
    } else {
        sql.push_str(&format!(
            " ORDER BY pinned DESC, ({}) DESC, last_used_at DESC, use_count DESC, updated_at DESC",
            relevance_parts.join(" + ")
        ));
    }

    let params: Vec<&dyn rusqlite::types::ToSql> =
        param_values.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("查询提示词失败: {}", e))?;
    let rows = stmt
        .query_map(params.as_slice(), map_prompt_row)
        .map_err(|e| format!("解析提示词失败: {}", e))?;
    let mut prompts = Vec::new();

    for row in rows {
        prompts.push(row.map_err(|e| format!("解析提示词失败: {}", e))?);
    }

    Ok(prompts)
}

#[derive(Default)]
struct ParsedSearch {
    terms: Vec<String>,
    tags: Vec<String>,
    category_name: Option<String>,
}

fn parse_search_query(query: &str) -> ParsedSearch {
    let mut parsed = ParsedSearch::default();
    for token in query.split_whitespace() {
        if let Some(tag) = token.strip_prefix('#').filter(|value| !value.is_empty()) {
            parsed.tags.push(tag.to_string());
        } else if let Some(category) = token.strip_prefix("cat:").filter(|value| !value.is_empty())
        {
            parsed.category_name = Some(category.to_string());
        } else {
            parsed.terms.push(token.to_string());
        }
    }
    parsed
}

/// 创建提示词
pub fn create_prompt(conn: &Connection, input: &CreatePromptInput) -> Result<Prompt, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let tags_json = serde_json::to_string(&input.tags).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO prompts (id, title, content, category_id, tags, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, input.title, input.content, input.category_id, tags_json, now, now],
    )
    .map_err(|e| format!("创建提示词失败: {}", e))?;

    Ok(Prompt {
        id,
        title: input.title.clone(),
        content: input.content.clone(),
        category_id: input.category_id.clone(),
        tags: input.tags.clone(),
        created_at: now.clone(),
        updated_at: now,
        pinned: false,
        last_used_at: None,
        use_count: 0,
    })
}

/// 更新提示词
pub fn update_prompt(conn: &Connection, input: &UpdatePromptInput) -> Result<Prompt, String> {
    let now = chrono::Utc::now().to_rfc3339();

    // 先获取现有数据
    let existing: Prompt = conn
        .query_row(
            "SELECT id, title, content, category_id, tags, created_at, updated_at, pinned, last_used_at, use_count FROM prompts WHERE id = ?1",
            params![input.id],
            map_prompt_row,
        )
        .map_err(|e| format!("查找提示词失败: {}", e))?;

    // 合并更新
    let title = input.title.as_ref().unwrap_or(&existing.title);
    let content = input.content.as_ref().unwrap_or(&existing.content);
    let category_id = input.category_id.as_ref().unwrap_or(&existing.category_id);
    let tags = input.tags.as_ref().unwrap_or(&existing.tags);
    let tags_json = serde_json::to_string(tags).map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE prompts SET title = ?1, content = ?2, category_id = ?3, tags = ?4, updated_at = ?5 WHERE id = ?6",
        params![title, content, category_id, tags_json, now, input.id],
    )
    .map_err(|e| format!("更新提示词失败: {}", e))?;

    Ok(Prompt {
        id: input.id.clone(),
        title: title.clone(),
        content: content.clone(),
        category_id: category_id.clone(),
        tags: tags.clone(),
        created_at: existing.created_at,
        updated_at: now,
        pinned: existing.pinned,
        last_used_at: existing.last_used_at,
        use_count: existing.use_count,
    })
}

pub fn set_prompt_pinned(conn: &Connection, id: &str, pinned: bool) -> Result<Prompt, String> {
    conn.execute(
        "UPDATE prompts SET pinned = ?1 WHERE id = ?2",
        params![pinned, id],
    )
    .map_err(|e| format!("更新置顶状态失败: {}", e))?;
    get_prompt_by_id(conn, id)
}

pub fn record_prompt_use(conn: &Connection, id: &str) -> Result<Prompt, String> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE prompts SET last_used_at = ?1, use_count = use_count + 1 WHERE id = ?2",
        params![now, id],
    )
    .map_err(|e| format!("记录提示词使用失败: {}", e))?;
    get_prompt_by_id(conn, id)
}

fn get_prompt_by_id(conn: &Connection, id: &str) -> Result<Prompt, String> {
    conn.query_row(
        "SELECT id, title, content, category_id, tags, created_at, updated_at, pinned, last_used_at, use_count FROM prompts WHERE id = ?1",
        params![id],
        map_prompt_row,
    )
    .map_err(|e| format!("查找提示词失败: {}", e))
}

/// 删除提示词
pub fn delete_prompt(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM prompts WHERE id = ?1", params![id])
        .map_err(|e| format!("删除提示词失败: {}", e))?;
    Ok(())
}

/// 获取所有标签
pub fn get_all_tags(conn: &Connection) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT DISTINCT tags FROM prompts")
        .map_err(|e| format!("查询标签失败: {}", e))?;

    let mut tag_set = std::collections::HashSet::new();
    let rows = stmt
        .query_map([], |row| {
            let tags_str: String = row.get(0)?;
            Ok(tags_str)
        })
        .map_err(|e| e.to_string())?;

    for row in rows {
        let tags_str = row.map_err(|e| format!("读取标签失败: {}", e))?;
        let tags = serde_json::from_str::<Vec<String>>(&tags_str)
            .map_err(|e| format!("解析标签失败: {}", e))?;

        for tag in tags {
            tag_set.insert(tag);
        }
    }

    let mut tags: Vec<String> = tag_set.into_iter().collect();
    tags.sort();
    Ok(tags)
}

// ============ 分类操作 ============

/// 获取所有分类（带提示词数量）
pub fn get_categories(conn: &Connection) -> Result<Vec<CategoryWithCount>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT c.id, c.name, c.sort_order, c.created_at, COUNT(p.id) as count
             FROM categories c
             LEFT JOIN prompts p ON c.id = p.category_id
             GROUP BY c.id
             ORDER BY c.sort_order",
        )
        .map_err(|e| format!("查询分类失败: {}", e))?;

    let categories = stmt
        .query_map([], |row| {
            Ok(CategoryWithCount {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
                created_at: row.get(3)?,
                count: row.get(4)?,
            })
        })
        .map_err(|e| format!("解析分类失败: {}", e))?
        .filter_map(|c| c.ok())
        .collect();

    Ok(categories)
}

/// 创建分类
pub fn create_category(conn: &Connection, input: &CreateCategoryInput) -> Result<Category, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    // 获取最大排序值
    let max_sort: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), 0) FROM categories",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let sort_order = max_sort + 1;

    conn.execute(
        "INSERT INTO categories (id, name, sort_order, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![id, input.name, sort_order, now],
    )
    .map_err(|e| format!("创建分类失败: {}", e))?;

    Ok(Category {
        id,
        name: input.name.clone(),
        sort_order,
        created_at: now,
    })
}

/// 更新分类
pub fn update_category(conn: &Connection, input: &UpdateCategoryInput) -> Result<Category, String> {
    let existing = conn
        .query_row(
            "SELECT id, name, sort_order, created_at FROM categories WHERE id = ?1",
            params![input.id],
            |row| {
                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    sort_order: row.get(2)?,
                    created_at: row.get(3)?,
                })
            },
        )
        .map_err(|e| format!("查找分类失败: {}", e))?;

    let name = input.name.as_ref().unwrap_or(&existing.name);
    let sort_order = input.sort_order.unwrap_or(existing.sort_order);

    conn.execute(
        "UPDATE categories SET name = ?1, sort_order = ?2 WHERE id = ?3",
        params![name, sort_order, input.id],
    )
    .map_err(|e| format!("更新分类失败: {}", e))?;

    Ok(Category {
        id: input.id.clone(),
        name: name.clone(),
        sort_order,
        created_at: existing.created_at,
    })
}

/// 删除分类
pub fn delete_category(conn: &Connection, id: &str) -> Result<(), String> {
    // 先将该分类下的提示词设为未分类
    conn.execute(
        "UPDATE prompts SET category_id = NULL WHERE category_id = ?1",
        params![id],
    )
    .map_err(|e| format!("解除提示词分类关联失败: {}", e))?;

    conn.execute("DELETE FROM categories WHERE id = ?1", params![id])
        .map_err(|e| format!("删除分类失败: {}", e))?;

    Ok(())
}

/// 获取未分类提示词数量
pub fn get_uncategorized_count(conn: &Connection) -> Result<i32, String> {
    let count: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM prompts WHERE category_id IS NULL",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(count)
}

// ============ 导入导出 ============

/// 导出所有数据
pub fn export_all_data(conn: &Connection) -> Result<ExportData, String> {
    let categories = get_categories(conn)?;
    let categories: Vec<Category> = categories
        .into_iter()
        .map(|c| Category {
            id: c.id,
            name: c.name,
            sort_order: c.sort_order,
            created_at: c.created_at,
        })
        .collect();

    let prompts = get_prompts(conn, None, None, None)?;

    Ok(ExportData {
        version: "1.1".to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        categories,
        prompts,
    })
}

/// 预览导入结果，不修改数据库
pub fn preview_import_data(conn: &Connection, data: &ExportData) -> Result<ImportPreview, String> {
    let existing_categories = get_categories(conn)?;
    let mut seen_category_ids: std::collections::HashSet<String> = existing_categories
        .iter()
        .map(|category| category.id.clone())
        .collect();
    let mut seen_category_names: std::collections::HashSet<String> = existing_categories
        .iter()
        .map(|category| category.name.trim().to_lowercase())
        .collect();
    let mut new_categories = 0;
    for category in &data.categories {
        let normalized_name = category.name.trim().to_lowercase();
        if !seen_category_ids.contains(&category.id)
            && !seen_category_names.contains(&normalized_name)
        {
            new_categories += 1;
            seen_category_ids.insert(category.id.clone());
            seen_category_names.insert(normalized_name);
        }
    }

    let existing_prompts = get_prompts(conn, None, None, None)?;
    let mut seen_prompt_ids: std::collections::HashSet<String> = existing_prompts
        .iter()
        .map(|prompt| prompt.id.clone())
        .collect();
    let mut seen_prompts: Vec<(String, String, String)> = existing_prompts
        .iter()
        .map(|prompt| {
            (
                prompt.id.clone(),
                prompt.title.trim().to_lowercase(),
                prompt.content.trim().to_string(),
            )
        })
        .collect();
    let mut skipped_by_id = 0;
    let mut possible_duplicates = Vec::new();

    for incoming in &data.prompts {
        if !seen_prompt_ids.insert(incoming.id.clone()) {
            skipped_by_id += 1;
            continue;
        }

        let incoming_title = incoming.title.trim().to_lowercase();
        if let Some((existing_id, _, existing_content)) = seen_prompts
            .iter()
            .find(|(_, title, _)| title == &incoming_title)
        {
            let reason = if existing_content == incoming.content.trim() {
                "标题和内容相同"
            } else {
                "标题相同"
            };
            possible_duplicates.push(ImportDuplicate {
                incoming_id: incoming.id.clone(),
                existing_id: existing_id.clone(),
                title: incoming.title.clone(),
                reason: reason.to_string(),
            });
        }
        seen_prompts.push((
            incoming.id.clone(),
            incoming_title,
            incoming.content.trim().to_string(),
        ));
    }

    Ok(ImportPreview {
        total_categories: data.categories.len(),
        total_prompts: data.prompts.len(),
        new_categories,
        new_prompts: data.prompts.len().saturating_sub(skipped_by_id),
        skipped_by_id,
        possible_duplicates,
    })
}

/// 导入数据（合并模式，不覆盖已有）
pub fn import_data(conn: &mut Connection, data: &ExportData) -> Result<(usize, usize), String> {
    let tx = conn
        .transaction()
        .map_err(|e| format!("开始导入事务失败: {}", e))?;
    let mut categories_imported = 0;
    let mut prompts_imported = 0;
    let mut category_id_map: HashMap<String, String> = HashMap::new();

    // 导入分类（跳过已存在的，并记录旧 ID 到现有 ID 的映射）
    for cat in &data.categories {
        let existing_id = tx
            .query_row(
                "SELECT id FROM categories WHERE id = ?1 OR name = ?2 LIMIT 1",
                params![cat.id, cat.name],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|e| format!("检查分类是否已存在失败: {}", e))?;

        match existing_id {
            Some(existing_id) => {
                category_id_map.insert(cat.id.clone(), existing_id);
            }
            None => {
                tx.execute(
                    "INSERT INTO categories (id, name, sort_order, created_at) VALUES (?1, ?2, ?3, ?4)",
                    params![cat.id, cat.name, cat.sort_order, cat.created_at],
                )
                .map_err(|e| format!("导入分类失败: {}", e))?;
                category_id_map.insert(cat.id.clone(), cat.id.clone());
                categories_imported += 1;
            }
        }
    }

    // 导入提示词（跳过已存在的，并重映射分类）
    for prompt in &data.prompts {
        let exists: bool = tx
            .query_row(
                "SELECT COUNT(*) > 0 FROM prompts WHERE id = ?1",
                params![prompt.id],
                |row| row.get(0),
            )
            .map_err(|e| format!("检查提示词是否已存在失败: {}", e))?;

        if !exists {
            let category_id = match prompt.category_id.as_ref() {
                Some(category_id) => {
                    if let Some(mapped_id) = category_id_map.get(category_id) {
                        Some(mapped_id.clone())
                    } else {
                        tx.query_row(
                            "SELECT id FROM categories WHERE id = ?1",
                            params![category_id],
                            |row| row.get::<_, String>(0),
                        )
                        .optional()
                        .map_err(|e| format!("检查提示词分类是否存在失败: {}", e))?
                    }
                }
                None => None,
            };

            let tags_json = serde_json::to_string(&prompt.tags).map_err(|e| e.to_string())?;
            tx.execute(
                "INSERT INTO prompts (id, title, content, category_id, tags, created_at, updated_at, pinned, last_used_at, use_count) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![prompt.id, prompt.title, prompt.content, category_id, tags_json, prompt.created_at, prompt.updated_at, prompt.pinned, prompt.last_used_at, prompt.use_count],
            )
            .map_err(|e| format!("导入提示词失败: {}", e))?;
            prompts_imported += 1;
        }
    }

    tx.commit()
        .map_err(|e| format!("提交导入事务失败: {}", e))?;

    Ok((categories_imported, prompts_imported))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("failed to open test database");
        conn.execute_batch("PRAGMA foreign_keys=ON;")
            .expect("failed to enable foreign keys");
        conn.execute_batch(
            "CREATE TABLE categories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                sort_order INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            );

            CREATE TABLE prompts (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                category_id TEXT,
                tags TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                pinned INTEGER NOT NULL DEFAULT 0,
                last_used_at TEXT,
                use_count INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
            );",
        )
        .expect("failed to create schema");
        conn
    }

    fn sample_export_data() -> ExportData {
        ExportData {
            version: "1.0".to_string(),
            exported_at: "2026-04-02T00:00:00Z".to_string(),
            categories: vec![Category {
                id: "imported-cat".to_string(),
                name: "共享分类".to_string(),
                sort_order: 1,
                created_at: "2026-04-02T00:00:00Z".to_string(),
            }],
            prompts: vec![Prompt {
                id: "imported-prompt".to_string(),
                title: "导入提示词".to_string(),
                content: "content".to_string(),
                category_id: Some("imported-cat".to_string()),
                tags: vec!["tag-a".to_string()],
                created_at: "2026-04-02T00:00:00Z".to_string(),
                updated_at: "2026-04-02T00:00:00Z".to_string(),
                pinned: false,
                last_used_at: None,
                use_count: 0,
            }],
        }
    }

    fn insert_test_prompt(
        conn: &Connection,
        id: &str,
        title: &str,
        content: &str,
        category_id: Option<&str>,
        tags: &[&str],
    ) {
        let tags = serde_json::to_string(tags).expect("failed to serialize tags");
        conn.execute(
            "INSERT INTO prompts (id, title, content, category_id, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id,
                title,
                content,
                category_id,
                tags,
                "2026-04-02T00:00:00Z",
                "2026-04-02T00:00:00Z"
            ],
        )
        .expect("failed to insert test prompt");
    }

    #[test]
    fn migrates_existing_prompt_table_with_usage_defaults() {
        let conn = Connection::open_in_memory().expect("failed to open database");
        conn.execute_batch(
            "CREATE TABLE prompts (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                category_id TEXT,
                tags TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );",
        )
        .expect("failed to create legacy schema");

        migrate_prompt_usage_columns(&conn).expect("migration should succeed");
        insert_test_prompt(&conn, "legacy", "旧数据", "content", None, &[]);
        let prompt = get_prompt_by_id(&conn, "legacy").expect("prompt should load");

        assert!(!prompt.pinned);
        assert_eq!(prompt.last_used_at, None);
        assert_eq!(prompt.use_count, 0);
    }

    #[test]
    fn legacy_export_json_defaults_usage_fields() {
        let json = r#"{
            "version":"1.0",
            "exported_at":"2026-04-02T00:00:00Z",
            "categories":[],
            "prompts":[{
                "id":"legacy",
                "title":"旧导出",
                "content":"content",
                "category_id":null,
                "tags":[],
                "created_at":"2026-04-02T00:00:00Z",
                "updated_at":"2026-04-02T00:00:00Z"
            }]
        }"#;

        let data: ExportData = serde_json::from_str(json).expect("legacy export should parse");
        let prompt = &data.prompts[0];
        assert!(!prompt.pinned);
        assert_eq!(prompt.last_used_at, None);
        assert_eq!(prompt.use_count, 0);
    }

    #[test]
    fn search_supports_weighting_tag_and_category_syntax() {
        let conn = test_conn();
        conn.execute(
            "INSERT INTO categories (id, name, sort_order, created_at) VALUES (?1, ?2, 1, ?3)",
            params!["dev", "开发", "2026-04-02T00:00:00Z"],
        )
        .expect("failed to insert category");
        insert_test_prompt(
            &conn,
            "title",
            "Rust 指南",
            "ordinary",
            Some("dev"),
            &["guide"],
        );
        insert_test_prompt(
            &conn,
            "tag",
            "标签命中",
            "ordinary",
            None,
            &["rust", "backend"],
        );
        insert_test_prompt(
            &conn,
            "content",
            "内容命中",
            "learn rust here",
            None,
            &["backend"],
        );

        let ranked = get_prompts(&conn, None, Some("rust"), None).expect("search should work");
        assert_eq!(
            ranked.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(),
            vec!["title", "tag", "content"]
        );

        let tagged =
            get_prompts(&conn, None, Some("#backend"), None).expect("tag search should work");
        assert_eq!(tagged.len(), 2);

        let categorized =
            get_prompts(&conn, None, Some("cat:开发"), None).expect("category search should work");
        assert_eq!(categorized.len(), 1);
        assert_eq!(categorized[0].id, "title");
    }

    #[test]
    fn pinning_and_recording_use_update_prompt_state() {
        let conn = test_conn();
        insert_test_prompt(&conn, "used", "常用提示词", "content", None, &[]);

        let pinned = set_prompt_pinned(&conn, "used", true).expect("pin should succeed");
        assert!(pinned.pinned);
        let first = record_prompt_use(&conn, "used").expect("first use should succeed");
        let second = record_prompt_use(&conn, "used").expect("second use should succeed");

        assert_eq!(first.use_count, 1);
        assert_eq!(second.use_count, 2);
        assert!(second.last_used_at.is_some());
        assert!(second.pinned);
    }

    #[test]
    fn import_maps_prompt_to_existing_category_with_same_name() {
        let mut conn = test_conn();
        conn.execute(
            "INSERT INTO categories (id, name, sort_order, created_at) VALUES (?1, ?2, ?3, ?4)",
            params!["local-cat", "共享分类", 1, "2026-04-02T00:00:00Z"],
        )
        .expect("failed to insert existing category");

        let data = sample_export_data();
        let (categories_imported, prompts_imported) =
            import_data(&mut conn, &data).expect("import should succeed");

        assert_eq!(categories_imported, 0);
        assert_eq!(prompts_imported, 1);

        let prompt_category_id: Option<String> = conn
            .query_row(
                "SELECT category_id FROM prompts WHERE id = ?1",
                params!["imported-prompt"],
                |row| row.get(0),
            )
            .expect("failed to load imported prompt category");

        assert_eq!(prompt_category_id.as_deref(), Some("local-cat"));
    }

    #[test]
    fn preview_import_reports_id_skips_and_title_duplicates_without_writing() {
        let conn = test_conn();
        conn.execute(
            "INSERT INTO categories (id, name, sort_order, created_at) VALUES (?1, ?2, ?3, ?4)",
            params!["local-cat", "共享分类", 1, "2026-04-02T00:00:00Z"],
        )
        .expect("failed to insert existing category");
        conn.execute(
            "INSERT INTO prompts (id, title, content, category_id, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                "imported-prompt",
                "已有标题",
                "existing content",
                "local-cat",
                "[]",
                "2026-04-02T00:00:00Z",
                "2026-04-02T00:00:00Z"
            ],
        )
        .expect("failed to insert existing prompt");

        let mut data = sample_export_data();
        data.prompts[0].title = "ID 冲突".to_string();
        data.prompts.push(Prompt {
            id: "new-prompt".to_string(),
            title: "已有标题".to_string(),
            content: "different content".to_string(),
            category_id: Some("imported-cat".to_string()),
            tags: vec![],
            created_at: "2026-04-02T00:00:00Z".to_string(),
            updated_at: "2026-04-02T00:00:00Z".to_string(),
            pinned: false,
            last_used_at: None,
            use_count: 0,
        });

        let preview = preview_import_data(&conn, &data).expect("preview should succeed");

        assert_eq!(preview.new_categories, 0);
        assert_eq!(preview.total_prompts, 2);
        assert_eq!(preview.new_prompts, 1);
        assert_eq!(preview.skipped_by_id, 1);
        assert_eq!(preview.possible_duplicates.len(), 1);
        assert_eq!(preview.possible_duplicates[0].reason, "标题相同");
        let prompt_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM prompts", [], |row| row.get(0))
            .expect("failed to count prompts");
        assert_eq!(prompt_count, 1);
    }

    #[test]
    fn import_rolls_back_all_changes_when_prompt_insert_fails() {
        let mut conn = test_conn();
        conn.execute_batch(
            "CREATE TRIGGER fail_bad_prompt
             BEFORE INSERT ON prompts
             WHEN NEW.title = 'trigger-fail'
             BEGIN
               SELECT RAISE(ABORT, 'forced failure');
             END;",
        )
        .expect("failed to create test trigger");

        let data = ExportData {
            version: "1.0".to_string(),
            exported_at: "2026-04-02T00:00:00Z".to_string(),
            categories: vec![Category {
                id: "cat-a".to_string(),
                name: "事务分类".to_string(),
                sort_order: 1,
                created_at: "2026-04-02T00:00:00Z".to_string(),
            }],
            prompts: vec![
                Prompt {
                    id: "prompt-ok".to_string(),
                    title: "ok".to_string(),
                    content: "content".to_string(),
                    category_id: Some("cat-a".to_string()),
                    tags: vec!["t1".to_string()],
                    created_at: "2026-04-02T00:00:00Z".to_string(),
                    updated_at: "2026-04-02T00:00:00Z".to_string(),
                    pinned: false,
                    last_used_at: None,
                    use_count: 0,
                },
                Prompt {
                    id: "prompt-bad".to_string(),
                    title: "trigger-fail".to_string(),
                    content: "content".to_string(),
                    category_id: Some("cat-a".to_string()),
                    tags: vec!["t2".to_string()],
                    created_at: "2026-04-02T00:00:00Z".to_string(),
                    updated_at: "2026-04-02T00:00:00Z".to_string(),
                    pinned: false,
                    last_used_at: None,
                    use_count: 0,
                },
            ],
        };

        let err = import_data(&mut conn, &data).expect_err("import should fail");
        assert!(err.contains("导入提示词失败"));

        let category_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))
            .expect("failed to count categories");
        let prompt_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM prompts", [], |row| row.get(0))
            .expect("failed to count prompts");

        assert_eq!(category_count, 0);
        assert_eq!(prompt_count, 0);
    }

    #[test]
    fn get_prompts_returns_error_when_tags_json_is_invalid() {
        let conn = test_conn();
        conn.execute(
            "INSERT INTO prompts (id, title, content, category_id, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                "bad-tags",
                "坏标签",
                "content",
                Option::<String>::None,
                "not-json",
                "2026-04-02T00:00:00Z",
                "2026-04-02T00:00:00Z"
            ],
        )
        .expect("failed to insert malformed prompt");

        let err = get_prompts(&conn, None, None, None).expect_err("query should fail");
        assert!(err.contains("解析提示词失败"));
    }

    #[test]
    fn repair_orphaned_prompts_nulls_missing_category_ids() {
        let conn = test_conn();
        conn.execute_batch("PRAGMA foreign_keys=OFF;")
            .expect("failed to disable foreign keys");
        conn.execute(
            "INSERT INTO prompts (id, title, content, category_id, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                "orphaned-prompt",
                "孤儿提示词",
                "content",
                "missing-cat",
                "[]",
                "2026-04-02T00:00:00Z",
                "2026-04-02T00:00:00Z"
            ],
        )
        .expect("failed to insert orphaned prompt");
        conn.execute_batch("PRAGMA foreign_keys=ON;")
            .expect("failed to re-enable foreign keys");

        repair_orphaned_prompts(&conn).expect("repair should succeed");

        let category_id: Option<String> = conn
            .query_row(
                "SELECT category_id FROM prompts WHERE id = ?1",
                params!["orphaned-prompt"],
                |row| row.get(0),
            )
            .expect("failed to load repaired prompt");

        assert_eq!(category_id, None);
    }
}
