<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="modal-overlay" @click.self="handleClose">
        <div class="editor">
          <div class="editor__header">
            <h3 class="editor__title">{{ isEdit ? '编辑提示词' : '新建提示词' }}</h3>
            <IconButton label="关闭编辑器" @click="handleClose"><X /></IconButton>
          </div>

          <div class="editor__body">
            <!-- 标题 -->
            <div class="editor__field">
              <label class="editor__label">标题</label>
              <input
                v-model="form.title"
                class="editor__input"
                placeholder="输入提示词标题"
                ref="titleInput"
              />
            </div>

            <!-- 分类 -->
            <div class="editor__field">
              <label class="editor__label">分类</label>
              <select v-model="form.category_id" class="editor__select">
                <option :value="null">未分类</option>
                <option
                  v-for="cat in categories"
                  :key="cat.id"
                  :value="cat.id"
                >
                  {{ cat.name }}
                </option>
              </select>
            </div>

            <!-- 内容 -->
            <div class="editor__field editor__field--flex">
              <label class="editor__label">内容</label>
              <textarea
                v-model="form.content"
                class="editor__textarea"
                placeholder="输入提示词内容..."
              ></textarea>
            </div>

            <!-- 标签 -->
            <div class="editor__field">
              <label class="editor__label">标签</label>
              <TagInput
                v-model="form.tags"
                :all-tags="allTags"
              />
            </div>
          </div>

          <div class="editor__footer">
            <button class="btn btn--secondary" @click="handleClose">取消</button>
            <button
              class="btn btn--primary"
              :disabled="!canSave"
              @click="handleSave"
            >
              {{ isEdit ? '保存修改' : '创建' }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, nextTick } from 'vue'
import { X } from 'lucide-vue-next'
import TagInput from './TagInput.vue'
import IconButton from './IconButton.vue'
import type { Prompt, CategoryWithCount } from '../types'

const props = defineProps<{
  visible: boolean
  prompt: Prompt | null
  categories: CategoryWithCount[]
  allTags: string[]
}>()

const emit = defineEmits<{
  save: [data: { title: string; content: string; category_id: string | null; tags: string[] }]
  close: []
}>()

const titleInput = ref<HTMLInputElement | null>(null)
const initialSnapshot = ref('')

const isEdit = computed(() => props.prompt !== null)

const form = reactive({
  title: '',
  content: '',
  category_id: null as string | null,
  tags: [] as string[],
})

// 当 prompt 变化时填充表单
watch(
  () => props.visible,
  (visible) => {
    if (visible) {
      if (props.prompt) {
        form.title = props.prompt.title
        form.content = props.prompt.content
        form.category_id = props.prompt.category_id
        form.tags = [...props.prompt.tags]
      } else {
        form.title = ''
        form.content = ''
        form.category_id = null
        form.tags = []
      }
      initialSnapshot.value = snapshot()
      nextTick(() => titleInput.value?.focus())
    }
  }
)

const canSave = computed(() => form.title.trim() !== '' && form.content.trim() !== '')

function handleSave() {
  if (!canSave.value) return
  emit('save', {
    title: form.title.trim(),
    content: form.content.trim(),
    category_id: form.category_id,
    tags: form.tags,
  })
}

function handleClose() {
  if (snapshot() !== initialSnapshot.value && !window.confirm('有尚未保存的修改，确定要关闭吗？')) {
    return
  }
  emit('close')
}

function snapshot() {
  return JSON.stringify({
    title: form.title,
    content: form.content,
    category_id: form.category_id,
    tags: form.tags,
  })
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 5000;
}

.editor {
  background: #fff;
  border-radius: 12px;
  width: 600px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.editor__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #f0f0f0;
}

.editor__title {
  font-size: 16px;
  font-weight: 600;
  color: #1a1a2e;
  margin: 0;
}

.editor__body {
  padding: 20px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 16px;
  flex: 1;
}

.editor__field--flex {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.editor__label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: #666;
  margin-bottom: 6px;
}

.editor__input,
.editor__select,
.editor__textarea {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-size: 14px;
  color: #333;
  transition: border-color 0.2s;
  background: #fff;
  font-family: inherit;
}

.editor__input:focus,
.editor__select:focus,
.editor__textarea:focus {
  outline: none;
  border-color: #6366f1;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.editor__textarea {
  min-height: 200px;
  resize: vertical;
  line-height: 1.6;
}

.editor__footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 16px 20px;
  border-top: 1px solid #f0f0f0;
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
