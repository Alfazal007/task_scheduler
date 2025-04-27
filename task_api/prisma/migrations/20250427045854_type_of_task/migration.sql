-- CreateEnum
CREATE TYPE "TypeOfTask" AS ENUM ('BASH', 'DOCKER', 'NODE');

-- CreateTable
CREATE TABLE "Task" (
    "id" SERIAL NOT NULL,
    "command" TEXT NOT NULL,
    "scheduledAt" BIGINT NOT NULL,
    "typeOfTask" "TypeOfTask" NOT NULL,

    CONSTRAINT "Task_pkey" PRIMARY KEY ("id")
);
