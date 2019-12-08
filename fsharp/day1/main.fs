open System.IO
    
let CalcFuelForMass mass = (int)(mass / 3) - 2

let rec CalcFuelForFuel mass = 
    if mass > 8 then
        let fuel = (CalcFuelForMass mass)        
        fuel + CalcFuelForFuel fuel
    else
        0

let GetFileContent filename = File.ReadAllLines filename |> Array.map int

[<EntryPoint>]
let main _ =
    let mass = GetFileContent "input.txt"

    let fuelPart1 = mass |> Array.map CalcFuelForMass |> Array.sum    
    printfn "Part1: %A" fuelPart1
        
    let fuelPart2 = mass |> Array.map CalcFuelForFuel |> Array.sum    
    printfn "Part2: %A" fuelPart2

    0    
