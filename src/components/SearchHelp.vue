<template>
  <div ref="root" class="search-help">
    <IconButton label="搜索帮助" size="sm" @click="open = !open"><CircleHelp /></IconButton>
    <Transition name="help">
      <section v-if="open" class="search-help__popover">
        <header><strong>搜索技巧</strong><IconButton label="关闭" size="sm" @click="open = false"><X /></IconButton></header>
        <button class="search-example" @click="use('#开发')"><code>#开发</code><span>只看带“开发”标签的提示词</span></button>
        <button class="search-example" @click="use('cat:工作')"><code>cat:工作</code><span>只看“工作”分类下的提示词</span></button>
        <button class="search-example" @click="use('周报 #工作')"><code>周报 #工作</code><span>关键词和标签可以组合</span></button>
        <p>普通关键词按标题、标签、内容的顺序优先排序。</p>
      </section>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { CircleHelp, X } from 'lucide-vue-next'
import IconButton from './IconButton.vue'
const emit = defineEmits<{ example: [query: string] }>()
const open = ref(false)
const root = ref<HTMLElement | null>(null)
function use(query: string) { emit('example', query); open.value = false }
function closeOutside(event: MouseEvent) { if (root.value && !root.value.contains(event.target as Node)) open.value = false }
onMounted(() => document.addEventListener('click', closeOutside))
onBeforeUnmount(() => document.removeEventListener('click', closeOutside))
</script>

<style scoped>
.search-help { position: relative; display: flex; }
.search-help__popover { position: absolute; top: 34px; right: -8px; z-index: 1000; width: 330px; padding: 14px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--bg-white); box-shadow: 0 16px 40px rgba(15, 23, 42, .16); }
.search-help__popover header { display: flex; justify-content: space-between; margin-bottom: 8px; color: var(--text); }
.search-example { display: flex; flex-direction: column; gap: 3px; width: 100%; padding: 9px; border: 0; border-radius: var(--radius); background: transparent; text-align: left; cursor: pointer; }
.search-example:hover { background: var(--primary-bg); }
.search-example code { color: var(--primary); font-weight: 600; }
.search-example span, .search-help__popover p { color: var(--text-secondary); font-size: 12px; }
.search-help__popover p { margin-top: 9px; padding-top: 10px; border-top: 1px solid var(--border); }
.help-enter-active, .help-leave-active { transition: opacity .15s ease, transform .15s ease; }
.help-enter-from, .help-leave-to { opacity: 0; transform: translateY(-4px); }
</style>
