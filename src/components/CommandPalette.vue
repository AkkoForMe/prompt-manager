<template>
  <Teleport to="body">
    <Transition name="palette">
      <div v-if="visible" class="palette-overlay" @click.self="emit('close')">
        <section class="palette" role="dialog" aria-modal="true" aria-label="快速搜索提示词">
          <div class="palette__search">
            <Search :size="18" />
            <input
              ref="inputRef"
              v-model="query"
              placeholder="搜索标题、标签或内容…"
              @keydown="handleKeydown"
            />
            <kbd>Esc</kbd>
          </div>
          <div class="palette__results">
            <button
              v-for="(prompt, index) in results"
              :key="prompt.id"
              class="palette__result"
              :class="{ 'palette__result--active': index === selectedIndex }"
              @mouseenter="selectedIndex = index"
              @click="copyPrompt(prompt)"
            >
              <span class="palette__result-main">
                <strong>{{ prompt.title }}</strong>
                <small>{{ prompt.content }}</small>
              </span>
              <span class="palette__tags">{{ prompt.tags.map((tag) => `#${tag}`).join(' ') }}</span>
            </button>
            <div v-if="results.length === 0" class="palette__empty">没有匹配的提示词</div>
          </div>
          <footer class="palette__footer">
            <span><kbd>↑↓</kbd> 选择</span>
            <span><kbd>Enter</kbd> 复制</span>
            <span><kbd>Ctrl Enter</kbd> 编辑</span>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import { Search } from 'lucide-vue-next'
import type { Prompt } from '../types'

const props = defineProps<{ visible: boolean; prompts: Prompt[] }>()
const emit = defineEmits<{ close: []; copy: [prompt: Prompt]; edit: [prompt: Prompt] }>()
const query = ref('')
const selectedIndex = ref(0)
const inputRef = ref<HTMLInputElement | null>(null)

const results = computed(() => {
  const needle = query.value.trim().toLowerCase()
  const matches = needle
    ? props.prompts.filter((prompt) =>
        prompt.title.toLowerCase().includes(needle) ||
        prompt.content.toLowerCase().includes(needle) ||
        prompt.tags.some((tag) => tag.toLowerCase().includes(needle))
      )
    : props.prompts
  return matches.slice(0, 12)
})

watch(() => props.visible, (visible) => {
  if (!visible) return
  query.value = ''
  selectedIndex.value = 0
  nextTick(() => inputRef.value?.focus())
})
watch(results, () => { selectedIndex.value = 0 })

function copyPrompt(prompt: Prompt) {
  emit('copy', prompt)
  emit('close')
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') emit('close')
  if (event.key === 'ArrowDown') {
    event.preventDefault()
    selectedIndex.value = Math.min(selectedIndex.value + 1, results.value.length - 1)
  }
  if (event.key === 'ArrowUp') {
    event.preventDefault()
    selectedIndex.value = Math.max(selectedIndex.value - 1, 0)
  }
  if (event.key === 'Enter' && results.value[selectedIndex.value]) {
    event.preventDefault()
    const prompt = results.value[selectedIndex.value]
    if (event.ctrlKey || event.metaKey) {
      emit('edit', prompt)
      emit('close')
    } else {
      copyPrompt(prompt)
    }
  }
}
</script>

<style scoped>
.palette-overlay { position: fixed; inset: 0; z-index: 8000; display: flex; justify-content: center; padding-top: 12vh; background: rgba(15, 23, 42, .45); }
.palette { width: min(640px, calc(100vw - 32px)); max-height: 70vh; align-self: flex-start; overflow: hidden; border-radius: var(--radius-lg); background: var(--bg-white); box-shadow: 0 24px 64px rgba(15, 23, 42, .24); }
.palette__search { display: flex; align-items: center; gap: 10px; padding: 14px 16px; border-bottom: 1px solid var(--border); font-size: 20px; }
.palette__search input { flex: 1; min-width: 0; border: 0; outline: 0; background: transparent; color: var(--text); font: inherit; font-size: 16px; }
.palette__results { max-height: 48vh; overflow-y: auto; padding: 8px; }
.palette__result { width: 100%; display: flex; align-items: center; gap: 12px; padding: 11px 12px; border: 0; border-radius: var(--radius); background: transparent; color: var(--text); text-align: left; cursor: pointer; }
.palette__result--active { background: var(--primary-bg); }
.palette__result-main { min-width: 0; flex: 1; display: flex; flex-direction: column; gap: 3px; }
.palette__result-main small { overflow: hidden; color: var(--text-secondary); text-overflow: ellipsis; white-space: nowrap; }
.palette__tags { max-width: 180px; overflow: hidden; color: var(--primary); font-size: 12px; text-overflow: ellipsis; white-space: nowrap; }
.palette__empty { padding: 36px; color: var(--text-muted); text-align: center; }
.palette__footer { display: flex; gap: 18px; padding: 9px 16px; border-top: 1px solid var(--border); color: var(--text-muted); font-size: 12px; }
kbd { padding: 2px 5px; border: 1px solid var(--border); border-radius: 4px; background: var(--bg); color: var(--text-secondary); font: inherit; }
.palette-enter-active, .palette-leave-active { transition: opacity .15s ease; }
.palette-enter-from, .palette-leave-to { opacity: 0; }
</style>
