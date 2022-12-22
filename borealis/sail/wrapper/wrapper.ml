open Libsail

type error = Err_exception of string * string | Err_sail of Reporting.error

let exception_to_result f =
  Printexc.record_backtrace true;
  try Ok (f ()) with
  | Reporting.Fatal_error inner -> Error (Err_sail inner)
  | e -> Error (Err_exception (Printexc.to_string e, Printexc.get_backtrace ()))

let bindings_to_list map = map |> Ast_util.Bindings.to_seq |> List.of_seq

let () =
  (* Primary functions *)
  Callback.register "internal_util_dedup" (fun a ->
      exception_to_result (fun () -> Util.remove_duplicates a));

  Callback.register "internal_load_files"
    (fun default_sail_dir options type_envs file_paths ->
      exception_to_result (fun () ->
          Frontend.load_files default_sail_dir options type_envs file_paths));

  Callback.register "internal_descatter" (fun effect_info env ast ->
      exception_to_result (fun () -> Frontend.descatter effect_info env ast));

  Callback.register "internal_type_check_initial_env" (fun () ->
      exception_to_result (fun () -> Type_check.initial_env));

  (* CLI options *)
  Callback.register "internal_set_non_lexical_flow" (fun b ->
      exception_to_result (fun () -> Nl_flow.opt_nl_flow := b));
  Callback.register "internal_set_no_lexp_bounds_check" (fun b ->
      exception_to_result (fun () -> Type_check.opt_no_lexp_bounds_check := b));

  (* Utility *)
  Callback.register "internal_bindings_to_list" (fun a ->
      exception_to_result (fun () -> bindings_to_list a));

  Callback.register "internal_bigint_to_string" (fun i ->
      exception_to_result (fun () -> Nat_big_num.to_string i));

  Callback.register "internal_add_num" (fun a b ->
      exception_to_result (fun () ->
          Num.string_of_num
            (Num.add_num (Num.num_of_string a) (Num.num_of_string b))))
