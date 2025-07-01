export const useApplications = () => {
    const fetchApplications = async (): Promise<string[]> => {
        return await $fetch('/api/applications')
    }

    return { fetchApplications }
}