import type { Approver } from '~/types/Approver'

export const useApprovers = () => {
    const fetchApprovers = async (identityNo: number): Promise<Approver[]> => {
        const baseUrl = 'http://localhost:5099'
        return await $fetch<Approver[]>(`${baseUrl}/api/approvers/${identityNo}`)
    }

    return { fetchApprovers }
}