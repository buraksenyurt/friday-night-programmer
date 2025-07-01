<script setup lang="ts">

defineProps<{
    label: string
    modelValue: string | number
    options: any[]
    optionLabel: string
    optionValue: string
}>()

const emit = defineEmits(['update:modelValue'])

</script>

<template>
    <div class="mb-3">
        <label class="form-label">{{ label }}</label>
        <select class="form-select" :value="modelValue"
            @change="emit('update:modelValue', ($event.target as HTMLSelectElement).value)">
            <option disabled value="">Please select</option>
            <option v-for="item in options" :key="item[optionValue]" :value="item[optionValue]">
                {{ item[optionLabel] }}
            </option>
        </select>
    </div>
</template>