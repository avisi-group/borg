infixr 1 -->

type operator -->('p: Bool, 'q: Bool) -> Bool = not('p) | 'q

let l : list(int) = [|1, 2, 3|]
