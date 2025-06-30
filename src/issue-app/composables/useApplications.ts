export const useApplications = () => {
    const fetchApplications = async (): Promise<string[]> => {
        //TODO@buraksenyurt Fetch application names from external service
        return ['Invoice', 'Frontend', 'AzonPortal', 'Batch1001', 'CatalogService']
    }

    return { fetchApplications }
}