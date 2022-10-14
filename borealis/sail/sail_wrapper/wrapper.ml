let () =
  Callback.register "internal_util_dedup" Util.remove_duplicates;

  Callback.register "internal_process_file_load_files" (fun a b c d ->
      Process_file.load_files ?check:a b c d);

  Callback.register "internal_type_check_initial_env" (fun () ->
      Type_check.initial_env)
