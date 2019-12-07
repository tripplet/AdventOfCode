open System.IO
    
let CalcFuelForMass mass = (int)(mass / 3) - 2

let CalcFuelForFuel mass = 
    let mutable fuel = 0
    let mutable currentMass = mass
    while currentMass > 8 do
        currentMass <- CalcFuelForMass currentMass
        fuel <- fuel + currentMass
    fuel

let GetFileContent filename = File.ReadAllLines filename |> Array.map int

[<EntryPoint>]
let main _ =
    let mass = GetFileContent "input.txt"

    let fuelPart1 = mass |> Array.map CalcFuelForMass |> Array.sum    
    printfn "Part1: %A" fuelPart1
        
    let fuelPart2 = mass |> Array.map CalcFuelForFuel |> Array.sum    
    printfn "Part2: %A" fuelPart2

    0    
