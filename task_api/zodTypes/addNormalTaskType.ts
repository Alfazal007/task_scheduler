import { z } from "zod"

export const addNormalTaskType = z.object({
    command: z.string({ message: "Command to run not given" }).trim().min(1, { message: "Too short to run" })
})
