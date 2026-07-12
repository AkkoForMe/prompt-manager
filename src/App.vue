<template>
  <div class="app">
    <!-- 左侧边栏 -->
    <Sidebar
      :categories="categories"
      :active-category="activeCategory"
      :total-count="totalCount"
      :uncategorized-count="uncategorizedCount"
      @select-category="handleSelectCategory"
      @add-category="handleAddCategory"
      @rename-category="handleRenameCategory"
      @delete-category="handleDeleteCategory"
    />

    <!-- 主内容区 -->
    <main class="app__main">
      <!-- 顶部工具栏 -->
      <header v-if="currentPage === 'library'" class="toolbar">
        <div class="toolbar__search">
          <Search class="toolbar__search-icon" :size="16" />
          <input
            v-model="searchQuery"
            class="toolbar__search-input"
            placeholder="搜索提示词..."
            @input="handleSearch"
          />
          <SearchHelp @example="useSearchExample" />
          <button
            v-if="activeTag"
            class="toolbar__active-tag"
            @click="clearTagFilter"
          >
            #{{ activeTag }} <X :size="12" />
          </button>
        </div>
        <div class="toolbar__views" aria-label="列表视图">
          <button
            v-for="option in viewOptions"
            :key="option.value"
            :class="{ 'toolbar__view--active': viewMode === option.value }"
            @click="viewMode = option.value"
          >
            {{ option.label }}
          </button>
        </div>
        <div class="toolbar__actions">
          <button class="btn btn--ghost" @click="currentPage = 'stats'">
            <BarChart3 :size="16" />
            统计
          </button>
          <button class="btn btn--ghost" @click="handleImport">
            <Download :size="16" />
            导入
          </button>
          <button class="btn btn--ghost" @click="handleExport">
            <Upload :size="16" />
            导出
          </button>
        </div>
      </header>

      <!-- 提示词列表 -->
      <PromptList
        v-if="currentPage === 'library'"
        :prompts="filteredPrompts"
        :search="searchQuery"
        :mode="viewMode"
        @view="handleView"
        @toggle-pin="handleTogglePin"
        @copy="handleCopy"
        @edit="handleEdit"
        @delete="handleDelete"
        @tag-click="handleTagClick"
      />

      <!-- 新建按钮 -->
      <button v-if="currentPage === 'library'" class="fab" @click="showEditor = true" title="新建提示词">
        <Plus :size="24" />
      </button>
      <StatsPage
        v-else
        :prompts="visiblePrompts"
        @back="currentPage = 'library'"
        @view="handleView"
      />
    </main>

    <!-- 编辑器弹窗 -->
    <PromptEditor
      :visible="showEditor"
      :prompt="editingPrompt"
      :categories="categories"
      :all-tags="allTags"
      @save="handleSavePrompt"
      @close="handleCloseEditor"
    />

    <PromptDetail
      :visible="selectedPrompt !== null"
      :prompt="selectedPrompt"
      @close="selectedPrompt = null"
      @copy="handleCopy"
      @edit="handleEdit"
      @toggle-pin="handleTogglePin"
    />

    <CommandPalette
      :visible="showPalette"
      :prompts="visiblePrompts"
      @close="showPalette = false"
      @copy="handleCopy"
      @edit="handleEdit"
    />

    <ImportPreviewDialog
      :visible="importPreview !== null"
      :preview="importPreview"
      :loading="importing"
      @confirm="confirmImport"
      @cancel="cancelImport"
    />

    <UndoDeleteBar
      :visible="pendingDeletion !== null"
      :title="pendingDeletion?.prompt.title ?? ''"
      @undo="undoDelete"
    />

    <!-- 确认对话框 -->
    <ConfirmDialog
      :visible="showConfirm"
      :title="confirmTitle"
      :message="confirmMessage"
      @confirm="confirmAction"
      @cancel="showConfirm = false"
    />

    <!-- Toast 通知 -->
    <ToastNotification :toasts="toasts" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { BarChart3, Download, Plus, Search, Upload, X } from 'lucide-vue-next'
import Sidebar from './components/Sidebar.vue'
import PromptList from './components/PromptList.vue'
import PromptEditor from './components/PromptEditor.vue'
import ConfirmDialog from './components/ConfirmDialog.vue'
import ToastNotification from './components/ToastNotification.vue'
import PromptDetail from './components/PromptDetail.vue'
import CommandPalette from './components/CommandPalette.vue'
import ImportPreviewDialog from './components/ImportPreviewDialog.vue'
import UndoDeleteBar from './components/UndoDeleteBar.vue'
import StatsPage from './components/StatsPage.vue'
import SearchHelp from './components/SearchHelp.vue'
import type { Prompt, CategoryWithCount, ToastMessage, ImportPreview } from './types'
import {
  getPrompts,
  createPrompt,
  updatePrompt,
  deletePrompt,
  setPromptPinned,
  recordPromptUse,
  getAllTags,
  getCategories,
  createCategory,
  updateCategory,
  deleteCategory,
  exportToJson,
  exportToMarkdown,
  importFromJson,
  previewImportJson,
} from './utils/api'

