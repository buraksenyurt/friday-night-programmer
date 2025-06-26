<script setup lang="ts">
const title = 'Rent a Book Form'
const success = ref(false)

const book = reactive({
  title: '',
  author: '',
  published: '',
  hugoYear: ''
})

const submit = async () => {
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

    <form @submit.prevent="submit">
      <table>
        <tr>
          <td>
            <input v-model="book.title" placeholder="Book title" />
          </td>
        </tr>
        <tr>
          <td>
            <input v-model="book.author" placeholder="Author" />
          </td>
        </tr>
        <tr>
          <td>
            <input v-model="book.published" placeholder="Published" />
          </td>
        </tr>
        <tr>
          <td>
            <input v-model="book.hugoYear" placeholder="Hugo Year" />
          </td>
        </tr>
        <tr>
          <td>
            <button type="submit">Add</button>
          </td>
        </tr>
      </table>
    </form>

    <p v-if="success">Success</p>

    <p>
      <NuxtLink to="/">
        Go to main page
      </NuxtLink>
    </p>
  </div>
</template>
