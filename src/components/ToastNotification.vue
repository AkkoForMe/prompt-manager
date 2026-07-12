<template>
  <div class="toast-container">
    <TransitionGroup name="toast">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="toast"
        :class="'toast--' + toast.type"
      >
        <span class="toast__icon">
          <CircleCheck v-if="toast.type === 'success'" />
          <CircleX v-else-if="toast.type === 'error'" />
          <Info v-else />
        </span>
        <span class="toast__text">{{ toast.text }}</span>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import type { ToastMessage } from '../types'
import { CircleCheck, CircleX, Info } from 'lucide-vue-next'

defineProps<{
  toasts: ToastMessage[]
}>()
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 10000;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.toast {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border-radius: 8px;
  font-size: 14px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  min-width: 200px;
  max-width: 400px;
}

.toast--success {
  background: #e8f5e9;
  color: #2e7d32;
}

.toast--error {
  background: #fce4ec;
  color: #c62828;
}

.toast--info {
  background: #e3f2fd;
  color: #1565c0;
}

.toast__icon {
  display: inline-flex;
}
.toast__icon :deep(svg) { width: 16px; height: 16px; stroke-width: 1.75; }

.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100px);
}
</style>