// ============ 数据状态 ============
const prompts = ref<Prompt[]>([])
const listPrompts = ref<Prompt[]>([])
const categories = ref<CategoryWithCount[]>([])
const allTags = ref<string[]>([])

// ============ UI 状态 ============
const activeCategory = ref<string | null>(null)
const currentPage = ref<'library' | 'stats'>('library')
const activeTag = ref<string | null>(null)
const searchQuery = ref('')
const showEditor = ref(false)
const editingPrompt = ref<Prompt | null>(null)
const showConfirm = ref(false)
const confirmTitle = ref('')
const confirmMessage = ref('')
const confirmCallback = ref<(() => void) | null>(null)
const toasts = ref<ToastMessage[]>([])
const selectedPrompt = ref<Prompt | null>(null)
const showPalette = ref(false)
const importPreview = ref<ImportPreview | null>(null)
const pendingImportPath = ref<string | null>(null)
const importing = ref(false)
const pendingDeletion = ref<{ prompt: Prompt; timer: ReturnType<typeof setTimeout> } | null>(null)
type ViewMode = 'all' | 'pinned' | 'recent' | 'frequent'
const viewMode = ref<ViewMode>('all')
const viewOptions: Array<{ value: ViewMode; label: string }> = [
  { value: 'all', label: '全部' },
  { value: 'pinned', label: '置顶' },
  { value: 'recent', label: '最近' },
  { value: 'frequent', label: '常用' },
]
let toastId = 0
let searchTimer: ReturnType<typeof setTimeout> | null = null
let listRequestId = 0

// ============ 计算属性 ============
const visiblePrompts = computed(() =>
  pendingDeletion.value
    ? prompts.value.filter((prompt) => prompt.id !== pendingDeletion.value?.prompt.id)
    : prompts.value
)
const totalCount = computed(() => visiblePrompts.value.length)
const uncategorizedCount = computed(() =>
  visiblePrompts.value.filter((p) => p.category_id === null).length
)

const filteredPrompts = computed(() => {
  let result = pendingDeletion.value
    ? listPrompts.value.filter((prompt) => prompt.id !== pendingDeletion.value?.prompt.id)
    : [...listPrompts.value]
  if (viewMode.value === 'pinned') result = result.filter((prompt) => prompt.pinned)
  if (viewMode.value === 'recent') {
    result = result
      .filter((prompt) => prompt.last_used_at)
      .sort((a, b) => Date.parse(b.last_used_at!) - Date.parse(a.last_used_at!))
  }
  if (viewMode.value === 'frequent') {
    result = result
      .filter((prompt) => prompt.use_count > 0)
      .sort((a, b) => b.use_count - a.use_count)
  }
  if (viewMode.value === 'all' || viewMode.value === 'pinned') {
    result.sort((a, b) => Number(b.pinned) - Number(a.pinned))
  }
  return result
})

// ============ 数据加载 ============
async function loadData() {
  try {
    const [p, c, t] = await Promise.all([
      getPrompts(),
      getCategories(),
      getAllTags(),
    ])
    prompts.value = p
    listPrompts.value = p
    categories.value = c
    allTags.value = t
    if (activeCategory.value || activeTag.value || searchQuery.value.trim()) {
      await refreshList()
    }
  } catch (e: any) {
    showToast('加载数据失败: ' + e, 'error')
  }
}

// ============ 提示词操作 ============
async function handleSavePrompt(data: {
  title: string
  content: string
  category_id: string | null
  tags: string[]
}) {
  try {
    if (editingPrompt.value) {
      await updatePrompt({
        id: editingPrompt.value.id,
        ...data,
      })
      showToast('提示词已更新', 'success')
    } else {
      await createPrompt(data)
      showToast('提示词已创建', 'success')
    }
    showEditor.value = false
    editingPrompt.value = null
    await loadData()
  } catch (e: any) {
    showToast('保存失败: ' + e, 'error')
  }
}

function handleEdit(prompt: Prompt) {
  selectedPrompt.value = null
  editingPrompt.value = prompt
  showEditor.value = true
}

