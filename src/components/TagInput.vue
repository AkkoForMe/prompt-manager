<template>
  <div class="tag-input">
    <div class="tag-input__tags">
      <span
        v-for="tag in modelValue"
        :key="tag"
        class="tag-input__tag"
        :style="{ background: getTagColor(tag).bg, color: getTagColor(tag).text }"
      >
        {{ tag }}
        <button class="tag-input__remove" :aria-label="`移除标签 ${tag}`" @click="removeTag(tag)"><X :size="12" /></button>
      </span>
      <input
        ref="inputRef"
        v-model="inputValue"
        class="tag-input__field"
        placeholder="输入标签，回车添加"
        @keydown.enter.prevent="addTag"
        @keydown.backspace="handleBackspace"
        @input="handleInput"
      />
    </div>
    <Transition name="dropdown">
      <ul v-if="suggestions.length > 0" class="tag-input__suggestions">
        <li
          v-for="suggestion in suggestions"
          :key="suggestion"
          class="tag-input__suggestion"
          @click="selectSuggestion(suggestion)"
        >
          {{ suggestion }}
        </li>
      </ul>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { X } from 'lucide-vue-next'
import { getTagColor } from '../utils/colors'

const props = defineProps<{
  modelValue: string[]
  allTags: string[]
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string[]]
}>()

const inputRef = ref<HTMLInputElement | null>(null)
const inputValue = ref('')

// 自动补全建议
const suggestions = computed(() => {
  if (!inputValue.value.trim()) return []
  const query = inputValue.value.trim().toLowerCase()
  return props.allTags.filter(
    (tag) =>
      tag.toLowerCase().includes(query) &&
      !props.modelValue.includes(tag)
  )
})

function addTag() {
  const tag = inputValue.value.trim()
  if (tag && !props.modelValue.includes(tag)) {
    emit('update:modelValue', [...props.modelValue, tag])
  }
  inputValue.value = ''
}

function removeTag(tag: string) {
  emit(
    'update:modelValue',
    props.modelValue.filter((t) => t !== tag)
  )
  nextTick(() => inputRef.value?.focus())
}

function selectSuggestion(tag: string) {
  if (!props.modelValue.includes(tag)) {
    emit('update:modelValue', [...props.modelValue, tag])
  }
  inputValue.value = ''
  nextTick(() => inputRef.value?.focus())
}

function handleBackspace() {
  if (inputValue.value === '' && props.modelValue.length > 0) {
    const tags = [...props.modelValue]
    tags.pop()
    emit('update:modelValue', tags)
  }
}

function handleInput() {
  // 输入时触发自动补全（通过 computed 自动更新）
}

defineExpose({ focus: () => inputRef.value?.focus() })
</script>

<style scoped>
.tag-input {
  position: relative;
}

.tag-input__tags {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  min-height: 40px;
  background: #fff;
  transition: border-color 0.2s;
}

.tag-input__tags:focus-within {
  border-color: #6366f1;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.tag-input__tag {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
}

.tag-input__remove {
  display: inline-grid;
  place-items: center;
  background: none;
  border: none;
  cursor: pointer;
  line-height: 1;
  padding: 0 2px;
  opacity: 0.6;
  transition: opacity 0.15s;
}

.tag-input__remove:hover {
  opacity: 1;
}

.tag-input__field {
  border: none;
  outline: none;
  font-size: 13px;
  min-width: 120px;
  flex: 1;
  background: transparent;
  color: #333;
}

.tag-input__field::placeholder {
  color: #bbb;
}

.tag-input__suggestions {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: #fff;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  margin-top: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  z-index: 100;
  list-style: none;
  padding: 4px 0;
  max-height: 200px;
  overflow-y: auto;
}

.tag-input__suggestion {
  padding: 8px 12px;
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}

.tag-input__suggestion:hover {
  background: #f5f5f5;
}

.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
