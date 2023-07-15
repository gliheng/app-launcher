export async function useProjectList() {
  return await useFetch('/api/project/list');
}
