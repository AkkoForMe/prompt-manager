<template>
  <aside class="sidebar">
    <div class="sidebar__logo">
      <Sparkles class="sidebar__logo-icon" :size="18" />
      <span class="sidebar__logo-text">Prompt Manager</span>
    </div>

    <nav class="sidebar__nav">
      <!-- 全部 -->
      <button
        class="sidebar__item"
        :class="{ 'sidebar__item--active': activeCategory === null }"
        @click="$emit('select-category', null)"
      >
        <LayoutGrid :size="16" />
        <span class="sidebar__item-name">全部</span>
        <span class="sidebar__item-count">{{ totalCount }}</span>
      </button>

      <!-- 未分类 -->
      <button
        class="sidebar__item"
        :class="{ 'sidebar__item--active': activeCategory === 'uncategorized' }"
        @click="$emit('select-category', 'uncategorized')"
      >
        <CircleMinus :size="16" />
        <span class="sidebar__item-name">未分类</span>
        <span class="sidebar__item-count">{{ uncategorizedCount }}</span>
      </button>

      <div class="sidebar__divider"></div>

      <!-- 自定义分类 -->
      <div class="sidebar__section">
        <button class="sidebar__section-header" @click="collapsed = !collapsed">
          <span class="sidebar__section-title">分类</span>
          <ChevronDown
            :size="14"
            class="sidebar__collapse-icon"
            :class="{ 'sidebar__collapse-icon--collapsed': collapsed }"
          />
        </button>

        <Transition name="collapse">
          <div v-show="!collapsed" class="sidebar__categories">
            <div
              v-for="cat in categories"
              :key="cat.id"
              class="sidebar__item-wrapper"
            >
              <button
                class="sidebar__item"
                :class="{ 'sidebar__item--active': activeCategory === cat.id }"
                @click="$emit('select-category', cat.id)"
                @contextmenu.prevent="showContextMenu($event, cat)"
              >
                <Folder :size="16" />
                <span class="sidebar__item-name">{{ cat.name }}</span>
                <span class="sidebar__item-count">{{ cat.count }}</span>
              </button>
            </div>
          </div>
        </Transition>
      </div>
    </nav>

    <!-- 添加分类 -->
    <div class="sidebar__add">
      <button v-if="!showAddInput" class="sidebar__add-btn" @click="showAddInput = true">
        <Plus :size="14" />
        新分类
      </button>
      <div v-else class="sidebar__add-form">
        <input
          ref="addInputRef"
          v-model="newCategoryName"
          class="sidebar__add-input"
          placeholder="分类名称"
          @keydown.enter="handleAddCategory"
          @keydown.escape="cancelAdd"
          @blur="cancelAdd"
        />
      </div>
    </div>

    <!-- 右键菜单 -->
    <Teleport to="body">
      <div
        v-if="contextMenu.visible"
        class="context-menu"
        :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }"
      >
        <button class="context-menu__item" @click="startRename">重命名</button>
        <button class="context-menu__item context-menu__item--danger" @click="handleDeleteCategory">删除</button>
      </div>
    </Teleport>

    <!-- 重命名对话框 -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="renameVisible" class="modal-overlay" @click.self="renameVisible = false">
          <div class="rename-dialog">
            <h4 class="rename-dialog__title">重命名分类</h4>
            <input
              v-model="renameValue"
              class="rename-dialog__input"
              @keydown.enter="confirmRename"
              @keydown.escape="renameVisible = false"
            />
            <div class="rename-dialog__actions">
              <button class="btn btn--secondary" @click="renameVisible = false">取消</button>
              <button class="btn btn--primary" @click="confirmRename">确认</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </aside>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue'
import { ChevronDown, CircleMinus, Folder, LayoutGrid, Plus, Sparkles } from 'lucide-vue-next'
import type { CategoryWithCount } from '../types'

const props = defineProps<{
  categories: CategoryWithCount[]
  activeCategory: string | null
  totalCount: number
  uncategorizedCount: number
}>()

const emit = defineEmits<{
  'select-category': [id: string | null]
  'add-category': [name: string]
  'rename-category': [id: string, name: string]
  'delete-category': [id: string]
}>()

const collapsed = ref(false)
const showAddInput = ref(false)
const newCategoryName = ref('')
const addInputRef = ref<HTMLInputElement | null>(null)

// 右键菜单状态
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  category: null as CategoryWithCount | null,
})

// 重命名状态
const renameVisible = ref(false)
const renameValue = ref('')

const totalCount = computed(() => props.totalCount)
const uncategorizedCount = computed(() => props.uncategorizedCount)

// 显示添加输入框时自动聚焦
watch(showAddInput, async (val) => {
  if (val) {
    await nextTick()
    addInputRef.value?.focus()
  }
})

