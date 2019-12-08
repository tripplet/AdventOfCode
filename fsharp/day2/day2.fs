module Day2

open System.IO

let GetFileContent filename = ((File.ReadAllText filename).Split ',') |> Array.map int

let ProcessInstruction (mem : int[]) pos =
    if mem.[pos] = 1 then
        Array.set mem mem.[pos + 3] (mem.[mem.[pos + 1]] + mem.[mem.[pos + 2]])
        pos + 4
    elif mem.[pos] = 2 then
        Array.set mem mem.[pos + 3] (mem.[mem.[pos + 1]] * mem.[mem.[pos + 2]])
        pos + 4
    elif mem.[pos] = 99 then -1
    else failwith "Unknown instruction"

let Execute memory =
    let mutable pos = 0
    while pos <> -1 do pos <- ProcessInstruction memory pos

let Calc input_memory noun verb =
    let memory = Array.copy input_memory
    Array.set memory 1 noun
    Array.set memory 2 verb
    Execute memory
    memory.[0]

let FindTarget memory target =
    let mutable noun = 0
    let mutable verb = 0
    let mutable found = false

    while noun <= 99 && not found do
        while verb <= 99 && not found do
            found <- (Calc memory noun verb) = target
            if not found then
                verb <- verb + 1
        if not found then
            verb <- 0
            noun <- noun + 1
    if not found then
        failwith "Target not found"
    else
        noun, verb


let solve =
    let data = GetFileContent @"day2\input.txt"
    printfn "Part1: %A" (Calc data 12 2)

    let noun, verb = (FindTarget data 19690720)
    printfn "Part2: %A" (100 * noun + verb)