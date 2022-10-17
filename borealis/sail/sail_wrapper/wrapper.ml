let exception_to_result f =
  Printexc.record_backtrace true;
  try Ok (f ())
  with e -> Error (Printexc.to_string e ^ "\n" ^ Printexc.get_backtrace ())

let () =
  Callback.register "internal_util_dedup" (fun a ->
      exception_to_result (fun () -> Util.remove_duplicates a));

  Callback.register "internal_process_file_load_files" (fun a b c d ->
      exception_to_result (fun () -> Process_file.load_files ?check:a b c d));

  Callback.register "internal_type_check_initial_env" (fun () ->
      exception_to_result (fun () -> Type_check.initial_env))
