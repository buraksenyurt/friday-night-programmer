<script setup lang="ts">
import { useRouter } from 'vue-router'

useHead({
  title: 'Approve/Reject - Issue Tracker'
})

const store = useFormStore()
const router = useRouter()
const { showAlert } = useAlert()

const confirmSubmission = async () => {
    try {
        await $fetch('/api/issues', {
            method: 'POST',
            body: { ...store.formData }
        })
        showAlert('success', 'Request approved successfully',2000)
        setTimeout(() => router.push('/', 3000))

    } catch (err) {
        showAlert('error', 'Failed to save the requested issue')
    }
}

const cancelSubmission = () => {
    showAlert('info', 'Request cancelled',3000)
    router.push('/request/create')
}

</script>
<template>
    <div class="container mt-4">
        <h3 class="mb-4">Review</h3>

        <div class="card">
            <div class="card-body">
                <ul class="list-group list-group-flush">
                    <li class="list-group-item">
                        <strong>Title:</strong> {{ store.formData.title }}
                    </li>
                    <li class="list-group-item">
                        <strong>Owner:</strong> {{ store.formData.owner }}
                    </li>
                    <li class="list-group-item">
                        <strong>Application:</strong> {{ store.formData.application }}
                    </li>
                    <li class="list-group-item">
                        <strong>Severity:</strong> {{ store.formData.severity }}
                    </li>
                    <li class="list-group-item">
                        <strong>Approver:</strong> {{ store.formData.approver }}
                    </li>
                    <li class="list-group-item">
                        <strong>Create Date:</strong> {{ store.formData.createDate }}
                    </li>
                    <li class="list-group-item">
                        <strong>Summary:</strong> {{ store.formData.summary }}
                    </li>
                </ul>

                <div class="mt-4 d-flex justify-content-end gap-3">
                    <button class="btn btn-success" @click="confirmSubmission">Approve</button>
                    <button class="btn btn-secondary" @click="cancelSubmission">Reject</button>
                </div>
            </div>
        </div>
    </div>
</template>
