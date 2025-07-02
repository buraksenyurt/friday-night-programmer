import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();

export default defineEventHandler(async (event) => {
    const body = await readBody(event);
    if (!body.id || typeof body.id !== 'number') {
        throw createError({
            statusCode: 400,
            statusMessage: 'Invalid issue ID or missing.'
        });
    }

    await prisma.issue.delete({
        where: {
            id: body.id
        }
    });
    return { success: true, message: 'Issue deleted successfully.' };

});
