import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient()

export default defineEventHandler(async (event) => {
    const body = await readBody(event)

    const issue = await prisma.issue.create({
        data: {
            title: body.title,
            owner: body.owner,
            summary: body.summary,
            createDate: body.createDate,
            application: body.application,
            severity: body.severity,
            approver: body.approver
        }
    })

    return issue
})