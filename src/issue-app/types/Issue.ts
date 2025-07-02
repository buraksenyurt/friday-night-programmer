import type { Approver } from "./Approver"
import type { Severity } from "./Severtiy"
import type { Status } from "./Status"

export interface Issue {
    id: number,
    title: string,
    owner: string,
    summary: string,
    createDate: string,
    application: string
    severity: Severity
    approver: Approver
    status: Status
}