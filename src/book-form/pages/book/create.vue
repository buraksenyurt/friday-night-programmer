<script setup lang="ts">
const title = 'Rent a Book Form'
const { showAlert } = useAlert()

const addBook = async (book: Book) => {
  const response = await postBook(book)
  if (response.success) {
    showAlert('success', response.message)
  } else {
    showAlert('error', response.message)
  }
}

</script>

<template>
  <div>
    <h1>{{ title }}</h1>

    <BookForm @submit="addBook" />

    <p>
      <NuxtLink to="/">
        Go to main page
      </NuxtLink>
    </p>

  </div>
</template>
