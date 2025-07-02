import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();

export default defineEventHandler(async (_) => {
    const issues = await prisma.issue.findMany({
        orderBy: {
            createDate: 'desc'
        }
    });
    return issues
});
