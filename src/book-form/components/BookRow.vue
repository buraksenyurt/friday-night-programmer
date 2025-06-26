<script setup lang="ts">

defineProps<{
    title: string
    author: string
    published: number
    hugoYear: number
}>()

</script>

<template>
    <tr>
        <td>{{ title }}</td>
        <td>{{ author }}</td>
        <td>{{ published }}</td>
        <td>{{ hugoYear }}</td>
    </tr>
</template>