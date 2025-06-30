import { reactive } from "vue";

export function useBookForm() {
    const book = reactive<Book>({
        title: '',
        author: '',
        published: 0,
        hugoYear: 0
    })

    const reset = () => {
        book.title = '',
            book.author = '',
            book.published = 0,
            book.hugoYear = 0
    }

    return { book, reset }
}