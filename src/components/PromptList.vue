<template>
  <div class="prompt-list">
    <div v-if="prompts.length === 0" class="prompt-list__empty">
      <FileText class="prompt-list__empty-icon" />
      <p class="prompt-list__empty-text">
        {{ emptyText }}
      </p>
    </div>
    <TransitionGroup v-else name="list" tag="div" class="prompt-list__items">
      <PromptCard
        v-for="prompt in prompts"
        :key="prompt.id"
        :prompt="prompt"
        :mode="mode"
        @view="$emit('view', $event)"
        @toggle-pin="$emit('toggle-pin', $event)"
        @copy="$emit('copy', $event)"
        @edit="$emit('edit', $event)"
        @delete="$emit('delete', $event)"
        @tag-click="$emit('tag-click', $event)"
      />
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import PromptCard from './PromptCard.vue'
import { FileText } from 'lucide-vue-next'
import type { Prompt } from '../types'

const props = defineProps<{
  prompts: Prompt[]
  search: string
  mode: 'all' | 'pinned' | 'recent' | 'frequent'
}>()

defineEmits<{
  copy: [prompt: Prompt]
  edit: [prompt: Prompt]
  delete: [prompt: Prompt]
  'tag-click': [tag: string]
  view: [prompt: Prompt]
  'toggle-pin': [prompt: Prompt]
}>()

const emptyText = computed(() => {
  if (props.search) return `未找到与"${props.search}"相关的提示词`
  return '暂无提示词，点击右下角按钮创建'
})

import { computed } from 'vue'
</script>

<style scoped>
.prompt-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
}

.prompt-list__items {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.prompt-list__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: #999;
}

.prompt-list__empty-icon {
  width: 44px;
  height: 44px;
  stroke-width: 1.4;
  margin-bottom: 16px;
}

.prompt-list__empty-text {
  font-size: 14px;
  text-align: center;
  line-height: 1.6;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease;
}

.list-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.list-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}
</style>
