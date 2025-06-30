export async function postBook(book: Book): Promise<AddBookResponse> {
    return await $fetch('/api/books', {
        method: 'POST',
        body: book
    })
}

export async function deleteBook(title: string): Promise<DeleteBookResponse> {
    return await $fetch('/api/books', {
        method: 'DELETE',
        body: { title }
    })
}