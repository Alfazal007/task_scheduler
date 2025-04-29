import { z } from "zod"

export const addNormalTaskType = z.object({
    command: z.string({ message: "Command to run not given" }).trim().min(1, { message: "Too short to run" }),
    scheduledAtEpoch: z.number({ message: "Scheduled at not provided" }),
    typeOfTask: z.enum(["BASH", "DOCKER", "NODE"], { message: "Type of task not provided properly, it can be either BASH, DOCKER or NODE" })
})
