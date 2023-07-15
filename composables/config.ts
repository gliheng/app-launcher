const staticWorkflow = {
  workflow: 'static',
  workflowArgs: {
    buildScript: 'build',
  },
};

const nodeWorkflow = {
  workflow: 'nodejs',
  workflowArgs: {
    buildScript: 'build',
    startScript: 'start',
  },
};

const cmdsFromGit = (git: string) => [`degit ${git} source`];

export const templates = {
  'vite-vue': {
    cmds: cmdsFromGit('git@github.com:vitejs/vite.git/packages/create-vite/template-vue#main'),
    ...staticWorkflow,
  },
  'vite-react': {
    cmds: cmdsFromGit('git@github.com:vitejs/vite.git/packages/create-vite/template-react#main'),
    ...staticWorkflow,
  },
  'vite-preact': {
    cmds: cmdsFromGit('git@github.com:vitejs/vite.git/packages/create-vite/template-preact#main'),
    ...staticWorkflow,
  },
  'vite-svelt': {
    cmds: cmdsFromGit('git@github.com:vitejs/vite.git/packages/create-vite/template-svelt#main'),
    ...staticWorkflow,
  },
  'vite-lit': {
    cmds: cmdsFromGit('git@github.com:vitejs/vite.git/packages/create-vite/template-lit#main'),
    ...staticWorkflow,
  },
  'vite-vanilla': {
    cmds: cmdsFromGit('git@github.com:vitejs/vite.git/packages/create-vite/template-vanilla#main'),
    ...staticWorkflow,
  },
  nuxt: {
    cmds: cmdsFromGit('git@github.com:nuxt/starter.git#v3'),
    ...nodeWorkflow,
    workflowArgs: {
      startScript: 'preview',
    },
  },
  nextjs: {
    cmds: [
      'npx create-next-app@latest {name} --ts --use-npm --eslint no --tailwind yes --src-dir no --app yes --import-alias yes',
      'mv {name} source',
    ],
    ...nodeWorkflow,
  },
  nestjs: {
    cmds: [
      'nest new {name} -p npm --skip-install',
      'mv {name} source',
    ],
    ...nodeWorkflow,
  },
};
