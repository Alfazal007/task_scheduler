/*
  Warnings:

  - The values [NODE] on the enum `TypeOfTask` will be removed. If these variants are still used in the database, this will fail.

*/
-- AlterEnum
BEGIN;
CREATE TYPE "TypeOfTask_new" AS ENUM ('BASH', 'DOCKER');
ALTER TABLE "Task" ALTER COLUMN "typeOfTask" TYPE "TypeOfTask_new" USING ("typeOfTask"::text::"TypeOfTask_new");
ALTER TYPE "TypeOfTask" RENAME TO "TypeOfTask_old";
ALTER TYPE "TypeOfTask_new" RENAME TO "TypeOfTask";
DROP TYPE "TypeOfTask_old";
COMMIT;
