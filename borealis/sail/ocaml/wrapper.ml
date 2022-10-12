let hello_world () = "test!💖"

let dedup l = Util.remove_duplicates l

let () =
  Callback.register "hello_world" hello_world;
  Callback.register "dedup" dedup
