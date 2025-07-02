<script setup lang="ts">

const emit = defineEmits(['close'])

const props = defineProps<{
    type: 'success' | 'error' | 'warning' | 'info'
    message: string
    autoDismiss?: number
}>()

const alertClass = computed(() => {
    return {
        success: 'alert-success',
        error: 'alert-danger',
        warning: 'alert-warning',
        info: 'alert-info',
    }[props.type] || 'alert-info'
})

onMounted(() => {
    if (props.autoDismiss && props.autoDismiss > 0) {
        setTimeout(() => {
            emit('close')
        }, props.autoDismiss)
    }
})

</script>

<template>
    <div :class="['alert', alertClass, 'd-flex', 'justify-content-between', 'align-items-center']" role="alert">
        <span>{{ props.message }}</span>
        <button type="button" class="btn-close" @click="$emit('close')" aria-label="Close"></button>
    </div>
</template>