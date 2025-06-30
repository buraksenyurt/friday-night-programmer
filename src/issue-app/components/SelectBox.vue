<script setup lang="ts">
defineProps<{
    label: string
    value: string
    options: string[]
}>

const emit = defineEmits(['update:value'])

</script>

<template>
    <div class="mb-3">
        <label class="form-label">{{ label }}</label>
        <select class="form-select" :value="value" @change="emit('update:value', $event.target.value)">
            <option disabled value="">Please select</option>
            <option v-for="option in options" :key="opt">{{ opt }}</option>
        </select>
    </div>
</template>