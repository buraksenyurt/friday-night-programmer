<script setup lang="ts">
useHead({
    title: 'About - Issue Tracker'
})
</script>

<template>
    <div class="container mt-5">
        <div class="card shadow-sm border-0">
            <div class="card-body">
                <p class="fs-5">
                    <strong>Issue Tracker</strong> is a lightweight and intuitive application designed to help software
                    teams
                    manage and track their support requests and issues efficiently.
                </p>

                <ul class="list-group list-group-flush my-4">
                    <li class="list-group-item">
                        <strong>Frontend:</strong> Vue 3, TypeScript, Nuxt 3
                    </li>
                    <li class="list-group-item">
                        <strong>Backend:</strong> Node.js API with Prisma and SQLite
                    </li>
                    <li class="list-group-item">
                        <strong>Styling:</strong> Bootstrap 5
                    </li>
                    <li class="list-group-item">
                        <strong>Version:</strong> 1.0.1
                    </li>
                    <li class="list-group-item">
                        <strong>Minimum Supported Browsers:</strong> Chrome 80, Firefox 75, Safari 13, Edge 80
                    </li>
                </ul>

                <p class="text-muted small">
                    This is an educational project built to demonstrate form workflows, state management with Pinia,
                    client-server interaction, and database integration using modern full-stack tools.
                </p>
                <p>
                    <a href="https://github.com/buraksenyurt/friday-night-programmer/tree/main/src/issue-app" target="_blank">View on GitHub</a>
                </p>
            </div>
        </div>
    </div>
</template>
