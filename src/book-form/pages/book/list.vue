<script setup lang="ts">

interface Book {
    title: string
    author: string
    published: number
    hugoYear: number
}

const { data: books } = await useFetch('/api/books')
</script>

<template>
    <div>
        <h1>Hugo Ödüllü Bilim Kurgu Kitapları</h1>

        <table>
            <thead>
                <tr>
                    <th>Eser</th>
                    <th>Yazarı</th>
                    <th>Basım Tarihi</th>
                    <th>Hugo Ödülü</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="book in books" :key="book.title">
                    <td>{{ book.title }}</td>
                    <td>{{ book.author }}</td>
                    <td>{{ book.published }}</td>
                    <td>{{ book.hugoYear }}</td>
                </tr>
            </tbody>
        </table>

        <p>
            <NuxtLink to="/">
                Go to main page
            </NuxtLink>
        </p>
    </div>

</template>