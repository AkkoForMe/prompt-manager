// 数据模型定义

use serde::{Deserialize, Serialize};

/// 提示词
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category_id: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub last_used_at: Option<String>,
    #[serde(default)]
    pub use_count: i64,
}

/// 分类
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub sort_order: i32,
    pub created_at: String,
}

/// 分类统计（带提示词数量）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryWithCount {
    pub id: String,
    pub name: String,
    pub sort_order: i32,
    pub created_at: String,
    pub count: i32,
}

/// 创建提示词输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePromptInput {
    pub title: String,
    pub content: String,
    pub category_id: Option<String>,
    pub tags: Vec<String>,
}

/// 更新提示词输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePromptInput {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<Option<String>>,
    pub tags: Option<Vec<String>>,
}

/// 创建分类输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryInput {
    pub name: String,
}

/// 更新分类输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategoryInput {
    pub id: String,
    pub name: Option<String>,
    pub sort_order: Option<i32>,
}

/// 导入导出数据格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub version: String,
    pub exported_at: String,
    pub categories: Vec<Category>,
    pub prompts: Vec<Prompt>,
}

/// 导入时发现的疑似重复提示词
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDuplicate {
    pub incoming_id: String,
    pub existing_id: String,
    pub title: String,
    pub reason: String,
}

/// 导入预览，不会修改数据库
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPreview {
    pub total_categories: usize,
    pub total_prompts: usize,
    pub new_categories: usize,
    pub new_prompts: usize,
    pub skipped_by_id: usize,
    pub possible_duplicates: Vec<ImportDuplicate>,
}
