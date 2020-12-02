module Day1

open System.IO


let CalcFuelForMass mass = (int)(mass / 3) - 2

let rec CalcFuelForFuel mass =
    if mass > 8 then
        let fuel = (CalcFuelForMass mass)
        fuel + CalcFuelForFuel fuel
    else
        0

let GetFileContent filename = File.ReadAllLines filename |> Array.map int

let solve =
    let mass = GetFileContent @"day1\input.txt"

    let fuelPart1 = mass |> Array.map CalcFuelForMass |> Array.sum
    printfn "Part1: %A" fuelPart1

    let fuelPart2 = mass |> Array.map CalcFuelForFuel |> Array.sum
    printfn "Part2: %A" fuelPart2