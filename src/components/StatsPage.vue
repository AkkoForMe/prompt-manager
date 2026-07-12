<template>
  <section class="stats-page">
    <header class="stats-header">
      <IconButton label="返回提示词列表" size="lg" @click="emit('back')"><ArrowLeft /></IconButton>
      <div>
        <h1>使用统计</h1>
        <p>每一次成功复制，都会在这里留下积累。</p>
      </div>
    </header>

    <div class="stats-summary">
      <article class="summary-card summary-card--primary">
        <span>累计复制</span><strong>{{ totalCopies }}</strong><small>次</small>
      </article>
      <article class="summary-card">
        <span>用过的提示词</span><strong>{{ usedCount }}</strong><small>/ {{ prompts.length }} 条</small>
      </article>
      <article class="summary-card">
        <span>使用覆盖率</span><strong>{{ coverage }}</strong><small>%</small>
      </article>
      <article class="summary-card">
        <span>已置顶</span><strong>{{ pinnedCount }}</strong><small>条</small>
      </article>
    </div>

    <div class="coverage-card">
      <div><strong>探索进度</strong><span>{{ coverage }}%</span></div>
      <div class="coverage-track"><i :style="{ width: `${coverage}%` }"></i></div>
      <p>还有 {{ Math.max(prompts.length - usedCount, 0) }} 条提示词等待第一次使用。</p>
    </div>

    <div class="stats-grid">
      <section class="stats-panel">
        <header><h2>常用榜单</h2><span>累计复制</span></header>
        <div v-if="topPrompts.length" class="ranking-list">
          <button v-for="(prompt, index) in topPrompts" :key="prompt.id" @click="emit('view', prompt)">
            <b>{{ index + 1 }}</b><span>{{ prompt.title }}</span><strong>{{ prompt.use_count }} 次</strong>
          </button>
        </div>
        <div v-else class="stats-empty">复制提示词后，这里会出现你的常用榜单。</div>
      </section>

      <section class="stats-panel">
        <header><h2>最近使用</h2><span>最近复制</span></header>
        <div v-if="recentPrompts.length" class="recent-list">
          <button v-for="prompt in recentPrompts" :key="prompt.id" @click="emit('view', prompt)">
            <span>{{ prompt.title }}</span><small>{{ relativeTime(prompt.last_used_at) }}</small>
          </button>
        </div>
        <div v-else class="stats-empty">还没有使用记录。</div>
      </section>
    </div>

    <section class="achievements">
      <header><h2>阶段成就</h2><span>{{ unlockedCount }} / {{ achievements.length }} 已达成</span></header>
      <div class="achievement-grid">
        <article
          v-for="achievement in achievements"
          :key="achievement.title"
          :class="{ 'achievement--unlocked': achievement.unlocked }"
        >
          <div class="achievement__icon"><Check v-if="achievement.unlocked"/><Circle v-else/></div>
          <div><strong>{{ achievement.title }}</strong><p>{{ achievement.description }}</p></div>
        </article>
      </div>
    </section>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ArrowLeft, Check, Circle } from 'lucide-vue-next'
import type { Prompt } from '../types'
import IconButton from './IconButton.vue'

const props = defineProps<{ prompts: Prompt[] }>()
const emit = defineEmits<{ back: []; view: [prompt: Prompt] }>()
const totalCopies = computed(() => props.prompts.reduce((sum, prompt) => sum + prompt.use_count, 0))
const usedCount = computed(() => props.prompts.filter((prompt) => prompt.use_count > 0).length)
const pinnedCount = computed(() => props.prompts.filter((prompt) => prompt.pinned).length)
const coverage = computed(() => props.prompts.length ? Math.round(usedCount.value / props.prompts.length * 100) : 0)
const topPrompts = computed(() => [...props.prompts].filter((p) => p.use_count > 0).sort((a, b) => b.use_count - a.use_count).slice(0, 5))
const recentPrompts = computed(() => [...props.prompts].filter((p) => p.last_used_at).sort((a, b) => Date.parse(b.last_used_at!) - Date.parse(a.last_used_at!)).slice(0, 5))
const achievements = computed(() => [
  { title: '第一次积累', description: '成功复制 1 次提示词', unlocked: totalCopies.value >= 1 },
  { title: '渐入佳境', description: '累计复制达到 10 次', unlocked: totalCopies.value >= 10 },
  { title: '高效达人', description: '累计复制达到 50 次', unlocked: totalCopies.value >= 50 },
  { title: '复制大师', description: '累计复制达到 100 次', unlocked: totalCopies.value >= 100 },
  { title: '探索家', description: '使用过一半以上的提示词', unlocked: coverage.value >= 50 },
  { title: '收藏家', description: '置顶至少 5 条提示词', unlocked: pinnedCount.value >= 5 },
])
const unlockedCount = computed(() => achievements.value.filter((item) => item.unlocked).length)

