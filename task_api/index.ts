import express from "express"
import { addNormalTaskType } from "./zodTypes/addNormalTaskType";

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

    res.status(200).send({
        message: "sent"
    })
    return
})

app.listen(8000, () => {
    console.log("App listening on port 8000")
})