function handleView(prompt: Prompt) {
  selectedPrompt.value = prompt
}

function handleDelete(prompt: Prompt) {
  if (selectedPrompt.value?.id === prompt.id) selectedPrompt.value = null
  confirmTitle.value = '删除提示词'
  confirmMessage.value = `确定要删除"${prompt.title}"吗？此操作不可撤销。`
  confirmCallback.value = async () => {
    await commitPendingDeletion()
    const timer = setTimeout(() => void commitPendingDeletion(), 5000)
    pendingDeletion.value = { prompt, timer }
  }
  showConfirm.value = true
}

async function commitPendingDeletion() {
  const pending = pendingDeletion.value
  if (!pending) return
  clearTimeout(pending.timer)
  try {
    await deletePrompt(pending.prompt.id)
    prompts.value = prompts.value.filter((prompt) => prompt.id !== pending.prompt.id)
    listPrompts.value = listPrompts.value.filter((prompt) => prompt.id !== pending.prompt.id)
    showToast('提示词已删除', 'success')
  } catch (e: any) {
    showToast('删除失败: ' + e, 'error')
  } finally {
    pendingDeletion.value = null
  }
}

function undoDelete() {
  if (!pendingDeletion.value) return
  clearTimeout(pendingDeletion.value.timer)
  pendingDeletion.value = null
  showToast('已撤销删除', 'info')
}

async function handleCopy(prompt: Prompt) {
  let copied = false
  try {
    const { writeText } = await import('@tauri-apps/plugin-clipboard-manager')
    await writeText(prompt.content)
    copied = true
  } catch {
    try {
      await navigator.clipboard.writeText(prompt.content)
      copied = true
    } catch {
      showToast('复制失败', 'error')
    }
  }
  if (!copied) return
  showToast('已复制到剪贴板', 'success')
  try {
    applyPromptUpdate(await recordPromptUse(prompt.id))
  } catch {
    showToast('内容已复制，但使用记录更新失败', 'info')
  }
}

async function handleTogglePin(prompt: Prompt) {
  try {
    applyPromptUpdate(await setPromptPinned(prompt.id, !prompt.pinned))
  } catch (e: any) {
    showToast('更新置顶状态失败: ' + e, 'error')
  }
}

function applyPromptUpdate(updated: Prompt) {
  const replace = (items: Prompt[]) => items.map((item) => item.id === updated.id ? updated : item)
  prompts.value = replace(prompts.value)
  listPrompts.value = replace(listPrompts.value)
  if (selectedPrompt.value?.id === updated.id) selectedPrompt.value = updated
  if (editingPrompt.value?.id === updated.id) editingPrompt.value = updated
}

function handleCloseEditor() {
  showEditor.value = false
  editingPrompt.value = null
}

function confirmAction() {
  showConfirm.value = false
  confirmCallback.value?.()
  confirmCallback.value = null
}

// ============ 分类操作 ============
function handleSelectCategory(id: string | null) {
  currentPage.value = 'library'
  activeCategory.value = id
  activeTag.value = null
  void refreshList()
}

async function handleAddCategory(name: string) {
  try {
    await createCategory({ name })
    await loadData()
    showToast('分类已创建', 'success')
  } catch (e: any) {
    showToast('创建分类失败: ' + e, 'error')
  }
}

async function handleRenameCategory(id: string, name: string) {
  try {
    await updateCategory({ id, name })
    await loadData()
    showToast('分类已重命名', 'success')
  } catch (e: any) {
    showToast('重命名失败: ' + e, 'error')
  }
}

function handleDeleteCategory(id: string) {
  const cat = categories.value.find((c) => c.id === id)
  confirmTitle.value = '删除分类'
  confirmMessage.value = `确定要删除分类"${cat?.name}"吗？该分类下的提示词将变为未分类。`
  confirmCallback.value = async () => {
    try {
      await deleteCategory(id)
      if (activeCategory.value === id) {
        activeCategory.value = null
      }
      await loadData()
      showToast('分类已删除', 'success')
    } catch (e: any) {
      showToast('删除分类失败: ' + e, 'error')
    }
  }
  showConfirm.value = true
}

// ============ 搜索和标签 ============
function handleSearch() {
  if (searchTimer) clearTimeout(searchTimer)
  searchTimer = setTimeout(() => void refreshList(), 200)
}

function handleTagClick(tag: string) {
  activeTag.value = activeTag.value === tag ? null : tag
  void refreshList()
}

function clearTagFilter() {
  activeTag.value = null
  void refreshList()
}

