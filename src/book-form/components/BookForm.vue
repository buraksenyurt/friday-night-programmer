<script setup lang="ts">
const { book, reset } = useBookForm()

const emit = defineEmits<{
    (e: 'submit', book: Book): void
}>()

const submit = () => {
    if (book.title && book.author && book.published && book.hugoYear) {
        emit('submit', { ...book })
        reset()
    } else {
        alert("Please fill all form")
    }
}
</script>

<template>
    <form @submit.prevent="submit">
        <table>
            <tbody>
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
            </tbody>
        </table>
    </form>
</template>
