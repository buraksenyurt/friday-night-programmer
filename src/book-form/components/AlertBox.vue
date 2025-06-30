<script setup lang="ts">
const props = defineProps<{
    type: 'success' | 'error' | 'warning' | 'info'
    message: string
}>()

const alertClass = computed(() => {
    return {
        success: 'alert-success',
        error: 'alert-danger',
        warning: 'alert-warning',
        info: 'alert-info',
    }[props.type] || 'alert-info' // fallback
})
</script>

<template>
    <div :class="['alert', alertClass, 'd-flex', 'justify-content-between', 'align-items-center']" role="alert">
        <span>{{ message }}</span>
        <button type="button" class="btn-close" @click="$emit('close')" aria-label="Close"></button>
    </div>
</template>
