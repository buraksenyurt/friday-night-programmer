import type { Approver } from "./approver"
import type { Severity } from "./Severtiy"

export interface Issue {
    id:number,
    title: string,
    owner: string,
    summary: string,
    createDate: string,
    application: string
    severity: Severity
    approver: Approver
}