<template>
  <Transition name="undo">
    <div v-if="visible" class="undo-bar" role="status">
      <span>“{{ title }}”已从列表移除</span>
      <button @click="$emit('undo')">撤销</button>
    </div>
  </Transition>
</template>

<script setup lang="ts">
defineProps<{ visible: boolean; title: string }>()
defineEmits<{ undo: [] }>()
</script>

<style scoped>
.undo-bar { position: fixed; left: 50%; bottom: 24px; z-index: 9000; display: flex; align-items: center; gap: 18px; padding: 12px 16px; border-radius: var(--radius); background: #1f2937; color: #fff; box-shadow: var(--shadow-md); transform: translateX(-50%); }
.undo-bar button { border: 0; background: transparent; color: #a5b4fc; font-weight: 600; cursor: pointer; }
.undo-enter-active, .undo-leave-active { transition: all .2s ease; }
.undo-enter-from, .undo-leave-to { opacity: 0; transform: translate(-50%, 12px); }
</style>
