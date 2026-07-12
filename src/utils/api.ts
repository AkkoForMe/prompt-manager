// Tauri IPC 调用封装

import { invoke } from '@tauri-apps/api/core'
import type {
  Prompt,
  Category,
  CategoryWithCount,
  CreatePromptInput,
  UpdatePromptInput,
  CreateCategoryInput,
  UpdateCategoryInput,
  ImportPreview,
} from '../types'

// ============ 提示词 API ============

/** 获取提示词列表 */
export async function getPrompts(params?: {
  categoryId?: string | null
  search?: string
  tag?: string
}): Promise<Prompt[]> {
  return invoke('get_prompts', {
    categoryId: params?.categoryId ?? null,
    search: params?.search ?? null,
    tag: params?.tag ?? null,
  })
}

/** 创建提示词 */
export async function createPrompt(input: CreatePromptInput): Promise<Prompt> {
  return invoke('create_prompt', { input })
}

/** 更新提示词 */
export async function updatePrompt(input: UpdatePromptInput): Promise<Prompt> {
  return invoke('update_prompt', { input })
}

/** 删除提示词 */
export async function deletePrompt(id: string): Promise<void> {
  return invoke('delete_prompt', { id })
}

export async function setPromptPinned(id: string, pinned: boolean): Promise<Prompt> {
  return invoke('set_prompt_pinned', { id, pinned })
}

export async function recordPromptUse(id: string): Promise<Prompt> {
  return invoke('record_prompt_use', { id })
}

/** 获取所有标签 */
export async function getAllTags(): Promise<string[]> {
  return invoke('get_all_tags')
}

// ============ 分类 API ============

/** 获取所有分类 */
export async function getCategories(): Promise<CategoryWithCount[]> {
  return invoke('get_categories')
}

/** 创建分类 */
export async function createCategory(input: CreateCategoryInput): Promise<Category> {
  return invoke('create_category', { input })
}

/** 更新分类 */
export async function updateCategory(input: UpdateCategoryInput): Promise<Category> {
  return invoke('update_category', { input })
}

/** 删除分类 */
export async function deleteCategory(id: string): Promise<void> {
  return invoke('delete_category', { id })
}

/** 获取未分类提示词数量 */
export async function getUncategorizedCount(): Promise<number> {
  return invoke('get_uncategorized_count')
}

// ============ 导入导出 API ============

/** 导出为 JSON */
export async function exportToJson(path: string): Promise<string> {
  return invoke('export_to_json', { path })
}

/** 导出为 Markdown */
export async function exportToMarkdown(path: string): Promise<string> {
  return invoke('export_to_markdown', { path })
}

/** 从 JSON 导入 */
export async function previewImportJson(path: string): Promise<ImportPreview> {
  return invoke('preview_import_json', { path })
}

/** 从 JSON 导入 */
export async function importFromJson(path: string): Promise<string> {
  return invoke('import_from_json', { path })
}
