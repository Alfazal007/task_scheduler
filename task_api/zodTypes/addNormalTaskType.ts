import { z } from "zod"

export const addNormalTaskType = z.object({
    command: z.string({ message: "Command to run not given" }).trim().min(1, { message: "Too short to run" }),
    scheduledAt: z.string({ message: "Scheduled at not provided" }).refine((val) => !isNaN(Date.parse(val)), {
        message: "Invalid date-time format",
    }).transform((val) => new Date(val)),
    typeOfTask: z.enum(["BASH", "DOCKER", "NODE"], { message: "Type of task not provided properly, it can be either BASH, DOCKER or NODE" })
})
