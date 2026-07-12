<template>
  <Teleport to="body">
    <Transition name="drawer">
      <div v-if="visible && prompt" class="detail-overlay" @click.self="emit('close')">
        <aside class="detail" role="dialog" aria-modal="true" :aria-label="prompt.title">
          <header class="detail__header">
            <div>
              <div class="detail__eyebrow">提示词详情</div>
              <h2>{{ prompt.title }}</h2>
            </div>
            <IconButton label="关闭详情" @click="emit('close')"><X /></IconButton>
          </header>
          <div class="detail__tags">
            <span v-for="tag in prompt.tags" :key="tag">#{{ tag }}</span>
          </div>
          <pre class="detail__content">{{ prompt.content }}</pre>
          <footer class="detail__footer">
            <span>更新于 {{ new Date(prompt.updated_at).toLocaleString('zh-CN') }}</span>
            <div>
              <button class="btn btn--secondary" @click="emit('toggle-pin', prompt)">
                {{ prompt.pinned ? '取消置顶' : '置顶' }}
              </button>
              <button class="btn btn--secondary" @click="emit('edit', prompt)">编辑</button>
              <button class="btn btn--primary" @click="emit('copy', prompt)">复制内容</button>
            </div>
          </footer>
        </aside>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import type { Prompt } from '../types'
import { X } from 'lucide-vue-next'
import IconButton from './IconButton.vue'
defineProps<{ visible: boolean; prompt: Prompt | null }>()
const emit = defineEmits<{
  close: []
  copy: [prompt: Prompt]
  edit: [prompt: Prompt]
  'toggle-pin': [prompt: Prompt]
}>()
</script>

<style scoped>
.detail-overlay { position: fixed; inset: 0; z-index: 6000; background: rgba(15, 23, 42, .35); }
.detail { position: absolute; inset: 0 0 0 auto; width: min(620px, 86vw); display: flex; flex-direction: column; padding: 24px; background: var(--bg-white); box-shadow: -12px 0 36px rgba(15, 23, 42, .14); }
.detail__header { display: flex; justify-content: space-between; gap: 20px; }
.detail__eyebrow { margin-bottom: 5px; color: var(--primary); font-size: 12px; font-weight: 600; }
.detail h2 { color: var(--text); font-size: 22px; }
.detail__tags { display: flex; flex-wrap: wrap; gap: 7px; margin: 16px 0; color: var(--primary); font-size: 13px; }
.detail__content { flex: 1; overflow: auto; padding: 18px; border: 1px solid var(--border); border-radius: var(--radius); background: var(--bg); color: var(--text); font: 14px/1.7 inherit; white-space: pre-wrap; word-break: break-word; }
.detail__footer { display: flex; align-items: center; justify-content: space-between; gap: 16px; padding-top: 16px; color: var(--text-muted); font-size: 12px; }
.detail__footer div { display: flex; gap: 8px; }
.drawer-enter-active, .drawer-leave-active { transition: opacity .2s ease; }
.drawer-enter-active .detail, .drawer-leave-active .detail { transition: transform .2s ease; }
.drawer-enter-from, .drawer-leave-to { opacity: 0; }
.drawer-enter-from .detail, .drawer-leave-to .detail { transform: translateX(100%); }
</style>