function relativeTime(value: string | null) {
  if (!value) return '未使用'
  const minutes = Math.floor((Date.now() - Date.parse(value)) / 60000)
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes} 分钟前`
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours} 小时前`
  return `${Math.floor(hours / 24)} 天前`
}
</script>

<style scoped>
.stats-page { flex: 1; overflow-y: auto; padding: 28px 32px 44px; background: var(--bg); }
.stats-header { display: flex; align-items: center; gap: 14px; margin-bottom: 24px; }
.stats-header h1 { color: var(--text); font-size: 24px; }
.stats-header p { margin-top: 3px; color: var(--text-muted); }
.stats-summary { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 12px; }
.summary-card { padding: 18px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--bg-white); box-shadow: var(--shadow-sm); }
.summary-card span { display: block; margin-bottom: 10px; color: var(--text-secondary); font-size: 12px; }
.summary-card strong { color: var(--text); font-size: 30px; }
.summary-card small { margin-left: 4px; color: var(--text-muted); }
.summary-card--primary { border: 0; background: linear-gradient(135deg, #6366f1, #8b5cf6); }
.summary-card--primary span, .summary-card--primary strong, .summary-card--primary small { color: #fff; }
.coverage-card { margin-top: 14px; padding: 18px; border-radius: var(--radius-lg); background: var(--bg-white); }
.coverage-card > div:first-child { display: flex; justify-content: space-between; color: var(--text); }
.coverage-track { height: 9px; margin: 12px 0 8px; overflow: hidden; border-radius: 99px; background: var(--primary-bg); }
.coverage-track i { display: block; height: 100%; border-radius: inherit; background: linear-gradient(90deg, #6366f1, #a78bfa); transition: width .3s ease; }
.coverage-card p { color: var(--text-muted); font-size: 12px; }
.stats-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 14px; margin-top: 14px; }
.stats-panel, .achievements { padding: 18px; border-radius: var(--radius-lg); background: var(--bg-white); }
.stats-panel header, .achievements > header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; }
.stats-panel h2, .achievements h2 { color: var(--text); font-size: 15px; }
.stats-panel header span, .achievements header span { color: var(--text-muted); font-size: 11px; }
.ranking-list, .recent-list { display: flex; flex-direction: column; }
.ranking-list button, .recent-list button { display: flex; align-items: center; gap: 10px; width: 100%; padding: 10px 6px; border: 0; border-bottom: 1px solid var(--border); background: transparent; color: var(--text); text-align: left; cursor: pointer; }
.ranking-list b { width: 24px; color: var(--primary); }
.ranking-list span, .recent-list span { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.ranking-list strong { color: #d97706; font-size: 12px; }
.recent-list small { color: var(--text-muted); }
.stats-empty { padding: 32px 12px; color: var(--text-muted); text-align: center; }
.achievements { margin-top: 14px; }
.achievement-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; }
.achievement-grid article { display: flex; gap: 10px; padding: 13px; border: 1px dashed var(--border); border-radius: var(--radius); opacity: .55; }
.achievement-grid article.achievement--unlocked { border-style: solid; border-color: #c7d2fe; background: var(--primary-bg); opacity: 1; }
.achievement__icon { flex: none; width: 28px; height: 28px; display: grid; place-items: center; border-radius: 50%; background: var(--bg); color: var(--text-muted); }
.achievement__icon :deep(svg) { width: 14px; height: 14px; stroke-width: 2; }
.achievement--unlocked .achievement__icon { background: var(--primary); color: #fff; }
.achievement-grid strong { color: var(--text); font-size: 13px; }
.achievement-grid p { margin-top: 3px; color: var(--text-muted); font-size: 11px; }
@media (max-width: 900px) { .stats-summary { grid-template-columns: repeat(2, 1fr); } .achievement-grid { grid-template-columns: repeat(2, 1fr); } }
</style>
