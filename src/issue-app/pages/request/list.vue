<script setup lang="ts">

useHead({
  title: 'Issue List - Issue Tracker'
})

const { showAlert } = useAlert();
const issues = ref<Issue[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
  try {
    const response = await fetch('/api/issues');
    if (!response.ok) {
      showAlert('error', 'Failed to fetch issues');
    }
    issues.value = await response.json();
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'An unknown error occurred';
  } finally {
    loading.value = false;
  }
});

</script>
<template>
  <div class="container mt-4">
    <h2 class="mb-4">Current Issues</h2>

    <div v-if="loading" class="text-center">
      <div class="spinner-border text-primary" role="status">
        <span class="visually-hidden">Loading issues...</span>
      </div>
    </div>

    <div v-else-if="error">
      <div class="alert alert-danger">{{ error }}</div>
    </div>

    <table v-else class="table table-bordered table-hover">
      <thead class="table-light">
        <tr>
          <th>Create Date</th>
          <th>Title</th>
          <th>Owner</th>
          <th>Application</th>
          <th>Severity</th>
          <th>Approver Id</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="issue in issues" :key="issue.id">
          <td>{{ issue.createDate }}</td>
          <td>{{ issue.title }}</td>
          <td>{{ issue.owner }}</td>
          <td>{{ issue.application }}</td>
          <td>{{ issue.severity }}</td>
          <td>{{ issue.approver }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
