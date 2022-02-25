(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func))
  (type (;5;) (func (param i32 i32 i32 i32 i32)))
  (type (;6;) (func (param i32 i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;8;) (func (result i64)))
  (type (;9;) (func (param i32 i32)))
  (type (;10;) (func (param i32 i32 i32)))
  (type (;11;) (func (param i32 i32) (result i32)))
  (type (;12;) (func (param i32 i64 i32) (result i64)))
  (type (;13;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;14;) (func (param i64 i32) (result i32)))
  (type (;15;) (func (param i32 i32 i32 i64 i64)))
  (type (;16;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;17;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;18;) (func (param i32 f64 i32 i32 i32 i32) (result i32)))
  (type (;19;) (func (param i64 i32 i32) (result i32)))
  (import "env" "_embind_register_function" (func $_embind_register_function (type 7)))
  (import "env" "_embind_register_void" (func $_embind_register_void (type 9)))
  (import "env" "_embind_register_bool" (func $_embind_register_bool (type 5)))
  (import "env" "_embind_register_std_string" (func $_embind_register_std_string (type 9)))
  (import "env" "_embind_register_std_wstring" (func $_embind_register_std_wstring (type 10)))
  (import "env" "_embind_register_emval" (func $_embind_register_emval (type 9)))
  (import "env" "_embind_register_integer" (func $_embind_register_integer (type 5)))
  (import "env" "_embind_register_bigint" (func $_embind_register_bigint (type 15)))
  (import "env" "_embind_register_float" (func $_embind_register_float (type 10)))
  (import "env" "_embind_register_memory_view" (func $_embind_register_memory_view (type 10)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (type 1)))
  (import "wasi_snapshot_preview1" "fd_write" (func $__wasi_fd_write (type 13)))
  (func $__wasm_call_ctors (type 4)
    (call $_GLOBAL__sub_I_add.cpp)
    (call $_GLOBAL__sub_I_bind.cpp))
  (func $temp_Math*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store
      (local.get 1)
      (local.get 0))
    (drop
      (call $iprintf
        (i32.const 1091)
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $__original_main (type 0) (result i32)
    (local i32)
    (i32.store
      (local.tee 0
        (call $operator_new_unsigned_long_
          (i32.const 4)))
      (i32.const 1824))
    (call $temp_Math*_
      (local.get 0))
    (i32.const 0))
  (func $emscripten::internal::Invoker<int>::invoke_int__*____ (type 2) (param i32) (result i32)
    (call_indirect (type 0)
      (local.get 0)))
  (func $Add::compute_int__int_ (type 3) (param i32 i32 i32) (result i32)
    (i32.add
      (local.get 1)
      (local.get 2)))
  (func $_GLOBAL__sub_I_add.cpp (type 4)
    (call $_embind_register_function
      (i32.const 1094)
      (i32.const 1)
      (i32.const 1860)
      (i32.const 1864)
      (i32.const 1)
      (i32.const 2)))
  (func $_start (type 4)
    (call $__wasm_call_ctors)
    (call $exit
      (call $__original_main))
    (unreachable))
  (func $__getTypeName (type 2) (param i32) (result i32)
    (local i32 i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (local.set 2
      (call $__strdup
        (call $std::type_info::name___const
          (i32.load offset=12
            (local.get 1)))))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16)))
    (local.get 2))
  (func $std::type_info::name___const (type 2) (param i32) (result i32)
    (local i32)
    (i32.store offset=8
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16)))
      (local.get 0))
    (i32.store offset=12
      (local.get 1)
      (i32.load offset=4
        (i32.load offset=8
          (local.get 1))))
    (i32.load offset=12
      (local.get 1)))
  (func $__embind_register_native_and_builtin_types (type 4)
    (call $_embind_register_void
      (call $emscripten::internal::TypeID<void__void>::get__)
      (i32.const 1196))
    (call $_embind_register_bool
      (call $emscripten::internal::TypeID<bool__void>::get__)
      (i32.const 1099)
      (i32.const 1)
      (i32.const 1)
      (i32.const 0))
    (call $void__anonymous_namespace_::register_integer<char>_char_const*_
      (i32.const 1086))
    (call $void__anonymous_namespace_::register_integer<signed_char>_char_const*_
      (i32.const 1079))
    (call $void__anonymous_namespace_::register_integer<unsigned_char>_char_const*_
      (i32.const 1077))
    (call $void__anonymous_namespace_::register_integer<short>_char_const*_
      (i32.const 1043))
    (call $void__anonymous_namespace_::register_integer<unsigned_short>_char_const*_
      (i32.const 1034))
    (call $void__anonymous_namespace_::register_integer<int>_char_const*_
      (i32.const 1058))
    (call $void__anonymous_namespace_::register_integer<unsigned_int>_char_const*_
      (i32.const 1049))
    (call $void__anonymous_namespace_::register_integer<long>_char_const*_
      (i32.const 1129))
    (call $void__anonymous_namespace_::register_integer<unsigned_long>_char_const*_
      (i32.const 1120))
    (call $void__anonymous_namespace_::register_bigint<long_long>_char_const*_
      (i32.const 1069))
    (call $void__anonymous_namespace_::register_bigint<unsigned_long_long>_char_const*_
      (i32.const 1068))
    (call $void__anonymous_namespace_::register_float<float>_char_const*_
      (i32.const 1062))
    (call $void__anonymous_namespace_::register_float<double>_char_const*_
      (i32.const 1189))
    (call $_embind_register_std_string
      (call $emscripten::internal::TypeID<std::__2::basic_string<char__std::__2::char_traits<char>__std::__2::allocator<char>_>__void>::get__)
      (i32.const 1147))
    (call $_embind_register_std_string
      (call $emscripten::internal::TypeID<std::__2::basic_string<unsigned_char__std::__2::char_traits<unsigned_char>__std::__2::allocator<unsigned_char>_>__void>::get__)
      (i32.const 1638))
    (call $_embind_register_std_wstring
      (call $emscripten::internal::TypeID<std::__2::basic_string<wchar_t__std::__2::char_traits<wchar_t>__std::__2::allocator<wchar_t>_>__void>::get__)
      (i32.const 4)
      (i32.const 1134))
    (call $_embind_register_std_wstring
      (call $emscripten::internal::TypeID<std::__2::basic_string<char16_t__std::__2::char_traits<char16_t>__std::__2::allocator<char16_t>_>__void>::get__)
      (i32.const 2)
      (i32.const 1159))
    (call $_embind_register_std_wstring
      (call $emscripten::internal::TypeID<std::__2::basic_string<char32_t__std::__2::char_traits<char32_t>__std::__2::allocator<char32_t>_>__void>::get__)
      (i32.const 4)
      (i32.const 1174))
    (call $_embind_register_emval
      (call $emscripten::internal::TypeID<emscripten::val__void>::get__)
      (i32.const 1104))
    (call $void__anonymous_namespace_::register_memory_view<char>_char_const*_
      (i32.const 1569))
    (call $void__anonymous_namespace_::register_memory_view<signed_char>_char_const*_
      (i32.const 1671))
    (call $void__anonymous_namespace_::register_memory_view<unsigned_char>_char_const*_
      (i32.const 1599))
    (call $void__anonymous_namespace_::register_memory_view<short>_char_const*_
      (i32.const 1201))
    (call $void__anonymous_namespace_::register_memory_view<unsigned_short>_char_const*_
      (i32.const 1232))
    (call $void__anonymous_namespace_::register_memory_view<int>_char_const*_
      (i32.const 1272))
    (call $void__anonymous_namespace_::register_memory_view<unsigned_int>_char_const*_
      (i32.const 1301))
    (call $void__anonymous_namespace_::register_memory_view<long>_char_const*_
      (i32.const 1708))
    (call $void__anonymous_namespace_::register_memory_view<unsigned_long>_char_const*_
      (i32.const 1738))
    (call $void__anonymous_namespace_::register_memory_view<signed_char>_char_const*_
      (i32.const 1403))
    (call $void__anonymous_namespace_::register_memory_view<unsigned_char>_char_const*_
      (i32.const 1370))
    (call $void__anonymous_namespace_::register_memory_view<short>_char_const*_
      (i32.const 1469))
    (call $void__anonymous_namespace_::register_memory_view<unsigned_short>_char_const*_
      (i32.const 1435))
    (call $void__anonymous_namespace_::register_memory_view<int>_char_const*_
      (i32.const 1536))
    (call $void__anonymous_namespace_::register_memory_view<unsigned_int>_char_const*_
      (i32.const 1502))
    (call $void__anonymous_namespace_::register_memory_view<float>_char_const*_
      (i32.const 1339))
    (call $void__anonymous_namespace_::register_memory_view<double>_char_const*_
      (i32.const 1777)))
  (func $emscripten::internal::TypeID<void__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<void>::get__))
  (func $emscripten::internal::TypeID<bool__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<bool>::get__))
  (func $void__anonymous_namespace_::register_integer<char>_char_const*_ (type 1) (param i32)
    (local i32 i32 i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<char__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 1)
      (i32.shr_s
        (i32.shl
          (call $std::__2::numeric_limits<char>::min__)
          (local.tee 2
            (i32.const 24)))
        (local.get 2))
      (i32.shr_s
        (i32.shl
          (call $std::__2::numeric_limits<char>::max__)
          (local.tee 3
            (i32.const 24)))
        (local.get 3)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_integer<signed_char>_char_const*_ (type 1) (param i32)
    (local i32 i32 i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<signed_char__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 1)
      (i32.shr_s
        (i32.shl
          (call $std::__2::numeric_limits<signed_char>::min__)
          (local.tee 2
            (i32.const 24)))
        (local.get 2))
      (i32.shr_s
        (i32.shl
          (call $std::__2::numeric_limits<signed_char>::max__)
          (local.tee 3
            (i32.const 24)))
        (local.get 3)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_integer<unsigned_char>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<unsigned_char__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 1)
      (i32.and
        (call $std::__2::numeric_limits<unsigned_char>::min__)
        (i32.const 255))
      (i32.and
        (call $std::__2::numeric_limits<unsigned_char>::max__)
        (i32.const 255)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_integer<short>_char_const*_ (type 1) (param i32)
    (local i32 i32 i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<short__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 2)
      (i32.shr_s
        (i32.shl
          (call $std::__2::numeric_limits<short>::min__)
          (local.tee 2
            (i32.const 16)))
        (local.get 2))
      (i32.shr_s
        (i32.shl
          (call $std::__2::numeric_limits<short>::max__)
          (local.tee 3
            (i32.const 16)))
        (local.get 3)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_integer<unsigned_short>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<unsigned_short__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 2)
      (i32.and
        (call $std::__2::numeric_limits<unsigned_short>::min__)
        (i32.const 65535))
      (i32.and
        (call $std::__2::numeric_limits<unsigned_short>::max__)
        (i32.const 65535)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_integer<int>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<int__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 4)
      (call $std::__2::numeric_limits<int>::min__)
      (call $std::__2::numeric_limits<int>::max__))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_integer<unsigned_int>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<unsigned_int__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 4)
      (call $std::__2::numeric_limits<unsigned_int>::min__)
      (call $std::__2::numeric_limits<unsigned_int>::max__))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_integer<long>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<long__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 4)
      (call $std::__2::numeric_limits<long>::min__)
      (call $std::__2::numeric_limits<long>::max__))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_integer<unsigned_long>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_integer
      (call $emscripten::internal::TypeID<unsigned_long__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 4)
      (call $std::__2::numeric_limits<unsigned_long>::min__)
      (call $std::__2::numeric_limits<unsigned_long>::max__))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_bigint<long_long>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_bigint
      (call $emscripten::internal::TypeID<long_long__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 8)
      (call $std::__2::numeric_limits<long_long>::min__)
      (call $std::__2::numeric_limits<long_long>::max__))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_bigint<unsigned_long_long>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_bigint
      (call $emscripten::internal::TypeID<unsigned_long_long__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 8)
      (call $std::__2::numeric_limits<unsigned_long_long>::min__)
      (call $std::__2::numeric_limits<unsigned_long_long>::max__))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_float<float>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_float
      (call $emscripten::internal::TypeID<float__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 4))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_float<double>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_float
      (call $emscripten::internal::TypeID<double__void>::get__)
      (i32.load offset=12
        (local.get 1))
      (i32.const 8))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $emscripten::internal::TypeID<std::__2::basic_string<char__std::__2::char_traits<char>__std::__2::allocator<char>_>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<std::__2::basic_string<char__std::__2::char_traits<char>__std::__2::allocator<char>_>_>::get__))
  (func $emscripten::internal::TypeID<std::__2::basic_string<unsigned_char__std::__2::char_traits<unsigned_char>__std::__2::allocator<unsigned_char>_>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<std::__2::basic_string<unsigned_char__std::__2::char_traits<unsigned_char>__std::__2::allocator<unsigned_char>_>_>::get__))
  (func $emscripten::internal::TypeID<std::__2::basic_string<wchar_t__std::__2::char_traits<wchar_t>__std::__2::allocator<wchar_t>_>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<std::__2::basic_string<wchar_t__std::__2::char_traits<wchar_t>__std::__2::allocator<wchar_t>_>_>::get__))
  (func $emscripten::internal::TypeID<std::__2::basic_string<char16_t__std::__2::char_traits<char16_t>__std::__2::allocator<char16_t>_>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<std::__2::basic_string<char16_t__std::__2::char_traits<char16_t>__std::__2::allocator<char16_t>_>_>::get__))
  (func $emscripten::internal::TypeID<std::__2::basic_string<char32_t__std::__2::char_traits<char32_t>__std::__2::allocator<char32_t>_>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<std::__2::basic_string<char32_t__std::__2::char_traits<char32_t>__std::__2::allocator<char32_t>_>_>::get__))
  (func $emscripten::internal::TypeID<emscripten::val__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::val>::get__))
  (func $void__anonymous_namespace_::register_memory_view<char>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<char>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<char>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<signed_char>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<signed_char>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<signed_char>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<unsigned_char>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<unsigned_char>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<unsigned_char>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<short>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<short>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<short>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<unsigned_short>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<unsigned_short>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<unsigned_short>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<int>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<int>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<int>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<unsigned_int>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<unsigned_int>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<unsigned_int>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<long>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<long>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<long>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<unsigned_long>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<unsigned_long>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<unsigned_long>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<float>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<float>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<float>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $void__anonymous_namespace_::register_memory_view<double>_char_const*_ (type 1) (param i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (call $_embind_register_memory_view
      (call $emscripten::internal::TypeID<emscripten::memory_view<double>__void>::get__)
      (call $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<double>__)
      (i32.load offset=12
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16))))
  (func $emscripten::internal::LightTypeID<void>::get__ (type 0) (result i32)
    (i32.const 3032))
  (func $emscripten::internal::LightTypeID<bool>::get__ (type 0) (result i32)
    (i32.const 3044))
  (func $emscripten::internal::TypeID<char__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<char>::get__))
  (func $std::__2::numeric_limits<char>::min__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (call $std::__2::__libcpp_numeric_limits<char__true>::min__)
        (local.tee 0
          (i32.const 24)))
      (local.get 0)))
  (func $std::__2::numeric_limits<char>::max__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (call $std::__2::__libcpp_numeric_limits<char__true>::max__)
        (local.tee 0
          (i32.const 24)))
      (local.get 0)))
  (func $emscripten::internal::TypeID<signed_char__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<signed_char>::get__))
  (func $std::__2::numeric_limits<signed_char>::min__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (call $std::__2::__libcpp_numeric_limits<signed_char__true>::min__)
        (local.tee 0
          (i32.const 24)))
      (local.get 0)))
  (func $std::__2::numeric_limits<signed_char>::max__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (call $std::__2::__libcpp_numeric_limits<signed_char__true>::max__)
        (local.tee 0
          (i32.const 24)))
      (local.get 0)))
  (func $emscripten::internal::TypeID<unsigned_char__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<unsigned_char>::get__))
  (func $std::__2::numeric_limits<unsigned_char>::min__ (type 0) (result i32)
    (i32.and
      (call $std::__2::__libcpp_numeric_limits<unsigned_char__true>::min__)
      (i32.const 255)))
  (func $std::__2::numeric_limits<unsigned_char>::max__ (type 0) (result i32)
    (i32.and
      (call $std::__2::__libcpp_numeric_limits<unsigned_char__true>::max__)
      (i32.const 255)))
  (func $emscripten::internal::TypeID<short__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<short>::get__))
  (func $std::__2::numeric_limits<short>::min__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (call $std::__2::__libcpp_numeric_limits<short__true>::min__)
        (local.tee 0
          (i32.const 16)))
      (local.get 0)))
  (func $std::__2::numeric_limits<short>::max__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (call $std::__2::__libcpp_numeric_limits<short__true>::max__)
        (local.tee 0
          (i32.const 16)))
      (local.get 0)))
  (func $emscripten::internal::TypeID<unsigned_short__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<unsigned_short>::get__))
  (func $std::__2::numeric_limits<unsigned_short>::min__ (type 0) (result i32)
    (i32.and
      (call $std::__2::__libcpp_numeric_limits<unsigned_short__true>::min__)
      (i32.const 65535)))
  (func $std::__2::numeric_limits<unsigned_short>::max__ (type 0) (result i32)
    (i32.and
      (call $std::__2::__libcpp_numeric_limits<unsigned_short__true>::max__)
      (i32.const 65535)))
  (func $emscripten::internal::TypeID<int__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<int>::get__))
  (func $std::__2::numeric_limits<int>::min__ (type 0) (result i32)
    (call $std::__2::__libcpp_numeric_limits<int__true>::min__))
  (func $std::__2::numeric_limits<int>::max__ (type 0) (result i32)
    (call $std::__2::__libcpp_numeric_limits<int__true>::max__))
  (func $emscripten::internal::TypeID<unsigned_int__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<unsigned_int>::get__))
  (func $std::__2::numeric_limits<unsigned_int>::min__ (type 0) (result i32)
    (call $std::__2::__libcpp_numeric_limits<unsigned_int__true>::min__))
  (func $std::__2::numeric_limits<unsigned_int>::max__ (type 0) (result i32)
    (call $std::__2::__libcpp_numeric_limits<unsigned_int__true>::max__))
  (func $emscripten::internal::TypeID<long__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<long>::get__))
  (func $std::__2::numeric_limits<long>::min__ (type 0) (result i32)
    (call $std::__2::__libcpp_numeric_limits<long__true>::min__))
  (func $std::__2::numeric_limits<long>::max__ (type 0) (result i32)
    (call $std::__2::__libcpp_numeric_limits<long__true>::max__))
  (func $emscripten::internal::TypeID<unsigned_long__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<unsigned_long>::get__))
  (func $std::__2::numeric_limits<unsigned_long>::min__ (type 0) (result i32)
    (call $std::__2::__libcpp_numeric_limits<unsigned_long__true>::min__))
  (func $std::__2::numeric_limits<unsigned_long>::max__ (type 0) (result i32)
    (call $std::__2::__libcpp_numeric_limits<unsigned_long__true>::max__))
  (func $emscripten::internal::TypeID<long_long__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<long_long>::get__))
  (func $std::__2::numeric_limits<long_long>::min__ (type 8) (result i64)
    (call $std::__2::__libcpp_numeric_limits<long_long__true>::min__))
  (func $std::__2::numeric_limits<long_long>::max__ (type 8) (result i64)
    (call $std::__2::__libcpp_numeric_limits<long_long__true>::max__))
  (func $emscripten::internal::TypeID<unsigned_long_long__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<unsigned_long_long>::get__))
  (func $std::__2::numeric_limits<unsigned_long_long>::min__ (type 8) (result i64)
    (call $std::__2::__libcpp_numeric_limits<unsigned_long_long__true>::min__))
  (func $std::__2::numeric_limits<unsigned_long_long>::max__ (type 8) (result i64)
    (call $std::__2::__libcpp_numeric_limits<unsigned_long_long__true>::max__))
  (func $emscripten::internal::TypeID<float__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<float>::get__))
  (func $emscripten::internal::TypeID<double__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<double>::get__))
  (func $emscripten::internal::LightTypeID<std::__2::basic_string<char__std::__2::char_traits<char>__std::__2::allocator<char>_>_>::get__ (type 0) (result i32)
    (i32.const 1976))
  (func $emscripten::internal::LightTypeID<std::__2::basic_string<unsigned_char__std::__2::char_traits<unsigned_char>__std::__2::allocator<unsigned_char>_>_>::get__ (type 0) (result i32)
    (i32.const 2064))
  (func $emscripten::internal::LightTypeID<std::__2::basic_string<wchar_t__std::__2::char_traits<wchar_t>__std::__2::allocator<wchar_t>_>_>::get__ (type 0) (result i32)
    (i32.const 2152))
  (func $emscripten::internal::LightTypeID<std::__2::basic_string<char16_t__std::__2::char_traits<char16_t>__std::__2::allocator<char16_t>_>_>::get__ (type 0) (result i32)
    (i32.const 2244))
  (func $emscripten::internal::LightTypeID<std::__2::basic_string<char32_t__std::__2::char_traits<char32_t>__std::__2::allocator<char32_t>_>_>::get__ (type 0) (result i32)
    (i32.const 2336))
  (func $emscripten::internal::LightTypeID<emscripten::val>::get__ (type 0) (result i32)
    (i32.const 2380))
  (func $emscripten::internal::TypeID<emscripten::memory_view<char>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<char>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<char>__ (type 0) (result i32)
    (i32.const 0))
  (func $emscripten::internal::TypeID<emscripten::memory_view<signed_char>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<signed_char>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<signed_char>__ (type 0) (result i32)
    (i32.const 0))
  (func $emscripten::internal::TypeID<emscripten::memory_view<unsigned_char>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<unsigned_char>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<unsigned_char>__ (type 0) (result i32)
    (i32.const 1))
  (func $emscripten::internal::TypeID<emscripten::memory_view<short>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<short>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<short>__ (type 0) (result i32)
    (i32.const 2))
  (func $emscripten::internal::TypeID<emscripten::memory_view<unsigned_short>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<unsigned_short>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<unsigned_short>__ (type 0) (result i32)
    (i32.const 3))
  (func $emscripten::internal::TypeID<emscripten::memory_view<int>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<int>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<int>__ (type 0) (result i32)
    (i32.const 4))
  (func $emscripten::internal::TypeID<emscripten::memory_view<unsigned_int>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<unsigned_int>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<unsigned_int>__ (type 0) (result i32)
    (i32.const 5))
  (func $emscripten::internal::TypeID<emscripten::memory_view<long>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<long>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<long>__ (type 0) (result i32)
    (i32.const 4))
  (func $emscripten::internal::TypeID<emscripten::memory_view<unsigned_long>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<unsigned_long>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<unsigned_long>__ (type 0) (result i32)
    (i32.const 5))
  (func $emscripten::internal::TypeID<emscripten::memory_view<float>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<float>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<float>__ (type 0) (result i32)
    (i32.const 6))
  (func $emscripten::internal::TypeID<emscripten::memory_view<double>__void>::get__ (type 0) (result i32)
    (call $emscripten::internal::LightTypeID<emscripten::memory_view<double>_>::get__))
  (func $_anonymous_namespace_::TypedArrayIndex__anonymous_namespace_::getTypedArrayIndex<double>__ (type 0) (result i32)
    (i32.const 7))
  (func $__cxx_global_var_init (type 4)
    (drop
      (call_indirect (type 2)
        (i32.const 4304)
        (i32.const 5))))
  (func $EmscriptenBindingInitializer_native_and_builtin_types::EmscriptenBindingInitializer_native_and_builtin_types__ (type 2) (param i32) (result i32)
    (local i32 i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 1)
      (local.get 0))
    (local.set 2
      (i32.load offset=12
        (local.get 1)))
    (call $__embind_register_native_and_builtin_types)
    (global.set $__stack_pointer
      (i32.add
        (local.get 1)
        (i32.const 16)))
    (local.get 2))
  (func $emscripten::internal::LightTypeID<char>::get__ (type 0) (result i32)
    (i32.const 3056))
  (func $std::__2::__libcpp_numeric_limits<char__true>::min__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (i32.const 128)
        (local.tee 0
          (i32.const 24)))
      (local.get 0)))
  (func $std::__2::__libcpp_numeric_limits<char__true>::max__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (i32.const 127)
        (local.tee 0
          (i32.const 24)))
      (local.get 0)))
  (func $emscripten::internal::LightTypeID<signed_char>::get__ (type 0) (result i32)
    (i32.const 3080))
  (func $std::__2::__libcpp_numeric_limits<signed_char__true>::min__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (i32.const 128)
        (local.tee 0
          (i32.const 24)))
      (local.get 0)))
  (func $std::__2::__libcpp_numeric_limits<signed_char__true>::max__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (i32.const 127)
        (local.tee 0
          (i32.const 24)))
      (local.get 0)))
  (func $emscripten::internal::LightTypeID<unsigned_char>::get__ (type 0) (result i32)
    (i32.const 3068))
  (func $std::__2::__libcpp_numeric_limits<unsigned_char__true>::min__ (type 0) (result i32)
    (i32.const 0))
  (func $std::__2::__libcpp_numeric_limits<unsigned_char__true>::max__ (type 0) (result i32)
    (i32.const 255))
  (func $emscripten::internal::LightTypeID<short>::get__ (type 0) (result i32)
    (i32.const 3092))
  (func $std::__2::__libcpp_numeric_limits<short__true>::min__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (i32.const 32768)
        (local.tee 0
          (i32.const 16)))
      (local.get 0)))
  (func $std::__2::__libcpp_numeric_limits<short__true>::max__ (type 0) (result i32)
    (local i32)
    (i32.shr_s
      (i32.shl
        (i32.const 32767)
        (local.tee 0
          (i32.const 16)))
      (local.get 0)))
  (func $emscripten::internal::LightTypeID<unsigned_short>::get__ (type 0) (result i32)
    (i32.const 3104))
  (func $std::__2::__libcpp_numeric_limits<unsigned_short__true>::min__ (type 0) (result i32)
    (i32.const 0))
  (func $std::__2::__libcpp_numeric_limits<unsigned_short__true>::max__ (type 0) (result i32)
    (i32.const 65535))
  (func $emscripten::internal::LightTypeID<int>::get__ (type 0) (result i32)
    (i32.const 3116))
  (func $std::__2::__libcpp_numeric_limits<int__true>::min__ (type 0) (result i32)
    (i32.const -2147483648))
  (func $std::__2::__libcpp_numeric_limits<int__true>::max__ (type 0) (result i32)
    (i32.const 2147483647))
  (func $emscripten::internal::LightTypeID<unsigned_int>::get__ (type 0) (result i32)
    (i32.const 3128))
  (func $std::__2::__libcpp_numeric_limits<unsigned_int__true>::min__ (type 0) (result i32)
    (i32.const 0))
  (func $std::__2::__libcpp_numeric_limits<unsigned_int__true>::max__ (type 0) (result i32)
    (i32.const -1))
  (func $emscripten::internal::LightTypeID<long>::get__ (type 0) (result i32)
    (i32.const 3140))
  (func $std::__2::__libcpp_numeric_limits<long__true>::min__ (type 0) (result i32)
    (i32.const -2147483648))
  (func $std::__2::__libcpp_numeric_limits<long__true>::max__ (type 0) (result i32)
    (i32.const 2147483647))
  (func $emscripten::internal::LightTypeID<unsigned_long>::get__ (type 0) (result i32)
    (i32.const 3152))
  (func $std::__2::__libcpp_numeric_limits<unsigned_long__true>::min__ (type 0) (result i32)
    (i32.const 0))
  (func $std::__2::__libcpp_numeric_limits<unsigned_long__true>::max__ (type 0) (result i32)
    (i32.const -1))
  (func $emscripten::internal::LightTypeID<long_long>::get__ (type 0) (result i32)
    (i32.const 3164))
  (func $std::__2::__libcpp_numeric_limits<long_long__true>::min__ (type 8) (result i64)
    (i64.const -9223372036854775808))
  (func $std::__2::__libcpp_numeric_limits<long_long__true>::max__ (type 8) (result i64)
    (i64.const 9223372036854775807))
  (func $emscripten::internal::LightTypeID<unsigned_long_long>::get__ (type 0) (result i32)
    (i32.const 3176))
  (func $std::__2::__libcpp_numeric_limits<unsigned_long_long__true>::min__ (type 8) (result i64)
    (i64.const 0))
  (func $std::__2::__libcpp_numeric_limits<unsigned_long_long__true>::max__ (type 8) (result i64)
    (i64.const -1))
  (func $emscripten::internal::LightTypeID<float>::get__ (type 0) (result i32)
    (i32.const 3188))
  (func $emscripten::internal::LightTypeID<double>::get__ (type 0) (result i32)
    (i32.const 3200))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<char>_>::get__ (type 0) (result i32)
    (i32.const 2420))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<signed_char>_>::get__ (type 0) (result i32)
    (i32.const 2460))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<unsigned_char>_>::get__ (type 0) (result i32)
    (i32.const 2500))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<short>_>::get__ (type 0) (result i32)
    (i32.const 2540))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<unsigned_short>_>::get__ (type 0) (result i32)
    (i32.const 2580))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<int>_>::get__ (type 0) (result i32)
    (i32.const 2620))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<unsigned_int>_>::get__ (type 0) (result i32)
    (i32.const 2660))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<long>_>::get__ (type 0) (result i32)
    (i32.const 2700))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<unsigned_long>_>::get__ (type 0) (result i32)
    (i32.const 2740))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<float>_>::get__ (type 0) (result i32)
    (i32.const 2780))
  (func $emscripten::internal::LightTypeID<emscripten::memory_view<double>_>::get__ (type 0) (result i32)
    (i32.const 2820))
  (func $_GLOBAL__sub_I_bind.cpp (type 4)
    (call $__cxx_global_var_init))
  (func $__strdup (type 2) (param i32) (result i32)
    (local i32 i32)
    (if  ;; label = @1
      (i32.eqz
        (local.tee 2
          (call $dlmalloc
            (local.tee 1
              (i32.add
                (call $strlen
                  (local.get 0))
                (i32.const 1))))))
      (then
        (return
          (i32.const 0))))
    (call $memcpy
      (local.get 2)
      (local.get 0)
      (local.get 1)))
  (func $operator_new_unsigned_long_ (type 2) (param i32) (result i32)
    (local i32)
    (local.set 1
      (select
        (local.get 0)
        (i32.const 1)
        (local.get 0)))
    (block  ;; label = @1
      (loop  ;; label = @2
        (br_if 1 (;@1;)
          (local.tee 0
            (call $dlmalloc
              (local.get 1))))
        (if  ;; label = @3
          (local.tee 0
            (call $std::get_new_handler__))
          (then
            (call_indirect (type 4)
              (local.get 0))
            (br 1 (;@2;)))))
      (call $abort)
      (unreachable))
    (local.get 0))
  (func $operator_delete_void*_ (type 1) (param i32)
    (call $dlfree
      (local.get 0)))
  (func $void__*std::__2::_anonymous_namespace_::__libcpp_atomic_load<void__*___>_void__*_const*_____int____ (type 2) (param i32) (result i32)
    (i32.load
      (local.get 0)))
  (func $std::get_new_handler__ (type 0) (result i32)
    (call $void__*std::__2::_anonymous_namespace_::__libcpp_atomic_load<void__*___>_void__*_const*_____int____
      (i32.const 4308)))
  (func $std::type_info::~type_info__ (type 2) (param i32) (result i32)
    (local.get 0))
  (func $strcmp (type 11) (param i32 i32) (result i32)
    (local i32 i32)
    (local.set 2
      (i32.load8_u
        (local.get 1)))
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.eqz
          (local.tee 3
            (i32.load8_u
              (local.get 0)))))
      (br_if 0 (;@1;)
        (i32.ne
          (local.get 2)
          (local.get 3)))
      (loop  ;; label = @2
        (local.set 2
          (i32.load8_u offset=1
            (local.get 1)))
        (br_if 1 (;@1;)
          (i32.eqz
            (local.tee 3
              (i32.load8_u offset=1
                (local.get 0)))))
        (local.set 1
          (i32.add
            (local.get 1)
            (i32.const 1)))
        (local.set 0
          (i32.add
            (local.get 0)
            (i32.const 1)))
        (br_if 0 (;@2;)
          (i32.eq
            (local.get 2)
            (local.get 3)))))
    (i32.sub
      (local.get 3)
      (local.get 2)))
  (func $__cxxabiv1::__shim_type_info::~__shim_type_info__ (type 2) (param i32) (result i32)
    (drop
      (call $std::type_info::~type_info__
        (local.get 0)))
    (local.get 0))
  (func $__cxxabiv1::__shim_type_info::noop1___const (type 1) (param i32)
    (nop))
  (func $__cxxabiv1::__shim_type_info::noop2___const (type 1) (param i32)
    (nop))
  (func $__cxxabiv1::__fundamental_type_info::~__fundamental_type_info__ (type 1) (param i32)
    (drop
      (call $__cxxabiv1::__shim_type_info::~__shim_type_info__
        (local.get 0)))
    (call $operator_delete_void*_
      (local.get 0)))
  (func $__cxxabiv1::__class_type_info::~__class_type_info__ (type 1) (param i32)
    (drop
      (call $__cxxabiv1::__shim_type_info::~__shim_type_info__
        (local.get 0)))
    (call $operator_delete_void*_
      (local.get 0)))
  (func $__cxxabiv1::__si_class_type_info::~__si_class_type_info__ (type 1) (param i32)
    (drop
      (call $__cxxabiv1::__shim_type_info::~__shim_type_info__
        (local.get 0)))
    (call $operator_delete_void*_
      (local.get 0)))
  (func $__cxxabiv1::__vmi_class_type_info::~__vmi_class_type_info__ (type 1) (param i32)
    (drop
      (call $__cxxabiv1::__shim_type_info::~__shim_type_info__
        (local.get 0)))
    (call $operator_delete_void*_
      (local.get 0)))
  (func $__cxxabiv1::__fundamental_type_info::can_catch___cxxabiv1::__shim_type_info_const*__void*&__const (type 3) (param i32 i32 i32) (result i32)
    (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
      (local.get 0)
      (local.get 1)
      (i32.const 0)))
  (func $is_equal_std::type_info_const*__std::type_info_const*__bool_ (type 3) (param i32 i32 i32) (result i32)
    (if  ;; label = @1
      (i32.eqz
        (local.get 2))
      (then
        (return
          (i32.eq
            (i32.load offset=4
              (local.get 0))
            (i32.load offset=4
              (local.get 1))))))
    (if  ;; label = @1
      (i32.eq
        (local.get 0)
        (local.get 1))
      (then
        (return
          (i32.const 1))))
    (i32.eqz
      (call $strcmp
        (call $std::type_info::name___const
          (local.get 0))
        (call $std::type_info::name___const
          (local.get 1)))))
  (func $__cxxabiv1::__class_type_info::can_catch___cxxabiv1::__shim_type_info_const*__void*&__const (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32)
    (global.set $__stack_pointer
      (local.tee 3
        (i32.add
          (global.get $__stack_pointer)
          (i32.const -64))))
    (local.set 4
      (i32.const 1))
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
          (local.get 0)
          (local.get 1)
          (i32.const 0)))
      (local.set 4
        (i32.const 0))
      (br_if 0 (;@1;)
        (i32.eqz
          (local.get 1)))
      (br_if 0 (;@1;)
        (i32.eqz
          (local.tee 1
            (call $__dynamic_cast
              (local.get 1)
              (i32.const 2888)
              (i32.const 2936)
              (i32.const 0)))))
      (drop
        (call $memset
          (i32.or
            (i32.add
              (local.get 3)
              (i32.const 8))
            (i32.const 4))
          (i32.const 0)
          (i32.const 52)))
      (i32.store offset=56
        (local.get 3)
        (i32.const 1))
      (i32.store offset=20
        (local.get 3)
        (i32.const -1))
      (i32.store offset=16
        (local.get 3)
        (local.get 0))
      (i32.store offset=8
        (local.get 3)
        (local.get 1))
      (call_indirect (type 6)
        (local.get 1)
        (i32.add
          (local.get 3)
          (i32.const 8))
        (i32.load
          (local.get 2))
        (i32.const 1)
        (i32.load offset=28
          (i32.load
            (local.get 1))))
      (if  ;; label = @2
        (i32.eq
          (local.tee 4
            (i32.load offset=32
              (local.get 3)))
          (i32.const 1))
        (then
          (i32.store
            (local.get 2)
            (i32.load offset=24
              (local.get 3)))))
      (local.set 4
        (i32.eq
          (local.get 4)
          (i32.const 1))))
    (global.set $__stack_pointer
      (i32.sub
        (local.get 3)
        (i32.const -64)))
    (local.get 4))
  (func $__dynamic_cast (type 13) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    (global.set $__stack_pointer
      (local.tee 4
        (i32.add
          (global.get $__stack_pointer)
          (i32.const -64))))
    (local.set 5
      (i32.load
        (i32.sub
          (local.tee 6
            (i32.load
              (local.get 0)))
          (i32.const 4))))
    (local.set 6
      (i32.load
        (i32.sub
          (local.get 6)
          (i32.const 8))))
    (i32.store offset=20
      (local.get 4)
      (local.get 3))
    (i32.store offset=16
      (local.get 4)
      (local.get 1))
    (i32.store offset=12
      (local.get 4)
      (local.get 0))
    (i32.store offset=8
      (local.get 4)
      (local.get 2))
    (local.set 1
      (i32.const 0))
    (drop
      (call $memset
        (i32.add
          (local.get 4)
          (i32.const 24))
        (i32.const 0)
        (i32.const 39)))
    (local.set 0
      (i32.add
        (local.get 0)
        (local.get 6)))
    (block  ;; label = @1
      (if  ;; label = @2
        (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
          (local.get 5)
          (local.get 2)
          (i32.const 0))
        (then
          (i32.store offset=56
            (local.get 4)
            (i32.const 1))
          (call_indirect (type 7)
            (local.get 5)
            (i32.add
              (local.get 4)
              (i32.const 8))
            (local.get 0)
            (local.get 0)
            (i32.const 1)
            (i32.const 0)
            (i32.load offset=20
              (i32.load
                (local.get 5))))
          (local.set 1
            (select
              (local.get 0)
              (i32.const 0)
              (i32.eq
                (i32.load offset=32
                  (local.get 4))
                (i32.const 1))))
          (br 1 (;@1;))))
      (call_indirect (type 5)
        (local.get 5)
        (i32.add
          (local.get 4)
          (i32.const 8))
        (local.get 0)
        (i32.const 1)
        (i32.const 0)
        (i32.load offset=24
          (i32.load
            (local.get 5))))
      (block  ;; label = @2
        (block  ;; label = @3
          (br_table 0 (;@3;) 1 (;@2;) 2 (;@1;)
            (i32.load offset=44
              (local.get 4))))
        (local.set 1
          (select
            (select
              (select
                (i32.load offset=28
                  (local.get 4))
                (i32.const 0)
                (i32.eq
                  (i32.load offset=40
                    (local.get 4))
                  (i32.const 1)))
              (i32.const 0)
              (i32.eq
                (i32.load offset=36
                  (local.get 4))
                (i32.const 1)))
            (i32.const 0)
            (i32.eq
              (i32.load offset=48
                (local.get 4))
              (i32.const 1))))
        (br 1 (;@1;)))
      (if  ;; label = @2
        (i32.ne
          (i32.load offset=32
            (local.get 4))
          (i32.const 1))
        (then
          (br_if 1 (;@1;)
            (i32.load offset=48
              (local.get 4)))
          (br_if 1 (;@1;)
            (i32.ne
              (i32.load offset=36
                (local.get 4))
              (i32.const 1)))
          (br_if 1 (;@1;)
            (i32.ne
              (i32.load offset=40
                (local.get 4))
              (i32.const 1)))))
      (local.set 1
        (i32.load offset=24
          (local.get 4))))
    (global.set $__stack_pointer
      (i32.sub
        (local.get 4)
        (i32.const -64)))
    (local.get 1))
  (func $__cxxabiv1::__class_type_info::process_found_base_class___cxxabiv1::__dynamic_cast_info*__void*__int__const (type 6) (param i32 i32 i32 i32)
    (local i32)
    (if  ;; label = @1
      (i32.eqz
        (local.tee 4
          (i32.load offset=16
            (local.get 1))))
      (then
        (i32.store offset=36
          (local.get 1)
          (i32.const 1))
        (i32.store offset=24
          (local.get 1)
          (local.get 3))
        (i32.store offset=16
          (local.get 1)
          (local.get 2))
        (return)))
    (block  ;; label = @1
      (if  ;; label = @2
        (i32.eq
          (local.get 2)
          (local.get 4))
        (then
          (br_if 1 (;@1;)
            (i32.ne
              (i32.load offset=24
                (local.get 1))
              (i32.const 2)))
          (i32.store offset=24
            (local.get 1)
            (local.get 3))
          (return)))
      (i32.store8 offset=54
        (local.get 1)
        (i32.const 1))
      (i32.store offset=24
        (local.get 1)
        (i32.const 2))
      (i32.store offset=36
        (local.get 1)
        (i32.add
          (i32.load offset=36
            (local.get 1))
          (i32.const 1)))))
  (func $__cxxabiv1::__class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const (type 6) (param i32 i32 i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (i32.const 0))
      (then
        (call $__cxxabiv1::__class_type_info::process_found_base_class___cxxabiv1::__dynamic_cast_info*__void*__int__const
          (local.get 1)
          (local.get 1)
          (local.get 2)
          (local.get 3)))))
  (func $__cxxabiv1::__si_class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const (type 6) (param i32 i32 i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (i32.const 0))
      (then
        (call $__cxxabiv1::__class_type_info::process_found_base_class___cxxabiv1::__dynamic_cast_info*__void*__int__const
          (local.get 1)
          (local.get 1)
          (local.get 2)
          (local.get 3))
        (return)))
    (call_indirect (type 6)
      (local.tee 0
        (i32.load offset=8
          (local.get 0)))
      (local.get 1)
      (local.get 2)
      (local.get 3)
      (i32.load offset=28
        (i32.load
          (local.get 0)))))
  (func $__cxxabiv1::__base_class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const (type 6) (param i32 i32 i32 i32)
    (local i32 i32)
    (local.set 4
      (i32.load offset=4
        (local.get 0)))
    (local.set 5
      (block (result i32)  ;; label = @1
        (drop
          (br_if 0 (;@1;)
            (i32.const 0)
            (i32.eqz
              (local.get 2))))
        (drop
          (br_if 0 (;@1;)
            (local.tee 5
              (i32.shr_s
                (local.get 4)
                (i32.const 8)))
            (i32.eqz
              (i32.and
                (local.get 4)
                (i32.const 1)))))
        (call $update_offset_to_base_char_const*__long_
          (i32.load
            (local.get 2))
          (local.get 5))))
    (call_indirect (type 6)
      (local.tee 0
        (i32.load
          (local.get 0)))
      (local.get 1)
      (i32.add
        (local.get 2)
        (local.get 5))
      (select
        (local.get 3)
        (i32.const 2)
        (i32.and
          (local.get 4)
          (i32.const 2)))
      (i32.load offset=28
        (i32.load
          (local.get 0)))))
  (func $update_offset_to_base_char_const*__long_ (type 11) (param i32 i32) (result i32)
    (i32.load
      (i32.add
        (local.get 0)
        (local.get 1))))
  (func $__cxxabiv1::__vmi_class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const (type 6) (param i32 i32 i32 i32)
    (local i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (i32.const 0))
      (then
        (call $__cxxabiv1::__class_type_info::process_found_base_class___cxxabiv1::__dynamic_cast_info*__void*__int__const
          (local.get 0)
          (local.get 1)
          (local.get 2)
          (local.get 3))
        (return)))
    (local.set 4
      (i32.load offset=12
        (local.get 0)))
    (call $__cxxabiv1::__base_class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const
      (local.tee 5
        (i32.add
          (local.get 0)
          (i32.const 16)))
      (local.get 1)
      (local.get 2)
      (local.get 3))
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.lt_s
          (local.get 4)
          (i32.const 2)))
      (local.set 4
        (i32.add
          (local.get 5)
          (i32.shl
            (local.get 4)
            (i32.const 3))))
      (local.set 0
        (i32.add
          (local.get 0)
          (i32.const 24)))
      (loop  ;; label = @2
        (call $__cxxabiv1::__base_class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const
          (local.get 0)
          (local.get 1)
          (local.get 2)
          (local.get 3))
        (br_if 1 (;@1;)
          (i32.load8_u offset=54
            (local.get 1)))
        (br_if 0 (;@2;)
          (i32.lt_u
            (local.tee 0
              (i32.add
                (local.get 0)
                (i32.const 8)))
            (local.get 4))))))
  (func $__cxxabiv1::__class_type_info::process_static_type_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__const (type 5) (param i32 i32 i32 i32 i32)
    (i32.store8 offset=53
      (local.get 1)
      (i32.const 1))
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.ne
          (i32.load offset=4
            (local.get 1))
          (local.get 3)))
      (i32.store8 offset=52
        (local.get 1)
        (i32.const 1))
      (block  ;; label = @2
        (if  ;; label = @3
          (i32.eqz
            (local.tee 3
              (i32.load offset=16
                (local.get 1))))
          (then
            (i32.store offset=36
              (local.get 1)
              (i32.const 1))
            (i32.store offset=24
              (local.get 1)
              (local.get 4))
            (i32.store offset=16
              (local.get 1)
              (local.get 2))
            (br_if 2 (;@1;)
              (i32.ne
                (i32.load offset=48
                  (local.get 1))
                (i32.const 1)))
            (br_if 1 (;@2;)
              (i32.eq
                (local.get 4)
                (i32.const 1)))
            (br 2 (;@1;))))
        (if  ;; label = @3
          (i32.eq
            (local.get 2)
            (local.get 3))
          (then
            (if  ;; label = @4
              (i32.eq
                (local.tee 3
                  (i32.load offset=24
                    (local.get 1)))
                (i32.const 2))
              (then
                (i32.store offset=24
                  (local.get 1)
                  (local.get 4))
                (local.set 3
                  (local.get 4))))
            (br_if 2 (;@1;)
              (i32.ne
                (i32.load offset=48
                  (local.get 1))
                (i32.const 1)))
            (br_if 1 (;@2;)
              (i32.eq
                (local.get 3)
                (i32.const 1)))
            (br 2 (;@1;))))
        (i32.store offset=36
          (local.get 1)
          (i32.add
            (i32.load offset=36
              (local.get 1))
            (i32.const 1))))
      (i32.store8 offset=54
        (local.get 1)
        (i32.const 1))))
  (func $__cxxabiv1::__class_type_info::process_static_type_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__const (type 6) (param i32 i32 i32 i32)
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.ne
          (i32.load offset=4
            (local.get 1))
          (local.get 2)))
      (br_if 0 (;@1;)
        (i32.eq
          (i32.load offset=28
            (local.get 1))
          (i32.const 1)))
      (i32.store offset=28
        (local.get 1)
        (local.get 3))))
  (func $__cxxabiv1::__vmi_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const (type 5) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (local.get 4))
      (then
        (call $__cxxabiv1::__class_type_info::process_static_type_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__const
          (local.get 1)
          (local.get 1)
          (local.get 2)
          (local.get 3))
        (return)))
    (block  ;; label = @1
      (if  ;; label = @2
        (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
          (local.get 0)
          (i32.load
            (local.get 1))
          (local.get 4))
        (then
          (block  ;; label = @3
            (if  ;; label = @4
              (i32.ne
                (local.get 2)
                (i32.load offset=16
                  (local.get 1)))
              (then
                (br_if 1 (;@3;)
                  (i32.ne
                    (i32.load offset=20
                      (local.get 1))
                    (local.get 2)))))
            (br_if 2 (;@1;)
              (i32.ne
                (local.get 3)
                (i32.const 1)))
            (i32.store offset=32
              (local.get 1)
              (i32.const 1))
            (return))
          (i32.store offset=32
            (local.get 1)
            (local.get 3))
          (if  ;; label = @3
            (i32.ne
              (i32.load offset=44
                (local.get 1))
              (i32.const 4))
            (then
              (local.set 3
                (i32.add
                  (local.tee 5
                    (i32.add
                      (local.get 0)
                      (i32.const 16)))
                  (i32.shl
                    (i32.load offset=12
                      (local.get 0))
                    (i32.const 3))))
              (i32.store offset=44
                (local.get 1)
                (local.tee 5
                  (block (result i32)  ;; label = @4
                    (block  ;; label = @5
                      (loop  ;; label = @6
                        (block  ;; label = @7
                          (br_if 0 (;@7;)
                            (i32.le_u
                              (local.get 3)
                              (local.get 5)))
                          (i32.store16 offset=52
                            (local.get 1)
                            (i32.const 0))
                          (call $__cxxabiv1::__base_class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const
                            (local.get 5)
                            (local.get 1)
                            (local.get 2)
                            (local.get 2)
                            (i32.const 1)
                            (local.get 4))
                          (br_if 0 (;@7;)
                            (i32.load8_u offset=54
                              (local.get 1)))
                          (block  ;; label = @8
                            (br_if 0 (;@8;)
                              (i32.eqz
                                (i32.load8_u offset=53
                                  (local.get 1))))
                            (if  ;; label = @9
                              (i32.load8_u offset=52
                                (local.get 1))
                              (then
                                (local.set 6
                                  (i32.const 1))
                                (br_if 4 (;@5;)
                                  (i32.eq
                                    (i32.load offset=24
                                      (local.get 1))
                                    (i32.const 1)))
                                (local.set 7
                                  (i32.const 1))
                                (local.set 8
                                  (i32.const 1))
                                (br_if 1 (;@8;)
                                  (i32.and
                                    (i32.load8_u offset=8
                                      (local.get 0))
                                    (i32.const 2)))
                                (br 4 (;@5;))))
                            (local.set 7
                              (i32.const 1))
                            (local.set 6
                              (local.get 8))
                            (br_if 3 (;@5;)
                              (i32.eqz
                                (i32.and
                                  (i32.load8_u offset=8
                                    (local.get 0))
                                  (i32.const 1)))))
                          (local.set 5
                            (i32.add
                              (local.get 5)
                              (i32.const 8)))
                          (br 1 (;@6;))))
                      (local.set 6
                        (local.get 8))
                      (drop
                        (br_if 1 (;@4;)
                          (local.tee 5
                            (i32.const 4))
                          (i32.eqz
                            (local.get 7)))))
                    (i32.const 3))))
              (br_if 2 (;@1;)
                (i32.and
                  (local.get 6)
                  (i32.const 1)))))
          (i32.store offset=20
            (local.get 1)
            (local.get 2))
          (i32.store offset=40
            (local.get 1)
            (i32.add
              (i32.load offset=40
                (local.get 1))
              (i32.const 1)))
          (br_if 1 (;@1;)
            (i32.ne
              (i32.load offset=36
                (local.get 1))
              (i32.const 1)))
          (br_if 1 (;@1;)
            (i32.ne
              (i32.load offset=24
                (local.get 1))
              (i32.const 2)))
          (i32.store8 offset=54
            (local.get 1)
            (i32.const 1))
          (return)))
      (local.set 5
        (i32.load offset=12
          (local.get 0)))
      (call $__cxxabiv1::__base_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const
        (local.tee 6
          (i32.add
            (local.get 0)
            (i32.const 16)))
        (local.get 1)
        (local.get 2)
        (local.get 3)
        (local.get 4))
      (br_if 0 (;@1;)
        (i32.lt_s
          (local.get 5)
          (i32.const 2)))
      (local.set 6
        (i32.add
          (local.get 6)
          (i32.shl
            (local.get 5)
            (i32.const 3))))
      (local.set 5
        (i32.add
          (local.get 0)
          (i32.const 24)))
      (block  ;; label = @2
        (if  ;; label = @3
          (i32.eqz
            (i32.and
              (local.tee 0
                (i32.load offset=8
                  (local.get 0)))
              (i32.const 2)))
          (then
            (br_if 1 (;@2;)
              (i32.ne
                (i32.load offset=36
                  (local.get 1))
                (i32.const 1)))))
        (loop  ;; label = @3
          (br_if 2 (;@1;)
            (i32.load8_u offset=54
              (local.get 1)))
          (call $__cxxabiv1::__base_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const
            (local.get 5)
            (local.get 1)
            (local.get 2)
            (local.get 3)
            (local.get 4))
          (br_if 0 (;@3;)
            (i32.lt_u
              (local.tee 5
                (i32.add
                  (local.get 5)
                  (i32.const 8)))
              (local.get 6))))
        (br 1 (;@1;)))
      (if  ;; label = @2
        (i32.eqz
          (i32.and
            (local.get 0)
            (i32.const 1)))
        (then
          (loop  ;; label = @3
            (br_if 2 (;@1;)
              (i32.load8_u offset=54
                (local.get 1)))
            (br_if 2 (;@1;)
              (i32.eq
                (i32.load offset=36
                  (local.get 1))
                (i32.const 1)))
            (call $__cxxabiv1::__base_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const
              (local.get 5)
              (local.get 1)
              (local.get 2)
              (local.get 3)
              (local.get 4))
            (br_if 0 (;@3;)
              (i32.lt_u
                (local.tee 5
                  (i32.add
                    (local.get 5)
                    (i32.const 8)))
                (local.get 6)))
            (br 2 (;@1;)))
          (unreachable)))
      (loop  ;; label = @2
        (br_if 1 (;@1;)
          (i32.load8_u offset=54
            (local.get 1)))
        (if  ;; label = @3
          (i32.eq
            (i32.load offset=36
              (local.get 1))
            (i32.const 1))
          (then
            (br_if 2 (;@1;)
              (i32.eq
                (i32.load offset=24
                  (local.get 1))
                (i32.const 1)))))
        (call $__cxxabiv1::__base_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const
          (local.get 5)
          (local.get 1)
          (local.get 2)
          (local.get 3)
          (local.get 4))
        (br_if 0 (;@2;)
          (i32.lt_u
            (local.tee 5
              (i32.add
                (local.get 5)
                (i32.const 8)))
            (local.get 6))))))
  (func $__cxxabiv1::__base_class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const (type 7) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32)
    (local.set 6
      (i32.shr_s
        (local.tee 7
          (i32.load offset=4
            (local.get 0)))
        (i32.const 8)))
    (if  ;; label = @1
      (i32.and
        (local.get 7)
        (i32.const 1))
      (then
        (local.set 6
          (call $update_offset_to_base_char_const*__long_
            (i32.load
              (local.get 3))
            (local.get 6)))))
    (call_indirect (type 7)
      (local.tee 0
        (i32.load
          (local.get 0)))
      (local.get 1)
      (local.get 2)
      (i32.add
        (local.get 3)
        (local.get 6))
      (select
        (local.get 4)
        (i32.const 2)
        (i32.and
          (local.get 7)
          (i32.const 2)))
      (local.get 5)
      (i32.load offset=20
        (i32.load
          (local.get 0)))))
  (func $__cxxabiv1::__base_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const (type 5) (param i32 i32 i32 i32 i32)
    (local i32 i32)
    (local.set 5
      (i32.shr_s
        (local.tee 6
          (i32.load offset=4
            (local.get 0)))
        (i32.const 8)))
    (if  ;; label = @1
      (i32.and
        (local.get 6)
        (i32.const 1))
      (then
        (local.set 5
          (call $update_offset_to_base_char_const*__long_
            (i32.load
              (local.get 2))
            (local.get 5)))))
    (call_indirect (type 5)
      (local.tee 0
        (i32.load
          (local.get 0)))
      (local.get 1)
      (i32.add
        (local.get 2)
        (local.get 5))
      (select
        (local.get 3)
        (i32.const 2)
        (i32.and
          (local.get 6)
          (i32.const 2)))
      (local.get 4)
      (i32.load offset=24
        (i32.load
          (local.get 0)))))
  (func $__cxxabiv1::__si_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const (type 5) (param i32 i32 i32 i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (local.get 4))
      (then
        (call $__cxxabiv1::__class_type_info::process_static_type_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__const
          (local.get 1)
          (local.get 1)
          (local.get 2)
          (local.get 3))
        (return)))
    (block  ;; label = @1
      (if  ;; label = @2
        (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
          (local.get 0)
          (i32.load
            (local.get 1))
          (local.get 4))
        (then
          (block  ;; label = @3
            (if  ;; label = @4
              (i32.ne
                (local.get 2)
                (i32.load offset=16
                  (local.get 1)))
              (then
                (br_if 1 (;@3;)
                  (i32.ne
                    (i32.load offset=20
                      (local.get 1))
                    (local.get 2)))))
            (br_if 2 (;@1;)
              (i32.ne
                (local.get 3)
                (i32.const 1)))
            (i32.store offset=32
              (local.get 1)
              (i32.const 1))
            (return))
          (i32.store offset=32
            (local.get 1)
            (local.get 3))
          (block  ;; label = @3
            (br_if 0 (;@3;)
              (i32.eq
                (i32.load offset=44
                  (local.get 1))
                (i32.const 4)))
            (i32.store16 offset=52
              (local.get 1)
              (i32.const 0))
            (call_indirect (type 7)
              (local.tee 0
                (i32.load offset=8
                  (local.get 0)))
              (local.get 1)
              (local.get 2)
              (local.get 2)
              (i32.const 1)
              (local.get 4)
              (i32.load offset=20
                (i32.load
                  (local.get 0))))
            (if  ;; label = @4
              (i32.load8_u offset=53
                (local.get 1))
              (then
                (i32.store offset=44
                  (local.get 1)
                  (i32.const 3))
                (br_if 1 (;@3;)
                  (i32.eqz
                    (i32.load8_u offset=52
                      (local.get 1))))
                (br 3 (;@1;))))
            (i32.store offset=44
              (local.get 1)
              (i32.const 4)))
          (i32.store offset=20
            (local.get 1)
            (local.get 2))
          (i32.store offset=40
            (local.get 1)
            (i32.add
              (i32.load offset=40
                (local.get 1))
              (i32.const 1)))
          (br_if 1 (;@1;)
            (i32.ne
              (i32.load offset=36
                (local.get 1))
              (i32.const 1)))
          (br_if 1 (;@1;)
            (i32.ne
              (i32.load offset=24
                (local.get 1))
              (i32.const 2)))
          (i32.store8 offset=54
            (local.get 1)
            (i32.const 1))
          (return)))
      (call_indirect (type 5)
        (local.tee 0
          (i32.load offset=8
            (local.get 0)))
        (local.get 1)
        (local.get 2)
        (local.get 3)
        (local.get 4)
        (i32.load offset=24
          (i32.load
            (local.get 0))))))
  (func $__cxxabiv1::__class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const (type 5) (param i32 i32 i32 i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (local.get 4))
      (then
        (call $__cxxabiv1::__class_type_info::process_static_type_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__const
          (local.get 1)
          (local.get 1)
          (local.get 2)
          (local.get 3))
        (return)))
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.eqz
          (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
            (local.get 0)
            (i32.load
              (local.get 1))
            (local.get 4))))
      (block  ;; label = @2
        (if  ;; label = @3
          (i32.ne
            (local.get 2)
            (i32.load offset=16
              (local.get 1)))
          (then
            (br_if 1 (;@2;)
              (i32.ne
                (i32.load offset=20
                  (local.get 1))
                (local.get 2)))))
        (br_if 1 (;@1;)
          (i32.ne
            (local.get 3)
            (i32.const 1)))
        (i32.store offset=32
          (local.get 1)
          (i32.const 1))
        (return))
      (i32.store offset=20
        (local.get 1)
        (local.get 2))
      (i32.store offset=32
        (local.get 1)
        (local.get 3))
      (i32.store offset=40
        (local.get 1)
        (i32.add
          (i32.load offset=40
            (local.get 1))
          (i32.const 1)))
      (block  ;; label = @2
        (br_if 0 (;@2;)
          (i32.ne
            (i32.load offset=36
              (local.get 1))
            (i32.const 1)))
        (br_if 0 (;@2;)
          (i32.ne
            (i32.load offset=24
              (local.get 1))
            (i32.const 2)))
        (i32.store8 offset=54
          (local.get 1)
          (i32.const 1)))
      (i32.store offset=44
        (local.get 1)
        (i32.const 4))))
  (func $__cxxabiv1::__vmi_class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const (type 7) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (local.get 5))
      (then
        (call $__cxxabiv1::__class_type_info::process_static_type_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__const
          (local.get 1)
          (local.get 1)
          (local.get 2)
          (local.get 3)
          (local.get 4))
        (return)))
    (local.set 7
      (i32.load8_u offset=53
        (local.get 1)))
    (local.set 6
      (i32.load offset=12
        (local.get 0)))
    (i32.store8 offset=53
      (local.get 1)
      (i32.const 0))
    (local.set 8
      (i32.load8_u offset=52
        (local.get 1)))
    (i32.store8 offset=52
      (local.get 1)
      (i32.const 0))
    (call $__cxxabiv1::__base_class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const
      (local.tee 9
        (i32.add
          (local.get 0)
          (i32.const 16)))
      (local.get 1)
      (local.get 2)
      (local.get 3)
      (local.get 4)
      (local.get 5))
    (local.set 7
      (i32.or
        (local.get 7)
        (local.tee 10
          (i32.load8_u offset=53
            (local.get 1)))))
    (local.set 8
      (i32.or
        (local.get 8)
        (local.tee 11
          (i32.load8_u offset=52
            (local.get 1)))))
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.lt_s
          (local.get 6)
          (i32.const 2)))
      (local.set 9
        (i32.add
          (local.get 9)
          (i32.shl
            (local.get 6)
            (i32.const 3))))
      (local.set 6
        (i32.add
          (local.get 0)
          (i32.const 24)))
      (loop  ;; label = @2
        (br_if 1 (;@1;)
          (i32.load8_u offset=54
            (local.get 1)))
        (block  ;; label = @3
          (if  ;; label = @4
            (local.get 11)
            (then
              (br_if 3 (;@1;)
                (i32.eq
                  (i32.load offset=24
                    (local.get 1))
                  (i32.const 1)))
              (br_if 1 (;@3;)
                (i32.and
                  (i32.load8_u offset=8
                    (local.get 0))
                  (i32.const 2)))
              (br 3 (;@1;))))
          (br_if 0 (;@3;)
            (i32.eqz
              (local.get 10)))
          (br_if 2 (;@1;)
            (i32.eqz
              (i32.and
                (i32.load8_u offset=8
                  (local.get 0))
                (i32.const 1)))))
        (i32.store16 offset=52
          (local.get 1)
          (i32.const 0))
        (call $__cxxabiv1::__base_class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const
          (local.get 6)
          (local.get 1)
          (local.get 2)
          (local.get 3)
          (local.get 4)
          (local.get 5))
        (local.set 7
          (i32.or
            (local.tee 10
              (i32.load8_u offset=53
                (local.get 1)))
            (local.get 7)))
        (local.set 8
          (i32.or
            (local.tee 11
              (i32.load8_u offset=52
                (local.get 1)))
            (local.get 8)))
        (br_if 0 (;@2;)
          (i32.lt_u
            (local.tee 6
              (i32.add
                (local.get 6)
                (i32.const 8)))
            (local.get 9)))))
    (i32.store8 offset=53
      (local.get 1)
      (i32.ne
        (i32.and
          (local.get 7)
          (i32.const 255))
        (i32.const 0)))
    (i32.store8 offset=52
      (local.get 1)
      (i32.ne
        (i32.and
          (local.get 8)
          (i32.const 255))
        (i32.const 0))))
  (func $__cxxabiv1::__si_class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const (type 7) (param i32 i32 i32 i32 i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (local.get 5))
      (then
        (call $__cxxabiv1::__class_type_info::process_static_type_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__const
          (local.get 1)
          (local.get 1)
          (local.get 2)
          (local.get 3)
          (local.get 4))
        (return)))
    (call_indirect (type 7)
      (local.tee 0
        (i32.load offset=8
          (local.get 0)))
      (local.get 1)
      (local.get 2)
      (local.get 3)
      (local.get 4)
      (local.get 5)
      (i32.load offset=20
        (i32.load
          (local.get 0)))))
  (func $__cxxabiv1::__class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const (type 7) (param i32 i32 i32 i32 i32 i32)
    (if  ;; label = @1
      (call $is_equal_std::type_info_const*__std::type_info_const*__bool_
        (local.get 0)
        (i32.load offset=8
          (local.get 1))
        (local.get 5))
      (then
        (call $__cxxabiv1::__class_type_info::process_static_type_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__const
          (local.get 1)
          (local.get 1)
          (local.get 2)
          (local.get 3)
          (local.get 4)))))
  (func $__errno_location (type 0) (result i32)
    (i32.const 4312))
  (func $dlmalloc (type 2) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    (global.set $__stack_pointer
      (local.tee 12
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (block  ;; label = @1
      (block  ;; label = @2
        (block  ;; label = @3
          (block  ;; label = @4
            (block  ;; label = @5
              (block  ;; label = @6
                (block  ;; label = @7
                  (block  ;; label = @8
                    (block  ;; label = @9
                      (block  ;; label = @10
                        (block  ;; label = @11
                          (block  ;; label = @12
                            (if  ;; label = @13
                              (i32.le_u
                                (local.get 0)
                                (i32.const 244))
                              (then
                                (if  ;; label = @14
                                  (i32.and
                                    (local.tee 0
                                      (i32.shr_u
                                        (local.tee 6
                                          (i32.load
                                            (i32.const 4316)))
                                        (local.tee 1
                                          (i32.shr_u
                                            (local.tee 4
                                              (select
                                                (i32.const 16)
                                                (i32.and
                                                  (i32.add
                                                    (local.get 0)
                                                    (i32.const 11))
                                                  (i32.const -8))
                                                (i32.lt_u
                                                  (local.get 0)
                                                  (i32.const 11))))
                                            (i32.const 3)))))
                                    (i32.const 3))
                                  (then
                                    (local.set 0
                                      (i32.add
                                        (local.tee 1
                                          (i32.load
                                            (i32.add
                                              (local.tee 3
                                                (i32.shl
                                                  (local.tee 2
                                                    (i32.add
                                                      (i32.and
                                                        (i32.xor
                                                          (local.get 0)
                                                          (i32.const -1))
                                                        (i32.const 1))
                                                      (local.get 1)))
                                                  (i32.const 3)))
                                              (i32.const 4364))))
                                        (i32.const 8)))
                                    (block  ;; label = @15
                                      (if  ;; label = @16
                                        (i32.eq
                                          (local.tee 4
                                            (i32.load offset=8
                                              (local.get 1)))
                                          (local.tee 3
                                            (i32.add
                                              (local.get 3)
                                              (i32.const 4356))))
                                        (then
                                          (i32.store
                                            (i32.const 4316)
                                            (i32.and
                                              (local.get 6)
                                              (i32.rotl
                                                (i32.const -2)
                                                (local.get 2))))
                                          (br 1 (;@15;))))
                                      (i32.store offset=12
                                        (local.get 4)
                                        (local.get 3))
                                      (i32.store offset=8
                                        (local.get 3)
                                        (local.get 4)))
                                    (i32.store offset=4
                                      (local.get 1)
                                      (i32.or
                                        (local.tee 2
                                          (i32.shl
                                            (local.get 2)
                                            (i32.const 3)))
                                        (i32.const 3)))
                                    (i32.store offset=4
                                      (local.tee 1
                                        (i32.add
                                          (local.get 1)
                                          (local.get 2)))
                                      (i32.or
                                        (i32.load offset=4
                                          (local.get 1))
                                        (i32.const 1)))
                                    (br 13 (;@1;))))
                                (br_if 1 (;@12;)
                                  (i32.le_u
                                    (local.get 4)
                                    (local.tee 8
                                      (i32.load
                                        (i32.const 4324)))))
                                (if  ;; label = @14
                                  (local.get 0)
                                  (then
                                    (block  ;; label = @15
                                      (if  ;; label = @16
                                        (i32.eq
                                          (local.tee 0
                                            (i32.load offset=8
                                              (local.tee 1
                                                (i32.load
                                                  (i32.add
                                                    (local.tee 3
                                                      (i32.shl
                                                        (local.tee 2
                                                          (i32.add
                                                            (i32.or
                                                              (i32.or
                                                                (i32.or
                                                                  (i32.or
                                                                    (local.tee 2
                                                                      (i32.and
                                                                        (i32.shr_u
                                                                          (local.tee 1
                                                                            (i32.shr_u
                                                                              (local.tee 0
                                                                                (i32.sub
                                                                                  (i32.and
                                                                                    (local.tee 0
                                                                                      (i32.and
                                                                                        (i32.shl
                                                                                          (local.get 0)
                                                                                          (local.get 1))
                                                                                        (i32.or
                                                                                          (local.tee 0
                                                                                            (i32.shl
                                                                                              (i32.const 2)
                                                                                              (local.get 1)))
                                                                                          (i32.sub
                                                                                            (i32.const 0)
                                                                                            (local.get 0)))))
                                                                                    (i32.sub
                                                                                      (i32.const 0)
                                                                                      (local.get 0)))
                                                                                  (i32.const 1)))
                                                                              (local.tee 0
                                                                                (i32.and
                                                                                  (i32.shr_u
                                                                                    (local.get 0)
                                                                                    (i32.const 12))
                                                                                  (i32.const 16)))))
                                                                          (i32.const 5))
                                                                        (i32.const 8)))
                                                                    (local.get 0))
                                                                  (local.tee 1
                                                                    (i32.and
                                                                      (i32.shr_u
                                                                        (local.tee 0
                                                                          (i32.shr_u
                                                                            (local.get 1)
                                                                            (local.get 2)))
                                                                        (i32.const 2))
                                                                      (i32.const 4))))
                                                                (local.tee 1
                                                                  (i32.and
                                                                    (i32.shr_u
                                                                      (local.tee 0
                                                                        (i32.shr_u
                                                                          (local.get 0)
                                                                          (local.get 1)))
                                                                      (i32.const 1))
                                                                    (i32.const 2))))
                                                              (local.tee 1
                                                                (i32.and
                                                                  (i32.shr_u
                                                                    (local.tee 0
                                                                      (i32.shr_u
                                                                        (local.get 0)
                                                                        (local.get 1)))
                                                                    (i32.const 1))
                                                                  (i32.const 1))))
                                                            (i32.shr_u
                                                              (local.get 0)
                                                              (local.get 1))))
                                                        (i32.const 3)))
                                                    (i32.const 4364))))))
                                          (local.tee 3
                                            (i32.add
                                              (local.get 3)
                                              (i32.const 4356))))
                                        (then
                                          (i32.store
                                            (i32.const 4316)
                                            (local.tee 6
                                              (i32.and
                                                (local.get 6)
                                                (i32.rotl
                                                  (i32.const -2)
                                                  (local.get 2)))))
                                          (br 1 (;@15;))))
                                      (i32.store offset=12
                                        (local.get 0)
                                        (local.get 3))
                                      (i32.store offset=8
                                        (local.get 3)
                                        (local.get 0)))
                                    (local.set 0
                                      (i32.add
                                        (local.get 1)
                                        (i32.const 8)))
                                    (i32.store offset=4
                                      (local.get 1)
                                      (i32.or
                                        (local.get 4)
                                        (i32.const 3)))
                                    (i32.store offset=4
                                      (local.tee 3
                                        (i32.add
                                          (local.get 1)
                                          (local.get 4)))
                                      (i32.or
                                        (local.tee 2
                                          (i32.sub
                                            (local.tee 5
                                              (i32.shl
                                                (local.get 2)
                                                (i32.const 3)))
                                            (local.get 4)))
                                        (i32.const 1)))
                                    (i32.store
                                      (i32.add
                                        (local.get 1)
                                        (local.get 5))
                                      (local.get 2))
                                    (if  ;; label = @15
                                      (local.get 8)
                                      (then
                                        (local.set 4
                                          (i32.add
                                            (i32.shl
                                              (local.tee 5
                                                (i32.shr_u
                                                  (local.get 8)
                                                  (i32.const 3)))
                                              (i32.const 3))
                                            (i32.const 4356)))
                                        (local.set 1
                                          (i32.load
                                            (i32.const 4336)))
                                        (local.set 5
                                          (block (result i32)  ;; label = @16
                                            (if  ;; label = @17
                                              (i32.eqz
                                                (i32.and
                                                  (local.get 6)
                                                  (local.tee 5
                                                    (i32.shl
                                                      (i32.const 1)
                                                      (local.get 5)))))
                                              (then
                                                (i32.store
                                                  (i32.const 4316)
                                                  (i32.or
                                                    (local.get 5)
                                                    (local.get 6)))
                                                (br 1 (;@16;)
                                                  (local.get 4))))
                                            (i32.load offset=8
                                              (local.get 4))))
                                        (i32.store offset=8
                                          (local.get 4)
                                          (local.get 1))
                                        (i32.store offset=12
                                          (local.get 5)
                                          (local.get 1))
                                        (i32.store offset=12
                                          (local.get 1)
                                          (local.get 4))
                                        (i32.store offset=8
                                          (local.get 1)
                                          (local.get 5))))
                                    (i32.store
                                      (i32.const 4336)
                                      (local.get 3))
                                    (i32.store
                                      (i32.const 4324)
                                      (local.get 2))
                                    (br 13 (;@1;))))
                                (br_if 1 (;@12;)
                                  (i32.eqz
                                    (local.tee 9
                                      (i32.load
                                        (i32.const 4320)))))
                                (local.set 1
                                  (i32.sub
                                    (i32.and
                                      (i32.load offset=4
                                        (local.tee 3
                                          (i32.load
                                            (i32.add
                                              (i32.shl
                                                (i32.add
                                                  (i32.or
                                                    (i32.or
                                                      (i32.or
                                                        (i32.or
                                                          (local.tee 2
                                                            (i32.and
                                                              (i32.shr_u
                                                                (local.tee 1
                                                                  (i32.shr_u
                                                                    (local.tee 0
                                                                      (i32.sub
                                                                        (i32.and
                                                                          (local.get 9)
                                                                          (i32.sub
                                                                            (i32.const 0)
                                                                            (local.get 9)))
                                                                        (i32.const 1)))
                                                                    (local.tee 0
                                                                      (i32.and
                                                                        (i32.shr_u
                                                                          (local.get 0)
                                                                          (i32.const 12))
                                                                        (i32.const 16)))))
                                                                (i32.const 5))
                                                              (i32.const 8)))
                                                          (local.get 0))
                                                        (local.tee 1
                                                          (i32.and
                                                            (i32.shr_u
                                                              (local.tee 0
                                                                (i32.shr_u
                                                                  (local.get 1)
                                                                  (local.get 2)))
                                                              (i32.const 2))
                                                            (i32.const 4))))
                                                      (local.tee 1
                                                        (i32.and
                                                          (i32.shr_u
                                                            (local.tee 0
                                                              (i32.shr_u
                                                                (local.get 0)
                                                                (local.get 1)))
                                                            (i32.const 1))
                                                          (i32.const 2))))
                                                    (local.tee 1
                                                      (i32.and
                                                        (i32.shr_u
                                                          (local.tee 0
                                                            (i32.shr_u
                                                              (local.get 0)
                                                              (local.get 1)))
                                                          (i32.const 1))
                                                        (i32.const 1))))
                                                  (i32.shr_u
                                                    (local.get 0)
                                                    (local.get 1)))
                                                (i32.const 2))
                                              (i32.const 4620)))))
                                      (i32.const -8))
                                    (local.get 4)))
                                (local.set 2
                                  (local.get 3))
                                (loop  ;; label = @14
                                  (block  ;; label = @15
                                    (if  ;; label = @16
                                      (i32.eqz
                                        (local.tee 0
                                          (i32.load offset=16
                                            (local.get 2))))
                                      (then
                                        (br_if 1 (;@15;)
                                          (i32.eqz
                                            (local.tee 0
                                              (i32.load offset=20
                                                (local.get 2)))))))
                                    (local.set 1
                                      (select
                                        (local.tee 2
                                          (i32.sub
                                            (i32.and
                                              (i32.load offset=4
                                                (local.get 0))
                                              (i32.const -8))
                                            (local.get 4)))
                                        (local.get 1)
                                        (local.tee 2
                                          (i32.gt_u
                                            (local.get 1)
                                            (local.get 2)))))
                                    (local.set 3
                                      (select
                                        (local.get 0)
                                        (local.get 3)
                                        (local.get 2)))
                                    (local.set 2
                                      (local.get 0))
                                    (br 1 (;@14;))))
                                (br_if 2 (;@11;)
                                  (i32.le_u
                                    (local.tee 11
                                      (i32.add
                                        (local.get 3)
                                        (local.get 4)))
                                    (local.get 3)))
                                (local.set 10
                                  (i32.load offset=24
                                    (local.get 3)))
                                (if  ;; label = @14
                                  (i32.ne
                                    (local.get 3)
                                    (local.tee 5
                                      (i32.load offset=12
                                        (local.get 3))))
                                  (then
                                    (drop
                                      (i32.lt_u
                                        (local.tee 0
                                          (i32.load offset=8
                                            (local.get 3)))
                                        (i32.load
                                          (i32.const 4332))))
                                    (i32.store offset=12
                                      (local.get 0)
                                      (local.get 5))
                                    (i32.store offset=8
                                      (local.get 5)
                                      (local.get 0))
                                    (br 12 (;@2;))))
                                (if  ;; label = @14
                                  (i32.eqz
                                    (local.tee 0
                                      (i32.load
                                        (local.tee 2
                                          (i32.add
                                            (local.get 3)
                                            (i32.const 20))))))
                                  (then
                                    (br_if 4 (;@10;)
                                      (i32.eqz
                                        (local.tee 0
                                          (i32.load offset=16
                                            (local.get 3)))))
                                    (local.set 2
                                      (i32.add
                                        (local.get 3)
                                        (i32.const 16)))))
                                (loop  ;; label = @14
                                  (local.set 7
                                    (local.get 2))
                                  (br_if 0 (;@14;)
                                    (local.tee 0
                                      (i32.load
                                        (local.tee 2
                                          (i32.add
                                            (local.tee 5
                                              (local.get 0))
                                            (i32.const 20))))))
                                  (local.set 2
                                    (i32.add
                                      (local.get 5)
                                      (i32.const 16)))
                                  (br_if 0 (;@14;)
                                    (local.tee 0
                                      (i32.load offset=16
                                        (local.get 5)))))
                                (i32.store
                                  (local.get 7)
                                  (i32.const 0))
                                (br 11 (;@2;))))
                            (local.set 4
                              (i32.const -1))
                            (br_if 0 (;@12;)
                              (i32.gt_u
                                (local.get 0)
                                (i32.const -65)))
                            (local.set 4
                              (i32.and
                                (local.tee 0
                                  (i32.add
                                    (local.get 0)
                                    (i32.const 11)))
                                (i32.const -8)))
                            (br_if 0 (;@12;)
                              (i32.eqz
                                (local.tee 8
                                  (i32.load
                                    (i32.const 4320)))))
                            (local.set 7
                              (block (result i32)  ;; label = @13
                                (drop
                                  (br_if 0 (;@13;)
                                    (i32.const 0)
                                    (i32.lt_u
                                      (local.get 4)
                                      (i32.const 256))))
                                (drop
                                  (br_if 0 (;@13;)
                                    (local.tee 7
                                      (i32.const 31))
                                    (i32.gt_u
                                      (local.get 4)
                                      (i32.const 16777215))))
                                (i32.add
                                  (i32.or
                                    (i32.shl
                                      (local.tee 0
                                        (i32.sub
                                          (i32.shr_u
                                            (i32.shl
                                              (local.tee 2
                                                (i32.shl
                                                  (local.tee 1
                                                    (i32.shl
                                                      (local.tee 0
                                                        (i32.shr_u
                                                          (local.get 0)
                                                          (i32.const 8)))
                                                      (local.tee 0
                                                        (i32.and
                                                          (i32.shr_u
                                                            (i32.add
                                                              (local.get 0)
                                                              (i32.const 1048320))
                                                            (i32.const 16))
                                                          (i32.const 8)))))
                                                  (local.tee 1
                                                    (i32.and
                                                      (i32.shr_u
                                                        (i32.add
                                                          (local.get 1)
                                                          (i32.const 520192))
                                                        (i32.const 16))
                                                      (i32.const 4)))))
                                              (local.tee 2
                                                (i32.and
                                                  (i32.shr_u
                                                    (i32.add
                                                      (local.get 2)
                                                      (i32.const 245760))
                                                    (i32.const 16))
                                                  (i32.const 2))))
                                            (i32.const 15))
                                          (i32.or
                                            (i32.or
                                              (local.get 0)
                                              (local.get 1))
                                            (local.get 2))))
                                      (i32.const 1))
                                    (i32.and
                                      (i32.shr_u
                                        (local.get 4)
                                        (i32.add
                                          (local.get 0)
                                          (i32.const 21)))
                                      (i32.const 1)))
                                  (i32.const 28))))
                            (local.set 1
                              (i32.sub
                                (i32.const 0)
                                (local.get 4)))
                            (block  ;; label = @13
                              (block  ;; label = @14
                                (block  ;; label = @15
                                  (if  ;; label = @16
                                    (i32.eqz
                                      (local.tee 2
                                        (i32.load
                                          (i32.add
                                            (i32.shl
                                              (local.get 7)
                                              (i32.const 2))
                                            (i32.const 4620)))))
                                    (then
                                      (local.set 0
                                        (i32.const 0))
                                      (br 1 (;@15;))))
                                  (local.set 0
                                    (i32.const 0))
                                  (local.set 3
                                    (i32.shl
                                      (local.get 4)
                                      (select
                                        (i32.const 0)
                                        (i32.sub
                                          (i32.const 25)
                                          (i32.shr_u
                                            (local.get 7)
                                            (i32.const 1)))
                                        (i32.eq
                                          (local.get 7)
                                          (i32.const 31)))))
                                  (loop  ;; label = @16
                                    (block  ;; label = @17
                                      (br_if 0 (;@17;)
                                        (i32.ge_u
                                          (local.tee 6
                                            (i32.sub
                                              (i32.and
                                                (i32.load offset=4
                                                  (local.get 2))
                                                (i32.const -8))
                                              (local.get 4)))
                                          (local.get 1)))
                                      (local.set 5
                                        (local.get 2))
                                      (br_if 0 (;@17;)
                                        (local.tee 1
                                          (local.get 6)))
                                      (local.set 1
                                        (i32.const 0))
                                      (local.set 0
                                        (local.get 2))
                                      (br 3 (;@14;)))
                                    (local.set 0
                                      (select
                                        (select
                                          (local.get 0)
                                          (local.tee 6
                                            (i32.load offset=20
                                              (local.get 2)))
                                          (i32.eq
                                            (local.get 6)
                                            (local.tee 2
                                              (i32.load offset=16
                                                (i32.add
                                                  (local.get 2)
                                                  (i32.and
                                                    (i32.shr_u
                                                      (local.get 3)
                                                      (i32.const 29))
                                                    (i32.const 4)))))))
                                        (local.get 0)
                                        (local.get 6)))
                                    (local.set 3
                                      (i32.shl
                                        (local.get 3)
                                        (i32.const 1)))
                                    (br_if 0 (;@16;)
                                      (local.get 2))))
                                (if  ;; label = @15
                                  (i32.eqz
                                    (i32.or
                                      (local.get 0)
                                      (local.get 5)))
                                  (then
                                    (local.set 5
                                      (i32.const 0))
                                    (br_if 3 (;@12;)
                                      (i32.eqz
                                        (local.tee 0
                                          (i32.and
                                            (i32.or
                                              (local.tee 0
                                                (i32.shl
                                                  (i32.const 2)
                                                  (local.get 7)))
                                              (i32.sub
                                                (i32.const 0)
                                                (local.get 0)))
                                            (local.get 8)))))
                                    (local.set 0
                                      (i32.load
                                        (i32.add
                                          (i32.shl
                                            (i32.add
                                              (i32.or
                                                (i32.or
                                                  (i32.or
                                                    (i32.or
                                                      (local.tee 3
                                                        (i32.and
                                                          (i32.shr_u
                                                            (local.tee 2
                                                              (i32.shr_u
                                                                (local.tee 0
                                                                  (i32.sub
                                                                    (i32.and
                                                                      (local.get 0)
                                                                      (i32.sub
                                                                        (i32.const 0)
                                                                        (local.get 0)))
                                                                    (i32.const 1)))
                                                                (local.tee 0
                                                                  (i32.and
                                                                    (i32.shr_u
                                                                      (local.get 0)
                                                                      (i32.const 12))
                                                                    (i32.const 16)))))
                                                            (i32.const 5))
                                                          (i32.const 8)))
                                                      (local.get 0))
                                                    (local.tee 2
                                                      (i32.and
                                                        (i32.shr_u
                                                          (local.tee 0
                                                            (i32.shr_u
                                                              (local.get 2)
                                                              (local.get 3)))
                                                          (i32.const 2))
                                                        (i32.const 4))))
                                                  (local.tee 2
                                                    (i32.and
                                                      (i32.shr_u
                                                        (local.tee 0
                                                          (i32.shr_u
                                                            (local.get 0)
                                                            (local.get 2)))
                                                        (i32.const 1))
                                                      (i32.const 2))))
                                                (local.tee 2
                                                  (i32.and
                                                    (i32.shr_u
                                                      (local.tee 0
                                                        (i32.shr_u
                                                          (local.get 0)
                                                          (local.get 2)))
                                                      (i32.const 1))
                                                    (i32.const 1))))
                                              (i32.shr_u
                                                (local.get 0)
                                                (local.get 2)))
                                            (i32.const 2))
                                          (i32.const 4620))))))
                                (br_if 1 (;@13;)
                                  (i32.eqz
                                    (local.get 0))))
                              (loop  ;; label = @14
                                (local.set 3
                                  (i32.lt_u
                                    (local.tee 6
                                      (i32.sub
                                        (i32.and
                                          (i32.load offset=4
                                            (local.get 0))
                                          (i32.const -8))
                                        (local.get 4)))
                                    (local.get 1)))
                                (local.set 1
                                  (select
                                    (local.get 6)
                                    (local.get 1)
                                    (local.get 3)))
                                (local.set 5
                                  (select
                                    (local.get 0)
                                    (local.get 5)
                                    (local.get 3)))
                                (if  ;; label = @15
                                  (i32.eqz
                                    (local.tee 2
                                      (i32.load offset=16
                                        (local.get 0))))
                                  (then
                                    (local.set 2
                                      (i32.load offset=20
                                        (local.get 0)))))
                                (br_if 0 (;@14;)
                                  (local.tee 0
                                    (local.get 2)))))
                            (br_if 0 (;@12;)
                              (i32.eqz
                                (local.get 5)))
                            (br_if 0 (;@12;)
                              (i32.ge_u
                                (local.get 1)
                                (i32.sub
                                  (i32.load
                                    (i32.const 4324))
                                  (local.get 4))))
                            (br_if 1 (;@11;)
                              (i32.le_u
                                (local.tee 7
                                  (i32.add
                                    (local.get 4)
                                    (local.get 5)))
                                (local.get 5)))
                            (local.set 9
                              (i32.load offset=24
                                (local.get 5)))
                            (if  ;; label = @13
                              (i32.ne
                                (local.get 5)
                                (local.tee 3
                                  (i32.load offset=12
                                    (local.get 5))))
                              (then
                                (drop
                                  (i32.lt_u
                                    (local.tee 0
                                      (i32.load offset=8
                                        (local.get 5)))
                                    (i32.load
                                      (i32.const 4332))))
                                (i32.store offset=12
                                  (local.get 0)
                                  (local.get 3))
                                (i32.store offset=8
                                  (local.get 3)
                                  (local.get 0))
                                (br 10 (;@3;))))
                            (if  ;; label = @13
                              (i32.eqz
                                (local.tee 0
                                  (i32.load
                                    (local.tee 2
                                      (i32.add
                                        (local.get 5)
                                        (i32.const 20))))))
                              (then
                                (br_if 4 (;@9;)
                                  (i32.eqz
                                    (local.tee 0
                                      (i32.load offset=16
                                        (local.get 5)))))
                                (local.set 2
                                  (i32.add
                                    (local.get 5)
                                    (i32.const 16)))))
                            (loop  ;; label = @13
                              (local.set 6
                                (local.get 2))
                              (br_if 0 (;@13;)
                                (local.tee 0
                                  (i32.load
                                    (local.tee 2
                                      (i32.add
                                        (local.tee 3
                                          (local.get 0))
                                        (i32.const 20))))))
                              (local.set 2
                                (i32.add
                                  (local.get 3)
                                  (i32.const 16)))
                              (br_if 0 (;@13;)
                                (local.tee 0
                                  (i32.load offset=16
                                    (local.get 3)))))
                            (i32.store
                              (local.get 6)
                              (i32.const 0))
                            (br 9 (;@3;)))
                          (if  ;; label = @12
                            (i32.le_u
                              (local.get 4)
                              (local.tee 0
                                (i32.load
                                  (i32.const 4324))))
                            (then
                              (local.set 1
                                (i32.load
                                  (i32.const 4336)))
                              (block  ;; label = @13
                                (if  ;; label = @14
                                  (i32.ge_u
                                    (local.tee 2
                                      (i32.sub
                                        (local.get 0)
                                        (local.get 4)))
                                    (i32.const 16))
                                  (then
                                    (i32.store
                                      (i32.const 4324)
                                      (local.get 2))
                                    (i32.store
                                      (i32.const 4336)
                                      (local.tee 3
                                        (i32.add
                                          (local.get 1)
                                          (local.get 4))))
                                    (i32.store offset=4
                                      (local.get 3)
                                      (i32.or
                                        (local.get 2)
                                        (i32.const 1)))
                                    (i32.store
                                      (i32.add
                                        (local.get 0)
                                        (local.get 1))
                                      (local.get 2))
                                    (i32.store offset=4
                                      (local.get 1)
                                      (i32.or
                                        (local.get 4)
                                        (i32.const 3)))
                                    (br 1 (;@13;))))
                                (i32.store
                                  (i32.const 4336)
                                  (i32.const 0))
                                (i32.store
                                  (i32.const 4324)
                                  (i32.const 0))
                                (i32.store offset=4
                                  (local.get 1)
                                  (i32.or
                                    (local.get 0)
                                    (i32.const 3)))
                                (i32.store offset=4
                                  (local.tee 0
                                    (i32.add
                                      (local.get 0)
                                      (local.get 1)))
                                  (i32.or
                                    (i32.load offset=4
                                      (local.get 0))
                                    (i32.const 1))))
                              (local.set 0
                                (i32.add
                                  (local.get 1)
                                  (i32.const 8)))
                              (br 11 (;@1;))))
                          (if  ;; label = @12
                            (i32.lt_u
                              (local.get 4)
                              (local.tee 3
                                (i32.load
                                  (i32.const 4328))))
                            (then
                              (i32.store
                                (i32.const 4328)
                                (local.tee 1
                                  (i32.sub
                                    (local.get 3)
                                    (local.get 4))))
                              (i32.store
                                (i32.const 4340)
                                (local.tee 2
                                  (i32.add
                                    (local.tee 0
                                      (i32.load
                                        (i32.const 4340)))
                                    (local.get 4))))
                              (i32.store offset=4
                                (local.get 2)
                                (i32.or
                                  (local.get 1)
                                  (i32.const 1)))
                              (i32.store offset=4
                                (local.get 0)
                                (i32.or
                                  (local.get 4)
                                  (i32.const 3)))
                              (local.set 0
                                (i32.add
                                  (local.get 0)
                                  (i32.const 8)))
                              (br 11 (;@1;))))
                          (local.set 0
                            (i32.const 0))
                          (br_if 10 (;@1;)
                            (i32.le_u
                              (local.tee 5
                                (i32.and
                                  (local.tee 6
                                    (i32.add
                                      (local.tee 8
                                        (i32.add
                                          (local.get 4)
                                          (i32.const 47)))
                                      (local.tee 1
                                        (block (result i32)  ;; label = @12
                                          (if  ;; label = @13
                                            (i32.load
                                              (i32.const 4788))
                                            (then
                                              (br 1 (;@12;)
                                                (i32.load
                                                  (i32.const 4796)))))
                                          (i64.store align=4
                                            (i32.const 4800)
                                            (i64.const -1))
                                          (i64.store align=4
                                            (i32.const 4792)
                                            (i64.const 17592186048512))
                                          (i32.store
                                            (i32.const 4788)
                                            (i32.xor
                                              (i32.and
                                                (i32.add
                                                  (local.get 12)
                                                  (i32.const 12))
                                                (i32.const -16))
                                              (i32.const 1431655768)))
                                          (i32.store
                                            (i32.const 4808)
                                            (i32.const 0))
                                          (i32.store
                                            (i32.const 4760)
                                            (i32.const 0))
                                          (i32.const 4096)))))
                                  (local.tee 7
                                    (i32.sub
                                      (i32.const 0)
                                      (local.get 1)))))
                              (local.get 4)))
                          (if  ;; label = @12
                            (local.tee 1
                              (i32.load
                                (i32.const 4756)))
                            (then
                              (br_if 11 (;@1;)
                                (i32.le_u
                                  (local.tee 9
                                    (i32.add
                                      (local.tee 2
                                        (i32.load
                                          (i32.const 4748)))
                                      (local.get 5)))
                                  (local.get 2)))
                              (br_if 11 (;@1;)
                                (i32.lt_u
                                  (local.get 1)
                                  (local.get 9)))))
                          (br_if 5 (;@6;)
                            (i32.and
                              (i32.load8_u
                                (i32.const 4760))
                              (i32.const 4)))
                          (block  ;; label = @12
                            (block  ;; label = @13
                              (if  ;; label = @14
                                (local.tee 1
                                  (i32.load
                                    (i32.const 4340)))
                                (then
                                  (local.set 0
                                    (i32.const 4764))
                                  (loop  ;; label = @15
                                    (if  ;; label = @16
                                      (i32.ge_u
                                        (local.get 1)
                                        (local.tee 2
                                          (i32.load
                                            (local.get 0))))
                                      (then
                                        (br_if 3 (;@13;)
                                          (i32.gt_u
                                            (i32.add
                                              (local.get 2)
                                              (i32.load offset=4
                                                (local.get 0)))
                                            (local.get 1)))))
                                    (br_if 0 (;@15;)
                                      (local.tee 0
                                        (i32.load offset=8
                                          (local.get 0)))))))
                              (br_if 6 (;@7;)
                                (i32.eq
                                  (local.tee 3
                                    (call $sbrk
                                      (i32.const 0)))
                                  (i32.const -1)))
                              (local.set 6
                                (local.get 5))
                              (if  ;; label = @14
                                (i32.and
                                  (local.tee 1
                                    (i32.sub
                                      (local.tee 0
                                        (i32.load
                                          (i32.const 4792)))
                                      (i32.const 1)))
                                  (local.get 3))
                                (then
                                  (local.set 6
                                    (i32.add
                                      (i32.sub
                                        (local.get 5)
                                        (local.get 3))
                                      (i32.and
                                        (i32.add
                                          (local.get 1)
                                          (local.get 3))
                                        (i32.sub
                                          (i32.const 0)
                                          (local.get 0)))))))
                              (br_if 6 (;@7;)
                                (i32.ge_u
                                  (local.get 4)
                                  (local.get 6)))
                              (br_if 6 (;@7;)
                                (i32.gt_u
                                  (local.get 6)
                                  (i32.const 2147483646)))
                              (if  ;; label = @14
                                (local.tee 0
                                  (i32.load
                                    (i32.const 4756)))
                                (then
                                  (br_if 7 (;@7;)
                                    (i32.le_u
                                      (local.tee 2
                                        (i32.add
                                          (local.tee 1
                                            (i32.load
                                              (i32.const 4748)))
                                          (local.get 6)))
                                      (local.get 1)))
                                  (br_if 7 (;@7;)
                                    (i32.lt_u
                                      (local.get 0)
                                      (local.get 2)))))
                              (br_if 1 (;@12;)
                                (i32.ne
                                  (local.tee 0
                                    (call $sbrk
                                      (local.get 6)))
                                  (local.get 3)))
                              (br 8 (;@5;)))
                            (br_if 5 (;@7;)
                              (i32.gt_u
                                (local.tee 6
                                  (i32.and
                                    (i32.sub
                                      (local.get 6)
                                      (local.get 3))
                                    (local.get 7)))
                                (i32.const 2147483646)))
                            (br_if 4 (;@8;)
                              (i32.eq
                                (local.tee 3
                                  (call $sbrk
                                    (local.get 6)))
                                (i32.add
                                  (i32.load
                                    (local.get 0))
                                  (i32.load offset=4
                                    (local.get 0)))))
                            (local.set 0
                              (local.get 3)))
                          (block  ;; label = @12
                            (br_if 0 (;@12;)
                              (i32.eq
                                (local.get 0)
                                (i32.const -1)))
                            (br_if 0 (;@12;)
                              (i32.le_u
                                (i32.add
                                  (local.get 4)
                                  (i32.const 48))
                                (local.get 6)))
                            (if  ;; label = @13
                              (i32.gt_u
                                (local.tee 1
                                  (i32.and
                                    (i32.add
                                      (local.tee 1
                                        (i32.load
                                          (i32.const 4796)))
                                      (i32.sub
                                        (local.get 8)
                                        (local.get 6)))
                                    (i32.sub
                                      (i32.const 0)
                                      (local.get 1))))
                                (i32.const 2147483646))
                              (then
                                (local.set 3
                                  (local.get 0))
                                (br 8 (;@5;))))
                            (if  ;; label = @13
                              (i32.ne
                                (call $sbrk
                                  (local.get 1))
                                (i32.const -1))
                              (then
                                (local.set 6
                                  (i32.add
                                    (local.get 1)
                                    (local.get 6)))
                                (local.set 3
                                  (local.get 0))
                                (br 8 (;@5;))))
                            (drop
                              (call $sbrk
                                (i32.sub
                                  (i32.const 0)
                                  (local.get 6))))
                            (br 5 (;@7;)))
                          (local.set 3
                            (local.get 0))
                          (br_if 6 (;@5;)
                            (i32.ne
                              (local.get 0)
                              (i32.const -1)))
                          (br 4 (;@7;)))
                        (unreachable))
                      (local.set 5
                        (i32.const 0))
                      (br 7 (;@2;)))
                    (local.set 3
                      (i32.const 0))
                    (br 5 (;@3;)))
                  (br_if 2 (;@5;)
                    (i32.ne
                      (local.get 3)
                      (i32.const -1))))
                (i32.store
                  (i32.const 4760)
                  (i32.or
                    (i32.load
                      (i32.const 4760))
                    (i32.const 4))))
              (br_if 1 (;@4;)
                (i32.gt_u
                  (local.get 5)
                  (i32.const 2147483646)))
              (local.set 3
                (call $sbrk
                  (local.get 5)))
              (local.set 0
                (call $sbrk
                  (i32.const 0)))
              (br_if 1 (;@4;)
                (i32.eq
                  (local.get 3)
                  (i32.const -1)))
              (br_if 1 (;@4;)
                (i32.eq
                  (local.get 0)
                  (i32.const -1)))
              (br_if 1 (;@4;)
                (i32.le_u
                  (local.get 0)
                  (local.get 3)))
              (br_if 1 (;@4;)
                (i32.le_u
                  (local.tee 6
                    (i32.sub
                      (local.get 0)
                      (local.get 3)))
                  (i32.add
                    (local.get 4)
                    (i32.const 40)))))
            (i32.store
              (i32.const 4748)
              (local.tee 0
                (i32.add
                  (i32.load
                    (i32.const 4748))
                  (local.get 6))))
            (if  ;; label = @5
              (i32.lt_u
                (i32.load
                  (i32.const 4752))
                (local.get 0))
              (then
                (i32.store
                  (i32.const 4752)
                  (local.get 0))))
            (block  ;; label = @5
              (block  ;; label = @6
                (block  ;; label = @7
                  (if  ;; label = @8
                    (local.tee 1
                      (i32.load
                        (i32.const 4340)))
                    (then
                      (local.set 0
                        (i32.const 4764))
                      (loop  ;; label = @9
                        (br_if 2 (;@7;)
                          (i32.eq
                            (local.get 3)
                            (i32.add
                              (local.tee 2
                                (i32.load
                                  (local.get 0)))
                              (local.tee 5
                                (i32.load offset=4
                                  (local.get 0))))))
                        (br_if 0 (;@9;)
                          (local.tee 0
                            (i32.load offset=8
                              (local.get 0)))))
                      (br 2 (;@6;))))
                  (if  ;; label = @8
                    (i32.eqz
                      (select
                        (local.tee 0
                          (i32.load
                            (i32.const 4332)))
                        (i32.const 0)
                        (i32.le_u
                          (local.get 0)
                          (local.get 3))))
                    (then
                      (i32.store
                        (i32.const 4332)
                        (local.get 3))))
                  (local.set 0
                    (i32.const 0))
                  (i32.store
                    (i32.const 4768)
                    (local.get 6))
                  (i32.store
                    (i32.const 4764)
                    (local.get 3))
                  (i32.store
                    (i32.const 4348)
                    (i32.const -1))
                  (i32.store
                    (i32.const 4352)
                    (i32.load
                      (i32.const 4788)))
                  (i32.store
                    (i32.const 4776)
                    (i32.const 0))
                  (loop  ;; label = @8
                    (i32.store
                      (i32.add
                        (local.tee 1
                          (i32.shl
                            (local.get 0)
                            (i32.const 3)))
                        (i32.const 4364))
                      (local.tee 2
                        (i32.add
                          (local.get 1)
                          (i32.const 4356))))
                    (i32.store
                      (i32.add
                        (local.get 1)
                        (i32.const 4368))
                      (local.get 2))
                    (br_if 0 (;@8;)
                      (i32.ne
                        (local.tee 0
                          (i32.add
                            (local.get 0)
                            (i32.const 1)))
                        (i32.const 32))))
                  (i32.store
                    (i32.const 4328)
                    (local.tee 2
                      (i32.sub
                        (local.tee 0
                          (i32.sub
                            (local.get 6)
                            (i32.const 40)))
                        (local.tee 1
                          (select
                            (i32.and
                              (i32.sub
                                (i32.const -8)
                                (local.get 3))
                              (i32.const 7))
                            (i32.const 0)
                            (i32.and
                              (i32.add
                                (local.get 3)
                                (i32.const 8))
                              (i32.const 7)))))))
                  (i32.store
                    (i32.const 4340)
                    (local.tee 1
                      (i32.add
                        (local.get 1)
                        (local.get 3))))
                  (i32.store offset=4
                    (local.get 1)
                    (i32.or
                      (local.get 2)
                      (i32.const 1)))
                  (i32.store offset=4
                    (i32.add
                      (local.get 0)
                      (local.get 3))
                    (i32.const 40))
                  (i32.store
                    (i32.const 4344)
                    (i32.load
                      (i32.const 4804)))
                  (br 2 (;@5;)))
                (br_if 0 (;@6;)
                  (i32.and
                    (i32.load8_u offset=12
                      (local.get 0))
                    (i32.const 8)))
                (br_if 0 (;@6;)
                  (i32.lt_u
                    (local.get 1)
                    (local.get 2)))
                (br_if 0 (;@6;)
                  (i32.ge_u
                    (local.get 1)
                    (local.get 3)))
                (i32.store offset=4
                  (local.get 0)
                  (i32.add
                    (local.get 5)
                    (local.get 6)))
                (i32.store
                  (i32.const 4340)
                  (local.tee 2
                    (i32.add
                      (local.get 1)
                      (local.tee 0
                        (select
                          (i32.and
                            (i32.sub
                              (i32.const -8)
                              (local.get 1))
                            (i32.const 7))
                          (i32.const 0)
                          (i32.and
                            (i32.add
                              (local.get 1)
                              (i32.const 8))
                            (i32.const 7)))))))
                (i32.store
                  (i32.const 4328)
                  (local.tee 0
                    (i32.sub
                      (local.tee 3
                        (i32.add
                          (i32.load
                            (i32.const 4328))
                          (local.get 6)))
                      (local.get 0))))
                (i32.store offset=4
                  (local.get 2)
                  (i32.or
                    (local.get 0)
                    (i32.const 1)))
                (i32.store offset=4
                  (i32.add
                    (local.get 1)
                    (local.get 3))
                  (i32.const 40))
                (i32.store
                  (i32.const 4344)
                  (i32.load
                    (i32.const 4804)))
                (br 1 (;@5;)))
              (if  ;; label = @6
                (i32.gt_u
                  (local.tee 5
                    (i32.load
                      (i32.const 4332)))
                  (local.get 3))
                (then
                  (i32.store
                    (i32.const 4332)
                    (local.get 3))
                  (local.set 5
                    (local.get 3))))
              (local.set 2
                (i32.add
                  (local.get 3)
                  (local.get 6)))
              (local.set 0
                (i32.const 4764))
              (block  ;; label = @6
                (block  ;; label = @7
                  (block  ;; label = @8
                    (block  ;; label = @9
                      (block  ;; label = @10
                        (block  ;; label = @11
                          (loop  ;; label = @12
                            (if  ;; label = @13
                              (i32.ne
                                (local.get 2)
                                (i32.load
                                  (local.get 0)))
                              (then
                                (br_if 1 (;@12;)
                                  (local.tee 0
                                    (i32.load offset=8
                                      (local.get 0))))
                                (br 2 (;@11;)))))
                          (br_if 1 (;@10;)
                            (i32.eqz
                              (i32.and
                                (i32.load8_u offset=12
                                  (local.get 0))
                                (i32.const 8)))))
                        (local.set 0
                          (i32.const 4764))
                        (loop  ;; label = @11
                          (if  ;; label = @12
                            (i32.ge_u
                              (local.get 1)
                              (local.tee 2
                                (i32.load
                                  (local.get 0))))
                            (then
                              (br_if 3 (;@9;)
                                (i32.gt_u
                                  (local.tee 2
                                    (i32.add
                                      (local.get 2)
                                      (i32.load offset=4
                                        (local.get 0))))
                                  (local.get 1)))))
                          (local.set 0
                            (i32.load offset=8
                              (local.get 0)))
                          (br 0 (;@11;)))
                        (unreachable))
                      (i32.store
                        (local.get 0)
                        (local.get 3))
                      (i32.store offset=4
                        (local.get 0)
                        (i32.add
                          (i32.load offset=4
                            (local.get 0))
                          (local.get 6)))
                      (i32.store offset=4
                        (local.tee 7
                          (i32.add
                            (local.get 3)
                            (select
                              (i32.and
                                (i32.sub
                                  (i32.const -8)
                                  (local.get 3))
                                (i32.const 7))
                              (i32.const 0)
                              (i32.and
                                (i32.add
                                  (local.get 3)
                                  (i32.const 8))
                                (i32.const 7)))))
                        (i32.or
                          (local.get 4)
                          (i32.const 3)))
                      (local.set 2
                        (i32.sub
                          (local.tee 6
                            (i32.add
                              (local.get 2)
                              (select
                                (i32.and
                                  (i32.sub
                                    (i32.const -8)
                                    (local.get 2))
                                  (i32.const 7))
                                (i32.const 0)
                                (i32.and
                                  (i32.add
                                    (local.get 2)
                                    (i32.const 8))
                                  (i32.const 7)))))
                          (local.tee 4
                            (i32.add
                              (local.get 4)
                              (local.get 7)))))
                      (if  ;; label = @10
                        (i32.eq
                          (local.get 1)
                          (local.get 6))
                        (then
                          (i32.store
                            (i32.const 4340)
                            (local.get 4))
                          (i32.store
                            (i32.const 4328)
                            (local.tee 0
                              (i32.add
                                (i32.load
                                  (i32.const 4328))
                                (local.get 2))))
                          (i32.store offset=4
                            (local.get 4)
                            (i32.or
                              (local.get 0)
                              (i32.const 1)))
                          (br 3 (;@7;))))
                      (if  ;; label = @10
                        (i32.eq
                          (local.get 6)
                          (i32.load
                            (i32.const 4336)))
                        (then
                          (i32.store
                            (i32.const 4336)
                            (local.get 4))
                          (i32.store
                            (i32.const 4324)
                            (local.tee 0
                              (i32.add
                                (i32.load
                                  (i32.const 4324))
                                (local.get 2))))
                          (i32.store offset=4
                            (local.get 4)
                            (i32.or
                              (local.get 0)
                              (i32.const 1)))
                          (i32.store
                            (i32.add
                              (local.get 0)
                              (local.get 4))
                            (local.get 0))
                          (br 3 (;@7;))))
                      (if  ;; label = @10
                        (i32.eq
                          (i32.and
                            (local.tee 0
                              (i32.load offset=4
                                (local.get 6)))
                            (i32.const 3))
                          (i32.const 1))
                        (then
                          (local.set 8
                            (i32.and
                              (local.get 0)
                              (i32.const -8)))
                          (block  ;; label = @11
                            (if  ;; label = @12
                              (i32.le_u
                                (local.get 0)
                                (i32.const 255))
                              (then
                                (drop
                                  (i32.eq
                                    (local.tee 1
                                      (i32.load offset=8
                                        (local.get 6)))
                                    (local.tee 3
                                      (i32.add
                                        (i32.shl
                                          (local.tee 5
                                            (i32.shr_u
                                              (local.get 0)
                                              (i32.const 3)))
                                          (i32.const 3))
                                        (i32.const 4356)))))
                                (if  ;; label = @13
                                  (i32.eq
                                    (local.get 1)
                                    (local.tee 0
                                      (i32.load offset=12
                                        (local.get 6))))
                                  (then
                                    (i32.store
                                      (i32.const 4316)
                                      (i32.and
                                        (i32.load
                                          (i32.const 4316))
                                        (i32.rotl
                                          (i32.const -2)
                                          (local.get 5))))
                                    (br 2 (;@11;))))
                                (i32.store offset=12
                                  (local.get 1)
                                  (local.get 0))
                                (i32.store offset=8
                                  (local.get 0)
                                  (local.get 1))
                                (br 1 (;@11;))))
                            (local.set 9
                              (i32.load offset=24
                                (local.get 6)))
                            (block  ;; label = @12
                              (if  ;; label = @13
                                (i32.ne
                                  (local.get 6)
                                  (local.tee 3
                                    (i32.load offset=12
                                      (local.get 6))))
                                (then
                                  (i32.store offset=12
                                    (local.tee 0
                                      (i32.load offset=8
                                        (local.get 6)))
                                    (local.get 3))
                                  (i32.store offset=8
                                    (local.get 3)
                                    (local.get 0))
                                  (br 1 (;@12;))))
                              (block  ;; label = @13
                                (br_if 0 (;@13;)
                                  (local.tee 1
                                    (i32.load
                                      (local.tee 0
                                        (i32.add
                                          (local.get 6)
                                          (i32.const 20))))))
                                (br_if 0 (;@13;)
                                  (local.tee 1
                                    (i32.load
                                      (local.tee 0
                                        (i32.add
                                          (local.get 6)
                                          (i32.const 16))))))
                                (local.set 3
                                  (i32.const 0))
                                (br 1 (;@12;)))
                              (loop  ;; label = @13
                                (local.set 5
                                  (local.get 0))
                                (br_if 0 (;@13;)
                                  (local.tee 1
                                    (i32.load
                                      (local.tee 0
                                        (i32.add
                                          (local.tee 3
                                            (local.get 1))
                                          (i32.const 20))))))
                                (local.set 0
                                  (i32.add
                                    (local.get 3)
                                    (i32.const 16)))
                                (br_if 0 (;@13;)
                                  (local.tee 1
                                    (i32.load offset=16
                                      (local.get 3)))))
                              (i32.store
                                (local.get 5)
                                (i32.const 0)))
                            (br_if 0 (;@11;)
                              (i32.eqz
                                (local.get 9)))
                            (block  ;; label = @12
                              (if  ;; label = @13
                                (i32.eq
                                  (local.get 6)
                                  (i32.load
                                    (local.tee 0
                                      (i32.add
                                        (i32.shl
                                          (local.tee 1
                                            (i32.load offset=28
                                              (local.get 6)))
                                          (i32.const 2))
                                        (i32.const 4620)))))
                                (then
                                  (i32.store
                                    (local.get 0)
                                    (local.get 3))
                                  (br_if 1 (;@12;)
                                    (local.get 3))
                                  (i32.store
                                    (i32.const 4320)
                                    (i32.and
                                      (i32.load
                                        (i32.const 4320))
                                      (i32.rotl
                                        (i32.const -2)
                                        (local.get 1))))
                                  (br 2 (;@11;))))
                              (i32.store
                                (i32.add
                                  (local.get 9)
                                  (select
                                    (i32.const 16)
                                    (i32.const 20)
                                    (i32.eq
                                      (i32.load offset=16
                                        (local.get 9))
                                      (local.get 6))))
                                (local.get 3))
                              (br_if 1 (;@11;)
                                (i32.eqz
                                  (local.get 3))))
                            (i32.store offset=24
                              (local.get 3)
                              (local.get 9))
                            (if  ;; label = @12
                              (local.tee 0
                                (i32.load offset=16
                                  (local.get 6)))
                              (then
                                (i32.store offset=16
                                  (local.get 3)
                                  (local.get 0))
                                (i32.store offset=24
                                  (local.get 0)
                                  (local.get 3))))
                            (br_if 0 (;@11;)
                              (i32.eqz
                                (local.tee 0
                                  (i32.load offset=20
                                    (local.get 6)))))
                            (i32.store offset=20
                              (local.get 3)
                              (local.get 0))
                            (i32.store offset=24
                              (local.get 0)
                              (local.get 3)))
                          (local.set 6
                            (i32.add
                              (local.get 6)
                              (local.get 8)))
                          (local.set 2
                            (i32.add
                              (local.get 2)
                              (local.get 8)))))
                      (i32.store offset=4
                        (local.get 6)
                        (i32.and
                          (i32.load offset=4
                            (local.get 6))
                          (i32.const -2)))
                      (i32.store offset=4
                        (local.get 4)
                        (i32.or
                          (local.get 2)
                          (i32.const 1)))
                      (i32.store
                        (i32.add
                          (local.get 2)
                          (local.get 4))
                        (local.get 2))
                      (if  ;; label = @10
                        (i32.le_u
                          (local.get 2)
                          (i32.const 255))
                        (then
                          (local.set 0
                            (i32.add
                              (i32.shl
                                (local.tee 1
                                  (i32.shr_u
                                    (local.get 2)
                                    (i32.const 3)))
                                (i32.const 3))
                              (i32.const 4356)))
                          (local.set 1
                            (block (result i32)  ;; label = @11
                              (if  ;; label = @12
                                (i32.eqz
                                  (i32.and
                                    (local.tee 2
                                      (i32.load
                                        (i32.const 4316)))
                                    (local.tee 1
                                      (i32.shl
                                        (i32.const 1)
                                        (local.get 1)))))
                                (then
                                  (i32.store
                                    (i32.const 4316)
                                    (i32.or
                                      (local.get 1)
                                      (local.get 2)))
                                  (br 1 (;@11;)
                                    (local.get 0))))
                              (i32.load offset=8
                                (local.get 0))))
                          (i32.store offset=8
                            (local.get 0)
                            (local.get 4))
                          (i32.store offset=12
                            (local.get 1)
                            (local.get 4))
                          (i32.store offset=12
                            (local.get 4)
                            (local.get 0))
                          (i32.store offset=8
                            (local.get 4)
                            (local.get 1))
                          (br 3 (;@7;))))
                      (local.set 0
                        (i32.const 31))
                      (if  ;; label = @10
                        (i32.le_u
                          (local.get 2)
                          (i32.const 16777215))
                        (then
                          (local.set 0
                            (i32.add
                              (i32.or
                                (i32.shl
                                  (local.tee 0
                                    (i32.sub
                                      (i32.shr_u
                                        (i32.shl
                                          (local.tee 3
                                            (i32.shl
                                              (local.tee 1
                                                (i32.shl
                                                  (local.tee 0
                                                    (i32.shr_u
                                                      (local.get 2)
                                                      (i32.const 8)))
                                                  (local.tee 0
                                                    (i32.and
                                                      (i32.shr_u
                                                        (i32.add
                                                          (local.get 0)
                                                          (i32.const 1048320))
                                                        (i32.const 16))
                                                      (i32.const 8)))))
                                              (local.tee 1
                                                (i32.and
                                                  (i32.shr_u
                                                    (i32.add
                                                      (local.get 1)
                                                      (i32.const 520192))
                                                    (i32.const 16))
                                                  (i32.const 4)))))
                                          (local.tee 3
                                            (i32.and
                                              (i32.shr_u
                                                (i32.add
                                                  (local.get 3)
                                                  (i32.const 245760))
                                                (i32.const 16))
                                              (i32.const 2))))
                                        (i32.const 15))
                                      (i32.or
                                        (i32.or
                                          (local.get 0)
                                          (local.get 1))
                                        (local.get 3))))
                                  (i32.const 1))
                                (i32.and
                                  (i32.shr_u
                                    (local.get 2)
                                    (i32.add
                                      (local.get 0)
                                      (i32.const 21)))
                                  (i32.const 1)))
                              (i32.const 28)))))
                      (i32.store offset=28
                        (local.get 4)
                        (local.get 0))
                      (i64.store offset=16 align=4
                        (local.get 4)
                        (i64.const 0))
                      (local.set 1
                        (i32.add
                          (i32.shl
                            (local.get 0)
                            (i32.const 2))
                          (i32.const 4620)))
                      (block  ;; label = @10
                        (if  ;; label = @11
                          (i32.eqz
                            (i32.and
                              (local.tee 3
                                (i32.load
                                  (i32.const 4320)))
                              (local.tee 5
                                (i32.shl
                                  (i32.const 1)
                                  (local.get 0)))))
                          (then
                            (i32.store
                              (i32.const 4320)
                              (i32.or
                                (local.get 3)
                                (local.get 5)))
                            (i32.store
                              (local.get 1)
                              (local.get 4))
                            (i32.store offset=24
                              (local.get 4)
                              (local.get 1))
                            (br 1 (;@10;))))
                        (local.set 0
                          (i32.shl
                            (local.get 2)
                            (select
                              (i32.const 0)
                              (i32.sub
                                (i32.const 25)
                                (i32.shr_u
                                  (local.get 0)
                                  (i32.const 1)))
                              (i32.eq
                                (local.get 0)
                                (i32.const 31)))))
                        (local.set 3
                          (i32.load
                            (local.get 1)))
                        (loop  ;; label = @11
                          (br_if 3 (;@8;)
                            (i32.eq
                              (i32.and
                                (i32.load offset=4
                                  (local.tee 1
                                    (local.get 3)))
                                (i32.const -8))
                              (local.get 2)))
                          (local.set 3
                            (i32.shr_u
                              (local.get 0)
                              (i32.const 29)))
                          (local.set 0
                            (i32.shl
                              (local.get 0)
                              (i32.const 1)))
                          (br_if 0 (;@11;)
                            (local.tee 3
                              (i32.load
                                (local.tee 5
                                  (i32.add
                                    (i32.add
                                      (local.get 1)
                                      (i32.and
                                        (local.get 3)
                                        (i32.const 4)))
                                    (i32.const 16)))))))
                        (i32.store
                          (local.get 5)
                          (local.get 4))
                        (i32.store offset=24
                          (local.get 4)
                          (local.get 1)))
                      (i32.store offset=12
                        (local.get 4)
                        (local.get 4))
                      (i32.store offset=8
                        (local.get 4)
                        (local.get 4))
                      (br 2 (;@7;)))
                    (i32.store
                      (i32.const 4328)
                      (local.tee 7
                        (i32.sub
                          (local.tee 0
                            (i32.sub
                              (local.get 6)
                              (i32.const 40)))
                          (local.tee 5
                            (select
                              (i32.and
                                (i32.sub
                                  (i32.const -8)
                                  (local.get 3))
                                (i32.const 7))
                              (i32.const 0)
                              (i32.and
                                (i32.add
                                  (local.get 3)
                                  (i32.const 8))
                                (i32.const 7)))))))
                    (i32.store
                      (i32.const 4340)
                      (local.tee 5
                        (i32.add
                          (local.get 3)
                          (local.get 5))))
                    (i32.store offset=4
                      (local.get 5)
                      (i32.or
                        (local.get 7)
                        (i32.const 1)))
                    (i32.store offset=4
                      (i32.add
                        (local.get 0)
                        (local.get 3))
                      (i32.const 40))
                    (i32.store
                      (i32.const 4344)
                      (i32.load
                        (i32.const 4804)))
                    (i32.store offset=4
                      (local.tee 5
                        (select
                          (local.get 1)
                          (local.tee 0
                            (i32.sub
                              (i32.add
                                (local.get 2)
                                (select
                                  (i32.and
                                    (i32.sub
                                      (i32.const 39)
                                      (local.get 2))
                                    (i32.const 7))
                                  (i32.const 0)
                                  (i32.and
                                    (i32.sub
                                      (local.get 2)
                                      (i32.const 39))
                                    (i32.const 7))))
                              (i32.const 47)))
                          (i32.lt_u
                            (local.get 0)
                            (i32.add
                              (local.get 1)
                              (i32.const 16)))))
                      (i32.const 27))
                    (i64.store offset=16 align=4
                      (local.get 5)
                      (i64.load align=4
                        (i32.const 4772)))
                    (i64.store offset=8 align=4
                      (local.get 5)
                      (i64.load align=4
                        (i32.const 4764)))
                    (i32.store
                      (i32.const 4772)
                      (i32.add
                        (local.get 5)
                        (i32.const 8)))
                    (i32.store
                      (i32.const 4768)
                      (local.get 6))
                    (i32.store
                      (i32.const 4764)
                      (local.get 3))
                    (i32.store
                      (i32.const 4776)
                      (i32.const 0))
                    (local.set 0
                      (i32.add
                        (local.get 5)
                        (i32.const 24)))
                    (loop  ;; label = @9
                      (i32.store offset=4
                        (local.get 0)
                        (i32.const 7))
                      (local.set 3
                        (i32.add
                          (local.get 0)
                          (i32.const 8)))
                      (local.set 0
                        (i32.add
                          (local.get 0)
                          (i32.const 4)))
                      (br_if 0 (;@9;)
                        (i32.gt_u
                          (local.get 2)
                          (local.get 3))))
                    (br_if 3 (;@5;)
                      (i32.eq
                        (local.get 1)
                        (local.get 5)))
                    (i32.store offset=4
                      (local.get 5)
                      (i32.and
                        (i32.load offset=4
                          (local.get 5))
                        (i32.const -2)))
                    (i32.store offset=4
                      (local.get 1)
                      (i32.or
                        (local.tee 6
                          (i32.sub
                            (local.get 5)
                            (local.get 1)))
                        (i32.const 1)))
                    (i32.store
                      (local.get 5)
                      (local.get 6))
                    (if  ;; label = @9
                      (i32.le_u
                        (local.get 6)
                        (i32.const 255))
                      (then
                        (local.set 0
                          (i32.add
                            (i32.shl
                              (local.tee 2
                                (i32.shr_u
                                  (local.get 6)
                                  (i32.const 3)))
                              (i32.const 3))
                            (i32.const 4356)))
                        (local.set 2
                          (block (result i32)  ;; label = @10
                            (if  ;; label = @11
                              (i32.eqz
                                (i32.and
                                  (local.tee 3
                                    (i32.load
                                      (i32.const 4316)))
                                  (local.tee 2
                                    (i32.shl
                                      (i32.const 1)
                                      (local.get 2)))))
                              (then
                                (i32.store
                                  (i32.const 4316)
                                  (i32.or
                                    (local.get 2)
                                    (local.get 3)))
                                (br 1 (;@10;)
                                  (local.get 0))))
                            (i32.load offset=8
                              (local.get 0))))
                        (i32.store offset=8
                          (local.get 0)
                          (local.get 1))
                        (i32.store offset=12
                          (local.get 2)
                          (local.get 1))
                        (i32.store offset=12
                          (local.get 1)
                          (local.get 0))
                        (i32.store offset=8
                          (local.get 1)
                          (local.get 2))
                        (br 4 (;@5;))))
                    (local.set 0
                      (i32.const 31))
                    (i64.store offset=16 align=4
                      (local.get 1)
                      (i64.const 0))
                    (if  ;; label = @9
                      (i32.le_u
                        (local.get 6)
                        (i32.const 16777215))
                      (then
                        (local.set 0
                          (i32.add
                            (i32.or
                              (i32.shl
                                (local.tee 0
                                  (i32.sub
                                    (i32.shr_u
                                      (i32.shl
                                        (local.tee 3
                                          (i32.shl
                                            (local.tee 2
                                              (i32.shl
                                                (local.tee 0
                                                  (i32.shr_u
                                                    (local.get 6)
                                                    (i32.const 8)))
                                                (local.tee 0
                                                  (i32.and
                                                    (i32.shr_u
                                                      (i32.add
                                                        (local.get 0)
                                                        (i32.const 1048320))
                                                      (i32.const 16))
                                                    (i32.const 8)))))
                                            (local.tee 2
                                              (i32.and
                                                (i32.shr_u
                                                  (i32.add
                                                    (local.get 2)
                                                    (i32.const 520192))
                                                  (i32.const 16))
                                                (i32.const 4)))))
                                        (local.tee 3
                                          (i32.and
                                            (i32.shr_u
                                              (i32.add
                                                (local.get 3)
                                                (i32.const 245760))
                                              (i32.const 16))
                                            (i32.const 2))))
                                      (i32.const 15))
                                    (i32.or
                                      (i32.or
                                        (local.get 0)
                                        (local.get 2))
                                      (local.get 3))))
                                (i32.const 1))
                              (i32.and
                                (i32.shr_u
                                  (local.get 6)
                                  (i32.add
                                    (local.get 0)
                                    (i32.const 21)))
                                (i32.const 1)))
                            (i32.const 28)))))
                    (i32.store offset=28
                      (local.get 1)
                      (local.get 0))
                    (local.set 2
                      (i32.add
                        (i32.shl
                          (local.get 0)
                          (i32.const 2))
                        (i32.const 4620)))
                    (block  ;; label = @9
                      (if  ;; label = @10
                        (i32.eqz
                          (i32.and
                            (local.tee 3
                              (i32.load
                                (i32.const 4320)))
                            (local.tee 5
                              (i32.shl
                                (i32.const 1)
                                (local.get 0)))))
                        (then
                          (i32.store
                            (i32.const 4320)
                            (i32.or
                              (local.get 3)
                              (local.get 5)))
                          (i32.store
                            (local.get 2)
                            (local.get 1))
                          (i32.store offset=24
                            (local.get 1)
                            (local.get 2))
                          (br 1 (;@9;))))
                      (local.set 0
                        (i32.shl
                          (local.get 6)
                          (select
                            (i32.const 0)
                            (i32.sub
                              (i32.const 25)
                              (i32.shr_u
                                (local.get 0)
                                (i32.const 1)))
                            (i32.eq
                              (local.get 0)
                              (i32.const 31)))))
                      (local.set 3
                        (i32.load
                          (local.get 2)))
                      (loop  ;; label = @10
                        (br_if 4 (;@6;)
                          (i32.eq
                            (i32.and
                              (i32.load offset=4
                                (local.tee 2
                                  (local.get 3)))
                              (i32.const -8))
                            (local.get 6)))
                        (local.set 3
                          (i32.shr_u
                            (local.get 0)
                            (i32.const 29)))
                        (local.set 0
                          (i32.shl
                            (local.get 0)
                            (i32.const 1)))
                        (br_if 0 (;@10;)
                          (local.tee 3
                            (i32.load
                              (local.tee 5
                                (i32.add
                                  (i32.add
                                    (local.get 2)
                                    (i32.and
                                      (local.get 3)
                                      (i32.const 4)))
                                  (i32.const 16)))))))
                      (i32.store
                        (local.get 5)
                        (local.get 1))
                      (i32.store offset=24
                        (local.get 1)
                        (local.get 2)))
                    (i32.store offset=12
                      (local.get 1)
                      (local.get 1))
                    (i32.store offset=8
                      (local.get 1)
                      (local.get 1))
                    (br 3 (;@5;)))
                  (i32.store offset=12
                    (local.tee 0
                      (i32.load offset=8
                        (local.get 1)))
                    (local.get 4))
                  (i32.store offset=8
                    (local.get 1)
                    (local.get 4))
                  (i32.store offset=24
                    (local.get 4)
                    (i32.const 0))
                  (i32.store offset=12
                    (local.get 4)
                    (local.get 1))
                  (i32.store offset=8
                    (local.get 4)
                    (local.get 0)))
                (local.set 0
                  (i32.add
                    (local.get 7)
                    (i32.const 8)))
                (br 5 (;@1;)))
              (i32.store offset=12
                (local.tee 0
                  (i32.load offset=8
                    (local.get 2)))
                (local.get 1))
              (i32.store offset=8
                (local.get 2)
                (local.get 1))
              (i32.store offset=24
                (local.get 1)
                (i32.const 0))
              (i32.store offset=12
                (local.get 1)
                (local.get 2))
              (i32.store offset=8
                (local.get 1)
                (local.get 0)))
            (br_if 0 (;@4;)
              (i32.le_u
                (local.tee 0
                  (i32.load
                    (i32.const 4328)))
                (local.get 4)))
            (i32.store
              (i32.const 4328)
              (local.tee 1
                (i32.sub
                  (local.get 0)
                  (local.get 4))))
            (i32.store
              (i32.const 4340)
              (local.tee 2
                (i32.add
                  (local.tee 0
                    (i32.load
                      (i32.const 4340)))
                  (local.get 4))))
            (i32.store offset=4
              (local.get 2)
              (i32.or
                (local.get 1)
                (i32.const 1)))
            (i32.store offset=4
              (local.get 0)
              (i32.or
                (local.get 4)
                (i32.const 3)))
            (local.set 0
              (i32.add
                (local.get 0)
                (i32.const 8)))
            (br 3 (;@1;)))
          (i32.store
            (call $__errno_location)
            (i32.const 48))
          (local.set 0
            (i32.const 0))
          (br 2 (;@1;)))
        (block  ;; label = @3
          (br_if 0 (;@3;)
            (i32.eqz
              (local.get 9)))
          (block  ;; label = @4
            (if  ;; label = @5
              (i32.eq
                (i32.load
                  (local.tee 0
                    (i32.add
                      (i32.shl
                        (local.tee 2
                          (i32.load offset=28
                            (local.get 5)))
                        (i32.const 2))
                      (i32.const 4620))))
                (local.get 5))
              (then
                (i32.store
                  (local.get 0)
                  (local.get 3))
                (br_if 1 (;@4;)
                  (local.get 3))
                (i32.store
                  (i32.const 4320)
                  (local.tee 8
                    (i32.and
                      (local.get 8)
                      (i32.rotl
                        (i32.const -2)
                        (local.get 2)))))
                (br 2 (;@3;))))
            (i32.store
              (i32.add
                (local.get 9)
                (select
                  (i32.const 16)
                  (i32.const 20)
                  (i32.eq
                    (i32.load offset=16
                      (local.get 9))
                    (local.get 5))))
              (local.get 3))
            (br_if 1 (;@3;)
              (i32.eqz
                (local.get 3))))
          (i32.store offset=24
            (local.get 3)
            (local.get 9))
          (if  ;; label = @4
            (local.tee 0
              (i32.load offset=16
                (local.get 5)))
            (then
              (i32.store offset=16
                (local.get 3)
                (local.get 0))
              (i32.store offset=24
                (local.get 0)
                (local.get 3))))
          (br_if 0 (;@3;)
            (i32.eqz
              (local.tee 0
                (i32.load offset=20
                  (local.get 5)))))
          (i32.store offset=20
            (local.get 3)
            (local.get 0))
          (i32.store offset=24
            (local.get 0)
            (local.get 3)))
        (block  ;; label = @3
          (if  ;; label = @4
            (i32.le_u
              (local.get 1)
              (i32.const 15))
            (then
              (i32.store offset=4
                (local.get 5)
                (i32.or
                  (local.tee 0
                    (i32.add
                      (local.get 1)
                      (local.get 4)))
                  (i32.const 3)))
              (i32.store offset=4
                (local.tee 0
                  (i32.add
                    (local.get 0)
                    (local.get 5)))
                (i32.or
                  (i32.load offset=4
                    (local.get 0))
                  (i32.const 1)))
              (br 1 (;@3;))))
          (i32.store offset=4
            (local.get 5)
            (i32.or
              (local.get 4)
              (i32.const 3)))
          (i32.store offset=4
            (local.get 7)
            (i32.or
              (local.get 1)
              (i32.const 1)))
          (i32.store
            (i32.add
              (local.get 1)
              (local.get 7))
            (local.get 1))
          (if  ;; label = @4
            (i32.le_u
              (local.get 1)
              (i32.const 255))
            (then
              (local.set 0
                (i32.add
                  (i32.shl
                    (local.tee 1
                      (i32.shr_u
                        (local.get 1)
                        (i32.const 3)))
                    (i32.const 3))
                  (i32.const 4356)))
              (local.set 1
                (block (result i32)  ;; label = @5
                  (if  ;; label = @6
                    (i32.eqz
                      (i32.and
                        (local.tee 2
                          (i32.load
                            (i32.const 4316)))
                        (local.tee 1
                          (i32.shl
                            (i32.const 1)
                            (local.get 1)))))
                    (then
                      (i32.store
                        (i32.const 4316)
                        (i32.or
                          (local.get 1)
                          (local.get 2)))
                      (br 1 (;@5;)
                        (local.get 0))))
                  (i32.load offset=8
                    (local.get 0))))
              (i32.store offset=8
                (local.get 0)
                (local.get 7))
              (i32.store offset=12
                (local.get 1)
                (local.get 7))
              (i32.store offset=12
                (local.get 7)
                (local.get 0))
              (i32.store offset=8
                (local.get 7)
                (local.get 1))
              (br 1 (;@3;))))
          (local.set 0
            (i32.const 31))
          (if  ;; label = @4
            (i32.le_u
              (local.get 1)
              (i32.const 16777215))
            (then
              (local.set 0
                (i32.add
                  (i32.or
                    (i32.shl
                      (local.tee 0
                        (i32.sub
                          (i32.shr_u
                            (i32.shl
                              (local.tee 4
                                (i32.shl
                                  (local.tee 2
                                    (i32.shl
                                      (local.tee 0
                                        (i32.shr_u
                                          (local.get 1)
                                          (i32.const 8)))
                                      (local.tee 0
                                        (i32.and
                                          (i32.shr_u
                                            (i32.add
                                              (local.get 0)
                                              (i32.const 1048320))
                                            (i32.const 16))
                                          (i32.const 8)))))
                                  (local.tee 2
                                    (i32.and
                                      (i32.shr_u
                                        (i32.add
                                          (local.get 2)
                                          (i32.const 520192))
                                        (i32.const 16))
                                      (i32.const 4)))))
                              (local.tee 4
                                (i32.and
                                  (i32.shr_u
                                    (i32.add
                                      (local.get 4)
                                      (i32.const 245760))
                                    (i32.const 16))
                                  (i32.const 2))))
                            (i32.const 15))
                          (i32.or
                            (i32.or
                              (local.get 0)
                              (local.get 2))
                            (local.get 4))))
                      (i32.const 1))
                    (i32.and
                      (i32.shr_u
                        (local.get 1)
                        (i32.add
                          (local.get 0)
                          (i32.const 21)))
                      (i32.const 1)))
                  (i32.const 28)))))
          (i32.store offset=28
            (local.get 7)
            (local.get 0))
          (i64.store offset=16 align=4
            (local.get 7)
            (i64.const 0))
          (local.set 2
            (i32.add
              (i32.shl
                (local.get 0)
                (i32.const 2))
              (i32.const 4620)))
          (block  ;; label = @4
            (block  ;; label = @5
              (if  ;; label = @6
                (i32.eqz
                  (i32.and
                    (local.get 8)
                    (local.tee 4
                      (i32.shl
                        (i32.const 1)
                        (local.get 0)))))
                (then
                  (i32.store
                    (i32.const 4320)
                    (i32.or
                      (local.get 4)
                      (local.get 8)))
                  (i32.store
                    (local.get 2)
                    (local.get 7))
                  (i32.store offset=24
                    (local.get 7)
                    (local.get 2))
                  (br 1 (;@5;))))
              (local.set 0
                (i32.shl
                  (local.get 1)
                  (select
                    (i32.const 0)
                    (i32.sub
                      (i32.const 25)
                      (i32.shr_u
                        (local.get 0)
                        (i32.const 1)))
                    (i32.eq
                      (local.get 0)
                      (i32.const 31)))))
              (local.set 4
                (i32.load
                  (local.get 2)))
              (loop  ;; label = @6
                (br_if 2 (;@4;)
                  (i32.eq
                    (i32.and
                      (i32.load offset=4
                        (local.tee 2
                          (local.get 4)))
                      (i32.const -8))
                    (local.get 1)))
                (local.set 4
                  (i32.shr_u
                    (local.get 0)
                    (i32.const 29)))
                (local.set 0
                  (i32.shl
                    (local.get 0)
                    (i32.const 1)))
                (br_if 0 (;@6;)
                  (local.tee 4
                    (i32.load
                      (local.tee 3
                        (i32.add
                          (i32.add
                            (local.get 2)
                            (i32.and
                              (local.get 4)
                              (i32.const 4)))
                          (i32.const 16)))))))
              (i32.store
                (local.get 3)
                (local.get 7))
              (i32.store offset=24
                (local.get 7)
                (local.get 2)))
            (i32.store offset=12
              (local.get 7)
              (local.get 7))
            (i32.store offset=8
              (local.get 7)
              (local.get 7))
            (br 1 (;@3;)))
          (i32.store offset=12
            (local.tee 0
              (i32.load offset=8
                (local.get 2)))
            (local.get 7))
          (i32.store offset=8
            (local.get 2)
            (local.get 7))
          (i32.store offset=24
            (local.get 7)
            (i32.const 0))
          (i32.store offset=12
            (local.get 7)
            (local.get 2))
          (i32.store offset=8
            (local.get 7)
            (local.get 0)))
        (local.set 0
          (i32.add
            (local.get 5)
            (i32.const 8)))
        (br 1 (;@1;)))
      (block  ;; label = @2
        (br_if 0 (;@2;)
          (i32.eqz
            (local.get 10)))
        (block  ;; label = @3
          (if  ;; label = @4
            (i32.eq
              (i32.load
                (local.tee 0
                  (i32.add
                    (i32.shl
                      (local.tee 2
                        (i32.load offset=28
                          (local.get 3)))
                      (i32.const 2))
                    (i32.const 4620))))
              (local.get 3))
            (then
              (i32.store
                (local.get 0)
                (local.get 5))
              (br_if 1 (;@3;)
                (local.get 5))
              (i32.store
                (i32.const 4320)
                (i32.and
                  (local.get 9)
                  (i32.rotl
                    (i32.const -2)
                    (local.get 2))))
              (br 2 (;@2;))))
          (i32.store
            (i32.add
              (local.get 10)
              (select
                (i32.const 16)
                (i32.const 20)
                (i32.eq
                  (i32.load offset=16
                    (local.get 10))
                  (local.get 3))))
            (local.get 5))
          (br_if 1 (;@2;)
            (i32.eqz
              (local.get 5))))
        (i32.store offset=24
          (local.get 5)
          (local.get 10))
        (if  ;; label = @3
          (local.tee 0
            (i32.load offset=16
              (local.get 3)))
          (then
            (i32.store offset=16
              (local.get 5)
              (local.get 0))
            (i32.store offset=24
              (local.get 0)
              (local.get 5))))
        (br_if 0 (;@2;)
          (i32.eqz
            (local.tee 0
              (i32.load offset=20
                (local.get 3)))))
        (i32.store offset=20
          (local.get 5)
          (local.get 0))
        (i32.store offset=24
          (local.get 0)
          (local.get 5)))
      (block  ;; label = @2
        (if  ;; label = @3
          (i32.le_u
            (local.get 1)
            (i32.const 15))
          (then
            (i32.store offset=4
              (local.get 3)
              (i32.or
                (local.tee 0
                  (i32.add
                    (local.get 1)
                    (local.get 4)))
                (i32.const 3)))
            (i32.store offset=4
              (local.tee 0
                (i32.add
                  (local.get 0)
                  (local.get 3)))
              (i32.or
                (i32.load offset=4
                  (local.get 0))
                (i32.const 1)))
            (br 1 (;@2;))))
        (i32.store offset=4
          (local.get 3)
          (i32.or
            (local.get 4)
            (i32.const 3)))
        (i32.store offset=4
          (local.get 11)
          (i32.or
            (local.get 1)
            (i32.const 1)))
        (i32.store
          (i32.add
            (local.get 1)
            (local.get 11))
          (local.get 1))
        (if  ;; label = @3
          (local.get 8)
          (then
            (local.set 2
              (i32.add
                (i32.shl
                  (local.tee 4
                    (i32.shr_u
                      (local.get 8)
                      (i32.const 3)))
                  (i32.const 3))
                (i32.const 4356)))
            (local.set 0
              (i32.load
                (i32.const 4336)))
            (local.set 4
              (block (result i32)  ;; label = @4
                (if  ;; label = @5
                  (i32.eqz
                    (i32.and
                      (local.tee 4
                        (i32.shl
                          (i32.const 1)
                          (local.get 4)))
                      (local.get 6)))
                  (then
                    (i32.store
                      (i32.const 4316)
                      (i32.or
                        (local.get 4)
                        (local.get 6)))
                    (br 1 (;@4;)
                      (local.get 2))))
                (i32.load offset=8
                  (local.get 2))))
            (i32.store offset=8
              (local.get 2)
              (local.get 0))
            (i32.store offset=12
              (local.get 4)
              (local.get 0))
            (i32.store offset=12
              (local.get 0)
              (local.get 2))
            (i32.store offset=8
              (local.get 0)
              (local.get 4))))
        (i32.store
          (i32.const 4336)
          (local.get 11))
        (i32.store
          (i32.const 4324)
          (local.get 1)))
      (local.set 0
        (i32.add
          (local.get 3)
          (i32.const 8))))
    (global.set $__stack_pointer
      (i32.add
        (local.get 12)
        (i32.const 16)))
    (local.get 0))
  (func $dlfree (type 1) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.eqz
          (local.get 0)))
      (local.set 5
        (i32.add
          (local.tee 2
            (i32.sub
              (local.get 0)
              (i32.const 8)))
          (local.tee 0
            (i32.and
              (local.tee 1
                (i32.load
                  (i32.sub
                    (local.get 0)
                    (i32.const 4))))
              (i32.const -8)))))
      (block  ;; label = @2
        (br_if 0 (;@2;)
          (i32.and
            (local.get 1)
            (i32.const 1)))
        (br_if 1 (;@1;)
          (i32.eqz
            (i32.and
              (local.get 1)
              (i32.const 3))))
        (br_if 1 (;@1;)
          (i32.lt_u
            (local.tee 2
              (i32.sub
                (local.get 2)
                (local.tee 1
                  (i32.load
                    (local.get 2)))))
            (local.tee 4
              (i32.load
                (i32.const 4332)))))
        (local.set 0
          (i32.add
            (local.get 0)
            (local.get 1)))
        (if  ;; label = @3
          (i32.ne
            (local.get 2)
            (i32.load
              (i32.const 4336)))
          (then
            (if  ;; label = @4
              (i32.le_u
                (local.get 1)
                (i32.const 255))
              (then
                (drop
                  (i32.eq
                    (local.tee 4
                      (i32.load offset=8
                        (local.get 2)))
                    (local.tee 3
                      (i32.add
                        (i32.shl
                          (local.tee 7
                            (i32.shr_u
                              (local.get 1)
                              (i32.const 3)))
                          (i32.const 3))
                        (i32.const 4356)))))
                (if  ;; label = @5
                  (i32.eq
                    (local.get 4)
                    (local.tee 1
                      (i32.load offset=12
                        (local.get 2))))
                  (then
                    (i32.store
                      (i32.const 4316)
                      (i32.and
                        (i32.load
                          (i32.const 4316))
                        (i32.rotl
                          (i32.const -2)
                          (local.get 7))))
                    (br 3 (;@2;))))
                (i32.store offset=12
                  (local.get 4)
                  (local.get 1))
                (i32.store offset=8
                  (local.get 1)
                  (local.get 4))
                (br 2 (;@2;))))
            (local.set 6
              (i32.load offset=24
                (local.get 2)))
            (block  ;; label = @4
              (if  ;; label = @5
                (i32.ne
                  (local.get 2)
                  (local.tee 3
                    (i32.load offset=12
                      (local.get 2))))
                (then
                  (i32.store offset=12
                    (local.tee 1
                      (i32.load offset=8
                        (local.get 2)))
                    (local.get 3))
                  (i32.store offset=8
                    (local.get 3)
                    (local.get 1))
                  (br 1 (;@4;))))
              (block  ;; label = @5
                (br_if 0 (;@5;)
                  (local.tee 4
                    (i32.load
                      (local.tee 1
                        (i32.add
                          (local.get 2)
                          (i32.const 20))))))
                (br_if 0 (;@5;)
                  (local.tee 4
                    (i32.load
                      (local.tee 1
                        (i32.add
                          (local.get 2)
                          (i32.const 16))))))
                (local.set 3
                  (i32.const 0))
                (br 1 (;@4;)))
              (loop  ;; label = @5
                (local.set 7
                  (local.get 1))
                (br_if 0 (;@5;)
                  (local.tee 4
                    (i32.load
                      (local.tee 1
                        (i32.add
                          (local.tee 3
                            (local.get 4))
                          (i32.const 20))))))
                (local.set 1
                  (i32.add
                    (local.get 3)
                    (i32.const 16)))
                (br_if 0 (;@5;)
                  (local.tee 4
                    (i32.load offset=16
                      (local.get 3)))))
              (i32.store
                (local.get 7)
                (i32.const 0)))
            (br_if 1 (;@2;)
              (i32.eqz
                (local.get 6)))
            (block  ;; label = @4
              (if  ;; label = @5
                (i32.eq
                  (local.get 2)
                  (i32.load
                    (local.tee 1
                      (i32.add
                        (i32.shl
                          (local.tee 4
                            (i32.load offset=28
                              (local.get 2)))
                          (i32.const 2))
                        (i32.const 4620)))))
                (then
                  (i32.store
                    (local.get 1)
                    (local.get 3))
                  (br_if 1 (;@4;)
                    (local.get 3))
                  (i32.store
                    (i32.const 4320)
                    (i32.and
                      (i32.load
                        (i32.const 4320))
                      (i32.rotl
                        (i32.const -2)
                        (local.get 4))))
                  (br 3 (;@2;))))
              (i32.store
                (i32.add
                  (local.get 6)
                  (select
                    (i32.const 16)
                    (i32.const 20)
                    (i32.eq
                      (i32.load offset=16
                        (local.get 6))
                      (local.get 2))))
                (local.get 3))
              (br_if 2 (;@2;)
                (i32.eqz
                  (local.get 3))))
            (i32.store offset=24
              (local.get 3)
              (local.get 6))
            (if  ;; label = @4
              (local.tee 1
                (i32.load offset=16
                  (local.get 2)))
              (then
                (i32.store offset=16
                  (local.get 3)
                  (local.get 1))
                (i32.store offset=24
                  (local.get 1)
                  (local.get 3))))
            (br_if 1 (;@2;)
              (i32.eqz
                (local.tee 1
                  (i32.load offset=20
                    (local.get 2)))))
            (i32.store offset=20
              (local.get 3)
              (local.get 1))
            (i32.store offset=24
              (local.get 1)
              (local.get 3))
            (br 1 (;@2;))))
        (br_if 0 (;@2;)
          (i32.ne
            (i32.and
              (local.tee 1
                (i32.load offset=4
                  (local.get 5)))
              (i32.const 3))
            (i32.const 3)))
        (i32.store
          (i32.const 4324)
          (local.get 0))
        (i32.store offset=4
          (local.get 5)
          (i32.and
            (local.get 1)
            (i32.const -2)))
        (i32.store offset=4
          (local.get 2)
          (i32.or
            (local.get 0)
            (i32.const 1)))
        (i32.store
          (i32.add
            (local.get 0)
            (local.get 2))
          (local.get 0))
        (return))
      (br_if 0 (;@1;)
        (i32.ge_u
          (local.get 2)
          (local.get 5)))
      (br_if 0 (;@1;)
        (i32.eqz
          (i32.and
            (local.tee 1
              (i32.load offset=4
                (local.get 5)))
            (i32.const 1))))
      (block  ;; label = @2
        (if  ;; label = @3
          (i32.eqz
            (i32.and
              (local.get 1)
              (i32.const 2)))
          (then
            (if  ;; label = @4
              (i32.eq
                (local.get 5)
                (i32.load
                  (i32.const 4340)))
              (then
                (i32.store
                  (i32.const 4340)
                  (local.get 2))
                (i32.store
                  (i32.const 4328)
                  (local.tee 0
                    (i32.add
                      (i32.load
                        (i32.const 4328))
                      (local.get 0))))
                (i32.store offset=4
                  (local.get 2)
                  (i32.or
                    (local.get 0)
                    (i32.const 1)))
                (br_if 3 (;@1;)
                  (i32.ne
                    (local.get 2)
                    (i32.load
                      (i32.const 4336))))
                (i32.store
                  (i32.const 4324)
                  (i32.const 0))
                (i32.store
                  (i32.const 4336)
                  (i32.const 0))
                (return)))
            (if  ;; label = @4
              (i32.eq
                (local.get 5)
                (i32.load
                  (i32.const 4336)))
              (then
                (i32.store
                  (i32.const 4336)
                  (local.get 2))
                (i32.store
                  (i32.const 4324)
                  (local.tee 0
                    (i32.add
                      (i32.load
                        (i32.const 4324))
                      (local.get 0))))
                (i32.store offset=4
                  (local.get 2)
                  (i32.or
                    (local.get 0)
                    (i32.const 1)))
                (i32.store
                  (i32.add
                    (local.get 0)
                    (local.get 2))
                  (local.get 0))
                (return)))
            (local.set 0
              (i32.add
                (i32.and
                  (local.get 1)
                  (i32.const -8))
                (local.get 0)))
            (block  ;; label = @4
              (if  ;; label = @5
                (i32.le_u
                  (local.get 1)
                  (i32.const 255))
                (then
                  (drop
                    (i32.eq
                      (local.tee 4
                        (i32.load offset=8
                          (local.get 5)))
                      (local.tee 3
                        (i32.add
                          (i32.shl
                            (local.tee 7
                              (i32.shr_u
                                (local.get 1)
                                (i32.const 3)))
                            (i32.const 3))
                          (i32.const 4356)))))
                  (if  ;; label = @6
                    (i32.eq
                      (local.get 4)
                      (local.tee 1
                        (i32.load offset=12
                          (local.get 5))))
                    (then
                      (i32.store
                        (i32.const 4316)
                        (i32.and
                          (i32.load
                            (i32.const 4316))
                          (i32.rotl
                            (i32.const -2)
                            (local.get 7))))
                      (br 2 (;@4;))))
                  (i32.store offset=12
                    (local.get 4)
                    (local.get 1))
                  (i32.store offset=8
                    (local.get 1)
                    (local.get 4))
                  (br 1 (;@4;))))
              (local.set 6
                (i32.load offset=24
                  (local.get 5)))
              (block  ;; label = @5
                (if  ;; label = @6
                  (i32.ne
                    (local.get 5)
                    (local.tee 3
                      (i32.load offset=12
                        (local.get 5))))
                  (then
                    (drop
                      (i32.lt_u
                        (local.tee 1
                          (i32.load offset=8
                            (local.get 5)))
                        (i32.load
                          (i32.const 4332))))
                    (i32.store offset=12
                      (local.get 1)
                      (local.get 3))
                    (i32.store offset=8
                      (local.get 3)
                      (local.get 1))
                    (br 1 (;@5;))))
                (block  ;; label = @6
                  (br_if 0 (;@6;)
                    (local.tee 4
                      (i32.load
                        (local.tee 1
                          (i32.add
                            (local.get 5)
                            (i32.const 20))))))
                  (br_if 0 (;@6;)
                    (local.tee 4
                      (i32.load
                        (local.tee 1
                          (i32.add
                            (local.get 5)
                            (i32.const 16))))))
                  (local.set 3
                    (i32.const 0))
                  (br 1 (;@5;)))
                (loop  ;; label = @6
                  (local.set 7
                    (local.get 1))
                  (br_if 0 (;@6;)
                    (local.tee 4
                      (i32.load
                        (local.tee 1
                          (i32.add
                            (local.tee 3
                              (local.get 4))
                            (i32.const 20))))))
                  (local.set 1
                    (i32.add
                      (local.get 3)
                      (i32.const 16)))
                  (br_if 0 (;@6;)
                    (local.tee 4
                      (i32.load offset=16
                        (local.get 3)))))
                (i32.store
                  (local.get 7)
                  (i32.const 0)))
              (br_if 0 (;@4;)
                (i32.eqz
                  (local.get 6)))
              (block  ;; label = @5
                (if  ;; label = @6
                  (i32.eq
                    (local.get 5)
                    (i32.load
                      (local.tee 1
                        (i32.add
                          (i32.shl
                            (local.tee 4
                              (i32.load offset=28
                                (local.get 5)))
                            (i32.const 2))
                          (i32.const 4620)))))
                  (then
                    (i32.store
                      (local.get 1)
                      (local.get 3))
                    (br_if 1 (;@5;)
                      (local.get 3))
                    (i32.store
                      (i32.const 4320)
                      (i32.and
                        (i32.load
                          (i32.const 4320))
                        (i32.rotl
                          (i32.const -2)
                          (local.get 4))))
                    (br 2 (;@4;))))
                (i32.store
                  (i32.add
                    (local.get 6)
                    (select
                      (i32.const 16)
                      (i32.const 20)
                      (i32.eq
                        (i32.load offset=16
                          (local.get 6))
                        (local.get 5))))
                  (local.get 3))
                (br_if 1 (;@4;)
                  (i32.eqz
                    (local.get 3))))
              (i32.store offset=24
                (local.get 3)
                (local.get 6))
              (if  ;; label = @5
                (local.tee 1
                  (i32.load offset=16
                    (local.get 5)))
                (then
                  (i32.store offset=16
                    (local.get 3)
                    (local.get 1))
                  (i32.store offset=24
                    (local.get 1)
                    (local.get 3))))
              (br_if 0 (;@4;)
                (i32.eqz
                  (local.tee 1
                    (i32.load offset=20
                      (local.get 5)))))
              (i32.store offset=20
                (local.get 3)
                (local.get 1))
              (i32.store offset=24
                (local.get 1)
                (local.get 3)))
            (i32.store offset=4
              (local.get 2)
              (i32.or
                (local.get 0)
                (i32.const 1)))
            (i32.store
              (i32.add
                (local.get 0)
                (local.get 2))
              (local.get 0))
            (br_if 1 (;@2;)
              (i32.ne
                (local.get 2)
                (i32.load
                  (i32.const 4336))))
            (i32.store
              (i32.const 4324)
              (local.get 0))
            (return)))
        (i32.store offset=4
          (local.get 5)
          (i32.and
            (local.get 1)
            (i32.const -2)))
        (i32.store offset=4
          (local.get 2)
          (i32.or
            (local.get 0)
            (i32.const 1)))
        (i32.store
          (i32.add
            (local.get 0)
            (local.get 2))
          (local.get 0)))
      (if  ;; label = @2
        (i32.le_u
          (local.get 0)
          (i32.const 255))
        (then
          (local.set 0
            (i32.add
              (i32.shl
                (local.tee 1
                  (i32.shr_u
                    (local.get 0)
                    (i32.const 3)))
                (i32.const 3))
              (i32.const 4356)))
          (local.set 1
            (block (result i32)  ;; label = @3
              (if  ;; label = @4
                (i32.eqz
                  (i32.and
                    (local.tee 4
                      (i32.load
                        (i32.const 4316)))
                    (local.tee 1
                      (i32.shl
                        (i32.const 1)
                        (local.get 1)))))
                (then
                  (i32.store
                    (i32.const 4316)
                    (i32.or
                      (local.get 1)
                      (local.get 4)))
                  (br 1 (;@3;)
                    (local.get 0))))
              (i32.load offset=8
                (local.get 0))))
          (i32.store offset=8
            (local.get 0)
            (local.get 2))
          (i32.store offset=12
            (local.get 1)
            (local.get 2))
          (i32.store offset=12
            (local.get 2)
            (local.get 0))
          (i32.store offset=8
            (local.get 2)
            (local.get 1))
          (return)))
      (local.set 1
        (i32.const 31))
      (i64.store offset=16 align=4
        (local.get 2)
        (i64.const 0))
      (if  ;; label = @2
        (i32.le_u
          (local.get 0)
          (i32.const 16777215))
        (then
          (local.set 1
            (i32.add
              (i32.or
                (i32.shl
                  (local.tee 1
                    (i32.sub
                      (i32.shr_u
                        (i32.shl
                          (local.tee 3
                            (i32.shl
                              (local.tee 4
                                (i32.shl
                                  (local.tee 1
                                    (i32.shr_u
                                      (local.get 0)
                                      (i32.const 8)))
                                  (local.tee 1
                                    (i32.and
                                      (i32.shr_u
                                        (i32.add
                                          (local.get 1)
                                          (i32.const 1048320))
                                        (i32.const 16))
                                      (i32.const 8)))))
                              (local.tee 4
                                (i32.and
                                  (i32.shr_u
                                    (i32.add
                                      (local.get 4)
                                      (i32.const 520192))
                                    (i32.const 16))
                                  (i32.const 4)))))
                          (local.tee 3
                            (i32.and
                              (i32.shr_u
                                (i32.add
                                  (local.get 3)
                                  (i32.const 245760))
                                (i32.const 16))
                              (i32.const 2))))
                        (i32.const 15))
                      (i32.or
                        (i32.or
                          (local.get 1)
                          (local.get 4))
                        (local.get 3))))
                  (i32.const 1))
                (i32.and
                  (i32.shr_u
                    (local.get 0)
                    (i32.add
                      (local.get 1)
                      (i32.const 21)))
                  (i32.const 1)))
              (i32.const 28)))))
      (i32.store offset=28
        (local.get 2)
        (local.get 1))
      (local.set 4
        (i32.add
          (i32.shl
            (local.get 1)
            (i32.const 2))
          (i32.const 4620)))
      (block  ;; label = @2
        (block  ;; label = @3
          (block  ;; label = @4
            (if  ;; label = @5
              (i32.eqz
                (i32.and
                  (local.tee 3
                    (i32.load
                      (i32.const 4320)))
                  (local.tee 5
                    (i32.shl
                      (i32.const 1)
                      (local.get 1)))))
              (then
                (i32.store
                  (i32.const 4320)
                  (i32.or
                    (local.get 3)
                    (local.get 5)))
                (i32.store
                  (local.get 4)
                  (local.get 2))
                (i32.store offset=24
                  (local.get 2)
                  (local.get 4))
                (br 1 (;@4;))))
            (local.set 1
              (i32.shl
                (local.get 0)
                (select
                  (i32.const 0)
                  (i32.sub
                    (i32.const 25)
                    (i32.shr_u
                      (local.get 1)
                      (i32.const 1)))
                  (i32.eq
                    (local.get 1)
                    (i32.const 31)))))
            (local.set 3
              (i32.load
                (local.get 4)))
            (loop  ;; label = @5
              (br_if 2 (;@3;)
                (i32.eq
                  (i32.and
                    (i32.load offset=4
                      (local.tee 4
                        (local.get 3)))
                    (i32.const -8))
                  (local.get 0)))
              (local.set 3
                (i32.shr_u
                  (local.get 1)
                  (i32.const 29)))
              (local.set 1
                (i32.shl
                  (local.get 1)
                  (i32.const 1)))
              (br_if 0 (;@5;)
                (local.tee 3
                  (i32.load
                    (local.tee 5
                      (i32.add
                        (i32.add
                          (local.get 4)
                          (i32.and
                            (local.get 3)
                            (i32.const 4)))
                        (i32.const 16)))))))
            (i32.store
              (local.get 5)
              (local.get 2))
            (i32.store offset=24
              (local.get 2)
              (local.get 4)))
          (i32.store offset=12
            (local.get 2)
            (local.get 2))
          (i32.store offset=8
            (local.get 2)
            (local.get 2))
          (br 1 (;@2;)))
        (i32.store offset=12
          (local.tee 0
            (i32.load offset=8
              (local.get 4)))
          (local.get 2))
        (i32.store offset=8
          (local.get 4)
          (local.get 2))
        (i32.store offset=24
          (local.get 2)
          (i32.const 0))
        (i32.store offset=12
          (local.get 2)
          (local.get 4))
        (i32.store offset=8
          (local.get 2)
          (local.get 0)))
      (i32.store
        (i32.const 4348)
        (select
          (local.tee 2
            (i32.sub
              (i32.load
                (i32.const 4348))
              (i32.const 1)))
          (i32.const -1)
          (local.get 2)))))
  (func $emscripten_get_heap_size (type 0) (result i32)
    (i32.shl
      (memory.size)
      (i32.const 16)))
  (func $sbrk (type 2) (param i32) (result i32)
    (local i32 i32)
    (local.set 0
      (i32.add
        (local.tee 1
          (i32.load
            (i32.const 3920)))
        (local.tee 2
          (i32.and
            (i32.add
              (local.get 0)
              (i32.const 3))
            (i32.const -4)))))
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (select
          (local.get 2)
          (i32.const 0)
          (i32.le_u
            (local.get 0)
            (local.get 1))))
      (if  ;; label = @2
        (i32.lt_u
          (call $emscripten_get_heap_size)
          (local.get 0))
        (then
          (br_if 1 (;@1;)
            (i32.eqz
              (call $emscripten_resize_heap
                (local.get 0))))))
      (i32.store
        (i32.const 3920)
        (local.get 0))
      (return
        (local.get 1)))
    (i32.store
      (call $__errno_location)
      (i32.const 48))
    (i32.const -1))
  (func $_Exit (type 1) (param i32)
    (call $__wasi_proc_exit
      (local.get 0))
    (unreachable))
  (func $__wasi_syscall_ret (type 2) (param i32) (result i32)
    (if  ;; label = @1
      (i32.eqz
        (local.get 0))
      (then
        (return
          (i32.const 0))))
    (i32.store
      (call $__errno_location)
      (local.get 0))
    (i32.const -1))
  (func $abort (type 4)
    (call $_Exit
      (i32.const 1))
    (unreachable))
  (func $emscripten_memcpy_big (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32)
    (if  ;; label = @1
      (local.get 2)
      (then
        (local.set 3
          (local.get 0))
        (loop  ;; label = @2
          (local.set 3
            (call $memcpy
              (local.get 3)
              (local.get 1)
              (local.tee 4
                (select
                  (local.get 2)
                  (i32.const 508)
                  (i32.lt_u
                    (local.get 2)
                    (i32.const 508))))))
          (local.set 1
            (i32.add
              (local.get 1)
              (i32.const 508)))
          (local.set 3
            (i32.add
              (local.get 3)
              (i32.const 508)))
          (br_if 0 (;@2;)
            (local.tee 2
              (i32.sub
                (local.get 2)
                (local.get 4)))))))
    (local.get 0))
  (func $emscripten_resize_heap (type 2) (param i32) (result i32)
    (i32.const 0))
  (func $libc_exit_fini (type 4)
    (call $_fini))
  (func $exit (type 1) (param i32)
    (call $_fini)
    (call $libc_exit_fini)
    (call $__stdio_exit)
    (call $_Exit
      (local.get 0))
    (unreachable))
  (func $_fini (type 4)
    (nop))
  (func $memcpy (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    (if  ;; label = @1
      (i32.ge_u
        (local.get 2)
        (i32.const 512))
      (then
        (drop
          (call $emscripten_memcpy_big
            (local.get 0)
            (local.get 1)
            (local.get 2)))
        (return
          (local.get 0))))
    (local.set 3
      (i32.add
        (local.get 0)
        (local.get 2)))
    (block  ;; label = @1
      (if  ;; label = @2
        (i32.eqz
          (i32.and
            (i32.xor
              (local.get 0)
              (local.get 1))
            (i32.const 3)))
        (then
          (block  ;; label = @3
            (if  ;; label = @4
              (i32.eqz
                (i32.and
                  (local.get 0)
                  (i32.const 3)))
              (then
                (local.set 2
                  (local.get 0))
                (br 1 (;@3;))))
            (if  ;; label = @4
              (i32.lt_s
                (local.get 2)
                (i32.const 1))
              (then
                (local.set 2
                  (local.get 0))
                (br 1 (;@3;))))
            (local.set 2
              (local.get 0))
            (loop  ;; label = @4
              (i32.store8
                (local.get 2)
                (i32.load8_u
                  (local.get 1)))
              (local.set 1
                (i32.add
                  (local.get 1)
                  (i32.const 1)))
              (br_if 1 (;@3;)
                (i32.eqz
                  (i32.and
                    (local.tee 2
                      (i32.add
                        (local.get 2)
                        (i32.const 1)))
                    (i32.const 3))))
              (br_if 0 (;@4;)
                (i32.lt_u
                  (local.get 2)
                  (local.get 3)))))
          (block  ;; label = @3
            (br_if 0 (;@3;)
              (i32.lt_u
                (local.tee 4
                  (i32.and
                    (local.get 3)
                    (i32.const -4)))
                (i32.const 64)))
            (br_if 0 (;@3;)
              (i32.gt_u
                (local.get 2)
                (local.tee 5
                  (i32.add
                    (local.get 4)
                    (i32.const -64)))))
            (loop  ;; label = @4
              (i32.store
                (local.get 2)
                (i32.load
                  (local.get 1)))
              (i32.store offset=4
                (local.get 2)
                (i32.load offset=4
                  (local.get 1)))
              (i32.store offset=8
                (local.get 2)
                (i32.load offset=8
                  (local.get 1)))
              (i32.store offset=12
                (local.get 2)
                (i32.load offset=12
                  (local.get 1)))
              (i32.store offset=16
                (local.get 2)
                (i32.load offset=16
                  (local.get 1)))
              (i32.store offset=20
                (local.get 2)
                (i32.load offset=20
                  (local.get 1)))
              (i32.store offset=24
                (local.get 2)
                (i32.load offset=24
                  (local.get 1)))
              (i32.store offset=28
                (local.get 2)
                (i32.load offset=28
                  (local.get 1)))
              (i32.store offset=32
                (local.get 2)
                (i32.load offset=32
                  (local.get 1)))
              (i32.store offset=36
                (local.get 2)
                (i32.load offset=36
                  (local.get 1)))
              (i32.store offset=40
                (local.get 2)
                (i32.load offset=40
                  (local.get 1)))
              (i32.store offset=44
                (local.get 2)
                (i32.load offset=44
                  (local.get 1)))
              (i32.store offset=48
                (local.get 2)
                (i32.load offset=48
                  (local.get 1)))
              (i32.store offset=52
                (local.get 2)
                (i32.load offset=52
                  (local.get 1)))
              (i32.store offset=56
                (local.get 2)
                (i32.load offset=56
                  (local.get 1)))
              (i32.store offset=60
                (local.get 2)
                (i32.load offset=60
                  (local.get 1)))
              (local.set 1
                (i32.sub
                  (local.get 1)
                  (i32.const -64)))
              (br_if 0 (;@4;)
                (i32.le_u
                  (local.tee 2
                    (i32.sub
                      (local.get 2)
                      (i32.const -64)))
                  (local.get 5)))))
          (br_if 1 (;@1;)
            (i32.ge_u
              (local.get 2)
              (local.get 4)))
          (loop  ;; label = @3
            (i32.store
              (local.get 2)
              (i32.load
                (local.get 1)))
            (local.set 1
              (i32.add
                (local.get 1)
                (i32.const 4)))
            (br_if 0 (;@3;)
              (i32.lt_u
                (local.tee 2
                  (i32.add
                    (local.get 2)
                    (i32.const 4)))
                (local.get 4))))
          (br 1 (;@1;))))
      (if  ;; label = @2
        (i32.lt_u
          (local.get 3)
          (i32.const 4))
        (then
          (local.set 2
            (local.get 0))
          (br 1 (;@1;))))
      (if  ;; label = @2
        (i32.gt_u
          (local.get 0)
          (local.tee 4
            (i32.sub
              (local.get 3)
              (i32.const 4))))
        (then
          (local.set 2
            (local.get 0))
          (br 1 (;@1;))))
      (local.set 2
        (local.get 0))
      (loop  ;; label = @2
        (i32.store8
          (local.get 2)
          (i32.load8_u
            (local.get 1)))
        (i32.store8 offset=1
          (local.get 2)
          (i32.load8_u offset=1
            (local.get 1)))
        (i32.store8 offset=2
          (local.get 2)
          (i32.load8_u offset=2
            (local.get 1)))
        (i32.store8 offset=3
          (local.get 2)
          (i32.load8_u offset=3
            (local.get 1)))
        (local.set 1
          (i32.add
            (local.get 1)
            (i32.const 4)))
        (br_if 0 (;@2;)
          (i32.le_u
            (local.tee 2
              (i32.add
                (local.get 2)
                (i32.const 4)))
            (local.get 4)))))
    (if  ;; label = @1
      (i32.lt_u
        (local.get 2)
        (local.get 3))
      (then
        (loop  ;; label = @2
          (i32.store8
            (local.get 2)
            (i32.load8_u
              (local.get 1)))
          (local.set 1
            (i32.add
              (local.get 1)
              (i32.const 1)))
          (br_if 0 (;@2;)
            (i32.ne
              (local.tee 2
                (i32.add
                  (local.get 2)
                  (i32.const 1)))
              (local.get 3))))))
    (local.get 0))
  (func $memset (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i64 i32)
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.eqz
          (local.get 2)))
      (i32.store8
        (i32.sub
          (local.tee 3
            (i32.add
              (local.get 0)
              (local.get 2)))
          (i32.const 1))
        (local.get 1))
      (i32.store8
        (local.get 0)
        (local.get 1))
      (br_if 0 (;@1;)
        (i32.lt_u
          (local.get 2)
          (i32.const 3)))
      (i32.store8
        (i32.sub
          (local.get 3)
          (i32.const 2))
        (local.get 1))
      (i32.store8 offset=1
        (local.get 0)
        (local.get 1))
      (i32.store8
        (i32.sub
          (local.get 3)
          (i32.const 3))
        (local.get 1))
      (i32.store8 offset=2
        (local.get 0)
        (local.get 1))
      (br_if 0 (;@1;)
        (i32.lt_u
          (local.get 2)
          (i32.const 7)))
      (i32.store8
        (i32.sub
          (local.get 3)
          (i32.const 4))
        (local.get 1))
      (i32.store8 offset=3
        (local.get 0)
        (local.get 1))
      (br_if 0 (;@1;)
        (i32.lt_u
          (local.get 2)
          (i32.const 9)))
      (i32.store
        (local.tee 3
          (i32.add
            (local.get 0)
            (local.tee 4
              (i32.and
                (i32.sub
                  (i32.const 0)
                  (local.get 0))
                (i32.const 3)))))
        (local.tee 1
          (i32.mul
            (i32.and
              (local.get 1)
              (i32.const 255))
            (i32.const 16843009))))
      (i32.store
        (i32.sub
          (local.tee 2
            (i32.add
              (local.get 3)
              (local.tee 4
                (i32.and
                  (i32.sub
                    (local.get 2)
                    (local.get 4))
                  (i32.const -4)))))
          (i32.const 4))
        (local.get 1))
      (br_if 0 (;@1;)
        (i32.lt_u
          (local.get 4)
          (i32.const 9)))
      (i32.store offset=8
        (local.get 3)
        (local.get 1))
      (i32.store offset=4
        (local.get 3)
        (local.get 1))
      (i32.store
        (i32.sub
          (local.get 2)
          (i32.const 8))
        (local.get 1))
      (i32.store
        (i32.sub
          (local.get 2)
          (i32.const 12))
        (local.get 1))
      (br_if 0 (;@1;)
        (i32.lt_u
          (local.get 4)
          (i32.const 25)))
      (i32.store offset=24
        (local.get 3)
        (local.get 1))
      (i32.store offset=20
        (local.get 3)
        (local.get 1))
      (i32.store offset=16
        (local.get 3)
        (local.get 1))
      (i32.store offset=12
        (local.get 3)
        (local.get 1))
      (i32.store
        (i32.sub
          (local.get 2)
          (i32.const 16))
        (local.get 1))
      (i32.store
        (i32.sub
          (local.get 2)
          (i32.const 20))
        (local.get 1))
      (i32.store
        (i32.sub
          (local.get 2)
          (i32.const 24))
        (local.get 1))
      (i32.store
        (i32.sub
          (local.get 2)
          (i32.const 28))
        (local.get 1))
      (br_if 0 (;@1;)
        (i32.lt_u
          (local.tee 2
            (i32.sub
              (local.get 4)
              (local.tee 6
                (i32.or
                  (i32.and
                    (local.get 3)
                    (i32.const 4))
                  (i32.const 24)))))
          (i32.const 32)))
      (local.set 5
        (i64.mul
          (i64.extend_i32_u
            (local.get 1))
          (i64.const 4294967297)))
      (local.set 1
        (i32.add
          (local.get 3)
          (local.get 6)))
      (loop  ;; label = @2
        (i64.store offset=24
          (local.get 1)
          (local.get 5))
        (i64.store offset=16
          (local.get 1)
          (local.get 5))
        (i64.store offset=8
          (local.get 1)
          (local.get 5))
        (i64.store
          (local.get 1)
          (local.get 5))
        (local.set 1
          (i32.add
            (local.get 1)
            (i32.const 32)))
        (br_if 0 (;@2;)
          (i32.gt_u
            (local.tee 2
              (i32.sub
                (local.get 2)
                (i32.const 32)))
            (i32.const 31)))))
    (local.get 0))
  (func $__stdio_write (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    (global.set $__stack_pointer
      (local.tee 3
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 32))))
    (i32.store offset=16
      (local.get 3)
      (local.tee 4
        (i32.load offset=28
          (local.get 0))))
    (local.set 5
      (i32.load offset=20
        (local.get 0)))
    (i32.store offset=28
      (local.get 3)
      (local.get 2))
    (i32.store offset=24
      (local.get 3)
      (local.get 1))
    (i32.store offset=20
      (local.get 3)
      (local.tee 1
        (i32.sub
          (local.get 5)
          (local.get 4))))
    (local.set 6
      (i32.add
        (local.get 1)
        (local.get 2)))
    (local.set 7
      (i32.const 2))
    (local.set 1
      (i32.add
        (local.get 3)
        (i32.const 16)))
    (local.set 4
      (block (result i32)  ;; label = @1
        (block  ;; label = @2
          (block  ;; label = @3
            (if  ;; label = @4
              (i32.eqz
                (call $__wasi_syscall_ret
                  (call $__wasi_fd_write
                    (i32.load offset=60
                      (local.get 0))
                    (i32.add
                      (local.get 3)
                      (i32.const 16))
                    (i32.const 2)
                    (i32.add
                      (local.get 3)
                      (i32.const 12)))))
              (then
                (loop  ;; label = @5
                  (br_if 2 (;@3;)
                    (i32.eq
                      (local.get 6)
                      (local.tee 4
                        (i32.load offset=12
                          (local.get 3)))))
                  (br_if 3 (;@2;)
                    (i32.le_s
                      (local.get 4)
                      (i32.const -1)))
                  (i32.store
                    (local.tee 9
                      (i32.add
                        (local.get 1)
                        (i32.shl
                          (local.tee 5
                            (i32.gt_u
                              (local.get 4)
                              (local.tee 8
                                (i32.load offset=4
                                  (local.get 1)))))
                          (i32.const 3))))
                    (i32.add
                      (local.tee 8
                        (i32.sub
                          (local.get 4)
                          (select
                            (local.get 8)
                            (i32.const 0)
                            (local.get 5))))
                      (i32.load
                        (local.get 9))))
                  (i32.store
                    (local.tee 9
                      (i32.add
                        (local.get 1)
                        (select
                          (i32.const 12)
                          (i32.const 4)
                          (local.get 5))))
                    (i32.sub
                      (i32.load
                        (local.get 9))
                      (local.get 8)))
                  (local.set 6
                    (i32.sub
                      (local.get 6)
                      (local.get 4)))
                  (br_if 0 (;@5;)
                    (i32.eqz
                      (call $__wasi_syscall_ret
                        (call $__wasi_fd_write
                          (i32.load offset=60
                            (local.get 0))
                          (local.tee 1
                            (select
                              (i32.add
                                (local.get 1)
                                (i32.const 8))
                              (local.get 1)
                              (local.get 5)))
                          (local.tee 7
                            (i32.sub
                              (local.get 7)
                              (local.get 5)))
                          (i32.add
                            (local.get 3)
                            (i32.const 12)))))))))
            (br_if 1 (;@2;)
              (i32.ne
                (local.get 6)
                (i32.const -1))))
          (i32.store offset=28
            (local.get 0)
            (local.tee 1
              (i32.load offset=44
                (local.get 0))))
          (i32.store offset=20
            (local.get 0)
            (local.get 1))
          (i32.store offset=16
            (local.get 0)
            (i32.add
              (local.get 1)
              (i32.load offset=48
                (local.get 0))))
          (br 1 (;@1;)
            (local.get 2)))
        (i32.store offset=28
          (local.get 0)
          (i32.const 0))
        (i64.store offset=16
          (local.get 0)
          (i64.const 0))
        (i32.store
          (local.get 0)
          (i32.or
            (i32.load
              (local.get 0))
            (i32.const 32)))
        (drop
          (br_if 0 (;@1;)
            (local.tee 4
              (i32.const 0))
            (i32.eq
              (local.get 7)
              (i32.const 2))))
        (i32.sub
          (local.get 2)
          (i32.load offset=4
            (local.get 1)))))
    (global.set $__stack_pointer
      (i32.add
        (local.get 3)
        (i32.const 32)))
    (local.get 4))
  (func $__emscripten_stdout_close (type 2) (param i32) (result i32)
    (i32.const 0))
  (func $__emscripten_stdout_seek (type 12) (param i32 i64 i32) (result i64)
    (i64.const 0))
  (func $isdigit (type 2) (param i32) (result i32)
    (i32.lt_u
      (i32.sub
        (local.get 0)
        (i32.const 48))
      (i32.const 10)))
  (func $memchr (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32)
    (local.set 3
      (i32.ne
        (local.get 2)
        (i32.const 0)))
    (block  ;; label = @1
      (block  ;; label = @2
        (block  ;; label = @3
          (br_if 0 (;@3;)
            (i32.eqz
              (i32.and
                (local.get 0)
                (i32.const 3))))
          (br_if 0 (;@3;)
            (i32.eqz
              (local.get 2)))
          (local.set 4
            (i32.and
              (local.get 1)
              (i32.const 255)))
          (loop  ;; label = @4
            (br_if 2 (;@2;)
              (i32.eq
                (i32.load8_u
                  (local.get 0))
                (local.get 4)))
            (local.set 3
              (i32.ne
                (local.tee 2
                  (i32.sub
                    (local.get 2)
                    (i32.const 1)))
                (i32.const 0)))
            (br_if 1 (;@3;)
              (i32.eqz
                (i32.and
                  (local.tee 0
                    (i32.add
                      (local.get 0)
                      (i32.const 1)))
                  (i32.const 3))))
            (br_if 0 (;@4;)
              (local.get 2))))
        (br_if 1 (;@1;)
          (i32.eqz
            (local.get 3))))
      (block  ;; label = @2
        (br_if 0 (;@2;)
          (i32.eq
            (i32.load8_u
              (local.get 0))
            (i32.and
              (local.get 1)
              (i32.const 255))))
        (br_if 0 (;@2;)
          (i32.lt_u
            (local.get 2)
            (i32.const 4)))
        (local.set 4
          (i32.mul
            (i32.and
              (local.get 1)
              (i32.const 255))
            (i32.const 16843009)))
        (loop  ;; label = @3
          (br_if 1 (;@2;)
            (i32.and
              (i32.and
                (i32.xor
                  (local.tee 3
                    (i32.xor
                      (i32.load
                        (local.get 0))
                      (local.get 4)))
                  (i32.const -1))
                (i32.sub
                  (local.get 3)
                  (i32.const 16843009)))
              (i32.const -2139062144)))
          (local.set 0
            (i32.add
              (local.get 0)
              (i32.const 4)))
          (br_if 0 (;@3;)
            (i32.gt_u
              (local.tee 2
                (i32.sub
                  (local.get 2)
                  (i32.const 4)))
              (i32.const 3)))))
      (br_if 0 (;@1;)
        (i32.eqz
          (local.get 2)))
      (local.set 3
        (i32.and
          (local.get 1)
          (i32.const 255)))
      (loop  ;; label = @2
        (if  ;; label = @3
          (i32.eq
            (local.get 3)
            (i32.load8_u
              (local.get 0)))
          (then
            (return
              (local.get 0))))
        (local.set 0
          (i32.add
            (local.get 0)
            (i32.const 1)))
        (br_if 0 (;@2;)
          (local.tee 2
            (i32.sub
              (local.get 2)
              (i32.const 1))))))
    (i32.const 0))
  (func $__pthread_self (type 0) (result i32)
    (i32.const 4076))
  (func $wcrtomb (type 3) (param i32 i32 i32) (result i32)
    (local i32)
    (local.set 3
      (i32.const 1))
    (block  ;; label = @1
      (if  ;; label = @2
        (local.get 0)
        (then
          (br_if 1 (;@1;)
            (i32.le_u
              (local.get 1)
              (i32.const 127)))
          (block  ;; label = @3
            (if  ;; label = @4
              (i32.eqz
                (i32.load
                  (i32.load offset=172
                    (call $__pthread_self))))
              (then
                (br_if 3 (;@1;)
                  (i32.eq
                    (i32.and
                      (local.get 1)
                      (i32.const -128))
                    (i32.const 57216)))
                (i32.store
                  (call $__errno_location)
                  (i32.const 25))
                (br 1 (;@3;))))
            (if  ;; label = @4
              (i32.le_u
                (local.get 1)
                (i32.const 2047))
              (then
                (i32.store8 offset=1
                  (local.get 0)
                  (i32.or
                    (i32.and
                      (local.get 1)
                      (i32.const 63))
                    (i32.const 128)))
                (i32.store8
                  (local.get 0)
                  (i32.or
                    (i32.shr_u
                      (local.get 1)
                      (i32.const 6))
                    (i32.const 192)))
                (return
                  (i32.const 2))))
            (if  ;; label = @4
              (i32.eqz
                (select
                  (i32.ge_u
                    (local.get 1)
                    (i32.const 55296))
                  (i32.const 0)
                  (i32.ne
                    (i32.and
                      (local.get 1)
                      (i32.const -8192))
                    (i32.const 57344))))
              (then
                (i32.store8 offset=2
                  (local.get 0)
                  (i32.or
                    (i32.and
                      (local.get 1)
                      (i32.const 63))
                    (i32.const 128)))
                (i32.store8
                  (local.get 0)
                  (i32.or
                    (i32.shr_u
                      (local.get 1)
                      (i32.const 12))
                    (i32.const 224)))
                (i32.store8 offset=1
                  (local.get 0)
                  (i32.or
                    (i32.and
                      (i32.shr_u
                        (local.get 1)
                        (i32.const 6))
                      (i32.const 63))
                    (i32.const 128)))
                (return
                  (i32.const 3))))
            (if  ;; label = @4
              (i32.le_u
                (i32.sub
                  (local.get 1)
                  (i32.const 65536))
                (i32.const 1048575))
              (then
                (i32.store8 offset=3
                  (local.get 0)
                  (i32.or
                    (i32.and
                      (local.get 1)
                      (i32.const 63))
                    (i32.const 128)))
                (i32.store8
                  (local.get 0)
                  (i32.or
                    (i32.shr_u
                      (local.get 1)
                      (i32.const 18))
                    (i32.const 240)))
                (i32.store8 offset=2
                  (local.get 0)
                  (i32.or
                    (i32.and
                      (i32.shr_u
                        (local.get 1)
                        (i32.const 6))
                      (i32.const 63))
                    (i32.const 128)))
                (i32.store8 offset=1
                  (local.get 0)
                  (i32.or
                    (i32.and
                      (i32.shr_u
                        (local.get 1)
                        (i32.const 12))
                      (i32.const 63))
                    (i32.const 128)))
                (return
                  (i32.const 4))))
            (i32.store
              (call $__errno_location)
              (i32.const 25)))
          (local.set 3
            (i32.const -1))))
      (return
        (local.get 3)))
    (i32.store8
      (local.get 0)
      (local.get 1))
    (i32.const 1))
  (func $wctomb (type 11) (param i32 i32) (result i32)
    (if  ;; label = @1
      (i32.eqz
        (local.get 0))
      (then
        (return
          (i32.const 0))))
    (call $wcrtomb
      (local.get 0)
      (local.get 1)
      (i32.const 0)))
  (func $__lock (type 1) (param i32)
    (nop))
  (func $__unlock (type 1) (param i32)
    (nop))
  (func $__ofl_lock (type 0) (result i32)
    (call $__lock
      (i32.const 5912))
    (i32.const 5920))
  (func $__ofl_unlock (type 4)
    (call $__unlock
      (i32.const 5912)))
  (func $__stdio_exit (type 4)
    (local i32)
    (if  ;; label = @1
      (local.tee 0
        (i32.load
          (call $__ofl_lock)))
      (then
        (loop  ;; label = @2
          (call $close_file
            (local.get 0))
          (br_if 0 (;@2;)
            (local.tee 0
              (i32.load offset=56
                (local.get 0)))))))
    (call $close_file
      (i32.load
        (i32.const 5924)))
    (call $close_file
      (i32.load
        (i32.const 4072))))
  (func $close_file (type 1) (param i32)
    (local i32 i32)
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.eqz
          (local.get 0)))
      (if  ;; label = @2
        (i32.ge_s
          (i32.load offset=76
            (local.get 0))
          (i32.const 0))
        (then
          (drop
            (call $__lockfile
              (local.get 0)))))
      (if  ;; label = @2
        (i32.gt_u
          (i32.load offset=20
            (local.get 0))
          (i32.load offset=28
            (local.get 0)))
        (then
          (drop
            (call_indirect (type 3)
              (local.get 0)
              (i32.const 0)
              (i32.const 0)
              (i32.load offset=36
                (local.get 0))))))
      (br_if 0 (;@1;)
        (i32.ge_u
          (local.tee 1
            (i32.load offset=4
              (local.get 0)))
          (local.tee 2
            (i32.load offset=8
              (local.get 0)))))
      (drop
        (call_indirect (type 12)
          (local.get 0)
          (i64.extend_i32_s
            (i32.sub
              (local.get 1)
              (local.get 2)))
          (i32.const 1)
          (i32.load offset=40
            (local.get 0))))))
  (func $__towrite (type 2) (param i32) (result i32)
    (local i32)
    (i32.store8 offset=74
      (local.get 0)
      (i32.or
        (i32.sub
          (local.tee 1
            (i32.load8_u offset=74
              (local.get 0)))
          (i32.const 1))
        (local.get 1)))
    (if  ;; label = @1
      (i32.and
        (local.tee 1
          (i32.load
            (local.get 0)))
        (i32.const 8))
      (then
        (i32.store
          (local.get 0)
          (i32.or
            (local.get 1)
            (i32.const 32)))
        (return
          (i32.const -1))))
    (i64.store offset=4 align=4
      (local.get 0)
      (i64.const 0))
    (i32.store offset=28
      (local.get 0)
      (local.tee 1
        (i32.load offset=44
          (local.get 0))))
    (i32.store offset=20
      (local.get 0)
      (local.get 1))
    (i32.store offset=16
      (local.get 0)
      (i32.add
        (local.get 1)
        (i32.load offset=48
          (local.get 0))))
    (i32.const 0))
  (func $__fwritex (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    (block  ;; label = @1
      (if  ;; label = @2
        (block (result i32)  ;; label = @3
          (if  ;; label = @4
            (i32.eqz
              (local.tee 3
                (i32.load offset=16
                  (local.get 2))))
            (then
              (br_if 2 (;@2;)
                (call $__towrite
                  (local.get 2)))
              (local.set 3
                (i32.load offset=16
                  (local.get 2)))))
          (i32.gt_u
            (local.get 1)
            (i32.sub
              (local.get 3)
              (local.tee 5
                (i32.load offset=20
                  (local.get 2))))))
        (then
          (return
            (call_indirect (type 3)
              (local.get 2)
              (local.get 0)
              (local.get 1)
              (i32.load offset=36
                (local.get 2))))))
      (block  ;; label = @2
        (if  ;; label = @3
          (i32.lt_s
            (i32.load8_s offset=75
              (local.get 2))
            (i32.const 0))
          (then
            (local.set 3
              (i32.const 0))
            (br 1 (;@2;))))
        (local.set 4
          (local.get 1))
        (loop  ;; label = @3
          (if  ;; label = @4
            (i32.eqz
              (local.tee 3
                (local.get 4)))
            (then
              (local.set 3
                (i32.const 0))
              (br 2 (;@2;))))
          (br_if 0 (;@3;)
            (i32.ne
              (i32.load8_u
                (i32.add
                  (local.get 0)
                  (local.tee 4
                    (i32.sub
                      (local.get 3)
                      (i32.const 1)))))
              (i32.const 10))))
        (br_if 1 (;@1;)
          (i32.lt_u
            (local.tee 4
              (call_indirect (type 3)
                (local.get 2)
                (local.get 0)
                (local.get 3)
                (i32.load offset=36
                  (local.get 2))))
            (local.get 3)))
        (local.set 0
          (i32.add
            (local.get 0)
            (local.get 3)))
        (local.set 1
          (i32.sub
            (local.get 1)
            (local.get 3)))
        (local.set 5
          (i32.load offset=20
            (local.get 2))))
      (drop
        (call $memcpy
          (local.get 5)
          (local.get 0)
          (local.get 1)))
      (i32.store offset=20
        (local.get 2)
        (i32.add
          (i32.load offset=20
            (local.get 2))
          (local.get 1)))
      (local.set 4
        (i32.add
          (local.get 1)
          (local.get 3))))
    (local.get 4))
  (func $__vfprintf_internal (type 16) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    (global.set $__stack_pointer
      (local.tee 5
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 208))))
    (i32.store offset=204
      (local.get 5)
      (local.get 2))
    (local.set 2
      (i32.const 0))
    (drop
      (call $memset
        (i32.add
          (local.get 5)
          (i32.const 160))
        (i32.const 0)
        (i32.const 40)))
    (i32.store offset=200
      (local.get 5)
      (i32.load offset=204
        (local.get 5)))
    (block  ;; label = @1
      (if  ;; label = @2
        (i32.lt_s
          (call $printf_core
            (i32.const 0)
            (local.get 1)
            (i32.add
              (local.get 5)
              (i32.const 200))
            (i32.add
              (local.get 5)
              (i32.const 80))
            (i32.add
              (local.get 5)
              (i32.const 160))
            (local.get 3)
            (local.get 4))
          (i32.const 0))
        (then
          (local.set 1
            (i32.const -1))
          (br 1 (;@1;))))
      (if  ;; label = @2
        (i32.ge_s
          (i32.load offset=76
            (local.get 0))
          (i32.const 0))
        (then
          (local.set 2
            (call $__lockfile
              (local.get 0)))))
      (local.set 6
        (i32.load
          (local.get 0)))
      (if  ;; label = @2
        (i32.le_s
          (i32.load8_s offset=74
            (local.get 0))
          (i32.const 0))
        (then
          (i32.store
            (local.get 0)
            (i32.and
              (local.get 6)
              (i32.const -33)))))
      (local.set 6
        (i32.and
          (local.get 6)
          (i32.const 32)))
      (local.set 1
        (block (result i32)  ;; label = @2
          (if  ;; label = @3
            (i32.load offset=48
              (local.get 0))
            (then
              (br 1 (;@2;)
                (call $printf_core
                  (local.get 0)
                  (local.get 1)
                  (i32.add
                    (local.get 5)
                    (i32.const 200))
                  (i32.add
                    (local.get 5)
                    (i32.const 80))
                  (i32.add
                    (local.get 5)
                    (i32.const 160))
                  (local.get 3)
                  (local.get 4)))))
          (i32.store offset=48
            (local.get 0)
            (i32.const 80))
          (i32.store offset=16
            (local.get 0)
            (i32.add
              (local.get 5)
              (i32.const 80)))
          (i32.store offset=28
            (local.get 0)
            (local.get 5))
          (i32.store offset=20
            (local.get 0)
            (local.get 5))
          (local.set 7
            (i32.load offset=44
              (local.get 0)))
          (i32.store offset=44
            (local.get 0)
            (local.get 5))
          (drop
            (br_if 0 (;@2;)
              (local.tee 1
                (call $printf_core
                  (local.get 0)
                  (local.get 1)
                  (i32.add
                    (local.get 5)
                    (i32.const 200))
                  (i32.add
                    (local.get 5)
                    (i32.const 80))
                  (i32.add
                    (local.get 5)
                    (i32.const 160))
                  (local.get 3)
                  (local.get 4)))
              (i32.eqz
                (local.get 7))))
          (drop
            (call_indirect (type 3)
              (local.get 0)
              (i32.const 0)
              (i32.const 0)
              (i32.load offset=36
                (local.get 0))))
          (i32.store offset=48
            (local.get 0)
            (i32.const 0))
          (i32.store offset=44
            (local.get 0)
            (local.get 7))
          (i32.store offset=28
            (local.get 0)
            (i32.const 0))
          (i32.store offset=16
            (local.get 0)
            (i32.const 0))
          (local.set 3
            (i32.load offset=20
              (local.get 0)))
          (i32.store offset=20
            (local.get 0)
            (i32.const 0))
          (select
            (local.get 1)
            (i32.const -1)
            (local.get 3))))
      (i32.store
        (local.get 0)
        (i32.or
          (local.tee 3
            (i32.load
              (local.get 0)))
          (local.get 6)))
      (local.set 1
        (select
          (i32.const -1)
          (local.get 1)
          (i32.and
            (local.get 3)
            (i32.const 32))))
      (br_if 0 (;@1;)
        (i32.eqz
          (local.get 2)))
      (call $__unlockfile
        (local.get 0)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 5)
        (i32.const 208)))
    (local.get 1))
  (func $printf_core (type 17) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32)
    (global.set $__stack_pointer
      (local.tee 7
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 80))))
    (i32.store offset=76
      (local.get 7)
      (local.get 1))
    (local.set 22
      (i32.add
        (local.get 7)
        (i32.const 55)))
    (local.set 18
      (i32.add
        (local.get 7)
        (i32.const 56)))
    (local.set 1
      (i32.const 0))
    (block  ;; label = @1
      (loop  ;; label = @2
        (block  ;; label = @3
          (br_if 0 (;@3;)
            (i32.lt_s
              (local.get 16)
              (i32.const 0)))
          (if  ;; label = @4
            (i32.lt_s
              (i32.sub
                (i32.const 2147483647)
                (local.get 16))
              (local.get 1))
            (then
              (i32.store
                (call $__errno_location)
                (i32.const 61))
              (local.set 16
                (i32.const -1))
              (br 1 (;@3;))))
          (local.set 16
            (i32.add
              (local.get 1)
              (local.get 16))))
        (local.set 1
          (local.tee 11
            (i32.load offset=76
              (local.get 7))))
        (block  ;; label = @3
          (block  ;; label = @4
            (block  ;; label = @5
              (block  ;; label = @6
                (if  ;; label = @7
                  (local.tee 8
                    (i32.load8_u
                      (local.get 11)))
                  (then
                    (loop  ;; label = @8
                      (block  ;; label = @9
                        (block  ;; label = @10
                          (if  ;; label = @11
                            (i32.eqz
                              (local.tee 8
                                (i32.and
                                  (local.get 8)
                                  (i32.const 255))))
                            (then
                              (local.set 8
                                (local.get 1))
                              (br 1 (;@10;))))
                          (br_if 1 (;@9;)
                            (i32.ne
                              (local.get 8)
                              (i32.const 37)))
                          (local.set 8
                            (local.get 1))
                          (loop  ;; label = @11
                            (br_if 1 (;@10;)
                              (i32.ne
                                (i32.load8_u offset=1
                                  (local.get 1))
                                (i32.const 37)))
                            (i32.store offset=76
                              (local.get 7)
                              (local.tee 9
                                (i32.add
                                  (local.get 1)
                                  (i32.const 2))))
                            (local.set 8
                              (i32.add
                                (local.get 8)
                                (i32.const 1)))
                            (local.set 12
                              (i32.load8_u offset=2
                                (local.get 1)))
                            (local.set 1
                              (local.get 9))
                            (br_if 0 (;@11;)
                              (i32.eq
                                (local.get 12)
                                (i32.const 37)))))
                        (local.set 1
                          (i32.sub
                            (local.get 8)
                            (local.get 11)))
                        (if  ;; label = @10
                          (local.get 0)
                          (then
                            (call $out
                              (local.get 0)
                              (local.get 11)
                              (local.get 1))))
                        (br_if 7 (;@2;)
                          (local.get 1))
                        (local.set 17
                          (i32.const -1))
                        (local.set 8
                          (i32.const 1))
                        (local.set 9
                          (call $isdigit
                            (i32.load8_s offset=1
                              (i32.load offset=76
                                (local.get 7)))))
                        (local.set 1
                          (i32.load offset=76
                            (local.get 7)))
                        (block  ;; label = @10
                          (br_if 0 (;@10;)
                            (i32.eqz
                              (local.get 9)))
                          (br_if 0 (;@10;)
                            (i32.ne
                              (i32.load8_u offset=2
                                (local.get 1))
                              (i32.const 36)))
                          (local.set 17
                            (i32.sub
                              (i32.load8_s offset=1
                                (local.get 1))
                              (i32.const 48)))
                          (local.set 19
                            (i32.const 1))
                          (local.set 8
                            (i32.const 3)))
                        (i32.store offset=76
                          (local.get 7)
                          (local.tee 1
                            (i32.add
                              (local.get 1)
                              (local.get 8))))
                        (local.set 14
                          (i32.const 0))
                        (block  ;; label = @10
                          (if  ;; label = @11
                            (i32.gt_u
                              (local.tee 9
                                (i32.sub
                                  (local.tee 12
                                    (i32.load8_s
                                      (local.get 1)))
                                  (i32.const 32)))
                              (i32.const 31))
                            (then
                              (local.set 8
                                (local.get 1))
                              (br 1 (;@10;))))
                          (local.set 8
                            (local.get 1))
                          (br_if 0 (;@10;)
                            (i32.eqz
                              (i32.and
                                (local.tee 9
                                  (i32.shl
                                    (i32.const 1)
                                    (local.get 9)))
                                (i32.const 75913))))
                          (loop  ;; label = @11
                            (i32.store offset=76
                              (local.get 7)
                              (local.tee 8
                                (i32.add
                                  (local.get 1)
                                  (i32.const 1))))
                            (local.set 14
                              (i32.or
                                (local.get 9)
                                (local.get 14)))
                            (br_if 1 (;@10;)
                              (i32.ge_u
                                (local.tee 9
                                  (i32.sub
                                    (local.tee 12
                                      (i32.load8_s offset=1
                                        (local.get 1)))
                                    (i32.const 32)))
                                (i32.const 32)))
                            (local.set 1
                              (local.get 8))
                            (br_if 0 (;@11;)
                              (i32.and
                                (local.tee 9
                                  (i32.shl
                                    (i32.const 1)
                                    (local.get 9)))
                                (i32.const 75913)))))
                        (block  ;; label = @10
                          (if  ;; label = @11
                            (i32.eq
                              (local.get 12)
                              (i32.const 42))
                            (then
                              (i32.store offset=76
                                (local.get 7)
                                (local.tee 1
                                  (block (result i32)  ;; label = @12
                                    (block  ;; label = @13
                                      (br_if 0 (;@13;)
                                        (i32.eqz
                                          (call $isdigit
                                            (i32.load8_s offset=1
                                              (local.get 8)))))
                                      (br_if 0 (;@13;)
                                        (i32.ne
                                          (i32.load8_u offset=2
                                            (local.tee 8
                                              (i32.load offset=76
                                                (local.get 7))))
                                          (i32.const 36)))
                                      (i32.store
                                        (i32.sub
                                          (i32.add
                                            (i32.shl
                                              (i32.load8_s offset=1
                                                (local.get 8))
                                              (i32.const 2))
                                            (local.get 4))
                                          (i32.const 192))
                                        (i32.const 10))
                                      (local.set 15
                                        (i32.load
                                          (i32.sub
                                            (i32.add
                                              (i32.shl
                                                (i32.load8_s offset=1
                                                  (local.get 8))
                                                (i32.const 3))
                                              (local.get 3))
                                            (i32.const 384))))
                                      (local.set 19
                                        (i32.const 1))
                                      (br 1 (;@12;)
                                        (i32.add
                                          (local.get 8)
                                          (i32.const 3))))
                                    (br_if 6 (;@6;)
                                      (local.get 19))
                                    (local.set 19
                                      (i32.const 0))
                                    (local.set 15
                                      (i32.const 0))
                                    (if  ;; label = @13
                                      (local.get 0)
                                      (then
                                        (i32.store
                                          (local.get 2)
                                          (i32.add
                                            (local.tee 1
                                              (i32.load
                                                (local.get 2)))
                                            (i32.const 4)))
                                        (local.set 15
                                          (i32.load
                                            (local.get 1)))))
                                    (i32.add
                                      (i32.load offset=76
                                        (local.get 7))
                                      (i32.const 1)))))
                              (br_if 1 (;@10;)
                                (i32.gt_s
                                  (local.get 15)
                                  (i32.const -1)))
                              (local.set 15
                                (i32.sub
                                  (i32.const 0)
                                  (local.get 15)))
                              (local.set 14
                                (i32.or
                                  (local.get 14)
                                  (i32.const 8192)))
                              (br 1 (;@10;))))
                          (br_if 4 (;@6;)
                            (i32.lt_s
                              (local.tee 15
                                (call $getint
                                  (i32.add
                                    (local.get 7)
                                    (i32.const 76))))
                              (i32.const 0)))
                          (local.set 1
                            (i32.load offset=76
                              (local.get 7))))
                        (local.set 10
                          (i32.const -1))
                        (block  ;; label = @10
                          (br_if 0 (;@10;)
                            (i32.ne
                              (i32.load8_u
                                (local.get 1))
                              (i32.const 46)))
                          (if  ;; label = @11
                            (i32.eq
                              (i32.load8_u offset=1
                                (local.get 1))
                              (i32.const 42))
                            (then
                              (block  ;; label = @12
                                (br_if 0 (;@12;)
                                  (i32.eqz
                                    (call $isdigit
                                      (i32.load8_s offset=2
                                        (local.get 1)))))
                                (br_if 0 (;@12;)
                                  (i32.ne
                                    (i32.load8_u offset=3
                                      (local.tee 1
                                        (i32.load offset=76
                                          (local.get 7))))
                                    (i32.const 36)))
                                (i32.store
                                  (i32.sub
                                    (i32.add
                                      (i32.shl
                                        (i32.load8_s offset=2
                                          (local.get 1))
                                        (i32.const 2))
                                      (local.get 4))
                                    (i32.const 192))
                                  (i32.const 10))
                                (local.set 10
                                  (i32.load
                                    (i32.sub
                                      (i32.add
                                        (i32.shl
                                          (i32.load8_s offset=2
                                            (local.get 1))
                                          (i32.const 3))
                                        (local.get 3))
                                      (i32.const 384))))
                                (i32.store offset=76
                                  (local.get 7)
                                  (local.tee 1
                                    (i32.add
                                      (local.get 1)
                                      (i32.const 4))))
                                (br 2 (;@10;)))
                              (br_if 5 (;@6;)
                                (local.get 19))
                              (local.set 10
                                (if (result i32)  ;; label = @12
                                  (local.get 0)
                                  (then
                                    (i32.store
                                      (local.get 2)
                                      (i32.add
                                        (local.tee 1
                                          (i32.load
                                            (local.get 2)))
                                        (i32.const 4)))
                                    (i32.load
                                      (local.get 1)))
                                  (else
                                    (i32.const 0))))
                              (i32.store offset=76
                                (local.get 7)
                                (local.tee 1
                                  (i32.add
                                    (i32.load offset=76
                                      (local.get 7))
                                    (i32.const 2))))
                              (br 1 (;@10;))))
                          (i32.store offset=76
                            (local.get 7)
                            (i32.add
                              (local.get 1)
                              (i32.const 1)))
                          (local.set 10
                            (call $getint
                              (i32.add
                                (local.get 7)
                                (i32.const 76))))
                          (local.set 1
                            (i32.load offset=76
                              (local.get 7))))
                        (local.set 8
                          (i32.const 0))
                        (loop  ;; label = @10
                          (local.set 9
                            (local.get 8))
                          (local.set 13
                            (i32.const -1))
                          (br_if 9 (;@1;)
                            (i32.gt_u
                              (i32.sub
                                (i32.load8_s
                                  (local.get 1))
                                (i32.const 65))
                              (i32.const 57)))
                          (i32.store offset=76
                            (local.get 7)
                            (local.tee 12
                              (i32.add
                                (local.get 1)
                                (i32.const 1))))
                          (local.set 8
                            (i32.load8_s
                              (local.get 1)))
                          (local.set 1
                            (local.get 12))
                          (br_if 0 (;@10;)
                            (i32.lt_u
                              (i32.sub
                                (local.tee 8
                                  (i32.load8_u
                                    (i32.add
                                      (i32.add
                                        (local.get 8)
                                        (i32.mul
                                          (local.get 9)
                                          (i32.const 58)))
                                      (i32.const 3375))))
                                (i32.const 1))
                              (i32.const 8))))
                        (block  ;; label = @10
                          (block  ;; label = @11
                            (if  ;; label = @12
                              (i32.ne
                                (local.get 8)
                                (i32.const 19))
                              (then
                                (br_if 11 (;@1;)
                                  (i32.eqz
                                    (local.get 8)))
                                (if  ;; label = @13
                                  (i32.ge_s
                                    (local.get 17)
                                    (i32.const 0))
                                  (then
                                    (i32.store
                                      (i32.add
                                        (local.get 4)
                                        (i32.shl
                                          (local.get 17)
                                          (i32.const 2)))
                                      (local.get 8))
                                    (i64.store offset=64
                                      (local.get 7)
                                      (i64.load
                                        (i32.add
                                          (local.get 3)
                                          (i32.shl
                                            (local.get 17)
                                            (i32.const 3)))))
                                    (br 2 (;@11;))))
                                (br_if 9 (;@3;)
                                  (i32.eqz
                                    (local.get 0)))
                                (call $pop_arg
                                  (i32.sub
                                    (local.get 7)
                                    (i32.const -64))
                                  (local.get 8)
                                  (local.get 2)
                                  (local.get 6))
                                (local.set 12
                                  (i32.load offset=76
                                    (local.get 7)))
                                (br 2 (;@10;))))
                            (br_if 10 (;@1;)
                              (i32.gt_s
                                (local.get 17)
                                (i32.const -1))))
                          (local.set 1
                            (i32.const 0))
                          (br_if 8 (;@2;)
                            (i32.eqz
                              (local.get 0))))
                        (local.set 8
                          (select
                            (local.tee 21
                              (i32.and
                                (local.get 14)
                                (i32.const -65537)))
                            (local.get 14)
                            (i32.and
                              (local.get 14)
                              (i32.const 8192))))
                        (local.set 13
                          (i32.const 0))
                        (local.set 17
                          (i32.const 1024))
                        (local.set 14
                          (local.get 18))
                        (block  ;; label = @10
                          (block  ;; label = @11
                            (block  ;; label = @12
                              (local.set 9
                                (block (result i32)  ;; label = @13
                                  (block  ;; label = @14
                                    (block  ;; label = @15
                                      (block  ;; label = @16
                                        (block  ;; label = @17
                                          (local.set 17
                                            (block (result i32)  ;; label = @18
                                              (block  ;; label = @19
                                                (block  ;; label = @20
                                                  (block  ;; label = @21
                                                    (block  ;; label = @22
                                                      (block  ;; label = @23
                                                        (block  ;; label = @24
                                                          (block  ;; label = @25
                                                            (br_table 4 (;@21;) 21 (;@4;) 21 (;@4;) 21 (;@4;) 21 (;@4;) 21 (;@4;) 21 (;@4;) 21 (;@4;) 21 (;@4;) 14 (;@11;) 21 (;@4;) 15 (;@10;) 6 (;@19;) 14 (;@11;) 14 (;@11;) 14 (;@11;) 21 (;@4;) 6 (;@19;) 21 (;@4;) 21 (;@4;) 21 (;@4;) 21 (;@4;) 2 (;@23;) 5 (;@20;) 3 (;@22;) 21 (;@4;) 21 (;@4;) 9 (;@16;) 21 (;@4;) 1 (;@24;) 21 (;@4;) 21 (;@4;) 4 (;@21;) 0 (;@25;)
                                                              (i32.sub
                                                                (local.tee 1
                                                                  (select
                                                                    (select
                                                                      (i32.and
                                                                        (local.tee 1
                                                                          (i32.load8_s
                                                                            (i32.sub
                                                                              (local.get 12)
                                                                              (i32.const 1))))
                                                                        (i32.const -33))
                                                                      (local.get 1)
                                                                      (i32.eq
                                                                        (i32.and
                                                                          (local.get 1)
                                                                          (i32.const 15))
                                                                        (i32.const 3)))
                                                                    (local.get 1)
                                                                    (local.get 9)))
                                                                (i32.const 88))))
                                                          (block  ;; label = @25
                                                            (br_table 14 (;@11;) 21 (;@4;) 11 (;@14;) 21 (;@4;) 14 (;@11;) 14 (;@11;) 14 (;@11;) 0 (;@25;)
                                                              (i32.sub
                                                                (local.get 1)
                                                                (i32.const 65))))
                                                          (br_if 9 (;@15;)
                                                            (i32.eq
                                                              (local.get 1)
                                                              (i32.const 83)))
                                                          (br 19 (;@5;)))
                                                        (local.set 20
                                                          (i64.load offset=64
                                                            (local.get 7)))
                                                        (br 5 (;@18;)
                                                          (i32.const 1024)))
                                                      (local.set 1
                                                        (i32.const 0))
                                                      (block  ;; label = @23
                                                        (block  ;; label = @24
                                                          (block  ;; label = @25
                                                            (block  ;; label = @26
                                                              (block  ;; label = @27
                                                                (block  ;; label = @28
                                                                  (block  ;; label = @29
                                                                    (br_table 0 (;@29;) 1 (;@28;) 2 (;@27;) 3 (;@26;) 4 (;@25;) 27 (;@2;) 5 (;@24;) 6 (;@23;) 27 (;@2;)
                                                                      (i32.and
                                                                        (local.get 9)
                                                                        (i32.const 255))))
                                                                  (i32.store
                                                                    (i32.load offset=64
                                                                      (local.get 7))
                                                                    (local.get 16))
                                                                  (br 26 (;@2;)))
                                                                (i32.store
                                                                  (i32.load offset=64
                                                                    (local.get 7))
                                                                  (local.get 16))
                                                                (br 25 (;@2;)))
                                                              (i64.store
                                                                (i32.load offset=64
                                                                  (local.get 7))
                                                                (i64.extend_i32_s
                                                                  (local.get 16)))
                                                              (br 24 (;@2;)))
                                                            (i32.store16
                                                              (i32.load offset=64
                                                                (local.get 7))
                                                              (local.get 16))
                                                            (br 23 (;@2;)))
                                                          (i32.store8
                                                            (i32.load offset=64
                                                              (local.get 7))
                                                            (local.get 16))
                                                          (br 22 (;@2;)))
                                                        (i32.store
                                                          (i32.load offset=64
                                                            (local.get 7))
                                                          (local.get 16))
                                                        (br 21 (;@2;)))
                                                      (i64.store
                                                        (i32.load offset=64
                                                          (local.get 7))
                                                        (i64.extend_i32_s
                                                          (local.get 16)))
                                                      (br 20 (;@2;)))
                                                    (local.set 10
                                                      (select
                                                        (local.get 10)
                                                        (i32.const 8)
                                                        (i32.gt_u
                                                          (local.get 10)
                                                          (i32.const 8))))
                                                    (local.set 8
                                                      (i32.or
                                                        (local.get 8)
                                                        (i32.const 8)))
                                                    (local.set 1
                                                      (i32.const 120)))
                                                  (local.set 11
                                                    (call $fmt_x
                                                      (i64.load offset=64
                                                        (local.get 7))
                                                      (local.get 18)
                                                      (i32.and
                                                        (local.get 1)
                                                        (i32.const 32))))
                                                  (br_if 3 (;@17;)
                                                    (i64.eqz
                                                      (i64.load offset=64
                                                        (local.get 7))))
                                                  (br_if 3 (;@17;)
                                                    (i32.eqz
                                                      (i32.and
                                                        (local.get 8)
                                                        (i32.const 8))))
                                                  (local.set 17
                                                    (i32.add
                                                      (i32.shr_u
                                                        (local.get 1)
                                                        (i32.const 4))
                                                      (i32.const 1024)))
                                                  (local.set 13
                                                    (i32.const 2))
                                                  (br 3 (;@17;)))
                                                (local.set 11
                                                  (call $fmt_o
                                                    (i64.load offset=64
                                                      (local.get 7))
                                                    (local.get 18)))
                                                (br_if 2 (;@17;)
                                                  (i32.eqz
                                                    (i32.and
                                                      (local.get 8)
                                                      (i32.const 8))))
                                                (local.set 10
                                                  (select
                                                    (local.get 10)
                                                    (i32.add
                                                      (local.tee 1
                                                        (i32.sub
                                                          (local.get 18)
                                                          (local.get 11)))
                                                      (i32.const 1))
                                                    (i32.lt_s
                                                      (local.get 1)
                                                      (local.get 10))))
                                                (br 2 (;@17;)))
                                              (if  ;; label = @19
                                                (i64.le_s
                                                  (local.tee 20
                                                    (i64.load offset=64
                                                      (local.get 7)))
                                                  (i64.const -1))
                                                (then
                                                  (i64.store offset=64
                                                    (local.get 7)
                                                    (local.tee 20
                                                      (i64.sub
                                                        (i64.const 0)
                                                        (local.get 20))))
                                                  (local.set 13
                                                    (i32.const 1))
                                                  (br 1 (;@18;)
                                                    (i32.const 1024))))
                                              (if  ;; label = @19
                                                (i32.and
                                                  (local.get 8)
                                                  (i32.const 2048))
                                                (then
                                                  (local.set 13
                                                    (i32.const 1))
                                                  (br 1 (;@18;)
                                                    (i32.const 1025))))
                                              (select
                                                (i32.const 1026)
                                                (i32.const 1024)
                                                (local.tee 13
                                                  (i32.and
                                                    (local.get 8)
                                                    (i32.const 1))))))
                                          (local.set 11
                                            (call $fmt_u
                                              (local.get 20)
                                              (local.get 18))))
                                        (local.set 8
                                          (select
                                            (i32.and
                                              (local.get 8)
                                              (i32.const -65537))
                                            (local.get 8)
                                            (i32.gt_s
                                              (local.get 10)
                                              (i32.const -1))))
                                        (block  ;; label = @17
                                          (br_if 0 (;@17;)
                                            (i64.ne
                                              (local.tee 20
                                                (i64.load offset=64
                                                  (local.get 7)))
                                              (i64.const 0)))
                                          (br_if 0 (;@17;)
                                            (local.get 10))
                                          (local.set 10
                                            (i32.const 0))
                                          (local.set 11
                                            (local.get 18))
                                          (br 12 (;@5;)))
                                        (local.set 10
                                          (select
                                            (local.get 10)
                                            (local.tee 1
                                              (i32.add
                                                (i64.eqz
                                                  (local.get 20))
                                                (i32.sub
                                                  (local.get 18)
                                                  (local.get 11))))
                                            (i32.lt_s
                                              (local.get 1)
                                              (local.get 10))))
                                        (br 11 (;@5;)))
                                      (local.set 14
                                        (select
                                          (local.tee 1
                                            (call $memchr
                                              (local.tee 11
                                                (select
                                                  (local.tee 1
                                                    (i32.load offset=64
                                                      (local.get 7)))
                                                  (i32.const 1809)
                                                  (local.get 1)))
                                              (i32.const 0)
                                              (local.get 10)))
                                          (i32.add
                                            (local.get 10)
                                            (local.get 11))
                                          (local.get 1)))
                                      (local.set 8
                                        (local.get 21))
                                      (local.set 10
                                        (select
                                          (i32.sub
                                            (local.get 1)
                                            (local.get 11))
                                          (local.get 10)
                                          (local.get 1)))
                                      (br 11 (;@4;)))
                                    (if  ;; label = @15
                                      (local.get 10)
                                      (then
                                        (br 2 (;@13;)
                                          (i32.load offset=64
                                            (local.get 7)))))
                                    (local.set 1
                                      (i32.const 0))
                                    (call $pad
                                      (local.get 0)
                                      (i32.const 32)
                                      (local.get 15)
                                      (i32.const 0)
                                      (local.get 8))
                                    (br 2 (;@12;)))
                                  (i32.store offset=12
                                    (local.get 7)
                                    (i32.const 0))
                                  (i64.store32 offset=8
                                    (local.get 7)
                                    (i64.load offset=64
                                      (local.get 7)))
                                  (i32.store offset=64
                                    (local.get 7)
                                    (i32.add
                                      (local.get 7)
                                      (i32.const 8)))
                                  (local.set 10
                                    (i32.const -1))
                                  (i32.add
                                    (local.get 7)
                                    (i32.const 8))))
                              (local.set 1
                                (i32.const 0))
                              (block  ;; label = @13
                                (loop  ;; label = @14
                                  (br_if 1 (;@13;)
                                    (i32.eqz
                                      (local.tee 12
                                        (i32.load
                                          (local.get 9)))))
                                  (block  ;; label = @15
                                    (br_if 0 (;@15;)
                                      (local.tee 11
                                        (i32.lt_s
                                          (local.tee 12
                                            (call $wctomb
                                              (i32.add
                                                (local.get 7)
                                                (i32.const 4))
                                              (local.get 12)))
                                          (i32.const 0))))
                                    (br_if 0 (;@15;)
                                      (i32.gt_u
                                        (local.get 12)
                                        (i32.sub
                                          (local.get 10)
                                          (local.get 1))))
                                    (local.set 9
                                      (i32.add
                                        (local.get 9)
                                        (i32.const 4)))
                                    (br_if 1 (;@14;)
                                      (i32.gt_u
                                        (local.get 10)
                                        (local.tee 1
                                          (i32.add
                                            (local.get 1)
                                            (local.get 12)))))
                                    (br 2 (;@13;))))
                                (local.set 13
                                  (i32.const -1))
                                (br_if 12 (;@1;)
                                  (local.get 11)))
                              (call $pad
                                (local.get 0)
                                (i32.const 32)
                                (local.get 15)
                                (local.get 1)
                                (local.get 8))
                              (if  ;; label = @13
                                (i32.eqz
                                  (local.get 1))
                                (then
                                  (local.set 1
                                    (i32.const 0))
                                  (br 1 (;@12;))))
                              (local.set 9
                                (i32.const 0))
                              (local.set 12
                                (i32.load offset=64
                                  (local.get 7)))
                              (loop  ;; label = @13
                                (br_if 1 (;@12;)
                                  (i32.eqz
                                    (local.tee 11
                                      (i32.load
                                        (local.get 12)))))
                                (br_if 1 (;@12;)
                                  (i32.gt_s
                                    (local.tee 9
                                      (i32.add
                                        (local.tee 11
                                          (call $wctomb
                                            (i32.add
                                              (local.get 7)
                                              (i32.const 4))
                                            (local.get 11)))
                                        (local.get 9)))
                                    (local.get 1)))
                                (call $out
                                  (local.get 0)
                                  (i32.add
                                    (local.get 7)
                                    (i32.const 4))
                                  (local.get 11))
                                (local.set 12
                                  (i32.add
                                    (local.get 12)
                                    (i32.const 4)))
                                (br_if 0 (;@13;)
                                  (i32.gt_u
                                    (local.get 1)
                                    (local.get 9)))))
                            (call $pad
                              (local.get 0)
                              (i32.const 32)
                              (local.get 15)
                              (local.get 1)
                              (i32.xor
                                (local.get 8)
                                (i32.const 8192)))
                            (local.set 1
                              (select
                                (local.get 15)
                                (local.get 1)
                                (i32.lt_s
                                  (local.get 1)
                                  (local.get 15))))
                            (br 9 (;@2;)))
                          (local.set 1
                            (call_indirect (type 18)
                              (local.get 0)
                              (f64.load offset=64
                                (local.get 7))
                              (local.get 15)
                              (local.get 10)
                              (local.get 8)
                              (local.get 1)
                              (local.get 5)))
                          (br 8 (;@2;)))
                        (i64.store8 offset=55
                          (local.get 7)
                          (i64.load offset=64
                            (local.get 7)))
                        (local.set 10
                          (i32.const 1))
                        (local.set 11
                          (local.get 22))
                        (local.set 8
                          (local.get 21))
                        (br 5 (;@4;)))
                      (i32.store offset=76
                        (local.get 7)
                        (local.tee 9
                          (i32.add
                            (local.get 1)
                            (i32.const 1))))
                      (local.set 8
                        (i32.load8_u offset=1
                          (local.get 1)))
                      (local.set 1
                        (local.get 9))
                      (br 0 (;@8;)))
                    (unreachable)))
                (local.set 13
                  (local.get 16))
                (br_if 5 (;@1;)
                  (local.get 0))
                (br_if 3 (;@3;)
                  (i32.eqz
                    (local.get 19)))
                (local.set 1
                  (i32.const 1))
                (loop  ;; label = @7
                  (if  ;; label = @8
                    (local.tee 8
                      (i32.load
                        (i32.add
                          (local.get 4)
                          (i32.shl
                            (local.get 1)
                            (i32.const 2)))))
                    (then
                      (call $pop_arg
                        (i32.add
                          (local.get 3)
                          (i32.shl
                            (local.get 1)
                            (i32.const 3)))
                        (local.get 8)
                        (local.get 2)
                        (local.get 6))
                      (local.set 13
                        (i32.const 1))
                      (br_if 1 (;@7;)
                        (i32.ne
                          (local.tee 1
                            (i32.add
                              (local.get 1)
                              (i32.const 1)))
                          (i32.const 10)))
                      (br 7 (;@1;)))))
                (local.set 13
                  (i32.const 1))
                (br_if 5 (;@1;)
                  (i32.ge_u
                    (local.get 1)
                    (i32.const 10)))
                (loop  ;; label = @7
                  (br_if 1 (;@6;)
                    (i32.load
                      (i32.add
                        (local.get 4)
                        (i32.shl
                          (local.get 1)
                          (i32.const 2)))))
                  (br_if 0 (;@7;)
                    (i32.ne
                      (local.tee 1
                        (i32.add
                          (local.get 1)
                          (i32.const 1)))
                      (i32.const 10))))
                (br 5 (;@1;)))
              (local.set 13
                (i32.const -1))
              (br 4 (;@1;))))
          (call $pad
            (local.get 0)
            (i32.const 32)
            (local.tee 1
              (select
                (local.tee 9
                  (i32.add
                    (local.get 13)
                    (local.tee 14
                      (select
                        (local.tee 12
                          (i32.sub
                            (local.get 14)
                            (local.get 11)))
                        (local.get 10)
                        (i32.lt_s
                          (local.get 10)
                          (local.get 12))))))
                (local.get 15)
                (i32.gt_s
                  (local.get 9)
                  (local.get 15))))
            (local.get 9)
            (local.get 8))
          (call $out
            (local.get 0)
            (local.get 17)
            (local.get 13))
          (call $pad
            (local.get 0)
            (i32.const 48)
            (local.get 1)
            (local.get 9)
            (i32.xor
              (local.get 8)
              (i32.const 65536)))
          (call $pad
            (local.get 0)
            (i32.const 48)
            (local.get 14)
            (local.get 12)
            (i32.const 0))
          (call $out
            (local.get 0)
            (local.get 11)
            (local.get 12))
          (call $pad
            (local.get 0)
            (i32.const 32)
            (local.get 1)
            (local.get 9)
            (i32.xor
              (local.get 8)
              (i32.const 8192)))
          (br 1 (;@2;))))
      (local.set 13
        (i32.const 0)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 7)
        (i32.const 80)))
    (local.get 13))
  (func $out (type 10) (param i32 i32 i32)
    (if  ;; label = @1
      (i32.eqz
        (i32.and
          (i32.load8_u
            (local.get 0))
          (i32.const 32)))
      (then
        (drop
          (call $__fwritex
            (local.get 1)
            (local.get 2)
            (local.get 0))))))
  (func $getint (type 2) (param i32) (result i32)
    (local i32 i32 i32)
    (if  ;; label = @1
      (call $isdigit
        (i32.load8_s
          (i32.load
            (local.get 0))))
      (then
        (loop  ;; label = @2
          (local.set 3
            (i32.load8_s
              (local.tee 2
                (i32.load
                  (local.get 0)))))
          (i32.store
            (local.get 0)
            (i32.add
              (local.get 2)
              (i32.const 1)))
          (local.set 1
            (i32.sub
              (i32.add
                (local.get 3)
                (i32.mul
                  (local.get 1)
                  (i32.const 10)))
              (i32.const 48)))
          (br_if 0 (;@2;)
            (call $isdigit
              (i32.load8_s offset=1
                (local.get 2)))))))
    (local.get 1))
  (func $pop_arg (type 6) (param i32 i32 i32 i32)
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.gt_u
          (local.get 1)
          (i32.const 20)))
      (block  ;; label = @2
        (block  ;; label = @3
          (block  ;; label = @4
            (block  ;; label = @5
              (block  ;; label = @6
                (block  ;; label = @7
                  (block  ;; label = @8
                    (block  ;; label = @9
                      (block  ;; label = @10
                        (block  ;; label = @11
                          (br_table 0 (;@11;) 1 (;@10;) 2 (;@9;) 3 (;@8;) 4 (;@7;) 5 (;@6;) 6 (;@5;) 7 (;@4;) 8 (;@3;) 9 (;@2;) 10 (;@1;)
                            (i32.sub
                              (local.get 1)
                              (i32.const 9))))
                        (i32.store
                          (local.get 2)
                          (i32.add
                            (local.tee 1
                              (i32.load
                                (local.get 2)))
                            (i32.const 4)))
                        (i32.store
                          (local.get 0)
                          (i32.load
                            (local.get 1)))
                        (return))
                      (i32.store
                        (local.get 2)
                        (i32.add
                          (local.tee 1
                            (i32.load
                              (local.get 2)))
                          (i32.const 4)))
                      (i64.store
                        (local.get 0)
                        (i64.load32_s
                          (local.get 1)))
                      (return))
                    (i32.store
                      (local.get 2)
                      (i32.add
                        (local.tee 1
                          (i32.load
                            (local.get 2)))
                        (i32.const 4)))
                    (i64.store
                      (local.get 0)
                      (i64.load32_u
                        (local.get 1)))
                    (return))
                  (i32.store
                    (local.get 2)
                    (i32.add
                      (local.tee 1
                        (i32.and
                          (i32.add
                            (i32.load
                              (local.get 2))
                            (i32.const 7))
                          (i32.const -8)))
                      (i32.const 8)))
                  (i64.store
                    (local.get 0)
                    (i64.load
                      (local.get 1)))
                  (return))
                (i32.store
                  (local.get 2)
                  (i32.add
                    (local.tee 1
                      (i32.load
                        (local.get 2)))
                    (i32.const 4)))
                (i64.store
                  (local.get 0)
                  (i64.load16_s
                    (local.get 1)))
                (return))
              (i32.store
                (local.get 2)
                (i32.add
                  (local.tee 1
                    (i32.load
                      (local.get 2)))
                  (i32.const 4)))
              (i64.store
                (local.get 0)
                (i64.load16_u
                  (local.get 1)))
              (return))
            (i32.store
              (local.get 2)
              (i32.add
                (local.tee 1
                  (i32.load
                    (local.get 2)))
                (i32.const 4)))
            (i64.store
              (local.get 0)
              (i64.load8_s
                (local.get 1)))
            (return))
          (i32.store
            (local.get 2)
            (i32.add
              (local.tee 1
                (i32.load
                  (local.get 2)))
              (i32.const 4)))
          (i64.store
            (local.get 0)
            (i64.load8_u
              (local.get 1)))
          (return))
        (i32.store
          (local.get 2)
          (i32.add
            (local.tee 1
              (i32.and
                (i32.add
                  (i32.load
                    (local.get 2))
                  (i32.const 7))
                (i32.const -8)))
            (i32.const 8)))
        (f64.store
          (local.get 0)
          (f64.load
            (local.get 1)))
        (return))
      (call_indirect (type 9)
        (local.get 0)
        (local.get 2)
        (local.get 3))))
  (func $fmt_x (type 19) (param i64 i32 i32) (result i32)
    (local i32)
    (if  ;; label = @1
      (i32.eqz
        (i64.eqz
          (local.get 0)))
      (then
        (loop  ;; label = @2
          (i32.store8
            (local.tee 1
              (i32.sub
                (local.get 1)
                (i32.const 1)))
            (i32.or
              (i32.load8_u
                (i32.add
                  (i32.and
                    (i32.wrap_i64
                      (local.get 0))
                    (i32.const 15))
                  (i32.const 3904)))
              (local.get 2)))
          (local.set 3
            (i64.gt_u
              (local.get 0)
              (i64.const 15)))
          (local.set 0
            (i64.shr_u
              (local.get 0)
              (i64.const 4)))
          (br_if 0 (;@2;)
            (local.get 3)))))
    (local.get 1))
  (func $fmt_o (type 14) (param i64 i32) (result i32)
    (local i32)
    (if  ;; label = @1
      (i32.eqz
        (i64.eqz
          (local.get 0)))
      (then
        (loop  ;; label = @2
          (i32.store8
            (local.tee 1
              (i32.sub
                (local.get 1)
                (i32.const 1)))
            (i32.or
              (i32.and
                (i32.wrap_i64
                  (local.get 0))
                (i32.const 7))
              (i32.const 48)))
          (local.set 2
            (i64.gt_u
              (local.get 0)
              (i64.const 7)))
          (local.set 0
            (i64.shr_u
              (local.get 0)
              (i64.const 3)))
          (br_if 0 (;@2;)
            (local.get 2)))))
    (local.get 1))
  (func $fmt_u (type 14) (param i64 i32) (result i32)
    (local i32 i64 i32 i32)
    (block  ;; label = @1
      (if  ;; label = @2
        (i64.lt_u
          (local.get 0)
          (i64.const 4294967296))
        (then
          (local.set 3
            (local.get 0))
          (br 1 (;@1;))))
      (loop  ;; label = @2
        (i32.store8
          (local.tee 1
            (i32.sub
              (local.get 1)
              (i32.const 1)))
          (i32.or
            (i32.wrap_i64
              (i64.sub
                (local.get 0)
                (i64.mul
                  (local.tee 3
                    (i64.div_u
                      (local.get 0)
                      (i64.const 10)))
                  (i64.const 10))))
            (i32.const 48)))
        (local.set 2
          (i64.gt_u
            (local.get 0)
            (i64.const 42949672959)))
        (local.set 0
          (local.get 3))
        (br_if 0 (;@2;)
          (local.get 2))))
    (if  ;; label = @1
      (local.tee 2
        (i32.wrap_i64
          (local.get 3)))
      (then
        (loop  ;; label = @2
          (i32.store8
            (local.tee 1
              (i32.sub
                (local.get 1)
                (i32.const 1)))
            (i32.or
              (i32.sub
                (local.get 2)
                (i32.mul
                  (local.tee 4
                    (i32.div_u
                      (local.get 2)
                      (i32.const 10)))
                  (i32.const 10)))
              (i32.const 48)))
          (local.set 5
            (i32.gt_u
              (local.get 2)
              (i32.const 9)))
          (local.set 2
            (local.get 4))
          (br_if 0 (;@2;)
            (local.get 5)))))
    (local.get 1))
  (func $pad (type 5) (param i32 i32 i32 i32 i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 5
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 256))))
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.and
          (local.get 4)
          (i32.const 73728)))
      (br_if 0 (;@1;)
        (i32.le_s
          (local.get 2)
          (local.get 3)))
      (drop
        (call $memset
          (local.get 5)
          (i32.and
            (local.get 1)
            (i32.const 255))
          (select
            (local.tee 2
              (i32.sub
                (local.get 2)
                (local.get 3)))
            (i32.const 256)
            (local.tee 3
              (i32.lt_u
                (local.get 2)
                (i32.const 256))))))
      (if  ;; label = @2
        (i32.eqz
          (local.get 3))
        (then
          (loop  ;; label = @3
            (call $out
              (local.get 0)
              (local.get 5)
              (i32.const 256))
            (br_if 0 (;@3;)
              (i32.gt_u
                (local.tee 2
                  (i32.sub
                    (local.get 2)
                    (i32.const 256)))
                (i32.const 255))))))
      (call $out
        (local.get 0)
        (local.get 5)
        (local.get 2)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 5)
        (i32.const 256))))
  (func $vfiprintf (type 3) (param i32 i32 i32) (result i32)
    (call $__vfprintf_internal
      (local.get 0)
      (local.get 1)
      (local.get 2)
      (i32.const 0)
      (i32.const 0)))
  (func $iprintf (type 11) (param i32 i32) (result i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 2
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=12
      (local.get 2)
      (local.get 1))
    (local.set 1
      (call $vfiprintf
        (i32.load
          (i32.const 3432))
        (local.get 0)
        (local.get 1)))
    (global.set $__stack_pointer
      (i32.add
        (local.get 2)
        (i32.const 16)))
    (local.get 1))
  (func $__lockfile (type 2) (param i32) (result i32)
    (i32.const 1))
  (func $__unlockfile (type 1) (param i32)
    (nop))
  (func $strlen (type 2) (param i32) (result i32)
    (local i32 i32 i32)
    (local.set 1
      (local.get 0))
    (block  ;; label = @1
      (if  ;; label = @2
        (i32.and
          (local.get 0)
          (i32.const 3))
        (then
          (loop  ;; label = @3
            (br_if 2 (;@1;)
              (i32.eqz
                (i32.load8_u
                  (local.get 1))))
            (br_if 0 (;@3;)
              (i32.and
                (local.tee 1
                  (i32.add
                    (local.get 1)
                    (i32.const 1)))
                (i32.const 3))))))
      (loop  ;; label = @2
        (local.set 1
          (i32.add
            (local.tee 2
              (local.get 1))
            (i32.const 4)))
        (br_if 0 (;@2;)
          (i32.eqz
            (i32.and
              (i32.and
                (i32.xor
                  (local.tee 3
                    (i32.load
                      (local.get 2)))
                  (i32.const -1))
                (i32.sub
                  (local.get 3)
                  (i32.const 16843009)))
              (i32.const -2139062144)))))
      (if  ;; label = @2
        (i32.eqz
          (i32.and
            (local.get 3)
            (i32.const 255)))
        (then
          (return
            (i32.sub
              (local.get 2)
              (local.get 0)))))
      (loop  ;; label = @2
        (local.set 3
          (i32.load8_u offset=1
            (local.get 2)))
        (local.set 2
          (local.tee 1
            (i32.add
              (local.get 2)
              (i32.const 1))))
        (br_if 0 (;@2;)
          (local.get 3))))
    (i32.sub
      (local.get 1)
      (local.get 0)))
  (func $stackSave (type 0) (result i32)
    (global.get $__stack_pointer))
  (func $stackRestore (type 1) (param i32)
    (global.set $__stack_pointer
      (local.get 0)))
  (func $stackAlloc (type 2) (param i32) (result i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.and
          (i32.sub
            (global.get $__stack_pointer)
            (local.get 0))
          (i32.const -16))))
    (local.get 1))
  (func $fflush (type 2) (param i32) (result i32)
    (local i32 i32)
    (block  ;; label = @1
      (if  ;; label = @2
        (local.get 0)
        (then
          (if  ;; label = @3
            (i32.le_s
              (i32.load offset=76
                (local.get 0))
              (i32.const -1))
            (then
              (return
                (call $__fflush_unlocked
                  (local.get 0)))))
          (local.set 2
            (call $__lockfile
              (local.get 0)))
          (local.set 1
            (call $__fflush_unlocked
              (local.get 0)))
          (br_if 1 (;@1;)
            (i32.eqz
              (local.get 2)))
          (call $__unlockfile
            (local.get 0))
          (return
            (local.get 1))))
      (if  ;; label = @2
        (i32.load
          (i32.const 4072))
        (then
          (local.set 1
            (call $fflush
              (i32.load
                (i32.const 4072))))))
      (if  ;; label = @2
        (local.tee 0
          (i32.load
            (call $__ofl_lock)))
        (then
          (loop  ;; label = @3
            (local.set 2
              (i32.const 0))
            (if  ;; label = @4
              (i32.ge_s
                (i32.load offset=76
                  (local.get 0))
                (i32.const 0))
              (then
                (local.set 2
                  (call $__lockfile
                    (local.get 0)))))
            (if  ;; label = @4
              (i32.gt_u
                (i32.load offset=20
                  (local.get 0))
                (i32.load offset=28
                  (local.get 0)))
              (then
                (local.set 1
                  (i32.or
                    (call $__fflush_unlocked
                      (local.get 0))
                    (local.get 1)))))
            (if  ;; label = @4
              (local.get 2)
              (then
                (call $__unlockfile
                  (local.get 0))))
            (br_if 0 (;@3;)
              (local.tee 0
                (i32.load offset=56
                  (local.get 0)))))))
      (call $__ofl_unlock))
    (local.get 1))
  (func $__fflush_unlocked (type 2) (param i32) (result i32)
    (local i32 i32)
    (block  ;; label = @1
      (br_if 0 (;@1;)
        (i32.le_u
          (i32.load offset=20
            (local.get 0))
          (i32.load offset=28
            (local.get 0))))
      (drop
        (call_indirect (type 3)
          (local.get 0)
          (i32.const 0)
          (i32.const 0)
          (i32.load offset=36
            (local.get 0))))
      (br_if 0 (;@1;)
        (i32.load offset=20
          (local.get 0)))
      (return
        (i32.const -1)))
    (if  ;; label = @1
      (i32.lt_u
        (local.tee 1
          (i32.load offset=4
            (local.get 0)))
        (local.tee 2
          (i32.load offset=8
            (local.get 0))))
      (then
        (drop
          (call_indirect (type 12)
            (local.get 0)
            (i64.extend_i32_s
              (i32.sub
                (local.get 1)
                (local.get 2)))
            (i32.const 1)
            (i32.load offset=40
              (local.get 0))))))
    (i32.store offset=28
      (local.get 0)
      (i32.const 0))
    (i64.store offset=16
      (local.get 0)
      (i64.const 0))
    (i64.store offset=4 align=4
      (local.get 0)
      (i64.const 0))
    (i32.const 0))
  (table (;0;) 27 27 funcref)
  (memory (;0;) 256 256)
  (global $__stack_pointer (mut i32) (i32.const 5248816))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "_start" (func $_start))
  (export "__getTypeName" (func $__getTypeName))
  (export "__embind_register_native_and_builtin_types" (func $__embind_register_native_and_builtin_types))
  (export "malloc" (func $dlmalloc))
  (export "fflush" (func $fflush))
  (export "__errno_location" (func $__errno_location))
  (export "stackSave" (func $stackSave))
  (export "stackRestore" (func $stackRestore))
  (export "stackAlloc" (func $stackAlloc))
  (export "free" (func $dlfree))
  (elem (;0;) (i32.const 1) func $emscripten::internal::Invoker<int>::invoke_int__*____ $__original_main $Add::compute_int__int_ $__wasm_call_ctors $EmscriptenBindingInitializer_native_and_builtin_types::EmscriptenBindingInitializer_native_and_builtin_types__ $__cxxabiv1::__shim_type_info::~__shim_type_info__ $__cxxabiv1::__fundamental_type_info::~__fundamental_type_info__ $__cxxabiv1::__shim_type_info::noop1___const $__cxxabiv1::__shim_type_info::noop2___const $__cxxabiv1::__fundamental_type_info::can_catch___cxxabiv1::__shim_type_info_const*__void*&__const $__cxxabiv1::__class_type_info::~__class_type_info__ $__cxxabiv1::__class_type_info::can_catch___cxxabiv1::__shim_type_info_const*__void*&__const $__cxxabiv1::__class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const $__cxxabiv1::__class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const $__cxxabiv1::__class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const $__cxxabiv1::__si_class_type_info::~__si_class_type_info__ $__cxxabiv1::__si_class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const $__cxxabiv1::__si_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const $__cxxabiv1::__si_class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const $__cxxabiv1::__vmi_class_type_info::~__vmi_class_type_info__ $__cxxabiv1::__vmi_class_type_info::search_above_dst___cxxabiv1::__dynamic_cast_info*__void_const*__void_const*__int__bool__const $__cxxabiv1::__vmi_class_type_info::search_below_dst___cxxabiv1::__dynamic_cast_info*__void_const*__int__bool__const $__cxxabiv1::__vmi_class_type_info::has_unambiguous_public_base___cxxabiv1::__dynamic_cast_info*__void*__int__const $__emscripten_stdout_close $__stdio_write $__emscripten_stdout_seek)
  (data $.rodata (i32.const 1024) "-+   0X0x\00unsigned short\00unsigned int\00float\00uint64_t\00unsigned char\00%p\00main\00bool\00emscripten::val\00unsigned long\00std::wstring\00std::string\00std::u16string\00std::u32string\00double\00void\00emscripten::memory_view<short>\00emscripten::memory_view<unsigned short>\00emscripten::memory_view<int>\00emscripten::memory_view<unsigned int>\00emscripten::memory_view<float>\00emscripten::memory_view<uint8_t>\00emscripten::memory_view<int8_t>\00emscripten::memory_view<uint16_t>\00emscripten::memory_view<int16_t>\00emscripten::memory_view<uint32_t>\00emscripten::memory_view<int32_t>\00emscripten::memory_view<char>\00emscripten::memory_view<unsigned char>\00std::basic_string<unsigned char>\00emscripten::memory_view<signed char>\00emscripten::memory_view<long>\00emscripten::memory_view<unsigned long>\00emscripten::memory_view<double>\00(null)\00\00\00\00\008\07\00\00\03\00\00\003Add\004Math\00\00\90\0c\00\00)\07\00\00\b8\0c\00\00$\07\00\000\07\00\00,\0c\00\00ii\00NSt3__212basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE\00NSt3__221__basic_string_commonILb1EEE\00\90\0c\00\00\8a\07\00\00\14\0d\00\00K\07\00\00\00\00\00\00\01\00\00\00\b0\07\00\00\00\00\00\00NSt3__212basic_stringIhNS_11char_traitsIhEENS_9allocatorIhEEEE\00\00\14\0d\00\00\d0\07\00\00\00\00\00\00\01\00\00\00\b0\07\00\00\00\00\00\00NSt3__212basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEE\00\00\14\0d\00\00(\08\00\00\00\00\00\00\01\00\00\00\b0\07\00\00\00\00\00\00NSt3__212basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE\00\00\00\14\0d\00\00\80\08\00\00\00\00\00\00\01\00\00\00\b0\07\00\00\00\00\00\00NSt3__212basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE\00\00\00\14\0d\00\00\dc\08\00\00\00\00\00\00\01\00\00\00\b0\07\00\00\00\00\00\00N10emscripten3valE\00\00\90\0c\00\008\09\00\00N10emscripten11memory_viewIcEE\00\00\90\0c\00\00T\09\00\00N10emscripten11memory_viewIaEE\00\00\90\0c\00\00|\09\00\00N10emscripten11memory_viewIhEE\00\00\90\0c\00\00\a4\09\00\00N10emscripten11memory_viewIsEE\00\00\90\0c\00\00\cc\09\00\00N10emscripten11memory_viewItEE\00\00\90\0c\00\00\f4\09\00\00N10emscripten11memory_viewIiEE\00\00\90\0c\00\00\1c\0a\00\00N10emscripten11memory_viewIjEE\00\00\90\0c\00\00D\0a\00\00N10emscripten11memory_viewIlEE\00\00\90\0c\00\00l\0a\00\00N10emscripten11memory_viewImEE\00\00\90\0c\00\00\94\0a\00\00N10emscripten11memory_viewIfEE\00\00\90\0c\00\00\bc\0a\00\00N10emscripten11memory_viewIdEE\00\00\90\0c\00\00\e4\0a\00\00St9type_info\00\00\00\00\90\0c\00\00\0c\0b\00\00N10__cxxabiv116__shim_type_infoE\00\00\00\00\b8\0c\00\00$\0b\00\00\1c\0b\00\00N10__cxxabiv117__class_type_infoE\00\00\00\b8\0c\00\00T\0b\00\00H\0b\00\00\00\00\00\00\c8\0b\00\00\06\00\00\00\07\00\00\00\08\00\00\00\09\00\00\00\0a\00\00\00N10__cxxabiv123__fundamental_type_infoE\00\b8\0c\00\00\a0\0b\00\00H\0b\00\00v\00\00\00\8c\0b\00\00\d4\0b\00\00b\00\00\00\8c\0b\00\00\e0\0b\00\00c\00\00\00\8c\0b\00\00\ec\0b\00\00h\00\00\00\8c\0b\00\00\f8\0b\00\00a\00\00\00\8c\0b\00\00\04\0c\00\00s\00\00\00\8c\0b\00\00\10\0c\00\00t\00\00\00\8c\0b\00\00\1c\0c\00\00i\00\00\00\8c\0b\00\00(\0c\00\00j\00\00\00\8c\0b\00\004\0c\00\00l\00\00\00\8c\0b\00\00@\0c\00\00m\00\00\00\8c\0b\00\00L\0c\00\00x\00\00\00\8c\0b\00\00X\0c\00\00y\00\00\00\8c\0b\00\00d\0c\00\00f\00\00\00\8c\0b\00\00p\0c\00\00d\00\00\00\8c\0b\00\00|\0c\00\00\00\00\00\00x\0b\00\00\06\00\00\00\0b\00\00\00\08\00\00\00\09\00\00\00\0c\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00\00\00\00\00\00\0d\00\00\06\00\00\00\10\00\00\00\08\00\00\00\09\00\00\00\0c\00\00\00\11\00\00\00\12\00\00\00\13\00\00\00N10__cxxabiv120__si_class_type_infoE\00\00\00\00\b8\0c\00\00\d8\0c\00\00x\0b\00\00\00\00\00\00\5c\0d\00\00\06\00\00\00\14\00\00\00\08\00\00\00\09\00\00\00\0c\00\00\00\15\00\00\00\16\00\00\00\17\00\00\00N10__cxxabiv121__vmi_class_type_infoE\00\00\00\b8\0c\00\004\0d\00\00x\0b\00\00X\0f\00\00\00\00\00\00\11\00\0a\00\11\11\11\00\00\00\00\05\00\00\00\00\00\00\09\00\00\00\00\0b\00\00\00\00\00\00\00\00\11\00\0f\0a\11\11\11\03\0a\07\00\01\00\09\0b\0b\00\00\09\06\0b\00\00\0b\00\06\11\00\00\00\11\11\11")
  (data $.rodata.1 (i32.const 3521) "\0b\00\00\00\00\00\00\00\00\11\00\0a\0a\11\11\11\00\0a\00\00\02\00\09\0b\00\00\00\09\00\0b\00\00\0b")
  (data $.rodata.2 (i32.const 3579) "\0c")
  (data $.rodata.3 (i32.const 3591) "\0c\00\00\00\00\0c\00\00\00\00\09\0c\00\00\00\00\00\0c\00\00\0c")
  (data $.rodata.4 (i32.const 3637) "\0e")
  (data $.rodata.5 (i32.const 3649) "\0d\00\00\00\04\0d\00\00\00\00\09\0e\00\00\00\00\00\0e\00\00\0e")
  (data $.rodata.6 (i32.const 3695) "\10")
  (data $.rodata.7 (i32.const 3707) "\0f\00\00\00\00\0f\00\00\00\00\09\10\00\00\00\00\00\10\00\00\10\00\00\12\00\00\00\12\12\12")
  (data $.rodata.8 (i32.const 3762) "\12\00\00\00\12\12\12\00\00\00\00\00\00\09")
  (data $.rodata.9 (i32.const 3811) "\0b")
  (data $.rodata.10 (i32.const 3823) "\0a\00\00\00\00\0a\00\00\00\00\09\0b\00\00\00\00\00\0b\00\00\0b")
  (data $.rodata.11 (i32.const 3869) "\0c")
  (data $.rodata.12 (i32.const 3881) "\0c\00\00\00\00\0c\00\00\00\00\09\0c\00\00\00\00\00\0c\00\00\0c\00\000123456789ABCDEF")
  (data $.data (i32.const 3920) "0\17P\00\00\00\00\00\05")
  (data $.data.1 (i32.const 3940) "\18")
  (data $.data.2 (i32.const 3964) "\19\00\00\00\1a\00\00\00\d8\12\00\00\00\04")
  (data $.data.3 (i32.const 3988) "\01")
  (data $.data.4 (i32.const 4003) "\0a\ff\ff\ff\ff")
  (data $.data.5 (i32.const 4072) "X\0f")
  (data $.data.6 (i32.const 4249) "\17"))
