import express from "express"
import { addNormalTaskType } from "./zodTypes/addNormalTaskType"
import { tryCatch } from "./helpers/tryCatch"
import { prisma } from "./constants/prisma"

const app = express()

app.use(express.json())

app.post("/api/v1/addNormalTask", async (req, res) => {
    const data = req.body
    if (!data) {
        res.status(400).json({
            message: "No request body provided"
        })
        return
    }

    const parsedData = addNormalTaskType.safeParse(data)
    if (!parsedData.success) {
        const errors: string[] = []
        parsedData.error.errors.forEach((err) => {
            errors.push(err.message)
        })
        res.status(400).json({
            message: errors
        })
        return
    }

    const taskWrittenToDBResult = await tryCatch(prisma.task.create({
        data: {
            command: parsedData.data.command,
            typeOfTask: parsedData.data.typeOfTask,
            scheduledAt: parsedData.data.scheduledAtEpoch
        }
    }))
    if (taskWrittenToDBResult.error) {
        res.status(400).json({
            message: "Issue writing to the database"
        })
        return
    } else if (!taskWrittenToDBResult.data) {
        res.status(400).json({
            message: "Issue writing to the database"
        })
        return
    }

    res.status(200).send({
        message: "Added task successfully"
    })
    return
})

app.listen(8000, () => {
    console.log("App listening on port 8000")
})

