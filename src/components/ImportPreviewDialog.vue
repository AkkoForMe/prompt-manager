<template>
  <Teleport to="body">
    <div v-if="visible && preview" class="import-overlay" @click.self="emit('cancel')">
      <section class="import-dialog" role="dialog" aria-modal="true" aria-label="导入预览">
        <h2>导入预览</h2>
        <p class="import-dialog__summary">
          文件包含 {{ preview.total_prompts }} 条提示词和 {{ preview.total_categories }} 个分类；
          将新增 {{ preview.new_prompts }} 条提示词和 {{ preview.new_categories }} 个分类。
        </p>
        <p v-if="preview.skipped_by_id" class="import-dialog__notice">
          {{ preview.skipped_by_id }} 条提示词因 ID 已存在将被跳过。
        </p>
        <div v-if="preview.possible_duplicates.length" class="import-dialog__duplicates">
          <strong>发现 {{ preview.possible_duplicates.length }} 条疑似重复</strong>
          <ul>
            <li v-for="item in preview.possible_duplicates" :key="item.incoming_id">
              <span>{{ item.title }}</span><small>{{ item.reason }}</small>
            </li>
          </ul>
          <p>疑似重复项仍会导入，你可以导入后自行整理。</p>
        </div>
        <div v-else class="import-dialog__clean">未发现疑似重复内容。</div>
        <footer>
          <button class="btn btn--secondary" @click="emit('cancel')">取消</button>
          <button class="btn btn--primary" :disabled="loading" @click="emit('confirm')">
            {{ loading ? '正在导入…' : '确认导入' }}
          </button>
        </footer>
      </section>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import type { ImportPreview } from '../types'
defineProps<{ visible: boolean; preview: ImportPreview | null; loading: boolean }>()
const emit = defineEmits<{ confirm: []; cancel: [] }>()
</script>

<style scoped>
.import-overlay { position: fixed; inset: 0; z-index: 7000; display: flex; align-items: center; justify-content: center; padding: 20px; background: rgba(15, 23, 42, .42); }
.import-dialog { width: min(560px, 100%); max-height: 80vh; overflow: auto; padding: 24px; border-radius: var(--radius-lg); background: var(--bg-white); box-shadow: var(--shadow-md); }
.import-dialog h2 { margin-bottom: 10px; color: var(--text); }
.import-dialog__summary { color: var(--text-secondary); line-height: 1.7; }
.import-dialog__notice, .import-dialog__duplicates, .import-dialog__clean { margin-top: 16px; padding: 12px; border-radius: var(--radius); }
.import-dialog__notice { background: #fff7ed; color: #9a3412; }
.import-dialog__duplicates { background: #fff7ed; color: var(--text); }
.import-dialog__duplicates ul { max-height: 190px; overflow: auto; margin: 10px 0; list-style: none; }
.import-dialog__duplicates li { display: flex; justify-content: space-between; gap: 12px; padding: 7px 0; border-bottom: 1px solid #fed7aa; }
.import-dialog__duplicates small { flex: none; color: #c2410c; }
.import-dialog__duplicates p { color: var(--text-secondary); font-size: 12px; }
.import-dialog__clean { background: #ecfdf5; color: #047857; }
.import-dialog footer { display: flex; justify-content: flex-end; gap: 8px; margin-top: 22px; }
</style>
