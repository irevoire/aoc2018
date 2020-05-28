let alphabet = ['a'; 'b'; 'c'; 'd'; 'e'; 'f'; 'g'; 'h'; 'i'; 'j'; 'k'; 'l'; 'm'; 'n'; 'o'; 'p'; 'q'; 'r'; 's'; 't'; 'u'; 'v'; 'w'; 'x'; 'y'; 'z'];;

let rec count (c: 'a) (l: 'a list): int =
        match l with
        | [] -> 0
        | head::tail when head = c -> 1 + (count c tail)
        | _::tail -> count c tail

let rec _contains_n (alphabet: 'char list) (l: 'char list) (n: int): bool =
        match alphabet with
        | [] -> false
        | head::_ when count head l = n -> true
        | _::tail -> _contains_n tail l n

let contains_n l n = _contains_n alphabet l n;;
let two l = contains_n l 2;;
let three l = contains_n l 3;;

let explode (s: string): 'char list =
        List.init (String.length s) (String.get s)
;;

let rec compute (a: int * int) =
        try
                let (a, b) = a in
                let line = explode (read_line ()) in
                match two line, three line with
                | true, true -> compute (a + 1, b + 1)
                | true, false -> compute (a + 1, b)
                | false, true -> compute (a, b + 1)
                | false, false -> compute (a, b)
        with
        End_of_file -> a
;;

let (a, b) = compute (0, 0);;
Printf.printf "checksum :%d\n" (a * b);
