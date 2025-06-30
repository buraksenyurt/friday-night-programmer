<script setup lang="ts">
const title = 'Rent a Book Form'

const message = ref('')
const messageType = ref<'success' | 'error' | 'warning' | 'info'>('info')

const showAlert = (type: typeof messageType.value, text: string) => {
  messageType.value = type,
    message.value = text
}
const clearAlert = () => {
  message.value = ''
}

const addBook = async (book: Book) => {
  const res = await postBook(book)
  if (res.success) {
    showAlert('success', res.message)
  } else {
    showAlert('error', res.message)
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

    <AlertBox v-if="message" :type="messageType" :message="message" @close="clearAlert" />

  </div>
</template>
