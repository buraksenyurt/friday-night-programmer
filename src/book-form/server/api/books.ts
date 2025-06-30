let books = [
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
];
export default defineEventHandler(async (event) => {
    if (event.method === 'GET') {
        return books
    }

    if (event.method === 'POST') {
        const body = await readBody(event)
        books.push(body)
        return {
            success: true,
            message: 'Book added successfully'
        }
    }
    
    if (event.method === 'DELETE') {
        const body = await readBody(event)
        const index = books.findIndex(b => b.title === body.title)
        if (index !== -1) {
            books.splice(index, 1)
            return { success: true }
        }
        return { success: false, message: 'Book not found' }
    }

    return books;
})
