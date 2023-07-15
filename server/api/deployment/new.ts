import { prisma } from '../../db';

export default defineEventHandler(async (event) => {
  const body = await readBody(event);
  console.log(
    'body?', body
  );
  return await prisma.deployment.create({
    data: body,
  });
});
