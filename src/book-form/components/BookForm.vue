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
                        <label for="title">Title:</label>
                    </td>
                    <td>
                        <input id="title" v-model="book.title" type="text" placeholder="Title of book" required />
                    </td>
                </tr>
                <tr>
                    <td>
                        <label for="author">Author/Authors:</label>
                    </td>
                    <td>
                        <input id="author" v-model="book.author" type="text" placeholder="Authors" required />
                    </td>
                </tr>
                <tr>
                    <td>
                        <label for="published">Published Year:</label>
                    </td>
                    <td>
                        <input id="published" v-model.number="book.published" type="number" placeholder="Published Year"
                            min="1953" required />
                    </td>
                </tr>
                <tr>
                    <td>
                        <label for="hugoYear">Title:</label>
                    </td>
                    <td>
                        <input id="hugoYear" v-model.number="book.hugoYear" type="number" placeholder="Hugo Year"
                            min="1953" required />
                    </td>
                </tr>
                <tr>
                    <td colspan="2" style="text-align: left;">
                        <button type="submit">Create</button>
                    </td>
                </tr>
            </tbody>
        </table>
    </form>
</template>
