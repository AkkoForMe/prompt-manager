<template>
  <button
    v-bind="$attrs"
    type="button"
    class="icon-button"
    :class="[
      `icon-button--${size}`,
      `icon-button--${tone}`,
      { 'icon-button--active': active },
    ]"
    :title="title || label"
    :aria-label="label"
    :aria-pressed="pressed"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
defineOptions({ inheritAttrs: false })
withDefaults(defineProps<{
  label: string
  title?: string
  size?: 'sm' | 'md' | 'lg'
  tone?: 'neutral' | 'primary' | 'warning' | 'danger'
  active?: boolean
  pressed?: boolean
}>(), {
  size: 'md',
  tone: 'neutral',
  active: false,
  pressed: undefined,
})
</script>

<style scoped>
.icon-button {
  width: 30px;
  height: 30px;
  flex: none;
  display: inline-grid;
  place-items: center;
  padding: 0;
  border: 0;
  border-radius: 7px;
  background: transparent;
  color: var(--text-muted);
  line-height: 0;
  cursor: pointer;
  transition: color .15s ease, background-color .15s ease, box-shadow .15s ease;
}
.icon-button--sm { width: 26px; height: 26px; border-radius: 6px; }
.icon-button--lg { width: 40px; height: 40px; border-radius: 50%; }
.icon-button :deep(svg) { width: 16px; height: 16px; stroke-width: 1.75; }
.icon-button--lg :deep(svg) { width: 22px; height: 22px; }
.icon-button:hover { background: #f3f4f6; color: var(--text); }
.icon-button--primary:hover, .icon-button--primary.icon-button--active { background: var(--primary-bg); color: var(--primary); }
.icon-button--warning:hover, .icon-button--warning.icon-button--active { background: #fff7ed; color: #d97706; }
.icon-button--danger:hover { background: #fef2f2; color: #dc2626; }
.icon-button:focus-visible { outline: none; box-shadow: 0 0 0 2px var(--bg-white), 0 0 0 4px rgba(99, 102, 241, .35); }
.icon-button:disabled { opacity: .45; cursor: not-allowed; }
</style>
