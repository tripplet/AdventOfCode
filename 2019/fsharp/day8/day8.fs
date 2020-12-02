module Day8

open System
open System.IO


let GetFileContent filename = (File.ReadAllText filename).ToCharArray() |> Array.map string |> Array.map int

let fill t (len: int) f =
    let xs = Array.CreateInstance(t, len)
    for i in 0..len - 1 do xs.SetValue(f i, i)
    xs

let types (t: Type) n =
    (t, 0)
    |> Seq.unfold (fun (t, i) -> if i = n then None else Some (t, (t.MakeArrayType(), i + 1)))
    |> Seq.toArray
    |> Array.rev

// http://type-nat.ch/post/reshaping-arrays-in-net/
let reshape (arr: Array) (dims: int[]) =
    let t  = arr.GetType().GetElementType()
    let ts = types t dims.Length

    let rec init dim k =
        if dim = dims.Length - 1 then
            fill ts.[dim] dims.[dim] (fun i -> arr.GetValue (k * dims.[dim] + i))
        else
            fill ts.[dim] dims.[dim] (fun i -> init (dim+1) (k * dims.[dim] + i))
    init 0 0

let HowManySatisfy pred = Seq.filter pred >> Seq.length
let GetDigitCount arr digit = arr |> HowManySatisfy (fun d -> d = digit)

let solve =
    let cols = 25
    let rows = 6

    let pixels = GetFileContent @"day8\input.txt"
    let layers = reshape pixels [|pixels.Length / (rows * cols); rows * cols|]

    let count = layers |> (fun arr -> GetDigitCount arr 0)

    printfn "%A" count

    printfn "%A" pixels