function handleAddCategory() {
  const name = newCategoryName.value.trim()
  if (name) {
    emit('add-category', name)
    newCategoryName.value = ''
    showAddInput.value = false
  }
}

function cancelAdd() {
  showAddInput.value = false
  newCategoryName.value = ''
}

function showContextMenu(event: MouseEvent, cat: CategoryWithCount) {
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    category: cat,
  }
}

function hideContextMenu() {
  contextMenu.value.visible = false
}

function startRename() {
  if (contextMenu.value.category) {
    renameValue.value = contextMenu.value.category.name
    renameVisible.value = true
  }
  hideContextMenu()
}

function confirmRename() {
  const name = renameValue.value.trim()
  if (name && contextMenu.value.category) {
    emit('rename-category', contextMenu.value.category.id, name)
    renameVisible.value = false
  }
}

function handleDeleteCategory() {
  if (contextMenu.value.category) {
    emit('delete-category', contextMenu.value.category.id)
  }
  hideContextMenu()
}

// 点击其他地方关闭右键菜单
function handleClickOutside() {
  hideContextMenu()
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.sidebar {
  width: 220px;
  min-width: 220px;
  background: #fafafa;
  border-right: 1px solid #f0f0f0;
  display: flex;
  flex-direction: column;
  user-select: none;
}

.sidebar__logo {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 20px 16px 16px;
  font-size: 15px;
  font-weight: 700;
  color: #1a1a2e;
}

.sidebar__logo-icon {
  color: #6366f1;
}
.sidebar svg { flex: none; stroke-width: 1.75; }

.sidebar__nav {
  flex: 1;
  padding: 4px 8px;
  overflow-y: auto;
}

.sidebar__item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  background: none;
  border-radius: 8px;
  font-size: 13px;
  color: #555;
  cursor: pointer;
  transition: all 0.15s;
  text-align: left;
}

.sidebar__item:hover {
  background: #f0f0f0;
}

.sidebar__item--active {
  background: #eef2ff;
  color: #4f46e5;
}

.sidebar__item--active:hover {
  background: #e0e7ff;
}

.sidebar__item-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sidebar__item-count {
  font-size: 11px;
  color: #bbb;
  min-width: 20px;
  text-align: right;
}

.sidebar__item--active .sidebar__item-count {
  color: #818cf8;
}

.sidebar__divider {
  height: 1px;
  background: #eee;
  margin: 8px 12px;
}

.sidebar__section {
  margin-top: 4px;
}

.sidebar__section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 6px 12px;
  border: none;
  background: none;
  font-size: 11px;
  font-weight: 600;
  color: #999;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  cursor: pointer;
  border-radius: 6px;
  transition: background 0.15s;
}

.sidebar__section-header:hover {
  background: #f0f0f0;
}

.sidebar__collapse-icon {
  transition: transform 0.2s;
}

.sidebar__collapse-icon--collapsed {
  transform: rotate(-90deg);
}

.sidebar__categories {
  overflow: hidden;
}

.sidebar__item-wrapper {
  position: relative;
}

.sidebar__add {
  padding: 8px;
  border-top: 1px solid #f0f0f0;
}

.sidebar__add-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  background: none;
  border-radius: 8px;
  font-size: 13px;
  color: #999;
  cursor: pointer;
  transition: all 0.15s;
}

.sidebar__add-btn:hover {
  background: #f0f0f0;
  color: #666;
}

.sidebar__add-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-size: 13px;
  outline: none;
  transition: border-color 0.2s;
}

.sidebar__add-input:focus {
  border-color: #6366f1;
}

/* 右键菜单 */
.context-menu {
  position: fixed;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  padding: 4px 0;
  z-index: 6000;
  min-width: 120px;
}

.context-menu__item {
  display: block;
  width: 100%;
  padding: 8px 16px;
  border: none;
  background: none;
  font-size: 13px;
  color: #333;
  text-align: left;
  cursor: pointer;
  transition: background 0.15s;
}

.context-menu__item:hover {
  background: #f5f5f5;
}

.context-menu__item--danger {
  color: #ef4444;
}

.context-menu__item--danger:hover {
  background: #fef2f2;
}

/* 重命名对话框 */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 5000;
}

.rename-dialog {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  width: 320px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.rename-dialog__title {
  font-size: 15px;
  font-weight: 600;
  margin: 0 0 16px;
  color: #1a1a2e;
}

.rename-dialog__input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-size: 14px;
  margin-bottom: 16px;
  outline: none;
}

.rename-dialog__input:focus {
  border-color: #6366f1;
}

.rename-dialog__actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

/* 动画 */
.collapse-enter-active,
.collapse-leave-active {
  transition: all 0.2s ease;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  max-height: 0;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
