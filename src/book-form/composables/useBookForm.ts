import { reactive } from "vue";

export function useBookForm() {
    const book = reactive<Book>({
        title: '',
        author: '',
        published: 1953,
        hugoYear: 1953
    })

    const reset = () => {
        book.title = '',
            book.author = '',
            book.published = 1953,
            book.hugoYear = 1953
    }

    return { book, reset }
}