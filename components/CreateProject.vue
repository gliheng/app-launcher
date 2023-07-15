<script lang="ts" setup>
import { useDialogPluginComponent } from 'quasar';

defineEmits([
  ...useDialogPluginComponent.emits,
]);

const name = ref();
const git = ref();

const templateTypes = ['vite', 'nextjs', 'nuxt', 'nestjs'];
const templateType = ref(templateTypes[0]);
const templateSubTypes = computed(() => templateType.value == 'vite' ? ['vue', 'react', 'preact', 'lit', 'svelt', 'vanilla'] : []);
const templateSubType = ref<string>();

watch(templateType, (t: string) => {
  if (t != 'vite') {
    templateSubType.value = undefined;
  }
});

const type = ref('static');
const importTypes = [{
  label: 'Static',
  value: 'static',
}, {
  label: 'Node',
  value: 'node',
}];

const tab = ref('create');

async function create() {
  let body = {
    name: name.value,
    template: '',
    git: '',
    workflow: 'static',
    workflowArgs: {},
  };

  if (tab.value == 'create') {
    const type = templateType.value;
    const subType = templateSubType.value;
    let tmpl;
    if (subType) {
      tmpl = `${type}-${subType}`;
    } else {
      tmpl = type;
    }
    body.template = tmpl;
    const tmplObj = templates[tmpl];
    body.workflow = tmplObj.workflow;
    body.workflowArgs = tmplObj.workflowArgs;
  } else if (tab.value == 'import') {
    body.template = `git: ${git.value}`;
  }
  const { data } = await useFetch('/api/project/new', {
    method: 'POST',
    body: body,
  })

  navigateTo(`/project/${name.value}`);
}

const { dialogRef, onDialogHide, onDialogOK, onDialogCancel } = useDialogPluginComponent();
async function onOKClick() {
  await create();
  onDialogOK();
}
function onCancelClick() {
  onDialogCancel();
}

</script>

<template>
  <q-dialog ref="dialogRef" @hide="onDialogHide">
    <q-card class="create-project">
      <q-tabs
        v-model="tab"
        dense
      >
        <q-tab name="create" label="Create from template" />
        <q-tab name="import" label="Import git repository" />
      </q-tabs>

      <q-tab-panels v-model="tab" animated>
        <q-tab-panel name="create">
          <q-input label="Project name" v-model="name" />
          <q-select label="Template type" v-model="templateType" :options="templateTypes" />
          <q-select label="Template sub type" v-if="templateSubTypes.length" v-model="templateSubType" :options="templateSubTypes" />
        </q-tab-panel>

        <q-tab-panel name="import">
          <q-input label="Git url" v-model="git" />
          <p>Project type</p>
          <q-btn-toggle v-model="type" :options="importTypes"/>
        </q-tab-panel>
      </q-tab-panels>
      <q-card-actions align="right">
        <q-btn color="primary" label="Create" @click="onOKClick" />
        <q-btn label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<style lang="scss" scoped>
.create-project {
  width: 500px;
}
</style>