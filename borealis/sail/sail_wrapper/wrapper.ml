type error = Err_exception of string * string | Err_sail of Reporting.error

let exception_to_result f =
  Printexc.record_backtrace true;
  try Ok (f ()) with
  | Reporting.Fatal_error inner -> Error (Err_sail inner)
  | e -> Error (Err_exception (Printexc.to_string e, Printexc.get_backtrace ()))

let bindings_to_list map = map |> Ast_util.Bindings.to_seq |> List.of_seq

(* same as Type_check.env but replacing Map and Set types with simpler lists to ease FFI
   type env_wrapper = {
     top_val_specs : (Ast.id * (Ast.typquant * Ast.typ)) list;
     defined_val_specs : Ast_util.IdSet.t;
     locals : (Ast.id * (Ast_util.mut * Ast.typ)) list;
     top_letbinds : Ast_util.IdSet.t;
     union_ids : (Ast.typquant * Ast.typ) list;
     registers : (Ast.effect * Ast.effect * Ast.typ) list;
     variants : (Ast.typquant * Ast.type_union list) list;
     scattered_variant_envs : Type_check.env list;
     mappings : (Ast.typquant * Ast.typ * Ast.typ) list;
     typ_vars : (Ast.l * Ast.kind_aux) Ast_util.KBindings.t;
     shadow_vars : int Ast_util.KBindings.t;
     typ_synonyms : (Ast.typquant * Ast.typ_arg) list;
     overloads : Ast.id list list;
     enums : Ast_util.IdSet.t list;
     records : (Ast.typquant * (Ast.typ * Ast.id) list) list;
     accessors : (Ast.typquant * Ast.typ) list;
     externs : (string * string) list list;
     casts : Ast.id list;
     allow_casts : bool;
     allow_bindings : bool;
     constraints : Ast.n_constraint list;
     default_order : Ast.order option;
     ret_typ : Ast.typ option;
     poly_undefineds : bool;
     prove : (Type_check.env -> Ast.n_constraint -> bool) option;
     allow_unknowns : bool;
     bitfields : (Nat_big_num.num * Nat_big_num.num) list list;
   }

   let wrap_env (input_env : Type_check.Env.t) =
     {
       top_val_specs = bindings_to_list (Type_check.Env.get_val_specs input_env);
       defined_val_specs = Ast_util.IdSet.empty;
       locals = bindings_to_list (Type_check.Env.get_locals input_env);
       top_letbinds = Type_check.Env.get_toplevel_lets input_env;
       union_ids = input_env.union_ids;
       registers = input_env.registers;
       variants = input_env.variants;
       scattered_variant_envs = input_env.scattered_variant_envs;
       mappings = input_env.mappings;
       typ_vars = input_env.typ_vars;
       shadow_vars = input_env.shadow_vars;
       typ_synonyms = input_env.typ_synonyms;
       overloads = input_env.overloads;
       enums = input_env.enums;
       records = input_env.records;
       accessors = input_env.accessors;
       externs = input_env.externs;
       casts = input_env.casts;
       allow_bindings = input_env.allow_bindings;
       allow_casts = input_env.allow_casts;
       constraints = input_env.constraints;
       default_order = input_env.default_order;
       ret_typ = input_env.ret_typ;
       poly_undefineds = input_env.poly_undefineds;
       prove = input_env.prove;
       allow_unknowns = input_env.allow_unknowns;
       bitfields = input_env.bitfields;
     } *)

let () =
  Callback.register "internal_util_dedup" (fun a ->
      exception_to_result (fun () -> Util.remove_duplicates a));

  Callback.register "internal_process_file_load_files" (fun a b c d ->
      exception_to_result (fun () ->
          let name, ast, env = Process_file.load_files ?check:a b c d in
          (name, ast, env)));

  Callback.register "internal_type_check_initial_env" (fun () ->
      exception_to_result (fun () -> Type_check.initial_env));

  Callback.register "internal_bindings_to_list" (fun a ->
      exception_to_result (fun () -> bindings_to_list a));

  Callback.register "internal_dummy" (fun () ->
      exception_to_result (fun () ->
          Ast.P_aux (Ast.P_wild, (Parse_ast.Unknown, 5))))
