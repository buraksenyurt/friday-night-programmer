<script setup lang="ts">
const { showAlert } = useAlert()

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

    await deleteBook(title)
        .then(() => showAlert('success', 'Book removed succesfully'))
        .catch(() => showAlert('error', 'Error on remove operation'));

    // const response = await deleteBook(title)
    // if (!response?.success) {
    //     showAlert('error', response.message)
    //     return
    // }
    // showAlert('success', response.message)
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