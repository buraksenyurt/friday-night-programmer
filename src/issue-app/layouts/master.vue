<script setup lang="ts">

</script>

<template>
    <div class="d-flex min-vh-100">
        <aside class="bg-dark text-white p-3" style="width: 250px;">
            <h2>SIT</h2>
            <ul class="nav flex-column">
                <li class="nav-item mb-2">
                    <NuxtLink to="/" class="nav-link text-white">Home</NuxtLink>
                </li>
                <li class="nav-item mb-2">
                    <NuxtLink to="/request/create" class="nav-link text-white">Open a new Issue</NuxtLink>
                </li>
                <li class="nav-item mb-2">
                    <NuxtLink to="/request/list" class="nav-link text-white">List all Issues</NuxtLink>
                </li>
                <li class="nav-item mb-2">
                    <NuxtLink to="/system/settings" class="nav-link text-white">System settings</NuxtLink>
                </li>
            </ul>
        </aside>

        <main class="flex-grow-1 p-4 bg-light">
            <slot />
        </main>
    </div>
</template>