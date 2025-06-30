<script setup lang="ts">
import { useRouter } from 'vue-router'

const store = useFormStore()
const router = useRouter()

const handleSubmit = (issue: Issue) => {
    store.setFormData(issue)
    router.push('/request/confirm')
}
</script>
<template>
    <div class="container mt-4">
        <h2>Create Sofware Issue Support Form</h2>
        <IssueForm @submit="handleSubmit" />
    </div>
</template>