async function refreshList() {
  const requestId = ++listRequestId
  try {
    const result = await getPrompts({
      categoryId: activeCategory.value,
      search: searchQuery.value.trim() || undefined,
      tag: activeTag.value || undefined,
    })
    if (requestId === listRequestId) listPrompts.value = result
  } catch (e: any) {
    if (requestId === listRequestId) showToast('查询失败: ' + e, 'error')
  }
}

function useSearchExample(query: string) {
  searchQuery.value = query
  void refreshList()
}

// ============ 导入导出 ============
async function handleExport() {
  try {
    const { save } = await import('@tauri-apps/plugin-dialog')
    const path = await save({
      defaultPath: 'prompts-export.json',
      filters: [
        { name: 'JSON', extensions: ['json'] },
        { name: 'Markdown', extensions: ['md'] },
      ],
    })
    if (!path) return

    if (path.endsWith('.md')) {
      await exportToMarkdown(path)
    } else {
      await exportToJson(path)
    }
    showToast('导出成功', 'success')
  } catch (e: any) {
    showToast('导出失败: ' + e, 'error')
  }
}

async function handleImport() {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog')
    const path = await open({
      filters: [{ name: 'JSON', extensions: ['json'] }],
      multiple: false,
    })
    if (!path) return

    const filePath = typeof path === 'string' ? path : (path as any).path
    pendingImportPath.value = filePath
    importPreview.value = await previewImportJson(filePath)
  } catch (e: any) {
    showToast('导入失败: ' + e, 'error')
  }
}

async function confirmImport() {
  if (!pendingImportPath.value || importing.value) return
  importing.value = true
  try {
    const result = await importFromJson(pendingImportPath.value)
    showToast(result, 'success')
    cancelImport()
    await loadData()
  } catch (e: any) {
    showToast('导入失败: ' + e, 'error')
  } finally {
    importing.value = false
  }
}

function cancelImport() {
  importPreview.value = null
  pendingImportPath.value = null
}

// ============ Toast 通知 ============
function showToast(text: string, type: 'success' | 'error' | 'info' = 'info') {
  const id = ++toastId
  toasts.value.push({ id, text, type })
  setTimeout(() => {
    toasts.value = toasts.value.filter((t) => t.id !== id)
  }, 3000)
}

// ============ 初始化 ============
function handleGlobalKeydown(event: KeyboardEvent) {
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k') {
    event.preventDefault()
    showPalette.value = !showPalette.value
  }
  if (event.key === 'Escape' && selectedPrompt.value) selectedPrompt.value = null
}

onMounted(() => {
  void loadData()
  window.addEventListener('keydown', handleGlobalKeydown)
})
onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleGlobalKeydown)
  if (searchTimer) clearTimeout(searchTimer)
  if (pendingDeletion.value) clearTimeout(pendingDeletion.value.timer)
})
</script>

<style scoped>
.app {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: #f8f9fa;
}

.app__main {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
}

/* 工具栏 */
.toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  border-bottom: 1px solid #f0f0f0;
  background: #fff;
}

.toolbar__search {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 12px;
  height: 38px;
  background: #f5f5f5;
  border-radius: 8px;
  transition: all 0.2s;
}

.toolbar__views {
  display: flex;
  flex: none;
  padding: 3px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg);
}

.toolbar__views button {
  min-width: 46px;
  padding: 5px 8px;
  border: 0;
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
}

.toolbar__views .toolbar__view--active {
  background: var(--bg-white);
  color: var(--primary);
  box-shadow: var(--shadow-sm);
  font-weight: 600;
}

.toolbar__search:focus-within {
  background: #fff;
  box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
}

.toolbar__search-icon {
  color: #bbb;
  flex-shrink: 0;
  stroke-width: 1.75;
}

.toolbar__search-input {
  flex: 1;
  border: none;
  background: none;
  outline: none;
  font-size: 14px;
  color: #333;
}

.toolbar__search-input::placeholder {
  color: #bbb;
}

.toolbar__active-tag {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  background: #eef2ff;
  color: #4f46e5;
  border: none;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  white-space: nowrap;
  transition: background 0.15s;
}

.toolbar__active-tag:hover {
  background: #e0e7ff;
}

.toolbar__actions {
  display: flex;
  gap: 4px;
}

/* 悬浮操作按钮 */
.fab {
  position: absolute;
  bottom: 24px;
  right: 24px;
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: #6366f1;
  color: #fff;
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 16px rgba(99, 102, 241, 0.3);
  transition: all 0.2s;
  z-index: 100;
}

.fab:hover {
  transform: scale(1.08);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.4);
}

.fab:active {
  transform: scale(0.95);
}
</style>
