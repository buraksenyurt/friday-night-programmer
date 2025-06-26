<script setup lang="ts">
const title = 'Rent a Book Form'

const success = ref(false)

const addBook = async (book: any) => {
  const res = await $fetch('/api/books', {
    method: 'POST',
    body: book
  })

  if (res.success) {
    success.value = true
  }
}

</script>

<template>
  <div>
    <h1>{{ title }}</h1>

    <BookForm @submit="addBook" />

    <p v-if="success">Success</p>

    <p>
      <NuxtLink to="/">
        Go to main page
      </NuxtLink>
    </p>
  </div>
</template>
