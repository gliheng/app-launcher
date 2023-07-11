import { prisma } from '../../db';

export default defineEventHandler(async (event) => {
  const body = await readBody(event);
  return await prisma.project.create({
    data: {
      name: body.name,
      git: body.git,
      wfType: 'static',
      wfArgs: {
        buildScript: 'build',
      },
    },
  });
})