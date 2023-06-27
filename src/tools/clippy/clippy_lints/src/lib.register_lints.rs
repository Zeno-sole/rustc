// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

store.register_lints(&[
    #[cfg(feature = "internal")]
    utils::internal_lints::clippy_lints_internal::CLIPPY_LINTS_INTERNAL,
    #[cfg(feature = "internal")]
    utils::internal_lints::collapsible_calls::COLLAPSIBLE_SPAN_LINT_CALLS,
    #[cfg(feature = "internal")]
    utils::internal_lints::compiler_lint_functions::COMPILER_LINT_FUNCTIONS,
    #[cfg(feature = "internal")]
    utils::internal_lints::if_chain_style::IF_CHAIN_STYLE,
    #[cfg(feature = "internal")]
    utils::internal_lints::interning_defined_symbol::INTERNING_DEFINED_SYMBOL,
    #[cfg(feature = "internal")]
    utils::internal_lints::interning_defined_symbol::UNNECESSARY_SYMBOL_STR,
    #[cfg(feature = "internal")]
    utils::internal_lints::invalid_paths::INVALID_PATHS,
    #[cfg(feature = "internal")]
    utils::internal_lints::lint_without_lint_pass::DEFAULT_DEPRECATION_REASON,
    #[cfg(feature = "internal")]
    utils::internal_lints::lint_without_lint_pass::DEFAULT_LINT,
    #[cfg(feature = "internal")]
    utils::internal_lints::lint_without_lint_pass::INVALID_CLIPPY_VERSION_ATTRIBUTE,
    #[cfg(feature = "internal")]
    utils::internal_lints::lint_without_lint_pass::LINT_WITHOUT_LINT_PASS,
    #[cfg(feature = "internal")]
    utils::internal_lints::lint_without_lint_pass::MISSING_CLIPPY_VERSION_ATTRIBUTE,
    #[cfg(feature = "internal")]
    utils::internal_lints::msrv_attr_impl::MISSING_MSRV_ATTR_IMPL,
    #[cfg(feature = "internal")]
    utils::internal_lints::outer_expn_data_pass::OUTER_EXPN_EXPN_DATA,
    #[cfg(feature = "internal")]
    utils::internal_lints::produce_ice::PRODUCE_ICE,
    #[cfg(feature = "internal")]
    utils::internal_lints::unnecessary_def_path::UNNECESSARY_DEF_PATH,
    almost_complete_letter_range::ALMOST_COMPLETE_LETTER_RANGE,
    approx_const::APPROX_CONSTANT,
    as_conversions::AS_CONVERSIONS,
    asm_syntax::INLINE_ASM_X86_ATT_SYNTAX,
    asm_syntax::INLINE_ASM_X86_INTEL_SYNTAX,
    assertions_on_constants::ASSERTIONS_ON_CONSTANTS,
    assertions_on_result_states::ASSERTIONS_ON_RESULT_STATES,
    async_yields_async::ASYNC_YIELDS_ASYNC,
    attrs::ALLOW_ATTRIBUTES_WITHOUT_REASON,
    attrs::BLANKET_CLIPPY_RESTRICTION_LINTS,
    attrs::DEPRECATED_CFG_ATTR,
    attrs::DEPRECATED_SEMVER,
    attrs::EMPTY_LINE_AFTER_OUTER_ATTR,
    attrs::INLINE_ALWAYS,
    attrs::MISMATCHED_TARGET_OS,
    attrs::USELESS_ATTRIBUTE,
    await_holding_invalid::AWAIT_HOLDING_INVALID_TYPE,
    await_holding_invalid::AWAIT_HOLDING_LOCK,
    await_holding_invalid::AWAIT_HOLDING_REFCELL_REF,
    blocks_in_if_conditions::BLOCKS_IN_IF_CONDITIONS,
    bool_assert_comparison::BOOL_ASSERT_COMPARISON,
    bool_to_int_with_if::BOOL_TO_INT_WITH_IF,
    booleans::NONMINIMAL_BOOL,
    booleans::OVERLY_COMPLEX_BOOL_EXPR,
    borrow_deref_ref::BORROW_DEREF_REF,
    box_default::BOX_DEFAULT,
    cargo::CARGO_COMMON_METADATA,
    cargo::MULTIPLE_CRATE_VERSIONS,
    cargo::NEGATIVE_FEATURE_NAMES,
    cargo::REDUNDANT_FEATURE_NAMES,
    cargo::WILDCARD_DEPENDENCIES,
    casts::AS_PTR_CAST_MUT,
    casts::AS_UNDERSCORE,
    casts::BORROW_AS_PTR,
    casts::CAST_ABS_TO_UNSIGNED,
    casts::CAST_ENUM_CONSTRUCTOR,
    casts::CAST_ENUM_TRUNCATION,
    casts::CAST_LOSSLESS,
    casts::CAST_NAN_TO_INT,
    casts::CAST_POSSIBLE_TRUNCATION,
    casts::CAST_POSSIBLE_WRAP,
    casts::CAST_PRECISION_LOSS,
    casts::CAST_PTR_ALIGNMENT,
    casts::CAST_REF_TO_MUT,
    casts::CAST_SIGN_LOSS,
    casts::CAST_SLICE_DIFFERENT_SIZES,
    casts::CAST_SLICE_FROM_RAW_PARTS,
    casts::CHAR_LIT_AS_U8,
    casts::FN_TO_NUMERIC_CAST,
    casts::FN_TO_NUMERIC_CAST_ANY,
    casts::FN_TO_NUMERIC_CAST_WITH_TRUNCATION,
    casts::PTR_AS_PTR,
    casts::UNNECESSARY_CAST,
    checked_conversions::CHECKED_CONVERSIONS,
    cognitive_complexity::COGNITIVE_COMPLEXITY,
    collapsible_if::COLLAPSIBLE_ELSE_IF,
    collapsible_if::COLLAPSIBLE_IF,
    comparison_chain::COMPARISON_CHAIN,
    copies::BRANCHES_SHARING_CODE,
    copies::IFS_SAME_COND,
    copies::IF_SAME_THEN_ELSE,
    copies::SAME_FUNCTIONS_IN_IF_CONDITION,
    copy_iterator::COPY_ITERATOR,
    crate_in_macro_def::CRATE_IN_MACRO_DEF,
    create_dir::CREATE_DIR,
    dbg_macro::DBG_MACRO,
    default::DEFAULT_TRAIT_ACCESS,
    default::FIELD_REASSIGN_WITH_DEFAULT,
    default_instead_of_iter_empty::DEFAULT_INSTEAD_OF_ITER_EMPTY,
    default_numeric_fallback::DEFAULT_NUMERIC_FALLBACK,
    default_union_representation::DEFAULT_UNION_REPRESENTATION,
    dereference::EXPLICIT_AUTO_DEREF,
    dereference::EXPLICIT_DEREF_METHODS,
    dereference::NEEDLESS_BORROW,
    dereference::REF_BINDING_TO_REFERENCE,
    derivable_impls::DERIVABLE_IMPLS,
    derive::DERIVE_HASH_XOR_EQ,
    derive::DERIVE_ORD_XOR_PARTIAL_ORD,
    derive::DERIVE_PARTIAL_EQ_WITHOUT_EQ,
    derive::EXPL_IMPL_CLONE_ON_COPY,
    derive::UNSAFE_DERIVE_DESERIALIZE,
    disallowed_macros::DISALLOWED_MACROS,
    disallowed_methods::DISALLOWED_METHODS,
    disallowed_names::DISALLOWED_NAMES,
    disallowed_script_idents::DISALLOWED_SCRIPT_IDENTS,
    disallowed_types::DISALLOWED_TYPES,
    doc::DOC_LINK_WITH_QUOTES,
    doc::DOC_MARKDOWN,
    doc::MISSING_ERRORS_DOC,
    doc::MISSING_PANICS_DOC,
    doc::MISSING_SAFETY_DOC,
    doc::NEEDLESS_DOCTEST_MAIN,
    double_parens::DOUBLE_PARENS,
    drop_forget_ref::DROP_COPY,
    drop_forget_ref::DROP_NON_DROP,
    drop_forget_ref::DROP_REF,
    drop_forget_ref::FORGET_COPY,
    drop_forget_ref::FORGET_NON_DROP,
    drop_forget_ref::FORGET_REF,
    drop_forget_ref::UNDROPPED_MANUALLY_DROPS,
    duplicate_mod::DUPLICATE_MOD,
    else_if_without_else::ELSE_IF_WITHOUT_ELSE,
    empty_drop::EMPTY_DROP,
    empty_enum::EMPTY_ENUM,
    empty_structs_with_brackets::EMPTY_STRUCTS_WITH_BRACKETS,
    entry::MAP_ENTRY,
    enum_clike::ENUM_CLIKE_UNPORTABLE_VARIANT,
    enum_variants::ENUM_VARIANT_NAMES,
    enum_variants::MODULE_INCEPTION,
    enum_variants::MODULE_NAME_REPETITIONS,
    equatable_if_let::EQUATABLE_IF_LET,
    escape::BOXED_LOCAL,
    eta_reduction::REDUNDANT_CLOSURE,
    eta_reduction::REDUNDANT_CLOSURE_FOR_METHOD_CALLS,
    excessive_bools::FN_PARAMS_EXCESSIVE_BOOLS,
    excessive_bools::STRUCT_EXCESSIVE_BOOLS,
    exhaustive_items::EXHAUSTIVE_ENUMS,
    exhaustive_items::EXHAUSTIVE_STRUCTS,
    exit::EXIT,
    explicit_write::EXPLICIT_WRITE,
    fallible_impl_from::FALLIBLE_IMPL_FROM,
    float_literal::EXCESSIVE_PRECISION,
    float_literal::LOSSY_FLOAT_LITERAL,
    floating_point_arithmetic::IMPRECISE_FLOPS,
    floating_point_arithmetic::SUBOPTIMAL_FLOPS,
    format::USELESS_FORMAT,
    format_args::FORMAT_IN_FORMAT_ARGS,
    format_args::TO_STRING_IN_FORMAT_ARGS,
    format_args::UNINLINED_FORMAT_ARGS,
    format_args::UNUSED_FORMAT_SPECS,
    format_impl::PRINT_IN_FORMAT_IMPL,
    format_impl::RECURSIVE_FORMAT_IMPL,
    format_push_string::FORMAT_PUSH_STRING,
    formatting::POSSIBLE_MISSING_COMMA,
    formatting::SUSPICIOUS_ASSIGNMENT_FORMATTING,
    formatting::SUSPICIOUS_ELSE_FORMATTING,
    formatting::SUSPICIOUS_UNARY_OP_FORMATTING,
    from_over_into::FROM_OVER_INTO,
    from_str_radix_10::FROM_STR_RADIX_10,
    functions::DOUBLE_MUST_USE,
    functions::MUST_USE_CANDIDATE,
    functions::MUST_USE_UNIT,
    functions::NOT_UNSAFE_PTR_ARG_DEREF,
    functions::RESULT_LARGE_ERR,
    functions::RESULT_UNIT_ERR,
    functions::TOO_MANY_ARGUMENTS,
    functions::TOO_MANY_LINES,
    future_not_send::FUTURE_NOT_SEND,
    if_let_mutex::IF_LET_MUTEX,
    if_not_else::IF_NOT_ELSE,
    if_then_some_else_none::IF_THEN_SOME_ELSE_NONE,
    implicit_hasher::IMPLICIT_HASHER,
    implicit_return::IMPLICIT_RETURN,
    implicit_saturating_add::IMPLICIT_SATURATING_ADD,
    implicit_saturating_sub::IMPLICIT_SATURATING_SUB,
    inconsistent_struct_constructor::INCONSISTENT_STRUCT_CONSTRUCTOR,
    index_refutable_slice::INDEX_REFUTABLE_SLICE,
    indexing_slicing::INDEXING_SLICING,
    indexing_slicing::OUT_OF_BOUNDS_INDEXING,
    infinite_iter::INFINITE_ITER,
    infinite_iter::MAYBE_INFINITE_ITER,
    inherent_impl::MULTIPLE_INHERENT_IMPL,
    inherent_to_string::INHERENT_TO_STRING,
    inherent_to_string::INHERENT_TO_STRING_SHADOW_DISPLAY,
    init_numbered_fields::INIT_NUMBERED_FIELDS,
    inline_fn_without_body::INLINE_FN_WITHOUT_BODY,
    int_plus_one::INT_PLUS_ONE,
    invalid_upcast_comparisons::INVALID_UPCAST_COMPARISONS,
    invalid_utf8_in_unchecked::INVALID_UTF8_IN_UNCHECKED,
    items_after_statements::ITEMS_AFTER_STATEMENTS,
    iter_not_returning_iterator::ITER_NOT_RETURNING_ITERATOR,
    large_const_arrays::LARGE_CONST_ARRAYS,
    large_enum_variant::LARGE_ENUM_VARIANT,
    large_include_file::LARGE_INCLUDE_FILE,
    large_stack_arrays::LARGE_STACK_ARRAYS,
    len_zero::COMPARISON_TO_EMPTY,
    len_zero::LEN_WITHOUT_IS_EMPTY,
    len_zero::LEN_ZERO,
    let_if_seq::USELESS_LET_IF_SEQ,
    let_underscore::LET_UNDERSCORE_DROP,
    let_underscore::LET_UNDERSCORE_LOCK,
    let_underscore::LET_UNDERSCORE_MUST_USE,
    lifetimes::EXTRA_UNUSED_LIFETIMES,
    lifetimes::NEEDLESS_LIFETIMES,
    literal_representation::DECIMAL_LITERAL_REPRESENTATION,
    literal_representation::INCONSISTENT_DIGIT_GROUPING,
    literal_representation::LARGE_DIGIT_GROUPS,
    literal_representation::MISTYPED_LITERAL_SUFFIXES,
    literal_representation::UNREADABLE_LITERAL,
    literal_representation::UNUSUAL_BYTE_GROUPINGS,
    loops::EMPTY_LOOP,
    loops::EXPLICIT_COUNTER_LOOP,
    loops::EXPLICIT_INTO_ITER_LOOP,
    loops::EXPLICIT_ITER_LOOP,
    loops::FOR_KV_MAP,
    loops::ITER_NEXT_LOOP,
    loops::MANUAL_FIND,
    loops::MANUAL_FLATTEN,
    loops::MANUAL_MEMCPY,
    loops::MISSING_SPIN_LOOP,
    loops::MUT_RANGE_BOUND,
    loops::NEEDLESS_COLLECT,
    loops::NEEDLESS_RANGE_LOOP,
    loops::NEVER_LOOP,
    loops::SAME_ITEM_PUSH,
    loops::SINGLE_ELEMENT_LOOP,
    loops::WHILE_IMMUTABLE_CONDITION,
    loops::WHILE_LET_LOOP,
    loops::WHILE_LET_ON_ITERATOR,
    macro_use::MACRO_USE_IMPORTS,
    main_recursion::MAIN_RECURSION,
    manual_assert::MANUAL_ASSERT,
    manual_async_fn::MANUAL_ASYNC_FN,
    manual_bits::MANUAL_BITS,
    manual_clamp::MANUAL_CLAMP,
    manual_instant_elapsed::MANUAL_INSTANT_ELAPSED,
    manual_non_exhaustive::MANUAL_NON_EXHAUSTIVE,
    manual_rem_euclid::MANUAL_REM_EUCLID,
    manual_retain::MANUAL_RETAIN,
    manual_string_new::MANUAL_STRING_NEW,
    manual_strip::MANUAL_STRIP,
    map_unit_fn::OPTION_MAP_UNIT_FN,
    map_unit_fn::RESULT_MAP_UNIT_FN,
    match_result_ok::MATCH_RESULT_OK,
    matches::COLLAPSIBLE_MATCH,
    matches::INFALLIBLE_DESTRUCTURING_MATCH,
    matches::MANUAL_FILTER,
    matches::MANUAL_MAP,
    matches::MANUAL_UNWRAP_OR,
    matches::MATCH_AS_REF,
    matches::MATCH_BOOL,
    matches::MATCH_LIKE_MATCHES_MACRO,
    matches::MATCH_ON_VEC_ITEMS,
    matches::MATCH_OVERLAPPING_ARM,
    matches::MATCH_REF_PATS,
    matches::MATCH_SAME_ARMS,
    matches::MATCH_SINGLE_BINDING,
    matches::MATCH_STR_CASE_MISMATCH,
    matches::MATCH_WILDCARD_FOR_SINGLE_VARIANTS,
    matches::MATCH_WILD_ERR_ARM,
    matches::NEEDLESS_MATCH,
    matches::REDUNDANT_PATTERN_MATCHING,
    matches::REST_PAT_IN_FULLY_BOUND_STRUCTS,
    matches::SIGNIFICANT_DROP_IN_SCRUTINEE,
    matches::SINGLE_MATCH,
    matches::SINGLE_MATCH_ELSE,
    matches::TRY_ERR,
    matches::WILDCARD_ENUM_MATCH_ARM,
    matches::WILDCARD_IN_OR_PATTERNS,
    mem_forget::MEM_FORGET,
    mem_replace::MEM_REPLACE_OPTION_WITH_NONE,
    mem_replace::MEM_REPLACE_WITH_DEFAULT,
    mem_replace::MEM_REPLACE_WITH_UNINIT,
    methods::BIND_INSTEAD_OF_MAP,
    methods::BYTES_COUNT_TO_LEN,
    methods::BYTES_NTH,
    methods::CASE_SENSITIVE_FILE_EXTENSION_COMPARISONS,
    methods::CHARS_LAST_CMP,
    methods::CHARS_NEXT_CMP,
    methods::CLONED_INSTEAD_OF_COPIED,
    methods::CLONE_DOUBLE_REF,
    methods::CLONE_ON_COPY,
    methods::CLONE_ON_REF_PTR,
    methods::COLLAPSIBLE_STR_REPLACE,
    methods::ERR_EXPECT,
    methods::EXPECT_FUN_CALL,
    methods::EXPECT_USED,
    methods::EXTEND_WITH_DRAIN,
    methods::FILETYPE_IS_FILE,
    methods::FILTER_MAP_IDENTITY,
    methods::FILTER_MAP_NEXT,
    methods::FILTER_NEXT,
    methods::FLAT_MAP_IDENTITY,
    methods::FLAT_MAP_OPTION,
    methods::FROM_ITER_INSTEAD_OF_COLLECT,
    methods::GET_FIRST,
    methods::GET_LAST_WITH_LEN,
    methods::GET_UNWRAP,
    methods::IMPLICIT_CLONE,
    methods::INEFFICIENT_TO_STRING,
    methods::INSPECT_FOR_EACH,
    methods::INTO_ITER_ON_REF,
    methods::IS_DIGIT_ASCII_RADIX,
    methods::ITERATOR_STEP_BY_ZERO,
    methods::ITER_CLONED_COLLECT,
    methods::ITER_COUNT,
    methods::ITER_KV_MAP,
    methods::ITER_NEXT_SLICE,
    methods::ITER_NTH,
    methods::ITER_NTH_ZERO,
    methods::ITER_ON_EMPTY_COLLECTIONS,
    methods::ITER_ON_SINGLE_ITEMS,
    methods::ITER_OVEREAGER_CLONED,
    methods::ITER_SKIP_NEXT,
    methods::ITER_WITH_DRAIN,
    methods::MANUAL_FILTER_MAP,
    methods::MANUAL_FIND_MAP,
    methods::MANUAL_OK_OR,
    methods::MANUAL_SATURATING_ARITHMETIC,
    methods::MANUAL_SPLIT_ONCE,
    methods::MANUAL_STR_REPEAT,
    methods::MAP_CLONE,
    methods::MAP_COLLECT_RESULT_UNIT,
    methods::MAP_ERR_IGNORE,
    methods::MAP_FLATTEN,
    methods::MAP_IDENTITY,
    methods::MAP_UNWRAP_OR,
    methods::MUT_MUTEX_LOCK,
    methods::NAIVE_BYTECOUNT,
    methods::NEEDLESS_OPTION_AS_DEREF,
    methods::NEEDLESS_OPTION_TAKE,
    methods::NEEDLESS_SPLITN,
    methods::NEW_RET_NO_SELF,
    methods::NONSENSICAL_OPEN_OPTIONS,
    methods::NO_EFFECT_REPLACE,
    methods::OBFUSCATED_IF_ELSE,
    methods::OK_EXPECT,
    methods::OPTION_AS_REF_DEREF,
    methods::OPTION_FILTER_MAP,
    methods::OPTION_MAP_OR_NONE,
    methods::OR_FUN_CALL,
    methods::OR_THEN_UNWRAP,
    methods::PATH_BUF_PUSH_OVERWRITE,
    methods::RANGE_ZIP_WITH_LEN,
    methods::REPEAT_ONCE,
    methods::RESULT_MAP_OR_INTO_OPTION,
    methods::SEARCH_IS_SOME,
    methods::SHOULD_IMPLEMENT_TRAIT,
    methods::SINGLE_CHAR_ADD_STR,
    methods::SINGLE_CHAR_PATTERN,
    methods::SKIP_WHILE_NEXT,
    methods::STABLE_SORT_PRIMITIVE,
    methods::STRING_EXTEND_CHARS,
    methods::SUSPICIOUS_MAP,
    methods::SUSPICIOUS_SPLITN,
    methods::SUSPICIOUS_TO_OWNED,
    methods::UNINIT_ASSUMED_INIT,
    methods::UNIT_HASH,
    methods::UNNECESSARY_FILTER_MAP,
    methods::UNNECESSARY_FIND_MAP,
    methods::UNNECESSARY_FOLD,
    methods::UNNECESSARY_JOIN,
    methods::UNNECESSARY_LAZY_EVALUATIONS,
    methods::UNNECESSARY_SORT_BY,
    methods::UNNECESSARY_TO_OWNED,
    methods::UNWRAP_OR_ELSE_DEFAULT,
    methods::UNWRAP_USED,
    methods::USELESS_ASREF,
    methods::VEC_RESIZE_TO_ZERO,
    methods::VERBOSE_FILE_READS,
    methods::WRONG_SELF_CONVENTION,
    methods::ZST_OFFSET,
    minmax::MIN_MAX,
    misc::SHORT_CIRCUIT_STATEMENT,
    misc::TOPLEVEL_REF_ARG,
    misc::USED_UNDERSCORE_BINDING,
    misc::ZERO_PTR,
    misc_early::BUILTIN_TYPE_SHADOW,
    misc_early::DOUBLE_NEG,
    misc_early::DUPLICATE_UNDERSCORE_ARGUMENT,
    misc_early::MIXED_CASE_HEX_LITERALS,
    misc_early::REDUNDANT_PATTERN,
    misc_early::SEPARATED_LITERAL_SUFFIX,
    misc_early::UNNEEDED_FIELD_PATTERN,
    misc_early::UNNEEDED_WILDCARD_PATTERN,
    misc_early::UNSEPARATED_LITERAL_SUFFIX,
    misc_early::ZERO_PREFIXED_LITERAL,
    mismatching_type_param_order::MISMATCHING_TYPE_PARAM_ORDER,
    missing_const_for_fn::MISSING_CONST_FOR_FN,
    missing_doc::MISSING_DOCS_IN_PRIVATE_ITEMS,
    missing_enforced_import_rename::MISSING_ENFORCED_IMPORT_RENAMES,
    missing_inline::MISSING_INLINE_IN_PUBLIC_ITEMS,
    missing_trait_methods::MISSING_TRAIT_METHODS,
    mixed_read_write_in_expression::DIVERGING_SUB_EXPRESSION,
    mixed_read_write_in_expression::MIXED_READ_WRITE_IN_EXPRESSION,
    module_style::MOD_MODULE_FILES,
    module_style::SELF_NAMED_MODULE_FILES,
    multi_assignments::MULTI_ASSIGNMENTS,
    mut_key::MUTABLE_KEY_TYPE,
    mut_mut::MUT_MUT,
    mut_reference::UNNECESSARY_MUT_PASSED,
    mutable_debug_assertion::DEBUG_ASSERT_WITH_MUT_CALL,
    mutex_atomic::MUTEX_ATOMIC,
    mutex_atomic::MUTEX_INTEGER,
    needless_arbitrary_self_type::NEEDLESS_ARBITRARY_SELF_TYPE,
    needless_bool::BOOL_COMPARISON,
    needless_bool::NEEDLESS_BOOL,
    needless_borrowed_ref::NEEDLESS_BORROWED_REFERENCE,
    needless_continue::NEEDLESS_CONTINUE,
    needless_for_each::NEEDLESS_FOR_EACH,
    needless_late_init::NEEDLESS_LATE_INIT,
    needless_parens_on_range_literals::NEEDLESS_PARENS_ON_RANGE_LITERALS,
    needless_pass_by_value::NEEDLESS_PASS_BY_VALUE,
    needless_question_mark::NEEDLESS_QUESTION_MARK,
    needless_update::NEEDLESS_UPDATE,
    neg_cmp_op_on_partial_ord::NEG_CMP_OP_ON_PARTIAL_ORD,
    neg_multiply::NEG_MULTIPLY,
    new_without_default::NEW_WITHOUT_DEFAULT,
    no_effect::NO_EFFECT,
    no_effect::NO_EFFECT_UNDERSCORE_BINDING,
    no_effect::UNNECESSARY_OPERATION,
    non_copy_const::BORROW_INTERIOR_MUTABLE_CONST,
    non_copy_const::DECLARE_INTERIOR_MUTABLE_CONST,
    non_expressive_names::JUST_UNDERSCORES_AND_DIGITS,
    non_expressive_names::MANY_SINGLE_CHAR_NAMES,
    non_expressive_names::SIMILAR_NAMES,
    non_octal_unix_permissions::NON_OCTAL_UNIX_PERMISSIONS,
    non_send_fields_in_send_ty::NON_SEND_FIELDS_IN_SEND_TY,
    nonstandard_macro_braces::NONSTANDARD_MACRO_BRACES,
    octal_escapes::OCTAL_ESCAPES,
    only_used_in_recursion::ONLY_USED_IN_RECURSION,
    operators::ABSURD_EXTREME_COMPARISONS,
    operators::ARITHMETIC_SIDE_EFFECTS,
    operators::ASSIGN_OP_PATTERN,
    operators::BAD_BIT_MASK,
    operators::CMP_NAN,
    operators::CMP_OWNED,
    operators::DOUBLE_COMPARISONS,
    operators::DURATION_SUBSEC,
    operators::EQ_OP,
    operators::ERASING_OP,
    operators::FLOAT_ARITHMETIC,
    operators::FLOAT_CMP,
    operators::FLOAT_CMP_CONST,
    operators::FLOAT_EQUALITY_WITHOUT_ABS,
    operators::IDENTITY_OP,
    operators::INEFFECTIVE_BIT_MASK,
    operators::INTEGER_ARITHMETIC,
    operators::INTEGER_DIVISION,
    operators::MISREFACTORED_ASSIGN_OP,
    operators::MODULO_ARITHMETIC,
    operators::MODULO_ONE,
    operators::NEEDLESS_BITWISE_BOOL,
    operators::OP_REF,
    operators::PTR_EQ,
    operators::SELF_ASSIGNMENT,
    operators::VERBOSE_BIT_MASK,
    option_env_unwrap::OPTION_ENV_UNWRAP,
    option_if_let_else::OPTION_IF_LET_ELSE,
    overflow_check_conditional::OVERFLOW_CHECK_CONDITIONAL,
    panic_in_result_fn::PANIC_IN_RESULT_FN,
    panic_unimplemented::PANIC,
    panic_unimplemented::TODO,
    panic_unimplemented::UNIMPLEMENTED,
    panic_unimplemented::UNREACHABLE,
    partial_pub_fields::PARTIAL_PUB_FIELDS,
    partialeq_ne_impl::PARTIALEQ_NE_IMPL,
    partialeq_to_none::PARTIALEQ_TO_NONE,
    pass_by_ref_or_value::LARGE_TYPES_PASSED_BY_VALUE,
    pass_by_ref_or_value::TRIVIALLY_COPY_PASS_BY_REF,
    pattern_type_mismatch::PATTERN_TYPE_MISMATCH,
    precedence::PRECEDENCE,
    ptr::CMP_NULL,
    ptr::INVALID_NULL_PTR_USAGE,
    ptr::MUT_FROM_REF,
    ptr::PTR_ARG,
    ptr_offset_with_cast::PTR_OFFSET_WITH_CAST,
    pub_use::PUB_USE,
    question_mark::QUESTION_MARK,
    ranges::MANUAL_RANGE_CONTAINS,
    ranges::RANGE_MINUS_ONE,
    ranges::RANGE_PLUS_ONE,
    ranges::REVERSED_EMPTY_RANGES,
    rc_clone_in_vec_init::RC_CLONE_IN_VEC_INIT,
    read_zero_byte_vec::READ_ZERO_BYTE_VEC,
    redundant_clone::REDUNDANT_CLONE,
    redundant_closure_call::REDUNDANT_CLOSURE_CALL,
    redundant_else::REDUNDANT_ELSE,
    redundant_field_names::REDUNDANT_FIELD_NAMES,
    redundant_pub_crate::REDUNDANT_PUB_CRATE,
    redundant_slicing::DEREF_BY_SLICING,
    redundant_slicing::REDUNDANT_SLICING,
    redundant_static_lifetimes::REDUNDANT_STATIC_LIFETIMES,
    ref_option_ref::REF_OPTION_REF,
    reference::DEREF_ADDROF,
    regex::INVALID_REGEX,
    regex::TRIVIAL_REGEX,
    return_self_not_must_use::RETURN_SELF_NOT_MUST_USE,
    returns::LET_AND_RETURN,
    returns::NEEDLESS_RETURN,
    same_name_method::SAME_NAME_METHOD,
    self_named_constructors::SELF_NAMED_CONSTRUCTORS,
    semicolon_if_nothing_returned::SEMICOLON_IF_NOTHING_RETURNED,
    serde_api::SERDE_API_MISUSE,
    shadow::SHADOW_REUSE,
    shadow::SHADOW_SAME,
    shadow::SHADOW_UNRELATED,
    single_char_lifetime_names::SINGLE_CHAR_LIFETIME_NAMES,
    single_component_path_imports::SINGLE_COMPONENT_PATH_IMPORTS,
    size_of_in_element_count::SIZE_OF_IN_ELEMENT_COUNT,
    slow_vector_initialization::SLOW_VECTOR_INITIALIZATION,
    std_instead_of_core::ALLOC_INSTEAD_OF_CORE,
    std_instead_of_core::STD_INSTEAD_OF_ALLOC,
    std_instead_of_core::STD_INSTEAD_OF_CORE,
    strings::STRING_ADD,
    strings::STRING_ADD_ASSIGN,
    strings::STRING_FROM_UTF8_AS_BYTES,
    strings::STRING_LIT_AS_BYTES,
    strings::STRING_SLICE,
    strings::STRING_TO_STRING,
    strings::STR_TO_STRING,
    strings::TRIM_SPLIT_WHITESPACE,
    strlen_on_c_strings::STRLEN_ON_C_STRINGS,
    suspicious_operation_groupings::SUSPICIOUS_OPERATION_GROUPINGS,
    suspicious_trait_impl::SUSPICIOUS_ARITHMETIC_IMPL,
    suspicious_trait_impl::SUSPICIOUS_OP_ASSIGN_IMPL,
    swap::ALMOST_SWAPPED,
    swap::MANUAL_SWAP,
    swap_ptr_to_ref::SWAP_PTR_TO_REF,
    tabs_in_doc_comments::TABS_IN_DOC_COMMENTS,
    temporary_assignment::TEMPORARY_ASSIGNMENT,
    to_digit_is_some::TO_DIGIT_IS_SOME,
    trailing_empty_array::TRAILING_EMPTY_ARRAY,
    trait_bounds::TRAIT_DUPLICATION_IN_BOUNDS,
    trait_bounds::TYPE_REPETITION_IN_BOUNDS,
    transmute::CROSSPOINTER_TRANSMUTE,
    transmute::TRANSMUTES_EXPRESSIBLE_AS_PTR_CASTS,
    transmute::TRANSMUTE_BYTES_TO_STR,
    transmute::TRANSMUTE_FLOAT_TO_INT,
    transmute::TRANSMUTE_INT_TO_BOOL,
    transmute::TRANSMUTE_INT_TO_CHAR,
    transmute::TRANSMUTE_INT_TO_FLOAT,
    transmute::TRANSMUTE_NUM_TO_BYTES,
    transmute::TRANSMUTE_PTR_TO_PTR,
    transmute::TRANSMUTE_PTR_TO_REF,
    transmute::TRANSMUTE_UNDEFINED_REPR,
    transmute::TRANSMUTING_NULL,
    transmute::UNSOUND_COLLECTION_TRANSMUTE,
    transmute::USELESS_TRANSMUTE,
    transmute::WRONG_TRANSMUTE,
    types::BORROWED_BOX,
    types::BOX_COLLECTION,
    types::LINKEDLIST,
    types::OPTION_OPTION,
    types::RC_BUFFER,
    types::RC_MUTEX,
    types::REDUNDANT_ALLOCATION,
    types::TYPE_COMPLEXITY,
    types::VEC_BOX,
    undocumented_unsafe_blocks::UNDOCUMENTED_UNSAFE_BLOCKS,
    unicode::INVISIBLE_CHARACTERS,
    unicode::NON_ASCII_LITERAL,
    unicode::UNICODE_NOT_NFC,
    uninit_vec::UNINIT_VEC,
    unit_return_expecting_ord::UNIT_RETURN_EXPECTING_ORD,
    unit_types::LET_UNIT_VALUE,
    unit_types::UNIT_ARG,
    unit_types::UNIT_CMP,
    unnamed_address::FN_ADDRESS_COMPARISONS,
    unnamed_address::VTABLE_ADDRESS_COMPARISONS,
    unnecessary_owned_empty_strings::UNNECESSARY_OWNED_EMPTY_STRINGS,
    unnecessary_self_imports::UNNECESSARY_SELF_IMPORTS,
    unnecessary_wraps::UNNECESSARY_WRAPS,
    unnested_or_patterns::UNNESTED_OR_PATTERNS,
    unsafe_removed_from_name::UNSAFE_REMOVED_FROM_NAME,
    unused_async::UNUSED_ASYNC,
    unused_io_amount::UNUSED_IO_AMOUNT,
    unused_peekable::UNUSED_PEEKABLE,
    unused_rounding::UNUSED_ROUNDING,
    unused_self::UNUSED_SELF,
    unused_unit::UNUSED_UNIT,
    unwrap::PANICKING_UNWRAP,
    unwrap::UNNECESSARY_UNWRAP,
    unwrap_in_result::UNWRAP_IN_RESULT,
    upper_case_acronyms::UPPER_CASE_ACRONYMS,
    use_self::USE_SELF,
    useless_conversion::USELESS_CONVERSION,
    vec::USELESS_VEC,
    vec_init_then_push::VEC_INIT_THEN_PUSH,
    wildcard_imports::ENUM_GLOB_USE,
    wildcard_imports::WILDCARD_IMPORTS,
    write::PRINTLN_EMPTY_STRING,
    write::PRINT_LITERAL,
    write::PRINT_STDERR,
    write::PRINT_STDOUT,
    write::PRINT_WITH_NEWLINE,
    write::USE_DEBUG,
    write::WRITELN_EMPTY_STRING,
    write::WRITE_LITERAL,
    write::WRITE_WITH_NEWLINE,
    zero_div_zero::ZERO_DIVIDED_BY_ZERO,
    zero_sized_map_values::ZERO_SIZED_MAP_VALUES,
])