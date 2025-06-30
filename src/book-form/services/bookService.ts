export async function postBook(book: Book): Promise<AddBookResponse> {
    return await $fetch('/api/books', {
        method: 'POST',
        body: book
    })
}