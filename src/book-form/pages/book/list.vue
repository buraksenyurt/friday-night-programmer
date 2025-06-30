<script setup lang="ts">

const { data: books } = await useFetch<Book[]>('/api/books')

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
                </tr>
            </thead>
            <tbody>
                <BookRow v-for="book in books" :key="book.title" :title="book.title" :author="book.author"
                    :published="book.published" :hugoYear="book.hugoYear" />
            </tbody>
        </table>

        <p>
            <NuxtLink to="/">
                Go to main page
            </NuxtLink>
        </p>
    </div>

</template>