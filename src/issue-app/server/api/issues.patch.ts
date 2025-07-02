import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient()

export default defineEventHandler(async (event) => {
    const body = await readBody(event)
    const { id, status } = body

    if (!id || !status) {
        throw createError({
            statusCode: 400,
            statusMessage: "Missing identity or invalid status"
        })
    }

    const updated = await prisma.issue.update({
        where: { id },
        data: { status },
    })

    return updated
})