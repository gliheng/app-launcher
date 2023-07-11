import { prisma } from '../../db';

export default defineEventHandler(async (event) => {
  const query = getQuery(event);
  if (!query.name) throw 'name is required';

  return await prisma.project.findFirst({
    where: {
      name: String(query.name),
    },
  });
})