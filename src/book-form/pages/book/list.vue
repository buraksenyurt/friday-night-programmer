<script setup lang="ts">

// const { data: books } = await useFetch<Book[]>('/api/books')
const books = ref<Book[]>([])

const fetchBooks = async () => {
    const response = await $fetch<Book[]>('/api/books')
    books.value = response
}

await fetchBooks()

const removeBook = async (title: string) => {
    const confirmed = confirm(`"${title}" will be deleted. Are you sure?`)
    if (!confirmed)
        return

    const response = await deleteBook(title)
    if (!response?.success) {
        alert(response?.message ?? 'Unknown Error')
        return
    }

    books.value = books.value.filter(book => book.title !== title)
}

</script>

<template>
    <div>
        <h1>Hugo Award Winners</h1>

        <table>
            <thead>
                <tr>
                    <th>Book</th>
                    <th>Author/s</th>
                    <th>Published Year</th>
                    <th>Hugo Year</th>
                    <th></th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="book in books" :key="book.title">
                    <td>{{ book.title }}</td>
                    <td>{{ book.author }}</td>
                    <td>{{ book.published }}</td>
                    <td>{{ book.hugoYear }}</td>
                    <td>
                        <button @click="removeBook(book.title)">Remove</button>
                    </td>
                </tr>
                <!-- <BookRow v-for="book in books" :key="book.title" :title="book.title" :author="book.author"
                    :published="book.published" :hugoYear="book.hugoYear" /> -->
            </tbody>
        </table>

        <p>
            <NuxtLink to="/">
                Go to main page
            </NuxtLink>
        </p>
    </div>

</template>