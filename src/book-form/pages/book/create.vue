<script setup lang="ts">
const title = 'Rent a Book Form'
const description = 'A form to rent a book from the library'
</script>

<template>
  <div>
    <h1>{{ title }}</h1>
    <p>{{ description }}</p>

    <p>
      <NuxtLink to="/">
        Go to main page
      </NuxtLink>
    </p>
  </div>
</template>
