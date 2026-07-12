<template>
  <div class="prompt-card" role="button" tabindex="0" @click="$emit('view', prompt)" @keydown.enter="$emit('view', prompt)">
    <div class="prompt-card__header">
      <h3 class="prompt-card__title">{{ prompt.title }}</h3>
      <div class="prompt-card__actions">
        <IconButton
          :label="prompt.pinned ? '取消置顶' : '置顶'"
          tone="warning"
          :active="prompt.pinned"
          :pressed="prompt.pinned"
          @click.stop="$emit('toggle-pin', prompt)"
        >
          <Star :fill="prompt.pinned ? 'currentColor' : 'none'" />
        </IconButton>
        <IconButton label="复制内容" tone="primary" @click.stop="$emit('copy', prompt)"><Copy /></IconButton>
        <IconButton label="编辑" tone="warning" @click.stop="$emit('edit', prompt)"><SquarePen /></IconButton>
        <IconButton label="删除" tone="danger" @click.stop="$emit('delete', prompt)"><Trash2 /></IconButton>
      </div>
    </div>

    <div class="prompt-card__content">
      {{ truncatedContent }}
    </div>

    <div v-if="showUsageHighlight" class="prompt-card__usage" :class="`prompt-card__usage--${mode}`">
      <strong>{{ prompt.use_count }}</strong>
      <span>{{ mode === 'recent' ? `次累计复制 · 最近 ${relativeLastUsed}` : '次累计复制' }}</span>
      <em v-if="mode === 'frequent'">{{ usageMilestone }}</em>
    </div>

    <div class="prompt-card__footer">
      <div class="prompt-card__tags">
        <span
          v-for="tag in prompt.tags"
          :key="tag"
          class="prompt-card__tag"
          :style="{ background: getTagColor(tag).bg, color: getTagColor(tag).text }"
          @click.stop="$emit('tag-click', tag)"
        >
          #{{ tag }}
        </span>
      </div>
      <div class="prompt-card__meta">
        <span class="prompt-card__time">{{ formattedTime }}</span>
        <span v-if="prompt.use_count && !showUsageHighlight" class="prompt-card__uses">复制 {{ prompt.use_count }} 次</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Copy, SquarePen, Star, Trash2 } from 'lucide-vue-next'
import IconButton from './IconButton.vue'
import { getTagColor } from '../utils/colors'
import type { Prompt } from '../types'

const props = defineProps<{
  prompt: Prompt
  mode?: 'all' | 'pinned' | 'recent' | 'frequent'
}>()

defineEmits<{
  copy: [prompt: Prompt]
  edit: [prompt: Prompt]
  delete: [prompt: Prompt]
  'tag-click': [tag: string]
  view: [prompt: Prompt]
  'toggle-pin': [prompt: Prompt]
}>()

const truncatedContent = computed(() => {
  const content = props.prompt.content
  return content.length > 150 ? content.slice(0, 150) + '...' : content
})
const showUsageHighlight = computed(() => props.mode === 'recent' || props.mode === 'frequent')
const relativeLastUsed = computed(() => {
  if (!props.prompt.last_used_at) return '未使用'
  const minutes = Math.floor((Date.now() - Date.parse(props.prompt.last_used_at)) / 60000)
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes} 分钟前`
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours} 小时前`
  return `${Math.floor(hours / 24)} 天前`
})
const usageMilestone = computed(() => {
  if (props.prompt.use_count >= 100) return '复制大师'
  if (props.prompt.use_count >= 50) return '高效达人'
  if (props.prompt.use_count >= 10) return '渐入佳境'
  return '正在积累'
})

const formattedTime = computed(() => {
  const date = new Date(props.prompt.updated_at)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes} 分钟前`
  if (hours < 24) return `${hours} 小时前`
  if (days < 7) return `${days} 天前`

  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
  })
})
</script>

<style scoped>
.prompt-card {
  background: #fff;
  border-radius: 10px;
  padding: 16px 20px;
  border: 1px solid #f0f0f0;
  transition: all 0.2s ease;
}

.prompt-card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);
  border-color: #e0e0e0;
}

.prompt-card__header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 8px;
}

.prompt-card__uses { margin-left: 8px; color: var(--text-muted); font-size: 12px; }
.prompt-card__meta { display: flex; align-items: center; flex: none; }

.prompt-card__title {
  font-size: 15px;
  font-weight: 600;
  color: #1a1a2e;
  margin: 0;
  line-height: 1.4;
}

.prompt-card__actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.2s;
}

.prompt-card:hover .prompt-card__actions {
  opacity: 1;
}


.prompt-card__content {
  font-size: 13px;
  color: #666;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
  margin-bottom: 12px;
}
.prompt-card__usage { display: flex; align-items: baseline; gap: 6px; margin: -2px 0 12px; padding: 10px 12px; border-radius: var(--radius); background: linear-gradient(90deg, var(--primary-bg), #faf5ff); color: var(--text-secondary); }
.prompt-card__usage strong { color: var(--primary); font-size: 21px; line-height: 1; }
.prompt-card__usage span { font-size: 12px; }
.prompt-card__usage em { margin-left: auto; color: #d97706; font-size: 11px; font-style: normal; font-weight: 600; }

.prompt-card__footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.prompt-card__tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.prompt-card__tag {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 0.15s;
}

.prompt-card__tag:hover {
  opacity: 0.8;
}

.prompt-card__time {
  font-size: 11px;
  color: #bbb;
  white-space: nowrap;
}
</style>
