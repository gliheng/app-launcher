import { prisma } from '../../db';

export default defineEventHandler(async (event) => {
  const query = getQuery(event);
  return await prisma.deployment.findMany({
    where: {
      project: {
        name: String(query.name),
      },
    },
  });
});
