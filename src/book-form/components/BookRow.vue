<script setup lang="ts">

import type { Book } from '@/types/book'
defineProps<Book>()

</script>

<template>
    <tr>
        <td>{{ title }}</td>
        <td>{{ author }}</td>
        <td>{{ published }}</td>
        <td>{{ hugoYear }}</td>
    </tr>
</template>