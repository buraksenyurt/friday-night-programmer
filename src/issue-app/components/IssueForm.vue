<script setup lang="ts">

const { showAlert } = useAlert()

const emit = defineEmits<{
    (e: 'submit', issue: Issue): void
}>()

const issue = ref<Issue>({
    title: '',
    owner: '',
    summary: '',
    createDate: new Date().toISOString().split('T')[0],
    application: '',
    severity: Severity.Low
})

const applications = ref<string[]>([])
const severities = Object.values(Severity)
const { fetchApplications } = useApplications()

onMounted(async () => {
    applications.value = await fetchApplications()
})

const submitForm = () => {
    if (issue.value.title && issue.value.owner && issue.value.summary && issue.value.application && issue.value.severity) {
        emit('submit', { ...issue.value })
    } else {
        showAlert('warning', 'Please fill all fields')
    }
}

</script>
<template>
    <form @submit.prevent="submitForm" class="p-4 border rounded bg-light">
        <div class="mb-3">
            <label for="title" class="form-label">Short Title of Issue</label>
            <input id="title" v-model="issue.title" class="form-control" required />
        </div>

        <div class="mb-3">
            <label for="owner" class="form-label">Owner</label>
            <input id="owner" v-model="issue.owner" class="form-control" required />
        </div>

        <div class="mb-3">
            <label for="summary" class="form-label">Summary</label>
            <textarea id="summary" v-model="issue.summary" class="form-control" rows="3" required></textarea>
        </div>

        <SelectBox label="Application" :model-value="issue.application" :options="applications"
            @update:modelValue="issue.application = $event" />

        <SelectBox label="Severity" :model-value="issue.severity" :options="severities"
            @update:modelValue="issue.severity = $event" />

        <div class="text-end">
            <button class="btn btn-primary" type="submit">Send for Approve</button>
        </div>
    </form>
</template>