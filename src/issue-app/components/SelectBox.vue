<script setup lang="ts">

defineProps<{
    label: string
    modelValue: string
    options: string[]
}>()

const emit = defineEmits(['update:modelValue'])

</script>

<template>
    <div class="mb-3">
        <label class="form-label">{{ label }}</label>
        <select class="form-select" :value="modelValue" @change="emit('update:modelValue', $event.target.value)">
            <option disabled value="">Please select</option>
            <option v-for="option in options" :key="option">{{ option }}</option>
        </select>
    </div>
</template>