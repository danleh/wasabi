// low-level stuff, TODO auto-generated from WASM module to-instrument
let resolveTableIdx = function (i) {
    throw "internal error! the monkey-patched WebAssembly.instantiate should have replaced this function with WebAssembly.Instance.Table.get(i)"
}

const oldInstantiate = WebAssembly.instantiate;
WebAssembly.instantiate = function () {
    let importsObject = arguments[1] || {};
    importsObject.hooks = {
        nop: function (func, instr) {
            nop({func, instr});
        },
        unreachable: function (func, instr) {
            unreachable({func, instr});
        },
        drop: function (func, instr) {
            drop({func, instr});
        },
        select: function (func, instr, cond) {
            select({func, instr}, cond);
        },
        current_memory: function (func, instr, currentSizePages) {
            current_memory({func, instr}, currentSizePages);
        },
        grow_memory: function (func, instr, byPages, previousSizePages) {
            grow_memory({func, instr}, byPages, previousSizePages);
        },
        begin_function: function (func, instr) {
            begin({func, instr}, "function");
        },
        end_function: function (func, instr) {
            end({func, instr}, "function", {func, instr: -1});
        },
        begin_block: function (func, instr) {
            begin({func, instr}, "block");
        },
        end_block: function (func, instr, begin_instr) {
            end({func, instr}, "block", {func, instr: begin_instr});
        },
        begin_loop: function (func, instr) {
            begin({func, instr}, "loop");
        },
        end_loop: function (func, instr, begin_instr) {
            end({func, instr}, "loop", {func, instr: begin_instr});
        },
        begin_if: function (func, instr) {
            begin({func, instr}, "if");
        },
        end_if: function (func, instr, begin_instr) {
            end({func, instr}, "if", {func, instr: begin_instr});
        },
        begin_else: function (func, instr) {
            begin({func, instr}, "else");
        },
        end_else: function (func, instr, begin_instr) {
            end({func, instr}, "else", {func, instr: begin_instr});
        },

        // generated:
        return_: function (func, instr) {
            return_({func, instr}, []);
        },
        call_result_: function (func, instr) {
            call_result_({func, instr}, []);
        },
        return_i32: function (func, instr, result0) {
            return_({func, instr}, [result0]);
        },
        call_result_i32: function (func, instr, result0) {
            call_result_({func, instr}, [result0]);
        },
        return_i64: function (func, instr, result0_low, result0_high) {
            return_({func, instr}, [new Long(result0_low, result0_high)]);
        },
        call_result_i64: function (func, instr, result0_low, result0_high) {
            call_result_({func, instr}, [new Long(result0_low, result0_high)]);
        },
        return_f64: function (func, instr, result0) {
            return_({func, instr}, [result0]);
        },
        call_result_f64: function (func, instr, result0) {
            call_result_({func, instr}, [result0]);
        },
        get_local_i32: function (func, instr, index, v) {
            local({func, instr}, "get", index, v);
        },
        get_local_i64: function (func, instr, index, v_low, v_high) {
            local({func, instr}, "get", index, new Long(v_low, v_high));
        },
        get_local_f32: function (func, instr, index, v) {
            local({func, instr}, "get", index, v);
        },
        get_local_f64: function (func, instr, index, v) {
            local({func, instr}, "get", index, v);
        },
        set_local_i32: function (func, instr, index, v) {
            local({func, instr}, "set", index, v);
        },
        set_local_i64: function (func, instr, index, v_low, v_high) {
            local({func, instr}, "set", index, new Long(v_low, v_high));
        },
        set_local_f32: function (func, instr, index, v) {
            local({func, instr}, "set", index, v);
        },
        set_local_f64: function (func, instr, index, v) {
            local({func, instr}, "set", index, v);
        },
        tee_local_i32: function (func, instr, index, v) {
            local({func, instr}, "tee", index, v);
        },
        tee_local_i64: function (func, instr, index, v_low, v_high) {
            local({func, instr}, "tee", index, new Long(v_low, v_high));
        },
        tee_local_f32: function (func, instr, index, v) {
            local({func, instr}, "tee", index, v);
        },
        tee_local_f64: function (func, instr, index, v) {
            local({func, instr}, "tee", index, v);
        },
        get_global_i32: function (func, instr, index, v) {
            global({func, instr}, "get", index, v);
        },
        get_global_i64: function (func, instr, index, v_low, v_high) {
            global({func, instr}, "get", index, new Long(v_low, v_high));
        },
        get_global_f32: function (func, instr, index, v) {
            global({func, instr}, "get", index, v);
        },
        get_global_f64: function (func, instr, index, v) {
            global({func, instr}, "get", index, v);
        },
        set_global_i32: function (func, instr, index, v) {
            global({func, instr}, "set", index, v);
        },
        set_global_i64: function (func, instr, index, v_low, v_high) {
            global({func, instr}, "set", index, new Long(v_low, v_high));
        },
        set_global_f32: function (func, instr, index, v) {
            global({func, instr}, "set", index, v);
        },
        set_global_f64: function (func, instr, index, v) {
            global({func, instr}, "set", index, v);
        },
        call_: function (func, instr, targetFunc) {
            call_({func, instr}, targetFunc, false, []);
        },
        call_i32: function (func, instr, targetFunc, arg0) {
            call_({func, instr}, targetFunc, false, [arg0]);
        },
        call_i32_i32: function (func, instr, targetFunc, arg0, arg1) {
            call_({func, instr}, targetFunc, false, [arg0, arg1]);
        },
        call_i32_i32_i32: function (func, instr, targetFunc, arg0, arg1, arg2) {
            call_({func, instr}, targetFunc, false, [arg0, arg1, arg2]);
        },
        call_i32_i32_i32_i32: function (func, instr, targetFunc, arg0, arg1, arg2, arg3) {
            call_({func, instr}, targetFunc, false, [arg0, arg1, arg2, arg3]);
        },
        call_i32_i32_i32_i32_i32: function (func, instr, targetFunc, arg0, arg1, arg2, arg3, arg4) {
            call_({func, instr}, targetFunc, false, [arg0, arg1, arg2, arg3, arg4]);
        },
        call_i32_f64_i32_i32_i32_i32: function (func, instr, targetFunc, arg0, arg1, arg2, arg3, arg4, arg5) {
            call_({func, instr}, targetFunc, false, [arg0, arg1, arg2, arg3, arg4, arg5]);
        },
        call_i64_i32: function (func, instr, targetFunc, arg0_low, arg0_high, arg1) {
            call_({func, instr}, targetFunc, false, [new Long(arg0_low, arg0_high), arg1]);
        },
        call_i64_i32_i32: function (func, instr, targetFunc, arg0_low, arg0_high, arg1, arg2) {
            call_({func, instr}, targetFunc, false, [new Long(arg0_low, arg0_high), arg1, arg2]);
        },
        call_f64: function (func, instr, targetFunc, arg0) {
            call_({func, instr}, targetFunc, false, [arg0]);
        },
        call_f64_i32: function (func, instr, targetFunc, arg0, arg1) {
            call_({func, instr}, targetFunc, false, [arg0, arg1]);
        },
        call_indirect_: function (func, instr, targetTableIdx) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, []);
        },
        call_indirect_i32: function (func, instr, targetTableIdx, arg0) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [arg0]);
        },
        call_indirect_i32_i32: function (func, instr, targetTableIdx, arg0, arg1) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [arg0, arg1]);
        },
        call_indirect_i32_i32_i32: function (func, instr, targetTableIdx, arg0, arg1, arg2) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [arg0, arg1, arg2]);
        },
        call_indirect_i32_i32_i32_i32: function (func, instr, targetTableIdx, arg0, arg1, arg2, arg3) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [arg0, arg1, arg2, arg3]);
        },
        call_indirect_i32_i32_i32_i32_i32: function (func, instr, targetTableIdx, arg0, arg1, arg2, arg3, arg4) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [arg0, arg1, arg2, arg3, arg4]);
        },
        call_indirect_i32_f64_i32_i32_i32_i32: function (func, instr, targetTableIdx, arg0, arg1, arg2, arg3, arg4, arg5) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [arg0, arg1, arg2, arg3, arg4, arg5]);
        },
        call_indirect_i64_i32: function (func, instr, targetTableIdx, arg0_low, arg0_high, arg1) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [new Long(arg0_low, arg0_high), arg1]);
        },
        call_indirect_i64_i32_i32: function (func, instr, targetTableIdx, arg0_low, arg0_high, arg1, arg2) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [new Long(arg0_low, arg0_high), arg1, arg2]);
        },
        call_indirect_f64: function (func, instr, targetTableIdx, arg0) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [arg0]);
        },
        call_indirect_f64_i32: function (func, instr, targetTableIdx, arg0, arg1) {
            call_({func, instr}, resolveTableIdx(targetTableIdx), true, [arg0, arg1]);
        },
        i32_const: function (func, instr, v) {
            const_({func, instr}, v);
        },
        i64_const: function (func, instr, v_low, v_high) {
            const_({func, instr}, new Long(v_low, v_high));
        },
        f32_const: function (func, instr, v) {
            const_({func, instr}, v);
        },
        f64_const: function (func, instr, v) {
            const_({func, instr}, v);
        },
        i32_eqz: function (func, instr, input, result) {
            unary({func, instr}, "i32_eqz", input, result);
        },
        i64_eqz: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "i64_eqz", new Long(input_low, input_high), result);
        },
        i32_clz: function (func, instr, input, result) {
            unary({func, instr}, "i32_clz", input, result);
        },
        i32_ctz: function (func, instr, input, result) {
            unary({func, instr}, "i32_ctz", input, result);
        },
        i32_popcnt: function (func, instr, input, result) {
            unary({func, instr}, "i32_popcnt", input, result);
        },
        i64_clz: function (func, instr, input_low, input_high, result_low, result_high) {
            unary({func, instr}, "i64_clz", new Long(input_low, input_high), new Long(result_low, result_high));
        },
        i64_ctz: function (func, instr, input_low, input_high, result_low, result_high) {
            unary({func, instr}, "i64_ctz", new Long(input_low, input_high), new Long(result_low, result_high));
        },
        i64_popcnt: function (func, instr, input_low, input_high, result_low, result_high) {
            unary({func, instr}, "i64_popcnt", new Long(input_low, input_high), new Long(result_low, result_high));
        },
        f32_abs: function (func, instr, input, result) {
            unary({func, instr}, "f32_abs", input, result);
        },
        f32_neg: function (func, instr, input, result) {
            unary({func, instr}, "f32_neg", input, result);
        },
        f32_ceil: function (func, instr, input, result) {
            unary({func, instr}, "f32_ceil", input, result);
        },
        f32_floor: function (func, instr, input, result) {
            unary({func, instr}, "f32_floor", input, result);
        },
        f32_trunc: function (func, instr, input, result) {
            unary({func, instr}, "f32_trunc", input, result);
        },
        f32_nearest: function (func, instr, input, result) {
            unary({func, instr}, "f32_nearest", input, result);
        },
        f32_sqrt: function (func, instr, input, result) {
            unary({func, instr}, "f32_sqrt", input, result);
        },
        f64_abs: function (func, instr, input, result) {
            unary({func, instr}, "f64_abs", input, result);
        },
        f64_neg: function (func, instr, input, result) {
            unary({func, instr}, "f64_neg", input, result);
        },
        f64_ceil: function (func, instr, input, result) {
            unary({func, instr}, "f64_ceil", input, result);
        },
        f64_floor: function (func, instr, input, result) {
            unary({func, instr}, "f64_floor", input, result);
        },
        f64_trunc: function (func, instr, input, result) {
            unary({func, instr}, "f64_trunc", input, result);
        },
        f64_nearest: function (func, instr, input, result) {
            unary({func, instr}, "f64_nearest", input, result);
        },
        f64_sqrt: function (func, instr, input, result) {
            unary({func, instr}, "f64_sqrt", input, result);
        },
        i32_wrap_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "i32_wrap_i64", new Long(input_low, input_high), result);
        },
        i32_trunc_s_f32: function (func, instr, input, result) {
            unary({func, instr}, "i32_trunc_s_f32", input, result);
        },
        i32_trunc_u_f32: function (func, instr, input, result) {
            unary({func, instr}, "i32_trunc_u_f32", input, result);
        },
        i32_trunc_s_f64: function (func, instr, input, result) {
            unary({func, instr}, "i32_trunc_s_f64", input, result);
        },
        i32_trunc_u_f64: function (func, instr, input, result) {
            unary({func, instr}, "i32_trunc_u_f64", input, result);
        },
        i64_extend_s_i32: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_extend_s_i32", input, new Long(result_low, result_high));
        },
        i64_extend_u_i32: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_extend_u_i32", input, new Long(result_low, result_high));
        },
        i64_trunc_s_f32: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_trunc_s_f32", input, new Long(result_low, result_high));
        },
        i64_trunc_u_f32: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_trunc_u_f32", input, new Long(result_low, result_high));
        },
        i64_trunc_s_f64: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_trunc_s_f64", input, new Long(result_low, result_high));
        },
        i64_trunc_u_f64: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_trunc_u_f64", input, new Long(result_low, result_high));
        },
        f32_convert_s_i32: function (func, instr, input, result) {
            unary({func, instr}, "f32_convert_s_i32", input, result);
        },
        f32_convert_u_i32: function (func, instr, input, result) {
            unary({func, instr}, "f32_convert_u_i32", input, result);
        },
        f32_convert_s_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f32_convert_s_i64", new Long(input_low, input_high), result);
        },
        f32_convert_u_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f32_convert_u_i64", new Long(input_low, input_high), result);
        },
        f32_demote_f64: function (func, instr, input, result) {
            unary({func, instr}, "f32_demote_f64", input, result);
        },
        f64_convert_s_i32: function (func, instr, input, result) {
            unary({func, instr}, "f64_convert_s_i32", input, result);
        },
        f64_convert_u_i32: function (func, instr, input, result) {
            unary({func, instr}, "f64_convert_u_i32", input, result);
        },
        f64_convert_s_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f64_convert_s_i64", new Long(input_low, input_high), result);
        },
        f64_convert_u_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f64_convert_u_i64", new Long(input_low, input_high), result);
        },
        f64_promote_f32: function (func, instr, input, result) {
            unary({func, instr}, "f64_promote_f32", input, result);
        },
        i32_reinterpret_f32: function (func, instr, input, result) {
            unary({func, instr}, "i32_reinterpret_f32", input, result);
        },
        i64_reinterpret_f64: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_reinterpret_f64", input, new Long(result_low, result_high));
        },
        f32_reinterpret_i32: function (func, instr, input, result) {
            unary({func, instr}, "f32_reinterpret_i32", input, result);
        },
        f64_reinterpret_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f64_reinterpret_i64", new Long(input_low, input_high), result);
        },
        i32_eq: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_eq", first, second, result);
        },
        i32_ne: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_ne", first, second, result);
        },
        i32_lt_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_lt_s", first, second, result);
        },
        i32_lt_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_lt_u", first, second, result);
        },
        i32_gt_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_gt_s", first, second, result);
        },
        i32_gt_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_gt_u", first, second, result);
        },
        i32_le_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_le_s", first, second, result);
        },
        i32_le_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_le_u", first, second, result);
        },
        i32_ge_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_ge_s", first, second, result);
        },
        i32_ge_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_ge_u", first, second, result);
        },
        i64_eq: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({func, instr}, "i64_eq", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_ne: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({func, instr}, "i64_ne", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_lt_s: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_lt_s", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_lt_u: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_lt_u", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_gt_s: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_gt_s", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_gt_u: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_gt_u", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_le_s: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_le_s", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_le_u: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_le_u", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_ge_s: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_ge_s", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_ge_u: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_ge_u", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        f32_eq: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_eq", first, second, result);
        },
        f32_ne: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_ne", first, second, result);
        },
        f32_lt: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_lt", first, second, result);
        },
        f32_gt: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_gt", first, second, result);
        },
        f32_le: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_le", first, second, result);
        },
        f32_ge: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_ge", first, second, result);
        },
        f64_eq: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_eq", first, second, result);
        },
        f64_ne: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_ne", first, second, result);
        },
        f64_lt: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_lt", first, second, result);
        },
        f64_gt: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_gt", first, second, result);
        },
        f64_le: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_le", first, second, result);
        },
        f64_ge: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_ge", first, second, result);
        },
        i32_add: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_add", first, second, result);
        },
        i32_sub: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_sub", first, second, result);
        },
        i32_mul: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_mul", first, second, result);
        },
        i32_div_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_div_s", first, second, result);
        },
        i32_div_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_div_u", first, second, result);
        },
        i32_rem_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_rem_s", first, second, result);
        },
        i32_rem_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_rem_u", first, second, result);
        },
        i32_and: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_and", first, second, result);
        },
        i32_or: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_or", first, second, result);
        },
        i32_xor: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_xor", first, second, result);
        },
        i32_shl: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_shl", first, second, result);
        },
        i32_shr_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_shr_s", first, second, result);
        },
        i32_shr_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_shr_u", first, second, result);
        },
        i32_rotl: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_rotl", first, second, result);
        },
        i32_rotr: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_rotr", first, second, result);
        },
        i64_add: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_add", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_sub: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_sub", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_mul: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_mul", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_div_s: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_div_s", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_div_u: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_div_u", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_rem_s: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_rem_s", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_rem_u: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_rem_u", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_and: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_and", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_or: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_or", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_xor: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_xor", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_shl: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_shl", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_shr_s: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_shr_s", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_shr_u: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_shr_u", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_rotl: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_rotl", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        i64_rotr: function (func, instr, first_low, first_high, second_low, second_high, result_low, result_high) {
            binary({
                func,
                instr
            }, "i64_rotr", new Long(first_low, first_high), new Long(second_low, second_high), new Long(result_low, result_high));
        },
        f32_add: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_add", first, second, result);
        },
        f32_sub: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_sub", first, second, result);
        },
        f32_mul: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_mul", first, second, result);
        },
        f32_div: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_div", first, second, result);
        },
        f32_min: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_min", first, second, result);
        },
        f32_max: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_max", first, second, result);
        },
        f32_copysign: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_copysign", first, second, result);
        },
        f64_add: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_add", first, second, result);
        },
        f64_sub: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_sub", first, second, result);
        },
        f64_mul: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_mul", first, second, result);
        },
        f64_div: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_div", first, second, result);
        },
        f64_min: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_min", first, second, result);
        },
        f64_max: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_max", first, second, result);
        },
        f64_copysign: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_copysign", first, second, result);
        },
        i32_load: function (func, instr, addr, offset, align, v) {
            load({func, instr}, "i32_load", {addr, offset, align}, v);
        },
        i32_load_8_s: function (func, instr, addr, offset, align, v) {
            load({func, instr}, "i32_load_8_s", {addr, offset, align}, v);
        },
        i32_load_8_u: function (func, instr, addr, offset, align, v) {
            load({func, instr}, "i32_load_8_u", {addr, offset, align}, v);
        },
        i32_load_16_s: function (func, instr, addr, offset, align, v) {
            load({func, instr}, "i32_load_16_s", {addr, offset, align}, v);
        },
        i32_load_16_u: function (func, instr, addr, offset, align, v) {
            load({func, instr}, "i32_load_16_u", {addr, offset, align}, v);
        },
        i64_load: function (func, instr, addr, offset, align, v_low, v_high) {
            load({func, instr}, "i64_load", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_load_8_s: function (func, instr, addr, offset, align, v_low, v_high) {
            load({func, instr}, "i64_load_8_s", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_load_8_u: function (func, instr, addr, offset, align, v_low, v_high) {
            load({func, instr}, "i64_load_8_u", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_load_16_s: function (func, instr, addr, offset, align, v_low, v_high) {
            load({func, instr}, "i64_load_16_s", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_load_16_u: function (func, instr, addr, offset, align, v_low, v_high) {
            load({func, instr}, "i64_load_16_u", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_load_32_s: function (func, instr, addr, offset, align, v_low, v_high) {
            load({func, instr}, "i64_load_32_s", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_load_32_u: function (func, instr, addr, offset, align, v_low, v_high) {
            load({func, instr}, "i64_load_32_u", {addr, offset, align}, new Long(v_low, v_high));
        },
        f32_load: function (func, instr, addr, offset, align, v) {
            load({func, instr}, "f32_load", {addr, offset, align}, v);
        },
        f64_load: function (func, instr, addr, offset, align, v) {
            load({func, instr}, "f64_load", {addr, offset, align}, v);
        },
        i32_store: function (func, instr, addr, offset, align, v) {
            store({func, instr}, "i32_store", {addr, offset, align}, v);
        },
        i32_store_8: function (func, instr, addr, offset, align, v) {
            store({func, instr}, "i32_store_8", {addr, offset, align}, v);
        },
        i32_store_16: function (func, instr, addr, offset, align, v) {
            store({func, instr}, "i32_store_16", {addr, offset, align}, v);
        },
        i64_store: function (func, instr, addr, offset, align, v_low, v_high) {
            store({func, instr}, "i64_store", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_store_8: function (func, instr, addr, offset, align, v_low, v_high) {
            store({func, instr}, "i64_store_8", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_store_16: function (func, instr, addr, offset, align, v_low, v_high) {
            store({func, instr}, "i64_store_16", {addr, offset, align}, new Long(v_low, v_high));
        },
        i64_store_32: function (func, instr, addr, offset, align, v_low, v_high) {
            store({func, instr}, "i64_store_32", {addr, offset, align}, new Long(v_low, v_high));
        },
        f32_store: function (func, instr, addr, offset, align, v) {
            store({func, instr}, "f32_store", {addr, offset, align}, v);
        },
        f64_store: function (func, instr, addr, offset, align, v) {
            store({func, instr}, "f64_store", {addr, offset, align}, v);
        },
    };
    console.log(importsObject);

    arguments[1] = importsObject;
    const result = oldInstantiate.apply(this, arguments);
    result.then(result => {
        // FIXME replace "table" by exported name if already exported before instrumentation
        resolveTableIdx = tableIndex => parseInt(result.instance.exports["table"].get(tableIndex).name, 10);
        console.log("found exported table, using this to resolve call_indirect targets...");
    })
    return result;
};

// TODO find out WebAssembly.Table API to retrieve stuff

// const staticModuleInfo = {
//     functions: [
//         {
//             import: ["env", "bla"],
//             export: "exportedname",
//             type: {
//                 params: ["i32", "i32"],
//                 results: [],
//             },
//             locals: ["i32", "i32"],
//             instructions: 100,
//         }
//     ]
// };

// your instrumentation goes here...
// const coverageData = [];

// where type = "function" | "block" | "if" | "else"
// and else fires both begin and end, where the end has begin_location pointing to the if

function begin(location, type) {
    console.log("begin", type, "@", location);
}

function end(location, type, begin_location) {
    console.log("end @", location, "for begin", type, "@", begin_location);
}

function nop(location) {
    // console.log("nop @", location);
}

function unreachable(location) {
    // console.log("unreachable @", location);
}

function drop(location) {
    // console.log("drop @", location);
}

function select(location, cond) {
    // console.log("select @", location, "condition:", cond);
}

function call_(location, targetFunc, indirect, args) {
    // console.log("call", (indirect ? "(indirect)" : "(direct)"), "func #", targetFunc, "@", location, "with args:", args);
}

function return_(location, values) {
    // if (coverageData[location.func] === undefined) {
    //     coverageData[location.func] = new Set();
    // }
    // coverageData[location.func].add(location.instr);
    // if (coverageData[location.func][location.instr] === undefined) {
    //     coverageData[location.func][location.instr] = true;
    // }
}

function call_result_(location, values) {
    // console.log("result from call @", location, ":", values);
}

function const_(location, value) {
    // console.log("const @", location, ":", value);
}

function unary(location, op, input, result) {
    // console.log(op, "@", location, ":", input, "->", result);
}

function binary(location, op, first, second, result) {
    // console.log(op, "@", location, ":", first, ",", second, "->", result);
}

function load(location, op, memarg, value) {
    // console.log(op, "@", location, value, "from", memarg);
}

function store(location, op, memarg, value) {
    // console.log(op, "@", location, value, "to", memarg);
}

function current_memory(location, currentSizePages) {
    // console.log("current_memory @", location, "size (in pages):", currentSizePages);
}

function grow_memory(location, byPages, previousSizePages) {
    // console.log("grow_memory @", location, "delta (in pages):", byPages, "previous size (in pages):", previousSizePages);
}

function local(location, op, index, value) {
    // console.log(op, "local #", index, "@", location, ":", value);
}

function global(location, op, index, value) {
    // console.log(op, "global #", index, "@", location, ":", value);
}