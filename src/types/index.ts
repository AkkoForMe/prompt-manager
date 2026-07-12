// TypeScript 类型定义

/** 提示词 */
export interface Prompt {
  id: string
  title: string
  content: string
  category_id: string | null
  tags: string[]
  created_at: string
  updated_at: string
  pinned: boolean
  last_used_at: string | null
  use_count: number
}

/** 分类 */
export interface Category {
  id: string
  name: string
  sort_order: number
  created_at: string
}

/** 分类（带提示词数量） */
export interface CategoryWithCount extends Category {
  count: number
}

/** 创建提示词输入 */
export interface CreatePromptInput {
  title: string
  content: string
  category_id: string | null
  tags: string[]
}

/** 更新提示词输入 */
export interface UpdatePromptInput {
  id: string
  title?: string
  content?: string
  category_id?: string | null
  tags?: string[]
}

/** 创建分类输入 */
export interface CreateCategoryInput {
  name: string
}

/** 更新分类输入 */
export interface UpdateCategoryInput {
  id: string
  name?: string
  sort_order?: number
}

/** 导出数据格式 */
export interface ExportData {
  version: string
  exported_at: string
  categories: Category[]
  prompts: Prompt[]
}

export interface ImportDuplicate {
  incoming_id: string
  existing_id: string
  title: string
  reason: string
}

export interface ImportPreview {
  total_categories: number
  total_prompts: number
  new_categories: number
  new_prompts: number
  skipped_by_id: number
  possible_duplicates: ImportDuplicate[]
}

/** Toast 消息类型 */
export interface ToastMessage {
  id: number
  text: string
  type: 'success' | 'error' | 'info'
}
