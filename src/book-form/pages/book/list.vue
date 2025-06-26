<script setup lang="ts">

interface Book {
    title: string
    author: string
    published: number
    hugoYear: number
}

const { data: books } = await useAsyncData<Book[]>('books', () => {
    return [
        {
            title: "The Left Hand of Darkness",
            author: "Ursula K. Le Guin",
            published: 1969,
            hugoYear: 1970,
        },
        {
            title: "Neuromancer",
            author: "William Gibson",
            published: 1984,
            hugoYear: 1985,
        },
        {
            title: "Hyperion",
            author: "Dan Simmons",
            published: 1989,
            hugoYear: 1990,
        },
        {
            title: "A Deepness in the Sky",
            author: "Vernor Vinge",
            published: 1999,
            hugoYear: 2000,
        },
        {
            title: "The Three-Body Problem",
            author: "Liu Cixin",
            published: 2006,
            hugoYear: 2015,
        }
    ]
})
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

        <NuxtLink to="/">
            Go to main page
        </NuxtLink>
    </div>

</template>