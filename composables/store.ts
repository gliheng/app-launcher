export const useProjectStore = defineStore('projectStore', {
  state: () => {
    return {
      list: [] as Record<string, any>[],
    };
  },
  actions: {
    async fetch() {
      const data = await $fetch('/api/project/list');
      this.list = data;
    },
    find(name: string) {
      return this.list.find((e: any) => e.name == name);
    },
    async create(params: any) {
      const data = await $fetch('/api/project/new', {
        method: 'POST',
        body: params,
      });
      this.list.push(data);
    },
  },
});
