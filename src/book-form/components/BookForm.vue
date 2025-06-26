<script setup lang="ts">
import { reactive } from 'vue'

interface Book {
    title: string
    author: string
    published: number | null
    hugoYear: number | null
}

const emit = defineEmits<{
    (e: 'submit', book: Book): void
}>()

const book = reactive<Book>({
    title: '',
    author: '',
    published: null,
    hugoYear: null
})

const submit = () => {
    if (book.title && book.author && book.published && book.hugoYear) {
        emit('submit', { ...book })

        book.title = ''
        book.author = ''
        book.published = null

        book.hugoYear = null
    } else {
        alert("Please fill all form")
    }
}
</script>

<template>
    <form @submit.prevent="submit">
        <table>
            <tr>
                <td>
                    <input v-model="book.title" type="text" placeholder="Title of book" />
                </td>
            </tr>
            <tr>
                <td>
                    <input v-model="book.author" type="text" placeholder="Authors" />
                </td>
            </tr>
            <tr>
                <td>
                    <input v-model.number="book.published" type="number" placeholder="Published Year" />
                </td>
            </tr>
            <tr>
                <td>
                    <input v-model.number="book.hugoYear" type="number" placeholder="Hugo Year" />
                </td>
            </tr>
            <tr>
                <td>
                    <button type="submit">Create</button>
                </td>
            </tr>
        </table>
    </form>
</template>
