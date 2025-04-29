-- CreateEnum
CREATE TYPE "TypeOfTask" AS ENUM ('BASH', 'DOCKER', 'NODE');

-- CreateTable
CREATE TABLE "Task" (
    "id" SERIAL NOT NULL,
    "command" TEXT NOT NULL,
    "scheduledAt" BIGINT NOT NULL,
    "typeOfTask" "TypeOfTask" NOT NULL,
    "pickedAt" BIGINT NOT NULL DEFAULT 0,
    "startedAt" BIGINT NOT NULL DEFAULT 0,
    "completedAt" BIGINT NOT NULL DEFAULT 0,
    "failedAt" BIGINT NOT NULL DEFAULT 0,

    CONSTRAINT "Task_pkey" PRIMARY KEY ("id")
);
