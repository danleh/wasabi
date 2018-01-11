(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32 i32) (result i32)))
  (type (;3;) (func (param f32 f32) (result f32)))
  (type (;4;) (func (param f64 f64) (result f64)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i64 i64) (result i64)))
  (type (;7;) (func (param i64 i64 i32) (result i64)))
  (type (;8;) (func (param i32 i64 i64 i64 i64)))
  (type (;9;) (func (param i32 i64 i64 i64 i64 i32)))
  (type (;10;) (func (param f32 i32) (result f32)))
  (type (;11;) (func (param f64 i32) (result f64)))
  (type (;12;) (func (param i64 i32) (result i64)))
  (type (;13;) (func (param i32 i64 i64 i32)))
  (type (;14;) (func (param i32) (result f32)))
  (type (;15;) (func (param i32) (result f64)))
  (type (;16;) (func (param i64) (result f64)))
  (type (;17;) (func (param i64 i64) (result f32)))
  (type (;18;) (func (param i64 i64) (result f64)))
  (type (;19;) (func (param f32) (result i32)))
  (type (;20;) (func (param f32) (result i64)))
  (type (;21;) (func (param i32 f32)))
  (type (;22;) (func (param f64) (result i32)))
  (type (;23;) (func (param f64) (result i64)))
  (type (;24;) (func (param i32 f64)))
  (func (;0;) (type 0))
  (func (;1;) (type 1) (result i32)
    i32.const 1337)
  (func (;2;) (type 0))
  (func (;3;) (type 2) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      get_local 2
      i32.eqz
      br_if 0 (;@1;)
      get_local 0
      set_local 3
      loop  ;; label = @2
        get_local 3
        get_local 1
        i32.load8_u
        i32.store8
        get_local 1
        i32.const 1
        i32.add
        set_local 1
        get_local 3
        i32.const 1
        i32.add
        set_local 3
        get_local 2
        i32.const -1
        i32.add
        tee_local 2
        br_if 0 (;@2;)
      end
    end
    get_local 0)
  (func (;4;) (type 2) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        get_local 1
        get_local 0
        i32.ge_u
        br_if 0 (;@2;)
        get_local 2
        i32.eqz
        br_if 1 (;@1;)
        loop  ;; label = @3
          get_local 0
          get_local 2
          i32.add
          i32.const -1
          i32.add
          get_local 1
          get_local 2
          i32.add
          i32.const -1
          i32.add
          i32.load8_u
          i32.store8
          get_local 2
          i32.const -1
          i32.add
          tee_local 2
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
        unreachable
      end
      get_local 2
      i32.eqz
      br_if 0 (;@1;)
      get_local 0
      set_local 3
      loop  ;; label = @2
        get_local 3
        get_local 1
        i32.load8_u
        i32.store8
        get_local 1
        i32.const 1
        i32.add
        set_local 1
        get_local 3
        i32.const 1
        i32.add
        set_local 3
        get_local 2
        i32.const -1
        i32.add
        tee_local 2
        br_if 0 (;@2;)
      end
    end
    get_local 0)
  (func (;5;) (type 2) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      get_local 2
      i32.eqz
      br_if 0 (;@1;)
      get_local 0
      set_local 3
      loop  ;; label = @2
        get_local 3
        get_local 1
        i32.store8
        get_local 3
        i32.const 1
        i32.add
        set_local 3
        get_local 2
        i32.const -1
        i32.add
        tee_local 2
        br_if 0 (;@2;)
      end
    end
    get_local 0)
  (func (;6;) (type 2) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        get_local 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        set_local 5
        loop  ;; label = @3
          get_local 0
          get_local 5
          i32.add
          i32.load8_u
          tee_local 3
          get_local 1
          get_local 5
          i32.add
          i32.load8_u
          tee_local 4
          i32.ne
          br_if 2 (;@1;)
          get_local 5
          i32.const 1
          i32.add
          tee_local 5
          get_local 2
          i32.lt_u
          br_if 0 (;@3;)
        end
        i32.const 0
        return
      end
      i32.const 0
      return
    end
    get_local 3
    get_local 4
    i32.sub)
  (func (;7;) (type 3) (param f32 f32) (result f32)
    get_local 1
    i32.reinterpret/f32
    i32.const -2147483648
    i32.xor
    f32.reinterpret/i32
    get_local 0
    f32.add)
  (func (;8;) (type 4) (param f64 f64) (result f64)
    get_local 1
    i64.reinterpret/f64
    i64.const -9223372036854775808
    i64.xor
    f64.reinterpret/i64
    get_local 0
    f64.add)
  (func (;9;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      get_local 1
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      set_local 5
      block  ;; label = @2
        get_local 0
        i32.eqz
        br_if 0 (;@2;)
        get_local 1
        i32.clz
        get_local 0
        i32.clz
        i32.sub
        tee_local 4
        i32.const 32
        i32.ge_u
        br_if 0 (;@2;)
        block  ;; label = @3
          get_local 4
          i32.const 31
          i32.ne
          br_if 0 (;@3;)
          get_local 0
          return
        end
        get_local 0
        i32.const 31
        get_local 4
        i32.sub
        i32.const 31
        i32.and
        i32.shl
        set_local 5
        block  ;; label = @3
          block  ;; label = @4
            get_local 4
            i32.const 1
            i32.add
            tee_local 4
            i32.eqz
            br_if 0 (;@4;)
            get_local 1
            i32.const -1
            i32.add
            set_local 2
            get_local 0
            get_local 4
            i32.const 31
            i32.and
            i32.shr_u
            set_local 0
            i32.const 0
            set_local 6
            loop  ;; label = @5
              get_local 5
              i32.const 31
              i32.shr_u
              get_local 0
              i32.const 1
              i32.shl
              i32.or
              tee_local 0
              get_local 2
              get_local 0
              i32.sub
              i32.const 31
              i32.shr_s
              tee_local 3
              get_local 1
              i32.and
              i32.sub
              set_local 0
              get_local 5
              i32.const 1
              i32.shl
              get_local 6
              i32.or
              set_local 5
              get_local 3
              i32.const 1
              i32.and
              tee_local 3
              set_local 6
              get_local 4
              i32.const -1
              i32.add
              tee_local 4
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
            unreachable
          end
          i32.const 0
          set_local 3
        end
        get_local 5
        i32.const 1
        i32.shl
        get_local 3
        i32.or
        set_local 5
      end
      get_local 5
      return
    end
    unreachable
    unreachable)
  (func (;10;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      get_local 1
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      set_local 5
      block  ;; label = @2
        get_local 0
        i32.eqz
        br_if 0 (;@2;)
        get_local 1
        i32.clz
        get_local 0
        i32.clz
        i32.sub
        tee_local 4
        i32.const 31
        i32.gt_u
        br_if 0 (;@2;)
        get_local 0
        set_local 5
        get_local 4
        i32.const 31
        i32.eq
        br_if 0 (;@2;)
        get_local 0
        i32.const 31
        get_local 4
        i32.sub
        i32.const 31
        i32.and
        i32.shl
        set_local 5
        block  ;; label = @3
          block  ;; label = @4
            get_local 4
            i32.const 1
            i32.add
            tee_local 4
            i32.eqz
            br_if 0 (;@4;)
            get_local 1
            i32.const -1
            i32.add
            set_local 2
            get_local 0
            get_local 4
            i32.const 31
            i32.and
            i32.shr_u
            set_local 6
            i32.const 0
            set_local 7
            loop  ;; label = @5
              get_local 6
              i32.const 1
              i32.shl
              get_local 5
              i32.const 31
              i32.shr_u
              i32.or
              tee_local 6
              get_local 2
              get_local 6
              i32.sub
              i32.const 31
              i32.shr_s
              tee_local 3
              get_local 1
              i32.and
              i32.sub
              set_local 6
              get_local 7
              get_local 5
              i32.const 1
              i32.shl
              i32.or
              set_local 5
              get_local 3
              i32.const 1
              i32.and
              tee_local 3
              set_local 7
              get_local 4
              i32.const -1
              i32.add
              tee_local 4
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
            unreachable
          end
          i32.const 0
          set_local 3
        end
        get_local 5
        i32.const 1
        i32.shl
        get_local 3
        i32.or
        set_local 5
      end
      get_local 0
      get_local 5
      get_local 1
      i32.mul
      i32.sub
      return
    end
    unreachable
    unreachable)
  (func (;11;) (type 2) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      get_local 1
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      set_local 6
      block  ;; label = @2
        get_local 0
        i32.eqz
        br_if 0 (;@2;)
        get_local 1
        i32.clz
        get_local 0
        i32.clz
        i32.sub
        tee_local 5
        i32.const 31
        i32.gt_u
        br_if 0 (;@2;)
        get_local 0
        set_local 6
        get_local 5
        i32.const 31
        i32.eq
        br_if 0 (;@2;)
        get_local 0
        i32.const 31
        get_local 5
        i32.sub
        i32.const 31
        i32.and
        i32.shl
        set_local 6
        block  ;; label = @3
          block  ;; label = @4
            get_local 5
            i32.const 1
            i32.add
            tee_local 5
            i32.eqz
            br_if 0 (;@4;)
            get_local 1
            i32.const -1
            i32.add
            set_local 3
            get_local 0
            get_local 5
            i32.const 31
            i32.and
            i32.shr_u
            set_local 7
            i32.const 0
            set_local 8
            loop  ;; label = @5
              get_local 7
              i32.const 1
              i32.shl
              get_local 6
              i32.const 31
              i32.shr_u
              i32.or
              tee_local 7
              get_local 3
              get_local 7
              i32.sub
              i32.const 31
              i32.shr_s
              tee_local 4
              get_local 1
              i32.and
              i32.sub
              set_local 7
              get_local 8
              get_local 6
              i32.const 1
              i32.shl
              i32.or
              set_local 6
              get_local 4
              i32.const 1
              i32.and
              tee_local 4
              set_local 8
              get_local 5
              i32.const -1
              i32.add
              tee_local 5
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
            unreachable
          end
          i32.const 0
          set_local 4
        end
        get_local 6
        i32.const 1
        i32.shl
        get_local 4
        i32.or
        set_local 6
      end
      block  ;; label = @2
        get_local 2
        i32.eqz
        br_if 0 (;@2;)
        get_local 2
        get_local 0
        get_local 6
        get_local 1
        i32.mul
        i32.sub
        i32.store
      end
      get_local 6
      return
    end
    unreachable
    unreachable)
  (func (;12;) (type 6) (param i64 i64) (result i64)
    get_local 0
    get_local 1
    i32.const 0
    call 13)
  (func (;13;) (type 7) (param i64 i64 i32) (result i64)
    (local i32 i32 i64 i64 i64 i64)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    get_local 0
                                    i64.const 4294967295
                                    i64.gt_u
                                    br_if 0 (;@16;)
                                    get_local 1
                                    i64.const 4294967296
                                    i64.ge_u
                                    br_if 1 (;@15;)
                                    get_local 1
                                    i32.wrap/i64
                                    set_local 3
                                    get_local 2
                                    i32.eqz
                                    br_if 4 (;@12;)
                                    get_local 3
                                    i32.eqz
                                    br_if 13 (;@3;)
                                    get_local 2
                                    get_local 0
                                    i32.wrap/i64
                                    get_local 3
                                    i32.rem_u
                                    i64.extend_u/i32
                                    i64.store
                                    br 5 (;@11;)
                                  end
                                  get_local 1
                                  i32.wrap/i64
                                  tee_local 3
                                  i32.eqz
                                  br_if 1 (;@14;)
                                  get_local 1
                                  i64.const 4294967296
                                  i64.ge_u
                                  br_if 2 (;@13;)
                                  get_local 3
                                  i32.const -1
                                  i32.add
                                  get_local 3
                                  i32.and
                                  i32.eqz
                                  br_if 7 (;@8;)
                                  i32.const 0
                                  get_local 3
                                  i32.clz
                                  i32.const 33
                                  i32.add
                                  get_local 0
                                  i64.const 32
                                  i64.shr_u
                                  i32.wrap/i64
                                  i32.clz
                                  i32.sub
                                  tee_local 3
                                  i32.sub
                                  set_local 4
                                  br 11 (;@4;)
                                end
                                i64.const 0
                                set_local 8
                                get_local 2
                                i32.eqz
                                br_if 13 (;@1;)
                                get_local 2
                                get_local 0
                                i64.store
                                i64.const 0
                                return
                              end
                              get_local 1
                              i64.const 4294967296
                              i64.lt_u
                              br_if 10 (;@3;)
                              get_local 0
                              i32.wrap/i64
                              i32.eqz
                              br_if 4 (;@9;)
                              get_local 1
                              i64.const 32
                              i64.shr_u
                              i32.wrap/i64
                              tee_local 3
                              i32.eqz
                              br_if 6 (;@7;)
                              get_local 3
                              i32.const -1
                              i32.add
                              get_local 3
                              i32.and
                              br_if 6 (;@7;)
                              block  ;; label = @14
                                get_local 2
                                i32.eqz
                                br_if 0 (;@14;)
                                get_local 2
                                get_local 1
                                i64.const -4294967296
                                i64.add
                                i64.const 4294967295
                                i64.or
                                get_local 0
                                i64.and
                                i64.store
                              end
                              get_local 0
                              i64.const 32
                              i64.shr_u
                              i32.wrap/i64
                              get_local 3
                              i32.ctz
                              i32.const 31
                              i32.and
                              i32.shr_u
                              i64.extend_u/i32
                              return
                            end
                            get_local 1
                            i64.const 32
                            i64.shr_u
                            i32.wrap/i64
                            i32.clz
                            get_local 0
                            i64.const 32
                            i64.shr_u
                            i32.wrap/i64
                            i32.clz
                            i32.sub
                            tee_local 3
                            i32.const 31
                            i32.le_u
                            br_if 2 (;@10;)
                            i64.const 0
                            set_local 8
                            get_local 2
                            i32.eqz
                            br_if 11 (;@1;)
                            get_local 2
                            get_local 0
                            i64.store
                            i64.const 0
                            return
                          end
                          get_local 3
                          i32.eqz
                          br_if 9 (;@2;)
                        end
                        get_local 0
                        i32.wrap/i64
                        get_local 3
                        i32.div_u
                        i64.extend_u/i32
                        set_local 8
                        br 9 (;@1;)
                      end
                      i32.const 63
                      get_local 3
                      i32.sub
                      set_local 4
                      get_local 3
                      i32.const 1
                      i32.add
                      set_local 3
                      br 5 (;@4;)
                    end
                    get_local 1
                    i64.const 32
                    i64.shr_u
                    i32.wrap/i64
                    set_local 3
                    block  ;; label = @9
                      get_local 2
                      i32.eqz
                      br_if 0 (;@9;)
                      get_local 2
                      get_local 0
                      i64.const 32
                      i64.shr_u
                      i32.wrap/i64
                      get_local 3
                      i32.rem_u
                      i64.extend_u/i32
                      i64.const 32
                      i64.shl
                      i64.store
                    end
                    get_local 0
                    i64.const 32
                    i64.shr_u
                    i32.wrap/i64
                    get_local 3
                    i32.div_u
                    i64.extend_u/i32
                    return
                  end
                  block  ;; label = @8
                    get_local 2
                    i32.eqz
                    br_if 0 (;@8;)
                    get_local 2
                    get_local 1
                    i64.const 4294967295
                    i64.add
                    get_local 0
                    i64.and
                    i64.const 4294967295
                    i64.and
                    i64.store
                  end
                  get_local 3
                  i32.const 1
                  i32.ne
                  br_if 1 (;@6;)
                  get_local 0
                  return
                end
                get_local 3
                i32.clz
                get_local 0
                i64.const 32
                i64.shr_u
                i32.wrap/i64
                i32.clz
                i32.sub
                tee_local 3
                i32.const 30
                i32.le_u
                br_if 1 (;@5;)
                i64.const 0
                set_local 8
                get_local 2
                i32.eqz
                br_if 5 (;@1;)
                get_local 2
                get_local 0
                i64.store
                i64.const 0
                return
              end
              get_local 0
              get_local 3
              i32.ctz
              i64.extend_u/i32
              i64.shr_u
              return
            end
            i32.const 63
            get_local 3
            i32.sub
            set_local 4
            get_local 3
            i32.const 1
            i32.add
            set_local 3
          end
          get_local 0
          get_local 4
          i32.const 63
          i32.and
          i64.extend_u/i32
          i64.shl
          set_local 8
          get_local 0
          get_local 3
          i32.const 63
          i32.and
          i64.extend_u/i32
          i64.shr_u
          set_local 0
          block  ;; label = @4
            block  ;; label = @5
              get_local 3
              i32.eqz
              br_if 0 (;@5;)
              get_local 1
              i64.const -1
              i64.add
              set_local 5
              i64.const 0
              set_local 7
              loop  ;; label = @6
                get_local 0
                i64.const 1
                i64.shl
                get_local 8
                i64.const 63
                i64.shr_u
                i64.or
                tee_local 0
                get_local 5
                get_local 0
                i64.sub
                i64.const 63
                i64.shr_s
                tee_local 6
                get_local 1
                i64.and
                i64.sub
                set_local 0
                get_local 7
                get_local 8
                i64.const 1
                i64.shl
                i64.or
                set_local 8
                get_local 6
                i64.const 1
                i64.and
                tee_local 6
                set_local 7
                get_local 3
                i32.const -1
                i32.add
                tee_local 3
                br_if 0 (;@6;)
                br 2 (;@4;)
              end
              unreachable
            end
            i64.const 0
            set_local 6
          end
          block  ;; label = @4
            get_local 2
            i32.eqz
            br_if 0 (;@4;)
            get_local 2
            get_local 0
            i64.store
          end
          get_local 6
          get_local 8
          i64.const 1
          i64.shl
          i64.or
          return
        end
        unreachable
        unreachable
      end
      unreachable
      unreachable
    end
    get_local 8)
  (func (;14;) (type 6) (param i64 i64) (result i64)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 2
    i32.store offset=4
    get_local 2
    i64.const 0
    i64.store offset=8
    get_local 0
    get_local 1
    get_local 2
    i32.const 8
    i32.add
    call 13
    drop
    get_local 2
    i64.load offset=8
    set_local 0
    i32.const 0
    get_local 2
    i32.const 16
    i32.add
    i32.store offset=4
    get_local 0)
  (func (;15;) (type 8) (param i32 i64 i64 i64 i64)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 5
    i32.store offset=4
    get_local 5
    get_local 1
    get_local 2
    get_local 3
    get_local 4
    i32.const 0
    call 16
    get_local 5
    i64.load
    set_local 1
    get_local 0
    i32.const 8
    i32.add
    get_local 5
    i32.const 8
    i32.add
    i64.load
    i64.store
    get_local 0
    get_local 1
    i64.store
    i32.const 0
    get_local 5
    i32.const 16
    i32.add
    i32.store offset=4)
  (func (;16;) (type 9) (param i32 i64 i64 i64 i64 i32)
    (local i32 i32 i32 i64 i64 i64 i64 i64 i64 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 48
    i32.sub
    tee_local 8
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      get_local 2
                                      i64.const 0
                                      i64.ne
                                      br_if 0 (;@17;)
                                      get_local 4
                                      i64.eqz
                                      i32.eqz
                                      br_if 1 (;@16;)
                                      get_local 5
                                      i32.eqz
                                      br_if 4 (;@13;)
                                      get_local 3
                                      i64.const 0
                                      i64.eq
                                      br_if 15 (;@2;)
                                      get_local 5
                                      i32.const 8
                                      i32.add
                                      i64.const 0
                                      i64.store
                                      get_local 5
                                      get_local 1
                                      get_local 3
                                      i64.rem_u
                                      i64.store
                                      br 5 (;@12;)
                                    end
                                    get_local 3
                                    i64.eqz
                                    br_if 1 (;@15;)
                                    get_local 4
                                    i64.eqz
                                    i32.eqz
                                    br_if 2 (;@14;)
                                    get_local 3
                                    i64.const -1
                                    i64.add
                                    tee_local 13
                                    get_local 3
                                    i64.and
                                    i64.eqz
                                    br_if 7 (;@9;)
                                    i32.const 0
                                    get_local 3
                                    i64.clz
                                    i32.wrap/i64
                                    i32.const 65
                                    i32.add
                                    get_local 2
                                    i64.clz
                                    i32.wrap/i64
                                    i32.sub
                                    tee_local 6
                                    i32.sub
                                    set_local 7
                                    br 12 (;@4;)
                                  end
                                  get_local 5
                                  i32.eqz
                                  br_if 8 (;@7;)
                                  get_local 5
                                  get_local 1
                                  i64.store
                                  get_local 5
                                  i32.const 8
                                  i32.add
                                  get_local 2
                                  i64.store
                                  br 8 (;@7;)
                                end
                                get_local 4
                                i64.eqz
                                tee_local 6
                                br_if 12 (;@2;)
                                get_local 1
                                i64.const 0
                                i64.eq
                                br_if 4 (;@10;)
                                get_local 6
                                br_if 6 (;@8;)
                                get_local 4
                                i64.const -1
                                i64.add
                                tee_local 13
                                get_local 4
                                i64.and
                                i64.eqz
                                i32.eqz
                                br_if 6 (;@8;)
                                block  ;; label = @15
                                  get_local 5
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  get_local 5
                                  get_local 1
                                  i64.store
                                  get_local 5
                                  i32.const 8
                                  i32.add
                                  get_local 13
                                  get_local 2
                                  i64.and
                                  i64.store
                                end
                                get_local 2
                                get_local 4
                                i64.ctz
                                i64.const 63
                                i64.and
                                i64.shr_u
                                set_local 1
                                br 8 (;@6;)
                              end
                              get_local 4
                              i64.clz
                              i32.wrap/i64
                              get_local 2
                              i64.clz
                              i32.wrap/i64
                              i32.sub
                              tee_local 6
                              i32.const 63
                              i32.le_u
                              br_if 2 (;@11;)
                              get_local 5
                              i32.eqz
                              br_if 6 (;@7;)
                              get_local 5
                              get_local 1
                              i64.store
                              get_local 5
                              i32.const 8
                              i32.add
                              get_local 2
                              i64.store
                              br 6 (;@7;)
                            end
                            get_local 3
                            i64.const 0
                            i64.eq
                            br_if 11 (;@1;)
                          end
                          get_local 1
                          get_local 3
                          i64.div_u
                          set_local 1
                          br 5 (;@6;)
                        end
                        i32.const 127
                        get_local 6
                        i32.sub
                        set_local 7
                        get_local 6
                        i32.const 1
                        i32.add
                        set_local 6
                        br 6 (;@4;)
                      end
                      block  ;; label = @10
                        get_local 5
                        i32.eqz
                        br_if 0 (;@10;)
                        get_local 5
                        i64.const 0
                        i64.store
                        get_local 5
                        i32.const 8
                        i32.add
                        get_local 2
                        get_local 4
                        i64.rem_u
                        i64.store
                      end
                      get_local 2
                      get_local 4
                      i64.div_u
                      set_local 1
                      br 3 (;@6;)
                    end
                    block  ;; label = @9
                      get_local 5
                      i32.eqz
                      br_if 0 (;@9;)
                      get_local 5
                      i32.const 8
                      i32.add
                      i64.const 0
                      i64.store
                      get_local 5
                      get_local 13
                      get_local 1
                      i64.and
                      i64.store
                    end
                    get_local 3
                    i64.const 1
                    i64.eq
                    br_if 5 (;@3;)
                    get_local 8
                    i32.const 32
                    i32.add
                    get_local 1
                    get_local 2
                    get_local 3
                    i64.ctz
                    i32.wrap/i64
                    call 44
                    get_local 8
                    i32.const 40
                    i32.add
                    i64.load
                    set_local 2
                    get_local 8
                    i64.load offset=32
                    set_local 1
                    br 5 (;@3;)
                  end
                  get_local 4
                  i64.clz
                  i32.wrap/i64
                  get_local 2
                  i64.clz
                  i32.wrap/i64
                  i32.sub
                  tee_local 6
                  i32.const 62
                  i32.le_u
                  br_if 2 (;@5;)
                  get_local 5
                  i32.eqz
                  br_if 0 (;@7;)
                  get_local 5
                  get_local 1
                  i64.store
                  get_local 5
                  i32.const 8
                  i32.add
                  get_local 2
                  i64.store
                end
                i64.const 0
                set_local 1
              end
              i64.const 0
              set_local 2
              br 2 (;@3;)
            end
            i32.const 127
            get_local 6
            i32.sub
            set_local 7
            get_local 6
            i32.const 1
            i32.add
            set_local 6
          end
          get_local 8
          get_local 1
          get_local 2
          get_local 7
          i32.const 127
          i32.and
          call 40
          get_local 8
          i32.const 16
          i32.add
          get_local 1
          get_local 2
          get_local 6
          i32.const 127
          i32.and
          call 44
          get_local 8
          i32.const 8
          i32.add
          i64.load
          set_local 2
          get_local 8
          i32.const 16
          i32.add
          i32.const 8
          i32.add
          i64.load
          set_local 14
          get_local 8
          i64.load
          set_local 1
          get_local 8
          i64.load offset=16
          set_local 13
          block  ;; label = @4
            block  ;; label = @5
              get_local 6
              i32.eqz
              br_if 0 (;@5;)
              get_local 4
              i64.const 1
              get_local 3
              i64.const -1
              i64.add
              tee_local 9
              get_local 3
              i64.lt_u
              i64.extend_u/i32
              get_local 9
              i64.const -1
              i64.ne
              select
              i64.add
              i64.const -1
              i64.add
              set_local 10
              i64.const 0
              set_local 15
              i64.const 0
              set_local 16
              loop  ;; label = @6
                get_local 14
                i64.const 1
                i64.shl
                get_local 13
                i64.const 63
                i64.shr_u
                i64.or
                tee_local 11
                get_local 10
                get_local 11
                i64.sub
                get_local 9
                get_local 13
                i64.const 1
                i64.shl
                get_local 2
                i64.const 63
                i64.shr_u
                i64.or
                tee_local 13
                i64.lt_u
                i64.extend_u/i32
                i64.sub
                i64.const 63
                i64.shr_s
                tee_local 11
                get_local 4
                i64.and
                i64.sub
                get_local 13
                get_local 11
                get_local 3
                i64.and
                tee_local 12
                i64.lt_u
                i64.extend_u/i32
                i64.sub
                set_local 14
                get_local 13
                get_local 12
                i64.sub
                set_local 13
                i64.const 0
                get_local 2
                i64.const 1
                i64.shl
                get_local 1
                i64.const 63
                i64.shr_u
                i64.or
                i64.or
                set_local 2
                get_local 16
                get_local 1
                i64.const 1
                i64.shl
                i64.or
                set_local 1
                get_local 11
                i64.const 1
                i64.and
                tee_local 11
                set_local 16
                get_local 6
                i32.const -1
                i32.add
                tee_local 6
                br_if 0 (;@6;)
                br 2 (;@4;)
              end
              unreachable
            end
            i64.const 0
            set_local 11
            i64.const 0
            set_local 15
          end
          block  ;; label = @4
            get_local 5
            i32.eqz
            br_if 0 (;@4;)
            get_local 5
            get_local 13
            i64.store
            get_local 5
            i32.const 8
            i32.add
            get_local 14
            i64.store
          end
          get_local 15
          get_local 2
          i64.const 1
          i64.shl
          get_local 1
          i64.const 63
          i64.shr_u
          i64.or
          i64.or
          set_local 2
          get_local 11
          get_local 1
          i64.const 1
          i64.shl
          i64.or
          set_local 1
        end
        get_local 0
        get_local 1
        i64.store
        get_local 0
        i32.const 8
        i32.add
        get_local 2
        i64.store
        i32.const 0
        get_local 8
        i32.const 48
        i32.add
        i32.store offset=4
        return
      end
      unreachable
      unreachable
    end
    unreachable
    unreachable)
  (func (;17;) (type 8) (param i32 i64 i64 i64 i64)
    (local i32 i32)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 32
    i32.sub
    tee_local 6
    i32.store offset=4
    get_local 6
    i32.const 16
    i32.add
    i32.const 8
    i32.add
    tee_local 5
    i64.const 0
    i64.store
    get_local 6
    i64.const 0
    i64.store offset=16
    get_local 6
    get_local 1
    get_local 2
    get_local 3
    get_local 4
    get_local 6
    i32.const 16
    i32.add
    call 16
    get_local 6
    i64.load offset=16
    set_local 1
    get_local 0
    i32.const 8
    i32.add
    get_local 5
    i64.load
    i64.store
    get_local 0
    get_local 1
    i64.store
    i32.const 0
    get_local 6
    i32.const 32
    i32.add
    i32.store offset=4)
  (func (;18;) (type 3) (param f32 f32) (result f32)
    (local i32 i32 i32 i32 i32 i32 i32)
    get_local 1
    i32.reinterpret/f32
    tee_local 3
    i32.const 2147483647
    i32.and
    set_local 5
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              get_local 0
              i32.reinterpret/f32
              tee_local 2
              i32.const 2147483647
              i32.and
              tee_local 4
              i32.const -1
              i32.add
              i32.const 2139095038
              i32.le_u
              br_if 0 (;@5;)
              get_local 4
              i32.const 2139095041
              i32.lt_u
              br_if 1 (;@4;)
              get_local 4
              i32.const 4194304
              i32.or
              f32.reinterpret/i32
              return
            end
            get_local 5
            i32.const -1
            i32.add
            i32.const 2139095038
            i32.le_u
            br_if 1 (;@3;)
          end
          block  ;; label = @4
            get_local 5
            i32.const 2139095041
            i32.lt_u
            br_if 0 (;@4;)
            get_local 5
            i32.const 4194304
            i32.or
            f32.reinterpret/i32
            return
          end
          block  ;; label = @4
            get_local 4
            i32.const 2139095040
            i32.ne
            br_if 0 (;@4;)
            f32.const nan (;=nan;)
            get_local 0
            get_local 3
            get_local 2
            i32.xor
            i32.const -2147483648
            i32.eq
            select
            return
          end
          get_local 5
          i32.const 2139095040
          i32.eq
          br_if 2 (;@1;)
          get_local 4
          i32.eqz
          br_if 1 (;@2;)
          get_local 0
          set_local 1
          get_local 5
          i32.eqz
          br_if 2 (;@1;)
        end
        get_local 3
        get_local 2
        get_local 5
        get_local 4
        i32.gt_u
        tee_local 5
        select
        tee_local 4
        i32.const 8388607
        i32.and
        set_local 8
        get_local 2
        get_local 3
        get_local 5
        select
        tee_local 6
        i32.const 23
        i32.shr_u
        i32.const 255
        i32.and
        set_local 3
        block  ;; label = @3
          get_local 4
          i32.const 23
          i32.shr_u
          i32.const 255
          i32.and
          tee_local 5
          br_if 0 (;@3;)
          i32.const 9
          get_local 8
          i32.clz
          tee_local 2
          i32.sub
          set_local 5
          get_local 8
          get_local 2
          i32.const 24
          i32.add
          i32.const 31
          i32.and
          i32.shl
          set_local 8
        end
        get_local 6
        i32.const 8388607
        i32.and
        set_local 2
        block  ;; label = @3
          get_local 3
          br_if 0 (;@3;)
          i32.const 9
          get_local 2
          i32.clz
          tee_local 7
          i32.sub
          set_local 3
          get_local 2
          get_local 7
          i32.const 24
          i32.add
          i32.const 31
          i32.and
          i32.shl
          set_local 2
        end
        get_local 6
        get_local 4
        i32.xor
        set_local 6
        get_local 2
        i32.const 3
        i32.shl
        i32.const 67108864
        i32.or
        set_local 7
        get_local 8
        i32.const 3
        i32.shl
        set_local 8
        block  ;; label = @3
          block  ;; label = @4
            get_local 5
            get_local 3
            i32.sub
            tee_local 3
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            set_local 2
            get_local 3
            i32.const 31
            i32.gt_u
            br_if 1 (;@3;)
            get_local 7
            get_local 3
            i32.const 31
            i32.and
            i32.shr_u
            get_local 7
            i32.const 0
            get_local 3
            i32.sub
            i32.const 31
            i32.and
            i32.shl
            i32.const 0
            i32.ne
            i32.or
            set_local 2
            br 1 (;@3;)
          end
          get_local 7
          set_local 2
        end
        get_local 8
        i32.const 67108864
        i32.or
        set_local 3
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              get_local 6
              i32.const -1
              i32.le_s
              br_if 0 (;@5;)
              get_local 2
              get_local 3
              i32.add
              tee_local 3
              i32.const 134217728
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              get_local 2
              get_local 8
              i32.add
              i32.const 1
              i32.and
              get_local 3
              i32.const 1
              i32.shr_u
              i32.or
              set_local 3
              get_local 5
              i32.const 1
              i32.add
              set_local 5
              br 1 (;@4;)
            end
            get_local 3
            get_local 2
            i32.sub
            tee_local 3
            i32.eqz
            br_if 1 (;@3;)
            get_local 3
            i32.const 67108863
            i32.gt_u
            br_if 0 (;@4;)
            get_local 5
            get_local 3
            i32.clz
            i32.const -5
            i32.add
            tee_local 2
            i32.sub
            set_local 5
            get_local 3
            get_local 2
            i32.const 31
            i32.and
            i32.shl
            set_local 3
          end
          get_local 4
          i32.const -2147483648
          i32.and
          set_local 4
          block  ;; label = @4
            get_local 5
            i32.const 255
            i32.lt_s
            br_if 0 (;@4;)
            get_local 4
            i32.const 2139095040
            i32.or
            f32.reinterpret/i32
            return
          end
          i32.const 0
          set_local 2
          block  ;; label = @4
            block  ;; label = @5
              get_local 5
              i32.const 0
              i32.le_s
              br_if 0 (;@5;)
              get_local 5
              set_local 2
              br 1 (;@4;)
            end
            get_local 3
            i32.const 1
            get_local 5
            i32.sub
            tee_local 5
            i32.const 31
            i32.and
            i32.shr_u
            get_local 3
            i32.const 0
            get_local 5
            i32.sub
            i32.const 31
            i32.and
            i32.shl
            i32.const 0
            i32.ne
            i32.or
            set_local 3
          end
          get_local 3
          i32.const 3
          i32.shr_u
          tee_local 8
          i32.const 8388607
          i32.and
          get_local 4
          i32.or
          get_local 2
          i32.const 23
          i32.shl
          i32.or
          set_local 5
          block  ;; label = @4
            block  ;; label = @5
              get_local 3
              i32.const 7
              i32.and
              tee_local 4
              i32.const 5
              i32.lt_u
              br_if 0 (;@5;)
              get_local 5
              i32.const 1
              i32.add
              set_local 5
              br 1 (;@4;)
            end
            get_local 4
            i32.const 4
            i32.ne
            br_if 0 (;@4;)
            get_local 5
            get_local 8
            i32.const 1
            i32.and
            i32.add
            set_local 5
          end
          get_local 5
          f32.reinterpret/i32
          set_local 1
          br 2 (;@1;)
        end
        f32.const 0x0p+0 (;=0;)
        return
      end
      get_local 5
      br_if 0 (;@1;)
      get_local 3
      get_local 2
      i32.and
      f32.reinterpret/i32
      return
    end
    get_local 1)
  (func (;19;) (type 4) (param f64 f64) (result f64)
    (local i32 i32 i64 i64 i64 i64 i64 i64)
    get_local 1
    i64.reinterpret/f64
    tee_local 5
    i64.const 9223372036854775807
    i64.and
    set_local 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              get_local 0
              i64.reinterpret/f64
              tee_local 4
              i64.const 9223372036854775807
              i64.and
              tee_local 6
              i64.const -1
              i64.add
              i64.const 9218868437227405310
              i64.le_u
              br_if 0 (;@5;)
              get_local 6
              i64.const 9218868437227405313
              i64.lt_u
              br_if 1 (;@4;)
              get_local 6
              i64.const 2251799813685248
              i64.or
              f64.reinterpret/i64
              return
            end
            get_local 7
            i64.const -1
            i64.add
            i64.const 9218868437227405310
            i64.le_u
            br_if 1 (;@3;)
          end
          block  ;; label = @4
            get_local 7
            i64.const 9218868437227405313
            i64.lt_u
            br_if 0 (;@4;)
            get_local 7
            i64.const 2251799813685248
            i64.or
            f64.reinterpret/i64
            return
          end
          block  ;; label = @4
            get_local 6
            i64.const 9218868437227405312
            i64.ne
            br_if 0 (;@4;)
            f64.const nan (;=nan;)
            get_local 0
            get_local 5
            get_local 4
            i64.xor
            i64.const -9223372036854775808
            i64.eq
            select
            return
          end
          get_local 7
          i64.const 9218868437227405312
          i64.eq
          br_if 2 (;@1;)
          get_local 6
          i64.const 0
          i64.eq
          br_if 1 (;@2;)
          get_local 0
          set_local 1
          get_local 7
          i64.eqz
          br_if 2 (;@1;)
        end
        get_local 5
        get_local 4
        get_local 7
        get_local 6
        i64.gt_u
        tee_local 3
        select
        tee_local 7
        i64.const 4503599627370495
        i64.and
        set_local 6
        get_local 4
        get_local 5
        get_local 3
        select
        tee_local 4
        i64.const 52
        i64.shr_u
        i32.wrap/i64
        i32.const 2047
        i32.and
        set_local 2
        block  ;; label = @3
          get_local 7
          i64.const 52
          i64.shr_u
          i32.wrap/i64
          i32.const 2047
          i32.and
          tee_local 3
          br_if 0 (;@3;)
          i32.const 12
          get_local 6
          i64.clz
          tee_local 5
          i32.wrap/i64
          i32.sub
          set_local 3
          get_local 6
          get_local 5
          i64.const 53
          i64.add
          i64.const 63
          i64.and
          i64.shl
          set_local 6
        end
        get_local 4
        i64.const 4503599627370495
        i64.and
        set_local 5
        block  ;; label = @3
          get_local 2
          br_if 0 (;@3;)
          i32.const 12
          get_local 5
          i64.clz
          tee_local 8
          i32.wrap/i64
          i32.sub
          set_local 2
          get_local 5
          get_local 8
          i64.const 53
          i64.add
          i64.const 63
          i64.and
          i64.shl
          set_local 5
        end
        get_local 4
        get_local 7
        i64.xor
        set_local 8
        get_local 5
        i64.const 3
        i64.shl
        i64.const 36028797018963968
        i64.or
        set_local 9
        get_local 6
        i64.const 3
        i64.shl
        set_local 4
        block  ;; label = @3
          block  ;; label = @4
            get_local 3
            get_local 2
            i32.sub
            tee_local 2
            i32.eqz
            br_if 0 (;@4;)
            i64.const 1
            set_local 5
            get_local 2
            i32.const 63
            i32.gt_u
            br_if 1 (;@3;)
            get_local 9
            get_local 2
            i32.const 63
            i32.and
            i64.extend_u/i32
            i64.shr_u
            get_local 9
            i32.const 0
            get_local 2
            i32.sub
            i32.const 63
            i32.and
            i64.extend_u/i32
            i64.shl
            i64.const 0
            i64.ne
            i64.extend_u/i32
            i64.or
            set_local 5
            br 1 (;@3;)
          end
          get_local 9
          set_local 5
        end
        get_local 4
        i64.const 36028797018963968
        i64.or
        set_local 6
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              get_local 8
              i64.const -1
              i64.le_s
              br_if 0 (;@5;)
              get_local 5
              get_local 6
              i64.add
              tee_local 6
              i64.const 72057594037927936
              i64.and
              i64.eqz
              br_if 1 (;@4;)
              get_local 5
              get_local 4
              i64.add
              i64.const 1
              i64.and
              get_local 6
              i64.const 1
              i64.shr_u
              i64.or
              set_local 6
              get_local 3
              i32.const 1
              i32.add
              set_local 3
              br 1 (;@4;)
            end
            get_local 6
            get_local 5
            i64.sub
            tee_local 6
            i64.eqz
            br_if 1 (;@3;)
            get_local 6
            i64.const 36028797018963967
            i64.gt_u
            br_if 0 (;@4;)
            get_local 3
            get_local 6
            i64.clz
            i32.wrap/i64
            i32.const -8
            i32.add
            tee_local 2
            i32.sub
            set_local 3
            get_local 6
            get_local 2
            i32.const 63
            i32.and
            i64.extend_u/i32
            i64.shl
            set_local 6
          end
          get_local 7
          i64.const -9223372036854775808
          i64.and
          set_local 7
          block  ;; label = @4
            get_local 3
            i32.const 2047
            i32.lt_s
            br_if 0 (;@4;)
            get_local 7
            i64.const 9218868437227405312
            i64.or
            f64.reinterpret/i64
            return
          end
          i32.const 0
          set_local 2
          block  ;; label = @4
            block  ;; label = @5
              get_local 3
              i32.const 0
              i32.le_s
              br_if 0 (;@5;)
              get_local 3
              set_local 2
              br 1 (;@4;)
            end
            get_local 6
            i32.const 1
            get_local 3
            i32.sub
            tee_local 3
            i32.const 63
            i32.and
            i64.extend_u/i32
            i64.shr_u
            get_local 6
            i32.const 0
            get_local 3
            i32.sub
            i32.const 63
            i32.and
            i64.extend_u/i32
            i64.shl
            i64.const 0
            i64.ne
            i64.extend_u/i32
            i64.or
            set_local 6
          end
          get_local 6
          i64.const 3
          i64.shr_u
          tee_local 5
          i64.const 4503599627370495
          i64.and
          get_local 7
          i64.or
          get_local 2
          i64.extend_u/i32
          i64.const 52
          i64.shl
          i64.or
          set_local 7
          block  ;; label = @4
            block  ;; label = @5
              get_local 6
              i32.wrap/i64
              i32.const 7
              i32.and
              tee_local 3
              i32.const 5
              i32.lt_u
              br_if 0 (;@5;)
              get_local 7
              i64.const 1
              i64.add
              set_local 7
              br 1 (;@4;)
            end
            get_local 3
            i32.const 4
            i32.ne
            br_if 0 (;@4;)
            get_local 7
            get_local 5
            i64.const 1
            i64.and
            i64.add
            set_local 7
          end
          get_local 7
          f64.reinterpret/i64
          set_local 1
          br 2 (;@1;)
        end
        f64.const 0x0p+0 (;=0;)
        return
      end
      get_local 7
      i64.eqz
      i32.eqz
      br_if 0 (;@1;)
      get_local 5
      get_local 4
      i64.and
      f64.reinterpret/i64
      return
    end
    get_local 1)
  (func (;20;) (type 6) (param i64 i64) (result i64)
    (local i32 i32 i32 i32 i32 i32)
    get_local 1
    i64.const 32
    i64.shr_u
    i32.wrap/i64
    get_local 0
    i32.wrap/i64
    tee_local 2
    i32.mul
    get_local 1
    i32.wrap/i64
    tee_local 4
    get_local 0
    i64.const 32
    i64.shr_u
    i32.wrap/i64
    i32.mul
    get_local 2
    i32.const 65535
    i32.and
    tee_local 5
    get_local 4
    i32.const 16
    i32.shr_u
    tee_local 6
    i32.mul
    tee_local 7
    get_local 4
    i32.const 65535
    i32.and
    tee_local 4
    get_local 2
    i32.const 16
    i32.shr_u
    tee_local 3
    i32.mul
    get_local 4
    get_local 5
    i32.mul
    tee_local 4
    i32.const 16
    i32.shr_u
    i32.add
    tee_local 2
    i32.const 65535
    i32.and
    i32.add
    i32.const 16
    i32.shr_u
    get_local 2
    i32.const 16
    i32.shr_u
    i32.add
    get_local 6
    get_local 3
    i32.mul
    i32.add
    i32.add
    i32.add
    i64.extend_u/i32
    i64.const 32
    i64.shl
    get_local 7
    get_local 2
    i32.add
    i32.const 16
    i32.shl
    get_local 4
    i32.const 65535
    i32.and
    i32.or
    i64.extend_u/i32
    i64.or)
  (func (;21;) (type 8) (param i32 i64 i64 i64 i64)
    (local i64 i64 i64 i64 i64)
    get_local 0
    get_local 1
    i64.const 4294967295
    i64.and
    tee_local 7
    get_local 3
    i64.const 32
    i64.shr_u
    tee_local 8
    i64.mul
    tee_local 9
    get_local 3
    i64.const 4294967295
    i64.and
    tee_local 6
    get_local 1
    i64.const 32
    i64.shr_u
    tee_local 5
    i64.mul
    get_local 6
    get_local 7
    i64.mul
    tee_local 6
    i64.const 32
    i64.shr_u
    i64.add
    tee_local 7
    i64.add
    i64.const 32
    i64.shl
    get_local 6
    i64.const 4294967295
    i64.and
    i64.or
    i64.store
    get_local 0
    i32.const 8
    i32.add
    get_local 4
    get_local 1
    i64.mul
    get_local 3
    get_local 2
    i64.mul
    get_local 9
    get_local 7
    i64.const 4294967295
    i64.and
    i64.add
    i64.const 32
    i64.shr_u
    get_local 7
    i64.const 32
    i64.shr_u
    i64.add
    get_local 8
    get_local 5
    i64.mul
    i64.add
    i64.add
    i64.add
    i64.store)
  (func (;22;) (type 2) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    get_local 2
    i32.const 0
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 0
          i32.const -2147483648
          i32.ne
          br_if 0 (;@3;)
          get_local 1
          i32.const 2
          i32.lt_u
          br_if 1 (;@2;)
          get_local 2
          i32.const 1
          i32.store
          get_local 1
          get_local 0
          i32.mul
          return
        end
        block  ;; label = @3
          get_local 1
          i32.const -2147483648
          i32.ne
          br_if 0 (;@3;)
          get_local 0
          i32.const 2
          i32.lt_u
          br_if 1 (;@2;)
          get_local 2
          i32.const 1
          i32.store
          get_local 1
          get_local 0
          i32.mul
          return
        end
        get_local 0
        i32.const 31
        i32.shr_s
        tee_local 3
        get_local 0
        i32.xor
        get_local 3
        i32.sub
        tee_local 4
        i32.const 2
        i32.lt_s
        br_if 0 (;@2;)
        get_local 1
        i32.const 31
        i32.shr_s
        tee_local 5
        get_local 1
        i32.xor
        get_local 5
        i32.sub
        tee_local 6
        i32.const 2
        i32.lt_s
        br_if 0 (;@2;)
        block  ;; label = @3
          get_local 3
          get_local 5
          i32.ne
          br_if 0 (;@3;)
          get_local 6
          i32.eqz
          br_if 2 (;@1;)
          get_local 4
          i32.const 2147483647
          get_local 6
          i32.div_s
          i32.le_s
          br_if 1 (;@2;)
          get_local 2
          i32.const 1
          i32.store
          get_local 1
          get_local 0
          i32.mul
          return
        end
        get_local 6
        i32.eqz
        br_if 1 (;@1;)
        i32.const 0
        get_local 6
        i32.sub
        tee_local 3
        i32.const -1
        i32.eq
        br_if 1 (;@1;)
        get_local 4
        i32.const -2147483648
        get_local 3
        i32.div_s
        i32.le_s
        br_if 0 (;@2;)
        get_local 2
        i32.const 1
        i32.store
      end
      get_local 1
      get_local 0
      i32.mul
      return
    end
    unreachable
    unreachable)
  (func (;23;) (type 7) (param i64 i64 i32) (result i64)
    (local i64 i64 i64 i64)
    get_local 2
    i32.const 0
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 0
          i64.const -9223372036854775808
          i64.ne
          br_if 0 (;@3;)
          get_local 1
          i64.const 2
          i64.lt_u
          br_if 1 (;@2;)
          get_local 2
          i32.const 1
          i32.store
          get_local 1
          get_local 0
          i64.mul
          return
        end
        block  ;; label = @3
          get_local 1
          i64.const -9223372036854775808
          i64.ne
          br_if 0 (;@3;)
          get_local 0
          i64.const 2
          i64.lt_u
          br_if 1 (;@2;)
          get_local 2
          i32.const 1
          i32.store
          get_local 1
          get_local 0
          i64.mul
          return
        end
        get_local 0
        i64.const 63
        i64.shr_s
        tee_local 3
        get_local 0
        i64.xor
        get_local 3
        i64.sub
        tee_local 4
        i64.const 2
        i64.lt_s
        br_if 0 (;@2;)
        get_local 1
        i64.const 63
        i64.shr_s
        tee_local 5
        get_local 1
        i64.xor
        get_local 5
        i64.sub
        tee_local 6
        i64.const 2
        i64.lt_s
        br_if 0 (;@2;)
        block  ;; label = @3
          get_local 3
          get_local 5
          i64.ne
          br_if 0 (;@3;)
          get_local 6
          i64.const 0
          i64.eq
          br_if 2 (;@1;)
          get_local 4
          i64.const 9223372036854775807
          get_local 6
          i64.div_s
          i64.le_s
          br_if 1 (;@2;)
          get_local 2
          i32.const 1
          i32.store
          get_local 1
          get_local 0
          i64.mul
          return
        end
        get_local 6
        i64.eqz
        br_if 1 (;@1;)
        i64.const 0
        get_local 6
        i64.sub
        tee_local 3
        i64.const -1
        i64.eq
        br_if 1 (;@1;)
        get_local 4
        i64.const -9223372036854775808
        get_local 3
        i64.div_s
        i64.le_s
        br_if 0 (;@2;)
        get_local 2
        i32.const 1
        i32.store
      end
      get_local 1
      get_local 0
      i64.mul
      return
    end
    unreachable
    unreachable)
  (func (;24;) (type 9) (param i32 i64 i64 i64 i64 i32)
    (local i32 i64 i64 i64 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 48
    i32.sub
    tee_local 6
    i32.store offset=4
    get_local 6
    i32.const 32
    i32.add
    get_local 3
    get_local 4
    get_local 1
    get_local 2
    call 21
    get_local 5
    i32.const 0
    i32.store
    get_local 6
    i32.const 40
    i32.add
    i64.load
    set_local 8
    get_local 6
    i64.load offset=32
    set_local 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 1
          get_local 2
          i64.const -9223372036854775808
          i64.xor
          i64.or
          i64.const 0
          i64.ne
          br_if 0 (;@3;)
          get_local 3
          i64.const 2
          i64.lt_u
          i32.const 0
          get_local 4
          i64.eqz
          select
          br_if 1 (;@2;)
          get_local 5
          i32.const 1
          i32.store
          br 1 (;@2;)
        end
        block  ;; label = @3
          get_local 3
          get_local 4
          i64.const -9223372036854775808
          i64.xor
          i64.or
          i64.eqz
          i32.eqz
          br_if 0 (;@3;)
          get_local 1
          i64.const 2
          i64.lt_u
          i32.const 0
          get_local 2
          i64.eqz
          select
          br_if 1 (;@2;)
          get_local 5
          i32.const 1
          i32.store
          br 1 (;@2;)
        end
        get_local 2
        i64.const 63
        i64.shr_s
        tee_local 9
        get_local 1
        i64.xor
        tee_local 1
        get_local 9
        i64.sub
        tee_local 10
        i64.const 2
        i64.lt_u
        get_local 9
        get_local 2
        i64.xor
        get_local 9
        i64.sub
        get_local 1
        get_local 9
        i64.lt_u
        i64.extend_u/i32
        i64.sub
        tee_local 1
        i64.const 0
        i64.lt_s
        get_local 1
        i64.eqz
        select
        br_if 0 (;@2;)
        get_local 4
        i64.const 63
        i64.shr_s
        tee_local 2
        get_local 3
        i64.xor
        tee_local 11
        get_local 2
        i64.sub
        tee_local 3
        i64.const 2
        i64.lt_u
        get_local 2
        get_local 4
        i64.xor
        get_local 2
        i64.sub
        get_local 11
        get_local 2
        i64.lt_u
        i64.extend_u/i32
        i64.sub
        tee_local 4
        i64.const 0
        i64.lt_s
        get_local 4
        i64.eqz
        select
        br_if 0 (;@2;)
        block  ;; label = @3
          get_local 9
          get_local 2
          i64.xor
          tee_local 2
          get_local 2
          i64.or
          i64.const 0
          i64.ne
          br_if 0 (;@3;)
          get_local 3
          get_local 4
          i64.or
          i64.const 0
          i64.eq
          br_if 2 (;@1;)
          get_local 6
          i64.const -1
          i64.const 9223372036854775807
          get_local 3
          get_local 4
          call 31
          get_local 10
          get_local 6
          i64.load
          i64.gt_u
          get_local 1
          get_local 6
          i32.const 8
          i32.add
          i64.load
          tee_local 2
          i64.gt_s
          get_local 1
          get_local 2
          i64.eq
          select
          i32.eqz
          br_if 1 (;@2;)
          get_local 5
          i32.const 1
          i32.store
          br 1 (;@2;)
        end
        get_local 3
        get_local 4
        i64.or
        i64.eqz
        br_if 1 (;@1;)
        i64.const 0
        get_local 3
        i64.sub
        tee_local 2
        i64.const 0
        get_local 4
        i64.sub
        get_local 3
        i64.const 0
        i64.ne
        i64.extend_u/i32
        i64.sub
        tee_local 4
        i64.and
        i64.const -1
        i64.eq
        br_if 1 (;@1;)
        get_local 6
        i32.const 16
        i32.add
        i64.const 0
        i64.const -9223372036854775808
        get_local 2
        get_local 4
        call 31
        get_local 10
        get_local 6
        i64.load offset=16
        i64.gt_u
        get_local 1
        get_local 6
        i32.const 24
        i32.add
        i64.load
        tee_local 2
        i64.gt_s
        get_local 1
        get_local 2
        i64.eq
        select
        i32.eqz
        br_if 0 (;@2;)
        get_local 5
        i32.const 1
        i32.store
      end
      get_local 0
      get_local 7
      i64.store
      get_local 0
      i32.const 8
      i32.add
      get_local 8
      i64.store
      i32.const 0
      get_local 6
      i32.const 48
      i32.add
      i32.store offset=4
      return
    end
    unreachable
    unreachable)
  (func (;25;) (type 10) (param f32 i32) (result f32)
    (local i32 f32)
    get_local 0
    f32.const 0x1p+0 (;=1;)
    get_local 1
    i32.const 1
    i32.and
    select
    set_local 3
    block  ;; label = @1
      get_local 1
      i32.const 1
      i32.add
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      get_local 1
      set_local 2
      loop  ;; label = @2
        get_local 3
        get_local 0
        get_local 0
        f32.mul
        tee_local 0
        f32.mul
        get_local 3
        get_local 2
        i32.const 2
        i32.div_s
        tee_local 2
        i32.const 1
        i32.and
        select
        set_local 3
        get_local 2
        i32.const 1
        i32.add
        i32.const 2
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    f32.const 0x1p+0 (;=1;)
    get_local 3
    f32.div
    get_local 3
    get_local 1
    i32.const 0
    i32.lt_s
    select)
  (func (;26;) (type 11) (param f64 i32) (result f64)
    (local i32 f64)
    get_local 0
    f64.const 0x1p+0 (;=1;)
    get_local 1
    i32.const 1
    i32.and
    select
    set_local 3
    block  ;; label = @1
      get_local 1
      i32.const 1
      i32.add
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      get_local 1
      set_local 2
      loop  ;; label = @2
        get_local 3
        get_local 0
        get_local 0
        f64.mul
        tee_local 0
        f64.mul
        get_local 3
        get_local 2
        i32.const 2
        i32.div_s
        tee_local 2
        i32.const 1
        i32.and
        select
        set_local 3
        get_local 2
        i32.const 1
        i32.add
        i32.const 2
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    f64.const 0x1p+0 (;=1;)
    get_local 3
    f64.div
    get_local 3
    get_local 1
    i32.const 0
    i32.lt_s
    select)
  (func (;27;) (type 3) (param f32 f32) (result f32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    get_local 1
    i32.reinterpret/f32
    tee_local 3
    i32.const 8388607
    i32.and
    set_local 7
    get_local 0
    i32.reinterpret/f32
    tee_local 2
    i32.const 8388607
    i32.and
    set_local 6
    get_local 3
    get_local 2
    i32.xor
    i32.const -2147483648
    i32.and
    set_local 11
    get_local 3
    i32.const 23
    i32.shr_u
    i32.const 255
    i32.and
    set_local 5
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 2
          i32.const 23
          i32.shr_u
          i32.const 255
          i32.and
          tee_local 4
          i32.const -1
          i32.add
          i32.const 253
          i32.gt_u
          br_if 0 (;@3;)
          i32.const 0
          set_local 10
          get_local 5
          i32.const -1
          i32.add
          i32.const 254
          i32.lt_u
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          get_local 2
          i32.const 2147483647
          i32.and
          tee_local 8
          i32.const 2139095041
          i32.lt_u
          br_if 0 (;@3;)
          get_local 2
          i32.const 4194304
          i32.or
          f32.reinterpret/i32
          return
        end
        block  ;; label = @3
          get_local 3
          i32.const 2147483647
          i32.and
          tee_local 9
          i32.const 2139095041
          i32.lt_u
          br_if 0 (;@3;)
          get_local 3
          i32.const 4194304
          i32.or
          f32.reinterpret/i32
          return
        end
        block  ;; label = @3
          get_local 8
          i32.const 2139095040
          i32.ne
          br_if 0 (;@3;)
          get_local 3
          i32.const -2147483648
          i32.and
          get_local 2
          i32.xor
          i32.const 2143289344
          get_local 9
          select
          f32.reinterpret/i32
          return
        end
        block  ;; label = @3
          get_local 9
          i32.const 2139095040
          i32.ne
          br_if 0 (;@3;)
          get_local 2
          i32.const -2147483648
          i32.and
          get_local 3
          i32.xor
          i32.const 2143289344
          get_local 8
          select
          f32.reinterpret/i32
          return
        end
        get_local 8
        i32.eqz
        br_if 1 (;@1;)
        get_local 9
        i32.eqz
        br_if 1 (;@1;)
        i32.const 0
        set_local 10
        block  ;; label = @3
          get_local 8
          i32.const 8388607
          i32.gt_u
          br_if 0 (;@3;)
          i32.const 9
          get_local 6
          i32.clz
          tee_local 2
          i32.sub
          set_local 10
          get_local 6
          get_local 2
          i32.const 24
          i32.add
          i32.const 31
          i32.and
          i32.shl
          set_local 6
        end
        get_local 9
        i32.const 8388607
        i32.gt_u
        br_if 0 (;@2;)
        i32.const 9
        get_local 7
        i32.clz
        tee_local 2
        i32.sub
        get_local 10
        i32.add
        set_local 10
        get_local 7
        get_local 2
        i32.const 24
        i32.add
        i32.const 31
        i32.and
        i32.shl
        set_local 7
      end
      get_local 7
      i32.const 8
      i32.shl
      i32.const -2147483648
      i32.or
      i64.extend_u/i32
      get_local 6
      i32.const 8388608
      i32.or
      i64.extend_u/i32
      i64.mul
      tee_local 12
      i32.wrap/i64
      set_local 2
      get_local 4
      get_local 10
      i32.add
      get_local 5
      i32.add
      set_local 3
      block  ;; label = @2
        block  ;; label = @3
          get_local 12
          i64.const 32
          i64.shr_u
          tee_local 12
          i32.wrap/i64
          tee_local 5
          i32.const 8388608
          i32.and
          br_if 0 (;@3;)
          get_local 2
          i32.const 31
          i32.shr_u
          get_local 12
          i32.wrap/i64
          i32.const 1
          i32.shl
          i32.or
          set_local 5
          get_local 2
          i32.const 1
          i32.shl
          set_local 2
          get_local 3
          i32.const -127
          i32.add
          set_local 3
          br 1 (;@2;)
        end
        get_local 3
        i32.const -126
        i32.add
        set_local 3
      end
      block  ;; label = @2
        get_local 3
        i32.const 255
        i32.lt_s
        br_if 0 (;@2;)
        get_local 11
        i32.const 2139095040
        i32.or
        f32.reinterpret/i32
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          get_local 3
          i32.const 0
          i32.le_s
          br_if 0 (;@3;)
          get_local 5
          i32.const 8388607
          i32.and
          get_local 3
          i32.const 23
          i32.shl
          i32.or
          set_local 3
          br 1 (;@2;)
        end
        i32.const 1
        get_local 3
        i32.sub
        tee_local 3
        i32.const 31
        i32.gt_s
        br_if 1 (;@1;)
        get_local 2
        get_local 3
        i32.rotr
        get_local 5
        i32.const 0
        get_local 3
        i32.sub
        i32.const 31
        i32.and
        i32.shl
        i32.or
        set_local 2
        get_local 5
        get_local 3
        i32.const 31
        i32.and
        i32.shr_u
        set_local 3
      end
      get_local 3
      get_local 11
      i32.or
      set_local 11
      block  ;; label = @2
        get_local 2
        i32.const -2147483647
        i32.lt_u
        br_if 0 (;@2;)
        get_local 11
        i32.const 1
        i32.add
        f32.reinterpret/i32
        return
      end
      get_local 2
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      get_local 3
      i32.const 1
      i32.and
      get_local 11
      i32.add
      set_local 11
    end
    get_local 11
    f32.reinterpret/i32)
  (func (;28;) (type 4) (param f64 f64) (result f64)
    (local i32 i32 i64 i64 i64 i64 i64 i64 i64 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 3
    i32.store offset=4
    get_local 1
    i64.reinterpret/f64
    tee_local 5
    i64.const 4503599627370495
    i64.and
    set_local 9
    get_local 0
    i64.reinterpret/f64
    tee_local 4
    i64.const 4503599627370495
    i64.and
    set_local 8
    get_local 5
    get_local 4
    i64.xor
    i64.const -9223372036854775808
    i64.and
    set_local 12
    get_local 5
    i64.const 52
    i64.shr_u
    i64.const 2047
    i64.and
    set_local 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 4
          i64.const 52
          i64.shr_u
          i64.const 2047
          i64.and
          tee_local 6
          i64.const -1
          i64.add
          i64.const 2045
          i64.gt_u
          br_if 0 (;@3;)
          i32.const 0
          set_local 2
          get_local 7
          i64.const -1
          i64.add
          i64.const 2046
          i64.lt_u
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          get_local 4
          i64.const 9223372036854775807
          i64.and
          tee_local 10
          i64.const 9218868437227405313
          i64.lt_u
          br_if 0 (;@3;)
          get_local 4
          i64.const 2251799813685248
          i64.or
          set_local 12
          br 2 (;@1;)
        end
        block  ;; label = @3
          get_local 5
          i64.const 9223372036854775807
          i64.and
          tee_local 11
          i64.const 9218868437227405313
          i64.lt_u
          br_if 0 (;@3;)
          get_local 5
          i64.const 2251799813685248
          i64.or
          set_local 12
          br 2 (;@1;)
        end
        block  ;; label = @3
          get_local 10
          i64.const 9218868437227405312
          i64.ne
          br_if 0 (;@3;)
          get_local 5
          i64.const -9223372036854775808
          i64.and
          get_local 4
          i64.xor
          i64.const 9221120237041090560
          get_local 11
          i64.const 0
          i64.ne
          select
          set_local 12
          br 2 (;@1;)
        end
        block  ;; label = @3
          get_local 11
          i64.const 9218868437227405312
          i64.ne
          br_if 0 (;@3;)
          get_local 4
          i64.const -9223372036854775808
          i64.and
          get_local 5
          i64.xor
          i64.const 9221120237041090560
          get_local 10
          i64.const 0
          i64.ne
          select
          set_local 12
          br 2 (;@1;)
        end
        get_local 10
        i64.eqz
        br_if 1 (;@1;)
        get_local 11
        i64.eqz
        br_if 1 (;@1;)
        i32.const 0
        set_local 2
        block  ;; label = @3
          get_local 10
          i64.const 4503599627370495
          i64.gt_u
          br_if 0 (;@3;)
          i32.const 12
          get_local 8
          i64.clz
          tee_local 4
          i32.wrap/i64
          i32.sub
          set_local 2
          get_local 8
          get_local 4
          i64.const 53
          i64.add
          i64.const 63
          i64.and
          i64.shl
          set_local 8
        end
        get_local 11
        i64.const 4503599627370495
        i64.gt_u
        br_if 0 (;@2;)
        i32.const 12
        get_local 9
        i64.clz
        tee_local 4
        i32.wrap/i64
        i32.sub
        get_local 2
        i32.add
        set_local 2
        get_local 9
        get_local 4
        i64.const 53
        i64.add
        i64.const 63
        i64.and
        i64.shl
        set_local 9
      end
      get_local 3
      get_local 9
      i64.const 11
      i64.shl
      i64.const -9223372036854775808
      i64.or
      i64.const 0
      get_local 8
      i64.const 4503599627370496
      i64.or
      i64.const 0
      call 21
      get_local 6
      i32.wrap/i64
      get_local 2
      i32.add
      get_local 7
      i32.wrap/i64
      i32.add
      set_local 2
      get_local 3
      i64.load
      set_local 4
      block  ;; label = @2
        block  ;; label = @3
          get_local 3
          i32.const 8
          i32.add
          i64.load
          tee_local 5
          i64.const 4503599627370496
          i64.and
          i64.eqz
          i32.eqz
          br_if 0 (;@3;)
          get_local 4
          i64.const 63
          i64.shr_u
          get_local 5
          i64.const 1
          i64.shl
          i64.or
          set_local 5
          get_local 4
          i64.const 1
          i64.shl
          set_local 4
          get_local 2
          i32.const -1023
          i32.add
          set_local 2
          br 1 (;@2;)
        end
        get_local 2
        i32.const -1022
        i32.add
        set_local 2
      end
      block  ;; label = @2
        get_local 2
        i32.const 2047
        i32.lt_s
        br_if 0 (;@2;)
        get_local 12
        i64.const 9218868437227405312
        i64.or
        set_local 12
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          get_local 2
          i32.const 0
          i32.le_s
          br_if 0 (;@3;)
          get_local 5
          i64.const 4503599627370495
          i64.and
          get_local 2
          i64.extend_u/i32
          i64.const 52
          i64.shl
          i64.or
          set_local 5
          br 1 (;@2;)
        end
        i32.const 1
        get_local 2
        i32.sub
        tee_local 2
        i32.const 63
        i32.gt_s
        br_if 1 (;@1;)
        get_local 4
        get_local 2
        i32.const 63
        i32.and
        i64.extend_u/i32
        tee_local 7
        i64.rotr
        get_local 5
        i32.const 0
        get_local 2
        i32.sub
        i32.const 63
        i32.and
        i64.extend_u/i32
        i64.shl
        i64.or
        set_local 4
        get_local 5
        get_local 7
        i64.shr_u
        set_local 5
      end
      get_local 5
      get_local 12
      i64.or
      set_local 12
      block  ;; label = @2
        get_local 4
        i64.const -9223372036854775807
        i64.lt_u
        br_if 0 (;@2;)
        get_local 12
        i64.const 1
        i64.add
        set_local 12
        br 1 (;@1;)
      end
      get_local 4
      i64.const -9223372036854775808
      i64.ne
      br_if 0 (;@1;)
      get_local 5
      i64.const 1
      i64.and
      get_local 12
      i64.add
      set_local 12
    end
    i32.const 0
    get_local 3
    i32.const 16
    i32.add
    i32.store offset=4
    get_local 12
    f64.reinterpret/i64)
  (func (;29;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      get_local 1
      i32.const 31
      i32.shr_s
      tee_local 3
      get_local 1
      i32.xor
      get_local 3
      i32.sub
      tee_local 3
      i32.eqz
      br_if 0 (;@1;)
      get_local 0
      i32.const 31
      i32.shr_s
      tee_local 2
      get_local 0
      i32.xor
      get_local 2
      i32.sub
      get_local 3
      i32.div_u
      get_local 1
      get_local 0
      i32.xor
      i32.const 31
      i32.shr_s
      tee_local 1
      i32.xor
      get_local 1
      i32.sub
      return
    end
    unreachable
    unreachable)
  (func (;30;) (type 6) (param i64 i64) (result i64)
    (local i64 i64)
    block  ;; label = @1
      get_local 1
      i64.const 63
      i64.shr_s
      tee_local 3
      get_local 1
      i64.xor
      get_local 3
      i64.sub
      tee_local 3
      i64.const 0
      i64.eq
      br_if 0 (;@1;)
      get_local 0
      i64.const 63
      i64.shr_s
      tee_local 2
      get_local 0
      i64.xor
      get_local 2
      i64.sub
      get_local 3
      i64.div_u
      get_local 1
      get_local 0
      i64.xor
      i64.const 63
      i64.shr_s
      tee_local 1
      i64.xor
      get_local 1
      i64.sub
      return
    end
    unreachable
    unreachable)
  (func (;31;) (type 8) (param i32 i64 i64 i64 i64)
    (local i32 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 5
    i32.store offset=4
    block  ;; label = @1
      get_local 4
      i64.const 63
      i64.shr_s
      tee_local 7
      get_local 3
      i64.xor
      tee_local 3
      get_local 7
      i64.sub
      tee_local 6
      get_local 7
      get_local 4
      i64.xor
      get_local 7
      i64.sub
      get_local 3
      get_local 7
      i64.lt_u
      i64.extend_u/i32
      i64.sub
      tee_local 3
      i64.or
      i64.const 0
      i64.eq
      br_if 0 (;@1;)
      get_local 5
      get_local 2
      i64.const 63
      i64.shr_s
      tee_local 7
      get_local 1
      i64.xor
      tee_local 1
      get_local 7
      i64.sub
      get_local 7
      get_local 2
      i64.xor
      get_local 7
      i64.sub
      get_local 1
      get_local 7
      i64.lt_u
      i64.extend_u/i32
      i64.sub
      get_local 6
      get_local 3
      call 15
      get_local 0
      get_local 5
      i64.load
      get_local 4
      get_local 2
      i64.xor
      i64.const 63
      i64.shr_s
      tee_local 7
      i64.xor
      tee_local 4
      get_local 7
      i64.sub
      i64.store
      get_local 0
      i32.const 8
      i32.add
      get_local 5
      i32.const 8
      i32.add
      i64.load
      get_local 7
      i64.xor
      get_local 7
      i64.sub
      get_local 4
      get_local 7
      i64.lt_u
      i64.extend_u/i32
      i64.sub
      i64.store
      i32.const 0
      get_local 5
      i32.const 16
      i32.add
      i32.store offset=4
      return
    end
    unreachable
    unreachable)
  (func (;32;) (type 5) (param i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      get_local 1
      i32.const 31
      i32.shr_s
      tee_local 2
      get_local 1
      i32.xor
      get_local 2
      i32.sub
      tee_local 2
      i32.eqz
      br_if 0 (;@1;)
      get_local 0
      i32.const 31
      i32.shr_s
      tee_local 1
      get_local 0
      i32.xor
      get_local 1
      i32.sub
      get_local 2
      i32.rem_u
      get_local 1
      i32.xor
      get_local 1
      i32.sub
      return
    end
    unreachable
    unreachable)
  (func (;33;) (type 6) (param i64 i64) (result i64)
    (local i64)
    block  ;; label = @1
      get_local 1
      i64.const 63
      i64.shr_s
      tee_local 2
      get_local 1
      i64.xor
      get_local 2
      i64.sub
      tee_local 2
      i64.const 0
      i64.eq
      br_if 0 (;@1;)
      get_local 0
      i64.const 63
      i64.shr_s
      tee_local 1
      get_local 0
      i64.xor
      get_local 1
      i64.sub
      get_local 2
      i64.rem_u
      get_local 1
      i64.xor
      get_local 1
      i64.sub
      return
    end
    unreachable
    unreachable)
  (func (;34;) (type 8) (param i32 i64 i64 i64 i64)
    (local i32 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 5
    i32.store offset=4
    block  ;; label = @1
      get_local 4
      i64.const 63
      i64.shr_s
      tee_local 7
      get_local 3
      i64.xor
      tee_local 3
      get_local 7
      i64.sub
      tee_local 6
      get_local 7
      get_local 4
      i64.xor
      get_local 7
      i64.sub
      get_local 3
      get_local 7
      i64.lt_u
      i64.extend_u/i32
      i64.sub
      tee_local 4
      i64.or
      i64.const 0
      i64.eq
      br_if 0 (;@1;)
      get_local 5
      get_local 2
      i64.const 63
      i64.shr_s
      tee_local 7
      get_local 1
      i64.xor
      tee_local 3
      get_local 7
      i64.sub
      get_local 7
      get_local 2
      i64.xor
      get_local 7
      i64.sub
      get_local 3
      get_local 7
      i64.lt_u
      i64.extend_u/i32
      i64.sub
      get_local 6
      get_local 4
      call 17
      get_local 0
      get_local 5
      i64.load
      get_local 7
      i64.xor
      tee_local 4
      get_local 7
      i64.sub
      i64.store
      get_local 0
      i32.const 8
      i32.add
      get_local 5
      i32.const 8
      i32.add
      i64.load
      get_local 7
      i64.xor
      get_local 7
      i64.sub
      get_local 4
      get_local 7
      i64.lt_u
      i64.extend_u/i32
      i64.sub
      i64.store
      i32.const 0
      get_local 5
      i32.const 16
      i32.add
      i32.store offset=4
      return
    end
    unreachable
    unreachable)
  (func (;35;) (type 2) (param i32 i32 i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      get_local 1
      i32.const 31
      i32.shr_s
      tee_local 4
      get_local 1
      i32.xor
      get_local 4
      i32.sub
      tee_local 4
      i32.eqz
      br_if 0 (;@1;)
      get_local 2
      get_local 0
      get_local 0
      i32.const 31
      i32.shr_s
      tee_local 3
      get_local 0
      i32.xor
      get_local 3
      i32.sub
      get_local 4
      i32.div_u
      get_local 1
      get_local 0
      i32.xor
      i32.const 31
      i32.shr_s
      tee_local 4
      i32.xor
      get_local 4
      i32.sub
      tee_local 4
      get_local 1
      i32.mul
      i32.sub
      i32.store
      get_local 4
      return
    end
    unreachable
    unreachable)
  (func (;36;) (type 7) (param i64 i64 i32) (result i64)
    (local i64 i64)
    block  ;; label = @1
      get_local 1
      i64.const 63
      i64.shr_s
      tee_local 4
      get_local 1
      i64.xor
      get_local 4
      i64.sub
      tee_local 4
      i64.const 0
      i64.eq
      br_if 0 (;@1;)
      get_local 2
      get_local 0
      get_local 0
      i64.const 63
      i64.shr_s
      tee_local 3
      get_local 0
      i64.xor
      get_local 3
      i64.sub
      get_local 4
      i64.div_u
      get_local 1
      get_local 0
      i64.xor
      i64.const 63
      i64.shr_s
      tee_local 4
      i64.xor
      get_local 4
      i64.sub
      tee_local 4
      get_local 1
      i64.mul
      i64.sub
      i64.store
      get_local 4
      return
    end
    unreachable
    unreachable)
  (func (;37;) (type 3) (param f32 f32) (result f32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    get_local 1
    i32.reinterpret/f32
    tee_local 3
    i32.const 8388607
    i32.and
    set_local 7
    get_local 0
    i32.reinterpret/f32
    tee_local 2
    i32.const 8388607
    i32.and
    set_local 6
    get_local 3
    get_local 2
    i32.xor
    i32.const -2147483648
    i32.and
    set_local 11
    get_local 3
    i32.const 23
    i32.shr_u
    i32.const 255
    i32.and
    set_local 5
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 2
          i32.const 23
          i32.shr_u
          i32.const 255
          i32.and
          tee_local 4
          i32.const -1
          i32.add
          i32.const 253
          i32.gt_u
          br_if 0 (;@3;)
          i32.const 0
          set_local 10
          get_local 5
          i32.const -1
          i32.add
          i32.const 254
          i32.lt_u
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          get_local 2
          i32.const 2147483647
          i32.and
          tee_local 8
          i32.const 2139095041
          i32.lt_u
          br_if 0 (;@3;)
          get_local 2
          i32.const 4194304
          i32.or
          f32.reinterpret/i32
          return
        end
        block  ;; label = @3
          get_local 3
          i32.const 2147483647
          i32.and
          tee_local 9
          i32.const 2139095041
          i32.lt_u
          br_if 0 (;@3;)
          get_local 3
          i32.const 4194304
          i32.or
          f32.reinterpret/i32
          return
        end
        block  ;; label = @3
          get_local 8
          i32.const 2139095040
          i32.ne
          br_if 0 (;@3;)
          i32.const 2143289344
          get_local 3
          i32.const -2147483648
          i32.and
          get_local 2
          i32.xor
          get_local 9
          i32.const 2139095040
          i32.eq
          select
          f32.reinterpret/i32
          return
        end
        get_local 9
        i32.const 2139095040
        i32.eq
        br_if 1 (;@1;)
        block  ;; label = @3
          block  ;; label = @4
            get_local 8
            i32.eqz
            br_if 0 (;@4;)
            get_local 9
            i32.eqz
            br_if 1 (;@3;)
            i32.const 0
            set_local 10
            block  ;; label = @5
              get_local 8
              i32.const 8388607
              i32.gt_u
              br_if 0 (;@5;)
              i32.const 9
              get_local 6
              i32.clz
              tee_local 2
              i32.sub
              set_local 10
              get_local 6
              get_local 2
              i32.const 24
              i32.add
              i32.const 31
              i32.and
              i32.shl
              set_local 6
            end
            get_local 9
            i32.const 8388607
            i32.gt_u
            br_if 2 (;@2;)
            get_local 10
            get_local 7
            i32.clz
            tee_local 2
            i32.add
            i32.const -9
            i32.add
            set_local 10
            get_local 7
            get_local 2
            i32.const 24
            i32.add
            i32.const 31
            i32.and
            i32.shl
            set_local 7
            br 2 (;@2;)
          end
          get_local 11
          i32.const 2143289344
          get_local 9
          select
          f32.reinterpret/i32
          return
        end
        get_local 11
        i32.const 2139095040
        i32.or
        f32.reinterpret/i32
        return
      end
      block  ;; label = @2
        get_local 10
        get_local 4
        i32.add
        i32.const 127
        i32.add
        get_local 5
        i32.sub
        i32.const -1
        i32.const 0
        i64.const 0
        i64.const 0
        i64.const 0
        i32.const 1963258675
        get_local 7
        i32.const 8388608
        i32.or
        tee_local 3
        i32.const 8
        i32.shl
        tee_local 2
        i32.sub
        i64.extend_u/i32
        tee_local 13
        get_local 2
        i64.extend_u/i32
        tee_local 12
        i64.mul
        i64.const 32
        i64.shr_u
        i64.sub
        i64.const 4294967295
        i64.and
        get_local 13
        i64.mul
        i64.const 31
        i64.shr_u
        i64.const 4294967295
        i64.and
        tee_local 13
        get_local 12
        i64.mul
        i64.const 32
        i64.shr_u
        i64.sub
        i64.const 4294967295
        i64.and
        get_local 13
        i64.mul
        i64.const 31
        i64.shr_u
        i64.const 4294967295
        i64.and
        tee_local 13
        get_local 12
        i64.mul
        i64.const 32
        i64.shr_u
        i64.sub
        i64.const 4294967295
        i64.and
        get_local 13
        i64.mul
        i64.const 31
        i64.shr_u
        i64.const 4294967294
        i64.add
        i64.const 4294967295
        i64.and
        get_local 6
        i32.const 8388608
        i32.or
        tee_local 7
        i32.const 1
        i32.shl
        i64.extend_u/i32
        i64.mul
        i64.const 32
        i64.shr_u
        i32.wrap/i64
        tee_local 5
        i32.const 16777216
        i32.lt_u
        tee_local 6
        select
        i32.add
        tee_local 2
        i32.const 255
        i32.lt_s
        br_if 0 (;@2;)
        get_local 11
        i32.const 2139095040
        i32.or
        f32.reinterpret/i32
        return
      end
      get_local 2
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      get_local 7
      i32.const 24
      i32.const 23
      get_local 6
      select
      i32.shl
      get_local 3
      get_local 5
      get_local 5
      i32.const 16777215
      i32.gt_u
      i32.shr_u
      tee_local 5
      i32.mul
      i32.sub
      i32.const 1
      i32.shl
      get_local 3
      i32.gt_u
      get_local 2
      i32.const 23
      i32.shl
      get_local 5
      i32.const 8388607
      i32.and
      i32.or
      i32.add
      get_local 11
      i32.or
      set_local 11
    end
    get_local 11
    f32.reinterpret/i32)
  (func (;38;) (type 4) (param f64 f64) (result f64)
    (local i32 i32 i32 i64 i64 i64 i64 i64 i64 i64 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 4
    i32.store offset=4
    get_local 1
    i64.reinterpret/f64
    tee_local 6
    i64.const 4503599627370495
    i64.and
    set_local 10
    get_local 0
    i64.reinterpret/f64
    tee_local 5
    i64.const 4503599627370495
    i64.and
    set_local 9
    get_local 6
    get_local 5
    i64.xor
    i64.const -9223372036854775808
    i64.and
    set_local 13
    get_local 6
    i64.const 52
    i64.shr_u
    i64.const 2047
    i64.and
    set_local 8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 5
          i64.const 52
          i64.shr_u
          i64.const 2047
          i64.and
          tee_local 7
          i64.const -1
          i64.add
          i64.const 2045
          i64.gt_u
          br_if 0 (;@3;)
          i32.const 0
          set_local 3
          get_local 8
          i64.const -1
          i64.add
          i64.const 2046
          i64.lt_u
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          get_local 5
          i64.const 9223372036854775807
          i64.and
          tee_local 12
          i64.const 9218868437227405313
          i64.lt_u
          br_if 0 (;@3;)
          get_local 5
          i64.const 2251799813685248
          i64.or
          set_local 13
          br 2 (;@1;)
        end
        block  ;; label = @3
          get_local 6
          i64.const 9223372036854775807
          i64.and
          tee_local 11
          i64.const 9218868437227405313
          i64.lt_u
          br_if 0 (;@3;)
          get_local 6
          i64.const 2251799813685248
          i64.or
          set_local 13
          br 2 (;@1;)
        end
        block  ;; label = @3
          get_local 12
          i64.const 9218868437227405312
          i64.ne
          br_if 0 (;@3;)
          i64.const 9221120237041090560
          get_local 6
          i64.const -9223372036854775808
          i64.and
          get_local 5
          i64.xor
          get_local 11
          i64.const 9218868437227405312
          i64.eq
          select
          set_local 13
          br 2 (;@1;)
        end
        get_local 11
        i64.const 9218868437227405312
        i64.eq
        br_if 1 (;@1;)
        block  ;; label = @3
          block  ;; label = @4
            get_local 12
            i64.const 0
            i64.eq
            br_if 0 (;@4;)
            get_local 11
            i64.const 0
            i64.eq
            br_if 1 (;@3;)
            i32.const 0
            set_local 3
            block  ;; label = @5
              get_local 12
              i64.const 4503599627370495
              i64.gt_u
              br_if 0 (;@5;)
              i32.const 12
              get_local 9
              i64.clz
              tee_local 5
              i32.wrap/i64
              i32.sub
              set_local 3
              get_local 9
              get_local 5
              i64.const 53
              i64.add
              i64.const 63
              i64.and
              i64.shl
              set_local 9
            end
            get_local 11
            i64.const 4503599627370495
            i64.gt_u
            br_if 2 (;@2;)
            get_local 3
            get_local 10
            i64.clz
            tee_local 5
            i32.wrap/i64
            i32.add
            i32.const -12
            i32.add
            set_local 3
            get_local 10
            get_local 5
            i64.const 53
            i64.add
            i64.const 63
            i64.and
            i64.shl
            set_local 10
            br 2 (;@2;)
          end
          i64.const 9221120237041090560
          get_local 13
          get_local 11
          i64.eqz
          select
          set_local 13
          br 2 (;@1;)
        end
        get_local 13
        i64.const 9218868437227405312
        i64.or
        set_local 13
        br 1 (;@1;)
      end
      get_local 4
      i64.const 0
      i64.const 0
      i64.const 0
      i64.const 0
      i64.const 1963258675
      get_local 10
      i64.const 4503599627370496
      i64.or
      tee_local 12
      i64.const 21
      i64.shr_u
      tee_local 5
      i64.sub
      i64.const 4294967295
      i64.and
      tee_local 6
      get_local 5
      i64.const 4294967295
      i64.and
      tee_local 5
      i64.mul
      i64.const 32
      i64.shr_u
      i64.sub
      i64.const 4294967295
      i64.and
      get_local 6
      i64.mul
      i64.const 31
      i64.shr_u
      i64.const 4294967295
      i64.and
      tee_local 6
      get_local 5
      i64.mul
      i64.const 32
      i64.shr_u
      i64.sub
      i64.const 4294967295
      i64.and
      get_local 6
      i64.mul
      i64.const 31
      i64.shr_u
      i64.const 4294967295
      i64.and
      tee_local 6
      get_local 5
      i64.mul
      i64.const 32
      i64.shr_u
      i64.sub
      i64.const 4294967295
      i64.and
      get_local 6
      i64.mul
      i64.const 31
      i64.shr_u
      i64.const 4294967295
      i64.add
      i64.const 4294967295
      i64.and
      tee_local 6
      get_local 5
      i64.mul
      get_local 6
      get_local 10
      i64.const 11
      i64.shl
      i64.const 4294965248
      i64.and
      i64.mul
      i64.const 32
      i64.shr_u
      i64.add
      i64.sub
      tee_local 5
      i64.const 32
      i64.shr_u
      get_local 6
      i64.mul
      get_local 5
      i64.const 4294967295
      i64.and
      get_local 6
      i64.mul
      i64.const 32
      i64.shr_u
      i64.add
      i64.const -2
      i64.add
      i64.const 0
      get_local 9
      i64.const 4503599627370496
      i64.or
      tee_local 6
      i64.const 2
      i64.shl
      i64.const 0
      call 21
      block  ;; label = @2
        get_local 3
        get_local 7
        i32.wrap/i64
        i32.add
        i32.const 1023
        i32.add
        get_local 8
        i32.wrap/i64
        i32.sub
        i32.const -1
        i32.const 0
        get_local 4
        i32.const 8
        i32.add
        i64.load
        tee_local 5
        i64.const 9007199254740992
        i64.lt_u
        tee_local 2
        select
        i32.add
        tee_local 3
        i32.const 2047
        i32.lt_s
        br_if 0 (;@2;)
        get_local 13
        i64.const 9218868437227405312
        i64.or
        set_local 13
        br 1 (;@1;)
      end
      get_local 3
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      get_local 6
      i64.const 53
      i64.const 52
      get_local 2
      select
      i64.shl
      get_local 12
      get_local 5
      get_local 5
      i64.const 9007199254740991
      i64.gt_u
      i64.extend_u/i32
      i64.shr_u
      tee_local 5
      i64.mul
      i64.sub
      i64.const 1
      i64.shl
      get_local 12
      i64.gt_u
      i64.extend_u/i32
      get_local 3
      i64.extend_u/i32
      i64.const 52
      i64.shl
      get_local 5
      i64.const 4503599627370495
      i64.and
      i64.or
      i64.add
      get_local 13
      i64.or
      set_local 13
    end
    i32.const 0
    get_local 4
    i32.const 16
    i32.add
    i32.store offset=4
    get_local 13
    f64.reinterpret/i64)
  (func (;39;) (type 12) (param i64 i32) (result i64)
    (local i32)
    block  ;; label = @1
      get_local 1
      i32.const 32
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        get_local 1
        i32.eqz
        br_if 0 (;@2;)
        get_local 0
        i32.wrap/i64
        tee_local 2
        i32.const 0
        get_local 1
        i32.sub
        i32.const 31
        i32.and
        i32.shr_u
        get_local 0
        i64.const 32
        i64.shr_u
        i32.wrap/i64
        get_local 1
        i32.const 31
        i32.and
        tee_local 1
        i32.shl
        i32.or
        i64.extend_u/i32
        i64.const 32
        i64.shl
        get_local 2
        get_local 1
        i32.shl
        i64.extend_u/i32
        i64.or
        set_local 0
      end
      get_local 0
      return
    end
    get_local 0
    i32.wrap/i64
    get_local 1
    i32.const 31
    i32.and
    i32.shl
    i64.extend_u/i32
    i64.const 32
    i64.shl)
  (func (;40;) (type 13) (param i32 i64 i64 i32)
    (local i64)
    block  ;; label = @1
      block  ;; label = @2
        get_local 3
        i32.const 64
        i32.and
        br_if 0 (;@2;)
        get_local 3
        i32.eqz
        br_if 1 (;@1;)
        get_local 1
        i32.const 0
        get_local 3
        i32.sub
        i32.const 63
        i32.and
        i64.extend_u/i32
        i64.shr_u
        get_local 2
        get_local 3
        i32.const 63
        i32.and
        i64.extend_u/i32
        tee_local 4
        i64.shl
        i64.or
        set_local 2
        get_local 1
        get_local 4
        i64.shl
        set_local 1
        br 1 (;@1;)
      end
      get_local 1
      get_local 3
      i32.const 63
      i32.and
      i64.extend_u/i32
      i64.shl
      set_local 2
      i64.const 0
      set_local 1
    end
    get_local 0
    get_local 1
    i64.store
    get_local 0
    i32.const 8
    i32.add
    get_local 2
    i64.store)
  (func (;41;) (type 12) (param i64 i32) (result i64)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 1
          i32.const 32
          i32.and
          br_if 0 (;@3;)
          get_local 1
          i32.eqz
          br_if 2 (;@1;)
          get_local 0
          i32.wrap/i64
          get_local 1
          i32.const 31
          i32.and
          tee_local 3
          i32.shr_u
          get_local 0
          i64.const 32
          i64.shr_u
          i32.wrap/i64
          tee_local 2
          i32.const 0
          get_local 1
          i32.sub
          i32.const 31
          i32.and
          i32.shl
          i32.or
          set_local 1
          br 1 (;@2;)
        end
        i32.const 31
        set_local 3
        get_local 0
        i64.const 32
        i64.shr_u
        i32.wrap/i64
        tee_local 2
        get_local 1
        i32.const 31
        i32.and
        i32.shr_s
        set_local 1
      end
      get_local 2
      get_local 3
      i32.shr_s
      i64.extend_u/i32
      i64.const 32
      i64.shl
      get_local 1
      i64.extend_u/i32
      i64.or
      set_local 0
    end
    get_local 0)
  (func (;42;) (type 13) (param i32 i64 i64 i32)
    (local i64)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 3
          i32.const 64
          i32.and
          br_if 0 (;@3;)
          get_local 3
          i32.eqz
          br_if 2 (;@1;)
          get_local 1
          get_local 3
          i32.const 63
          i32.and
          i64.extend_u/i32
          tee_local 4
          i64.shr_u
          get_local 2
          i32.const 0
          get_local 3
          i32.sub
          i32.const 63
          i32.and
          i64.extend_u/i32
          i64.shl
          i64.or
          set_local 1
          br 1 (;@2;)
        end
        get_local 2
        get_local 3
        i32.const 63
        i32.and
        i64.extend_u/i32
        i64.shr_s
        set_local 1
        i64.const 63
        set_local 4
      end
      get_local 2
      get_local 4
      i64.shr_s
      set_local 2
    end
    get_local 0
    get_local 1
    i64.store
    get_local 0
    i32.const 8
    i32.add
    get_local 2
    i64.store)
  (func (;43;) (type 12) (param i64 i32) (result i64)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 1
          i32.const 32
          i32.and
          br_if 0 (;@3;)
          get_local 1
          i32.eqz
          br_if 2 (;@1;)
          get_local 0
          i32.wrap/i64
          get_local 1
          i32.const 31
          i32.and
          tee_local 3
          i32.shr_u
          get_local 0
          i64.const 32
          i64.shr_u
          i32.wrap/i64
          tee_local 2
          i32.const 0
          get_local 1
          i32.sub
          i32.const 31
          i32.and
          i32.shl
          i32.or
          set_local 1
          get_local 2
          get_local 3
          i32.shr_u
          i64.extend_u/i32
          i64.const 32
          i64.shl
          set_local 0
          br 1 (;@2;)
        end
        get_local 0
        i64.const 32
        i64.shr_u
        i32.wrap/i64
        get_local 1
        i32.const 31
        i32.and
        i32.shr_u
        set_local 1
        i64.const 0
        set_local 0
      end
      get_local 0
      get_local 1
      i64.extend_u/i32
      i64.or
      set_local 0
    end
    get_local 0)
  (func (;44;) (type 13) (param i32 i64 i64 i32)
    (local i64)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 3
          i32.const 64
          i32.and
          br_if 0 (;@3;)
          get_local 3
          i32.eqz
          br_if 2 (;@1;)
          get_local 1
          get_local 3
          i32.const 63
          i32.and
          i64.extend_u/i32
          tee_local 4
          i64.shr_u
          get_local 2
          i32.const 0
          get_local 3
          i32.sub
          i32.const 63
          i32.and
          i64.extend_u/i32
          i64.shl
          i64.or
          set_local 1
          get_local 2
          get_local 4
          i64.shr_u
          set_local 2
          i64.const 0
          set_local 4
          br 1 (;@2;)
        end
        get_local 2
        get_local 3
        i32.const 63
        i32.and
        i64.extend_u/i32
        i64.shr_u
        set_local 1
        i64.const 0
        set_local 4
        i64.const 0
        set_local 2
      end
      get_local 4
      get_local 1
      i64.or
      set_local 1
    end
    get_local 0
    get_local 1
    i64.store
    get_local 0
    i32.const 8
    i32.add
    get_local 2
    i64.store)
  (func (;45;) (type 14) (param i32) (result f32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              get_local 0
              i32.eqz
              br_if 0 (;@5;)
              i32.const 31
              get_local 0
              get_local 0
              i32.const 31
              i32.shr_s
              tee_local 1
              i32.add
              get_local 1
              i32.xor
              tee_local 1
              i32.clz
              tee_local 2
              i32.sub
              set_local 4
              i32.const 32
              get_local 2
              i32.sub
              tee_local 2
              i32.const 24
              i32.le_u
              br_if 1 (;@4;)
              get_local 2
              i32.const 25
              i32.eq
              br_if 2 (;@3;)
              get_local 2
              i32.const 26
              i32.eq
              br_if 3 (;@2;)
              get_local 1
              i32.const 58
              get_local 2
              i32.sub
              i32.const 31
              i32.and
              i32.shl
              i32.const 0
              i32.ne
              get_local 1
              get_local 2
              i32.const 6
              i32.add
              i32.const 31
              i32.and
              i32.shr_u
              i32.or
              set_local 1
              br 3 (;@2;)
            end
            f32.const 0x0p+0 (;=0;)
            return
          end
          get_local 1
          i32.const 24
          get_local 2
          i32.sub
          i32.const 31
          i32.and
          i32.shl
          set_local 1
          br 2 (;@1;)
        end
        get_local 1
        i32.const 1
        i32.shl
        set_local 1
      end
      get_local 1
      i32.const 2
      i32.shr_u
      i32.const 1
      i32.and
      get_local 1
      i32.or
      i32.const 1
      i32.add
      tee_local 1
      i32.const 3
      i32.shr_u
      get_local 1
      i32.const 2
      i32.shr_u
      tee_local 1
      get_local 1
      i32.const 16777216
      i32.and
      tee_local 3
      select
      set_local 1
      get_local 2
      get_local 4
      get_local 3
      select
      set_local 4
    end
    get_local 4
    i32.const 23
    i32.shl
    i32.const 1065353216
    i32.add
    i32.const 2139095040
    i32.and
    get_local 0
    i32.const -2147483648
    i32.and
    i32.or
    get_local 1
    i32.const 8388607
    i32.and
    i32.or
    f32.reinterpret/i32)
  (func (;46;) (type 15) (param i32) (result f64)
    (local i32 i32)
    block  ;; label = @1
      get_local 0
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1054
      get_local 0
      get_local 0
      i32.const 31
      i32.shr_s
      tee_local 1
      i32.add
      get_local 1
      i32.xor
      tee_local 1
      i32.clz
      tee_local 2
      i32.sub
      i64.extend_u/i32
      i64.const 52
      i64.shl
      get_local 0
      i32.const 31
      i32.shr_u
      i64.extend_u/i32
      i64.const 63
      i64.shl
      i64.or
      get_local 1
      i64.extend_u/i32
      i32.const 117
      i32.const 32
      get_local 2
      i32.sub
      i32.sub
      i32.const 63
      i32.and
      i64.extend_u/i32
      i64.shl
      i64.const 4503599627370495
      i64.and
      i64.or
      f64.reinterpret/i64
      return
    end
    f64.const 0x0p+0 (;=0;))
  (func (;47;) (type 16) (param i64) (result f64)
    (local i32 i32 i32 i64 i64)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              get_local 0
              i64.eqz
              br_if 0 (;@5;)
              i32.const 63
              get_local 0
              get_local 0
              i64.const 63
              i64.shr_s
              tee_local 4
              i64.add
              get_local 4
              i64.xor
              tee_local 4
              i64.clz
              i32.wrap/i64
              tee_local 1
              i32.sub
              set_local 3
              i32.const 64
              get_local 1
              i32.sub
              tee_local 1
              i32.const 53
              i32.le_u
              br_if 1 (;@4;)
              get_local 1
              i32.const 54
              i32.eq
              br_if 2 (;@3;)
              get_local 1
              i32.const 55
              i32.eq
              br_if 3 (;@2;)
              get_local 4
              i32.const 119
              get_local 1
              i32.sub
              i32.const 63
              i32.and
              i64.extend_u/i32
              i64.shl
              i64.const 0
              i64.ne
              i64.extend_u/i32
              get_local 4
              get_local 1
              i32.const 9
              i32.add
              i32.const 63
              i32.and
              i64.extend_u/i32
              i64.shr_u
              i64.or
              set_local 4
              br 3 (;@2;)
            end
            f64.const 0x0p+0 (;=0;)
            return
          end
          get_local 4
          i32.const 53
          get_local 1
          i32.sub
          i32.const 63
          i32.and
          i64.extend_u/i32
          i64.shl
          set_local 4
          br 2 (;@1;)
        end
        get_local 4
        i64.const 1
        i64.shl
        set_local 4
      end
      get_local 4
      i64.const 2
      i64.shr_u
      i64.const 1
      i64.and
      get_local 4
      i64.or
      i64.const 1
      i64.add
      tee_local 4
      i64.const 2
      i64.shr_u
      tee_local 5
      get_local 4
      i64.const 3
      i64.shr_u
      get_local 5
      i64.const 9007199254740992
      i64.and
      i64.eqz
      tee_local 2
      select
      set_local 4
      get_local 3
      get_local 1
      get_local 2
      select
      set_local 3
    end
    get_local 3
    i32.const 1023
    i32.add
    i64.extend_u/i32
    i64.const 52
    i64.shl
    i64.const 9218868437227405312
    i64.and
    get_local 0
    i64.const -9223372036854775808
    i64.and
    i64.or
    get_local 4
    i64.const 4503599627370495
    i64.and
    i64.or
    f64.reinterpret/i64)
  (func (;48;) (type 17) (param i64 i64) (result f32)
    (local i32 i32 i32 i32 i64 i64 f32)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 48
    i32.sub
    tee_local 5
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                get_local 0
                get_local 1
                i64.or
                i64.eqz
                br_if 0 (;@6;)
                i32.const 127
                get_local 1
                get_local 1
                i64.const 63
                i64.shr_s
                tee_local 6
                i64.add
                i64.const 1
                get_local 0
                get_local 6
                i64.add
                tee_local 7
                get_local 0
                i64.lt_u
                i64.extend_u/i32
                get_local 7
                get_local 6
                i64.lt_u
                select
                i64.add
                get_local 6
                i64.xor
                tee_local 0
                i64.clz
                get_local 7
                get_local 6
                i64.xor
                tee_local 6
                i64.clz
                i64.const 64
                i64.add
                get_local 0
                i64.const 0
                i64.ne
                select
                i32.wrap/i64
                tee_local 2
                i32.sub
                set_local 4
                i32.const 128
                get_local 2
                i32.sub
                tee_local 2
                i32.const 24
                i32.le_u
                br_if 1 (;@5;)
                get_local 2
                i32.const 25
                i32.eq
                br_if 2 (;@4;)
                get_local 2
                i32.const 26
                i32.eq
                br_if 3 (;@3;)
                get_local 5
                get_local 6
                get_local 0
                i32.const 154
                get_local 2
                i32.sub
                i32.const 127
                i32.and
                call 40
                get_local 5
                i32.const 16
                i32.add
                get_local 6
                get_local 0
                get_local 2
                i32.const 102
                i32.add
                i32.const 127
                i32.and
                call 44
                get_local 5
                i64.load
                get_local 5
                i32.const 8
                i32.add
                i64.load
                i64.or
                i64.const 0
                i64.ne
                i64.extend_u/i32
                get_local 5
                i64.load offset=16
                i64.or
                set_local 6
                get_local 5
                i32.const 16
                i32.add
                i32.const 8
                i32.add
                i64.load
                set_local 0
                br 3 (;@3;)
              end
              f32.const 0x0p+0 (;=0;)
              set_local 8
              br 4 (;@1;)
            end
            get_local 5
            i32.const 32
            i32.add
            get_local 6
            get_local 0
            i32.const 24
            get_local 2
            i32.sub
            i32.const 127
            i32.and
            call 40
            get_local 5
            i64.load offset=32
            set_local 0
            br 2 (;@2;)
          end
          get_local 0
          i64.const 1
          i64.shl
          get_local 6
          i64.const 63
          i64.shr_u
          i64.or
          set_local 0
          get_local 6
          i64.const 1
          i64.shl
          set_local 6
        end
        get_local 6
        i64.const 2
        i64.shr_u
        i64.const 1
        i64.and
        get_local 6
        i64.or
        tee_local 7
        i64.const 1
        i64.add
        tee_local 6
        i64.const 2
        i64.shr_u
        get_local 0
        i64.const 1
        get_local 6
        get_local 7
        i64.lt_u
        i64.extend_u/i32
        get_local 6
        i64.eqz
        select
        i64.add
        tee_local 0
        i64.const 62
        i64.shl
        i64.or
        tee_local 7
        get_local 6
        i64.const 3
        i64.shr_u
        get_local 0
        i64.const 61
        i64.shl
        i64.or
        get_local 7
        i64.const 16777216
        i64.and
        i64.eqz
        tee_local 3
        select
        set_local 0
        get_local 4
        get_local 2
        get_local 3
        select
        set_local 4
      end
      get_local 4
      i32.const 23
      i32.shl
      i32.const 1065353216
      i32.add
      i32.const 2139095040
      i32.and
      get_local 1
      i64.const 32
      i64.shr_u
      i32.wrap/i64
      i32.const -2147483648
      i32.and
      i32.or
      get_local 0
      i32.wrap/i64
      i32.const 8388607
      i32.and
      i32.or
      f32.reinterpret/i32
      set_local 8
    end
    i32.const 0
    get_local 5
    i32.const 48
    i32.add
    i32.store offset=4
    get_local 8)
  (func (;49;) (type 18) (param i64 i64) (result f64)
    (local i32 i32 i32 i32 i64 i64 f64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 48
    i32.sub
    tee_local 5
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                get_local 0
                get_local 1
                i64.or
                i64.eqz
                br_if 0 (;@6;)
                i32.const 127
                get_local 1
                get_local 1
                i64.const 63
                i64.shr_s
                tee_local 6
                i64.add
                i64.const 1
                get_local 0
                get_local 6
                i64.add
                tee_local 7
                get_local 0
                i64.lt_u
                i64.extend_u/i32
                get_local 7
                get_local 6
                i64.lt_u
                select
                i64.add
                get_local 6
                i64.xor
                tee_local 0
                i64.clz
                get_local 7
                get_local 6
                i64.xor
                tee_local 6
                i64.clz
                i64.const 64
                i64.add
                get_local 0
                i64.const 0
                i64.ne
                select
                i32.wrap/i64
                tee_local 2
                i32.sub
                set_local 4
                i32.const 128
                get_local 2
                i32.sub
                tee_local 2
                i32.const 53
                i32.le_u
                br_if 1 (;@5;)
                get_local 2
                i32.const 54
                i32.eq
                br_if 2 (;@4;)
                get_local 2
                i32.const 55
                i32.eq
                br_if 3 (;@3;)
                get_local 5
                get_local 6
                get_local 0
                i32.const 183
                get_local 2
                i32.sub
                i32.const 127
                i32.and
                call 40
                get_local 5
                i32.const 16
                i32.add
                get_local 6
                get_local 0
                get_local 2
                i32.const 73
                i32.add
                i32.const 127
                i32.and
                call 44
                get_local 5
                i64.load
                get_local 5
                i32.const 8
                i32.add
                i64.load
                i64.or
                i64.const 0
                i64.ne
                i64.extend_u/i32
                get_local 5
                i64.load offset=16
                i64.or
                set_local 6
                get_local 5
                i32.const 16
                i32.add
                i32.const 8
                i32.add
                i64.load
                set_local 0
                br 3 (;@3;)
              end
              f64.const 0x0p+0 (;=0;)
              set_local 8
              br 4 (;@1;)
            end
            get_local 5
            i32.const 32
            i32.add
            get_local 6
            get_local 0
            i32.const 53
            get_local 2
            i32.sub
            i32.const 127
            i32.and
            call 40
            get_local 5
            i64.load offset=32
            set_local 0
            br 2 (;@2;)
          end
          get_local 0
          i64.const 1
          i64.shl
          get_local 6
          i64.const 63
          i64.shr_u
          i64.or
          set_local 0
          get_local 6
          i64.const 1
          i64.shl
          set_local 6
        end
        get_local 6
        i64.const 2
        i64.shr_u
        i64.const 1
        i64.and
        get_local 6
        i64.or
        tee_local 7
        i64.const 1
        i64.add
        tee_local 6
        i64.const 2
        i64.shr_u
        get_local 0
        i64.const 1
        get_local 6
        get_local 7
        i64.lt_u
        i64.extend_u/i32
        get_local 6
        i64.eqz
        select
        i64.add
        tee_local 0
        i64.const 62
        i64.shl
        i64.or
        tee_local 7
        get_local 6
        i64.const 3
        i64.shr_u
        get_local 0
        i64.const 61
        i64.shl
        i64.or
        get_local 7
        i64.const 9007199254740992
        i64.and
        i64.eqz
        tee_local 3
        select
        set_local 0
        get_local 4
        get_local 2
        get_local 3
        select
        set_local 4
      end
      get_local 4
      i32.const 1023
      i32.add
      i64.extend_u/i32
      i64.const 52
      i64.shl
      i64.const 9218868437227405312
      i64.and
      get_local 1
      i64.const -9223372036854775808
      i64.and
      i64.or
      get_local 0
      i64.const 4503599627370495
      i64.and
      i64.or
      f64.reinterpret/i64
      set_local 8
    end
    i32.const 0
    get_local 5
    i32.const 48
    i32.add
    i32.store offset=4
    get_local 8)
  (func (;50;) (type 14) (param i32) (result f32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              get_local 0
              i32.eqz
              br_if 0 (;@5;)
              i32.const 31
              get_local 0
              i32.clz
              tee_local 1
              i32.sub
              set_local 3
              i32.const 32
              get_local 1
              i32.sub
              tee_local 1
              i32.const 24
              i32.le_u
              br_if 1 (;@4;)
              get_local 1
              i32.const 25
              i32.eq
              br_if 2 (;@3;)
              get_local 1
              i32.const 26
              i32.eq
              br_if 3 (;@2;)
              get_local 0
              i32.const 58
              get_local 1
              i32.sub
              i32.const 31
              i32.and
              i32.shl
              i32.const 0
              i32.ne
              get_local 0
              get_local 1
              i32.const 6
              i32.add
              i32.const 31
              i32.and
              i32.shr_u
              i32.or
              set_local 0
              br 3 (;@2;)
            end
            f32.const 0x0p+0 (;=0;)
            return
          end
          get_local 0
          i32.const 24
          get_local 1
          i32.sub
          i32.const 31
          i32.and
          i32.shl
          set_local 0
          br 2 (;@1;)
        end
        get_local 0
        i32.const 1
        i32.shl
        set_local 0
      end
      get_local 0
      i32.const 2
      i32.shr_u
      i32.const 1
      i32.and
      get_local 0
      i32.or
      i32.const 1
      i32.add
      tee_local 0
      i32.const 3
      i32.shr_u
      get_local 0
      i32.const 2
      i32.shr_u
      tee_local 0
      get_local 0
      i32.const 16777216
      i32.and
      tee_local 2
      select
      set_local 0
      get_local 1
      get_local 3
      get_local 2
      select
      set_local 3
    end
    get_local 3
    i32.const 23
    i32.shl
    i32.const 1065353216
    i32.add
    i32.const 2139095040
    i32.and
    get_local 0
    i32.const 8388607
    i32.and
    i32.or
    f32.reinterpret/i32)
  (func (;51;) (type 15) (param i32) (result f64)
    (local i32)
    block  ;; label = @1
      get_local 0
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1054
      get_local 0
      i32.clz
      tee_local 1
      i32.sub
      i64.extend_u/i32
      i64.const 52
      i64.shl
      get_local 0
      i64.extend_u/i32
      i32.const 117
      i32.const 32
      get_local 1
      i32.sub
      i32.sub
      i32.const 63
      i32.and
      i64.extend_u/i32
      i64.shl
      i64.const 4503599627370495
      i64.and
      i64.or
      f64.reinterpret/i64
      return
    end
    f64.const 0x0p+0 (;=0;))
  (func (;52;) (type 16) (param i64) (result f64)
    (local i32 i32 i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              get_local 0
              i64.eqz
              br_if 0 (;@5;)
              i32.const 63
              get_local 0
              i64.clz
              i32.wrap/i64
              tee_local 1
              i32.sub
              set_local 3
              i32.const 64
              get_local 1
              i32.sub
              tee_local 1
              i32.const 53
              i32.le_u
              br_if 1 (;@4;)
              get_local 1
              i32.const 54
              i32.eq
              br_if 2 (;@3;)
              get_local 1
              i32.const 55
              i32.eq
              br_if 3 (;@2;)
              get_local 0
              i32.const 119
              get_local 1
              i32.sub
              i32.const 63
              i32.and
              i64.extend_u/i32
              i64.shl
              i64.const 0
              i64.ne
              i64.extend_u/i32
              get_local 0
              get_local 1
              i32.const 9
              i32.add
              i32.const 63
              i32.and
              i64.extend_u/i32
              i64.shr_u
              i64.or
              set_local 0
              br 3 (;@2;)
            end
            f64.const 0x0p+0 (;=0;)
            return
          end
          get_local 0
          i32.const 53
          get_local 1
          i32.sub
          i32.const 63
          i32.and
          i64.extend_u/i32
          i64.shl
          set_local 0
          br 2 (;@1;)
        end
        get_local 0
        i64.const 1
        i64.shl
        set_local 0
      end
      get_local 0
      i64.const 2
      i64.shr_u
      i64.const 1
      i64.and
      get_local 0
      i64.or
      i64.const 1
      i64.add
      tee_local 0
      i64.const 2
      i64.shr_u
      tee_local 4
      get_local 0
      i64.const 3
      i64.shr_u
      get_local 4
      i64.const 9007199254740992
      i64.and
      i64.eqz
      tee_local 2
      select
      set_local 0
      get_local 3
      get_local 1
      get_local 2
      select
      set_local 3
    end
    get_local 3
    i32.const 1023
    i32.add
    i64.extend_u/i32
    i64.const 52
    i64.shl
    i64.const 9218868437227405312
    i64.and
    get_local 0
    i64.const 4503599627370495
    i64.and
    i64.or
    f64.reinterpret/i64)
  (func (;53;) (type 17) (param i64 i64) (result f32)
    (local i32 i32 i32 i32 i64 f32)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 48
    i32.sub
    tee_local 5
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                get_local 0
                get_local 1
                i64.or
                i64.eqz
                br_if 0 (;@6;)
                i32.const 127
                get_local 1
                i64.clz
                get_local 0
                i64.clz
                i64.const 64
                i64.add
                get_local 1
                i64.const 0
                i64.ne
                select
                i32.wrap/i64
                tee_local 2
                i32.sub
                set_local 4
                i32.const 128
                get_local 2
                i32.sub
                tee_local 2
                i32.const 24
                i32.le_u
                br_if 1 (;@5;)
                get_local 2
                i32.const 25
                i32.eq
                br_if 2 (;@4;)
                get_local 2
                i32.const 26
                i32.eq
                br_if 3 (;@3;)
                get_local 5
                get_local 0
                get_local 1
                i32.const 154
                get_local 2
                i32.sub
                i32.const 127
                i32.and
                call 40
                get_local 5
                i32.const 16
                i32.add
                get_local 0
                get_local 1
                get_local 2
                i32.const 102
                i32.add
                i32.const 127
                i32.and
                call 44
                get_local 5
                i64.load
                get_local 5
                i32.const 8
                i32.add
                i64.load
                i64.or
                i64.const 0
                i64.ne
                i64.extend_u/i32
                get_local 5
                i64.load offset=16
                i64.or
                set_local 0
                get_local 5
                i32.const 16
                i32.add
                i32.const 8
                i32.add
                i64.load
                set_local 1
                br 3 (;@3;)
              end
              f32.const 0x0p+0 (;=0;)
              set_local 7
              br 4 (;@1;)
            end
            get_local 5
            i32.const 32
            i32.add
            get_local 0
            get_local 1
            i32.const 24
            get_local 2
            i32.sub
            i32.const 127
            i32.and
            call 40
            get_local 5
            i64.load offset=32
            set_local 1
            br 2 (;@2;)
          end
          get_local 1
          i64.const 1
          i64.shl
          get_local 0
          i64.const 63
          i64.shr_u
          i64.or
          set_local 1
          get_local 0
          i64.const 1
          i64.shl
          set_local 0
        end
        get_local 0
        i64.const 2
        i64.shr_u
        i64.const 1
        i64.and
        get_local 0
        i64.or
        tee_local 6
        i64.const 1
        i64.add
        tee_local 0
        i64.const 2
        i64.shr_u
        get_local 1
        i64.const 1
        get_local 0
        get_local 6
        i64.lt_u
        i64.extend_u/i32
        get_local 0
        i64.eqz
        select
        i64.add
        tee_local 1
        i64.const 62
        i64.shl
        i64.or
        tee_local 6
        get_local 0
        i64.const 3
        i64.shr_u
        get_local 1
        i64.const 61
        i64.shl
        i64.or
        get_local 6
        i64.const 16777216
        i64.and
        i64.eqz
        tee_local 3
        select
        set_local 1
        get_local 4
        get_local 2
        get_local 3
        select
        set_local 4
      end
      get_local 4
      i32.const 23
      i32.shl
      i32.const 1065353216
      i32.add
      i32.const 2139095040
      i32.and
      get_local 1
      i32.wrap/i64
      i32.const 8388607
      i32.and
      i32.or
      f32.reinterpret/i32
      set_local 7
    end
    i32.const 0
    get_local 5
    i32.const 48
    i32.add
    i32.store offset=4
    get_local 7)
  (func (;54;) (type 18) (param i64 i64) (result f64)
    (local i32 i32 i32 i32 i64 f64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 48
    i32.sub
    tee_local 5
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                get_local 0
                get_local 1
                i64.or
                i64.eqz
                br_if 0 (;@6;)
                i32.const 127
                get_local 1
                i64.clz
                get_local 0
                i64.clz
                i64.const 64
                i64.add
                get_local 1
                i64.const 0
                i64.ne
                select
                i32.wrap/i64
                tee_local 2
                i32.sub
                set_local 4
                i32.const 128
                get_local 2
                i32.sub
                tee_local 2
                i32.const 53
                i32.le_u
                br_if 1 (;@5;)
                get_local 2
                i32.const 54
                i32.eq
                br_if 2 (;@4;)
                get_local 2
                i32.const 55
                i32.eq
                br_if 3 (;@3;)
                get_local 5
                get_local 0
                get_local 1
                i32.const 183
                get_local 2
                i32.sub
                i32.const 127
                i32.and
                call 40
                get_local 5
                i32.const 16
                i32.add
                get_local 0
                get_local 1
                get_local 2
                i32.const 73
                i32.add
                i32.const 127
                i32.and
                call 44
                get_local 5
                i64.load
                get_local 5
                i32.const 8
                i32.add
                i64.load
                i64.or
                i64.const 0
                i64.ne
                i64.extend_u/i32
                get_local 5
                i64.load offset=16
                i64.or
                set_local 0
                get_local 5
                i32.const 16
                i32.add
                i32.const 8
                i32.add
                i64.load
                set_local 1
                br 3 (;@3;)
              end
              f64.const 0x0p+0 (;=0;)
              set_local 7
              br 4 (;@1;)
            end
            get_local 5
            i32.const 32
            i32.add
            get_local 0
            get_local 1
            i32.const 53
            get_local 2
            i32.sub
            i32.const 127
            i32.and
            call 40
            get_local 5
            i64.load offset=32
            set_local 1
            br 2 (;@2;)
          end
          get_local 1
          i64.const 1
          i64.shl
          get_local 0
          i64.const 63
          i64.shr_u
          i64.or
          set_local 1
          get_local 0
          i64.const 1
          i64.shl
          set_local 0
        end
        get_local 0
        i64.const 2
        i64.shr_u
        i64.const 1
        i64.and
        get_local 0
        i64.or
        tee_local 6
        i64.const 1
        i64.add
        tee_local 0
        i64.const 2
        i64.shr_u
        get_local 1
        i64.const 1
        get_local 0
        get_local 6
        i64.lt_u
        i64.extend_u/i32
        get_local 0
        i64.eqz
        select
        i64.add
        tee_local 1
        i64.const 62
        i64.shl
        i64.or
        tee_local 6
        get_local 0
        i64.const 3
        i64.shr_u
        get_local 1
        i64.const 61
        i64.shl
        i64.or
        get_local 6
        i64.const 9007199254740992
        i64.and
        i64.eqz
        tee_local 3
        select
        set_local 1
        get_local 4
        get_local 2
        get_local 3
        select
        set_local 4
      end
      get_local 4
      i32.const 1023
      i32.add
      i64.extend_u/i32
      i64.const 52
      i64.shl
      i64.const 9218868437227405312
      i64.and
      get_local 1
      i64.const 4503599627370495
      i64.and
      i64.or
      f64.reinterpret/i64
      set_local 7
    end
    i32.const 0
    get_local 5
    i32.const 48
    i32.add
    i32.store offset=4
    get_local 7)
  (func (;55;) (type 19) (param f32) (result i32)
    (local i32 i32 i32 i32)
    i32.const 0
    set_local 4
    block  ;; label = @1
      block  ;; label = @2
        get_local 0
        i32.reinterpret/f32
        tee_local 1
        i32.const 2139095040
        i32.and
        tee_local 3
        i32.const 1065353216
        i32.lt_u
        br_if 0 (;@2;)
        get_local 3
        i32.const 23
        i32.shr_u
        tee_local 4
        i32.const -127
        i32.add
        tee_local 3
        i32.const 30
        i32.le_u
        br_if 1 (;@1;)
        get_local 1
        i32.const 31
        i32.shr_u
        i32.const 2147483647
        i32.add
        set_local 4
      end
      get_local 4
      return
    end
    get_local 1
    i32.const 8388607
    i32.and
    i32.const 8388608
    i32.or
    set_local 2
    block  ;; label = @1
      block  ;; label = @2
        get_local 3
        i32.const 22
        i32.gt_u
        br_if 0 (;@2;)
        get_local 2
        i32.const 150
        get_local 4
        i32.sub
        i32.const 31
        i32.and
        i32.shr_u
        set_local 4
        br 1 (;@1;)
      end
      get_local 2
      get_local 4
      i32.const 10
      i32.add
      i32.const 31
      i32.and
      i32.shl
      set_local 4
    end
    get_local 4
    i32.const 0
    get_local 4
    i32.sub
    get_local 1
    i32.const -1
    i32.gt_s
    select)
  (func (;56;) (type 20) (param f32) (result i64)
    (local i32 i32 i32 i32 i64)
    i64.const 0
    set_local 5
    block  ;; label = @1
      block  ;; label = @2
        get_local 0
        i32.reinterpret/f32
        tee_local 1
        i32.const 2139095040
        i32.and
        tee_local 4
        i32.const 1065353216
        i32.lt_u
        br_if 0 (;@2;)
        get_local 4
        i32.const 23
        i32.shr_u
        tee_local 4
        i32.const -127
        i32.add
        tee_local 3
        i32.const 62
        i32.le_u
        br_if 1 (;@1;)
        i64.const 9223372036854775807
        i64.const -9223372036854775808
        get_local 1
        i32.const -1
        i32.gt_s
        select
        set_local 5
      end
      get_local 5
      return
    end
    get_local 1
    i32.const 8388607
    i32.and
    i32.const 8388608
    i32.or
    set_local 2
    block  ;; label = @1
      block  ;; label = @2
        get_local 3
        i32.const 22
        i32.gt_u
        br_if 0 (;@2;)
        get_local 2
        i32.const 150
        get_local 4
        i32.sub
        i32.const 31
        i32.and
        i32.shr_u
        i64.extend_u/i32
        set_local 5
        br 1 (;@1;)
      end
      get_local 2
      i64.extend_u/i32
      get_local 4
      i32.const 42
      i32.add
      i32.const 63
      i32.and
      i64.extend_u/i32
      i64.shl
      set_local 5
    end
    get_local 5
    i64.const 0
    get_local 5
    i64.sub
    get_local 1
    i32.const -1
    i32.gt_s
    select)
  (func (;57;) (type 21) (param i32 f32)
    (local i32 i32 i32 i32 i32 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 6
    i32.store offset=4
    i64.const 0
    set_local 7
    i64.const 0
    set_local 8
    block  ;; label = @1
      block  ;; label = @2
        get_local 1
        i32.reinterpret/f32
        tee_local 2
        i32.const 2139095040
        i32.and
        tee_local 5
        i32.const 1065353216
        i32.lt_u
        br_if 0 (;@2;)
        get_local 5
        i32.const 23
        i32.shr_u
        tee_local 5
        i32.const -127
        i32.add
        tee_local 4
        i32.const 126
        i32.le_u
        br_if 1 (;@1;)
        i64.const 9223372036854775807
        i64.const -9223372036854775808
        get_local 2
        i32.const -1
        i32.gt_s
        tee_local 2
        select
        set_local 8
        i64.const -1
        i64.const 0
        get_local 2
        select
        set_local 7
      end
      get_local 0
      get_local 7
      i64.store
      get_local 0
      i32.const 8
      i32.add
      get_local 8
      i64.store
      i32.const 0
      get_local 6
      i32.const 16
      i32.add
      i32.store offset=4
      return
    end
    get_local 2
    i32.const 8388607
    i32.and
    i32.const 8388608
    i32.or
    set_local 3
    block  ;; label = @1
      block  ;; label = @2
        get_local 4
        i32.const 22
        i32.gt_u
        br_if 0 (;@2;)
        get_local 3
        i32.const 150
        get_local 5
        i32.sub
        i32.const 31
        i32.and
        i32.shr_u
        i64.extend_u/i32
        set_local 7
        i64.const 0
        set_local 8
        br 1 (;@1;)
      end
      get_local 6
      get_local 3
      i64.extend_u/i32
      i64.const 0
      get_local 5
      i32.const 106
      i32.add
      i32.const 127
      i32.and
      call 40
      get_local 6
      i32.const 8
      i32.add
      i64.load
      set_local 8
      get_local 6
      i64.load
      set_local 7
    end
    get_local 0
    get_local 7
    i64.const 0
    get_local 7
    i64.sub
    get_local 2
    i32.const -1
    i32.gt_s
    tee_local 2
    select
    i64.store
    get_local 0
    i32.const 8
    i32.add
    get_local 8
    i64.const 0
    get_local 8
    i64.sub
    get_local 7
    i64.const 0
    i64.ne
    i64.extend_u/i32
    i64.sub
    get_local 2
    select
    i64.store
    i32.const 0
    get_local 6
    i32.const 16
    i32.add
    i32.store offset=4)
  (func (;58;) (type 22) (param f64) (result i32)
    (local i32 i32 i64 i64)
    i32.const 0
    set_local 2
    block  ;; label = @1
      block  ;; label = @2
        get_local 0
        i64.reinterpret/f64
        tee_local 3
        i64.const 52
        i64.shr_u
        tee_local 4
        i32.wrap/i64
        i32.const 2047
        i32.and
        tee_local 1
        i32.const 1023
        i32.lt_u
        br_if 0 (;@2;)
        get_local 1
        i32.const -1023
        i32.add
        i32.const 30
        i32.le_u
        br_if 1 (;@1;)
        i32.const 2147483647
        i32.const -2147483648
        get_local 3
        i64.const -1
        i64.gt_s
        select
        set_local 2
      end
      get_local 2
      return
    end
    get_local 3
    i64.const 4503599627370495
    i64.and
    i64.const 4503599627370496
    i64.or
    i64.const 1075
    get_local 4
    i64.sub
    i64.const 63
    i64.and
    i64.shr_u
    i32.wrap/i64
    tee_local 2
    i32.const 0
    get_local 2
    i32.sub
    get_local 3
    i64.const -1
    i64.gt_s
    select)
  (func (;59;) (type 23) (param f64) (result i64)
    (local i32 i64 i64 i64)
    i64.const 0
    set_local 4
    block  ;; label = @1
      block  ;; label = @2
        get_local 0
        i64.reinterpret/f64
        tee_local 2
        i64.const 52
        i64.shr_u
        tee_local 3
        i32.wrap/i64
        i32.const 2047
        i32.and
        tee_local 1
        i32.const 1023
        i32.lt_u
        br_if 0 (;@2;)
        get_local 1
        i32.const -1023
        i32.add
        tee_local 1
        i32.const 62
        i32.le_u
        br_if 1 (;@1;)
        get_local 2
        i64.const 63
        i64.shr_u
        i64.const 9223372036854775807
        i64.add
        set_local 4
      end
      get_local 4
      return
    end
    get_local 2
    i64.const 4503599627370495
    i64.and
    i64.const 4503599627370496
    i64.or
    set_local 4
    block  ;; label = @1
      block  ;; label = @2
        get_local 1
        i32.const 51
        i32.gt_u
        br_if 0 (;@2;)
        get_local 4
        i64.const 1075
        get_local 3
        i64.sub
        i64.const 63
        i64.and
        i64.shr_u
        set_local 4
        br 1 (;@1;)
      end
      get_local 4
      get_local 3
      i64.const 13
      i64.add
      i64.const 63
      i64.and
      i64.shl
      set_local 4
    end
    get_local 4
    i64.const 0
    get_local 4
    i64.sub
    get_local 2
    i64.const -1
    i64.gt_s
    select)
  (func (;60;) (type 24) (param i32 f64)
    (local i32 i32 i32 i64 i64 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 4
    i32.store offset=4
    i64.const 0
    set_local 7
    i64.const 0
    set_local 8
    block  ;; label = @1
      block  ;; label = @2
        get_local 1
        i64.reinterpret/f64
        tee_local 5
        i64.const 52
        i64.shr_u
        tee_local 6
        i32.wrap/i64
        tee_local 2
        i32.const 2047
        i32.and
        tee_local 3
        i32.const 1023
        i32.lt_u
        br_if 0 (;@2;)
        get_local 3
        i32.const -1023
        i32.add
        tee_local 3
        i32.const 126
        i32.le_u
        br_if 1 (;@1;)
        i64.const 9223372036854775807
        i64.const -9223372036854775808
        get_local 5
        i64.const -1
        i64.gt_s
        tee_local 3
        select
        set_local 8
        i64.const -1
        i64.const 0
        get_local 3
        select
        set_local 7
      end
      get_local 0
      get_local 7
      i64.store
      get_local 0
      i32.const 8
      i32.add
      get_local 8
      i64.store
      i32.const 0
      get_local 4
      i32.const 16
      i32.add
      i32.store offset=4
      return
    end
    get_local 5
    i64.const 4503599627370495
    i64.and
    i64.const 4503599627370496
    i64.or
    set_local 7
    block  ;; label = @1
      block  ;; label = @2
        get_local 3
        i32.const 51
        i32.gt_u
        br_if 0 (;@2;)
        get_local 7
        i64.const 1075
        get_local 6
        i64.sub
        i64.const 63
        i64.and
        i64.shr_u
        set_local 7
        i64.const 0
        set_local 8
        br 1 (;@1;)
      end
      get_local 4
      get_local 7
      i64.const 0
      get_local 2
      i32.const 77
      i32.add
      i32.const 127
      i32.and
      call 40
      get_local 4
      i32.const 8
      i32.add
      i64.load
      set_local 8
      get_local 4
      i64.load
      set_local 7
    end
    get_local 0
    get_local 7
    i64.const 0
    get_local 7
    i64.sub
    get_local 5
    i64.const -1
    i64.gt_s
    tee_local 3
    select
    i64.store
    get_local 0
    i32.const 8
    i32.add
    get_local 8
    i64.const 0
    get_local 8
    i64.sub
    get_local 7
    i64.const 0
    i64.ne
    i64.extend_u/i32
    i64.sub
    get_local 3
    select
    i64.store
    i32.const 0
    get_local 4
    i32.const 16
    i32.add
    i32.store offset=4)
  (func (;61;) (type 19) (param f32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    set_local 3
    block  ;; label = @1
      get_local 0
      i32.reinterpret/f32
      tee_local 1
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      get_local 1
      i32.const 2139095040
      i32.and
      tee_local 2
      i32.const 1065353216
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        get_local 2
        i32.const 23
        i32.shr_u
        tee_local 3
        i32.const -127
        i32.add
        tee_local 2
        i32.const 31
        i32.le_u
        br_if 0 (;@2;)
        get_local 1
        i32.const 31
        i32.shr_s
        i32.const -1
        i32.xor
        return
      end
      get_local 1
      i32.const 8388607
      i32.and
      i32.const 8388608
      i32.or
      set_local 1
      block  ;; label = @2
        get_local 2
        i32.const 22
        i32.gt_u
        br_if 0 (;@2;)
        get_local 1
        i32.const 150
        get_local 3
        i32.sub
        i32.const 31
        i32.and
        i32.shr_u
        return
      end
      get_local 1
      get_local 3
      i32.const 10
      i32.add
      i32.const 31
      i32.and
      i32.shl
      set_local 3
    end
    get_local 3)
  (func (;62;) (type 20) (param f32) (result i64)
    (local i32 i32 i32 i64)
    i64.const 0
    set_local 4
    block  ;; label = @1
      get_local 0
      i32.reinterpret/f32
      tee_local 1
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      get_local 1
      i32.const 2139095040
      i32.and
      tee_local 3
      i32.const 1065353216
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        get_local 3
        i32.const 23
        i32.shr_u
        tee_local 3
        i32.const -127
        i32.add
        tee_local 2
        i32.const 63
        i32.le_u
        br_if 0 (;@2;)
        get_local 1
        i32.const 31
        i32.shr_s
        i64.extend_s/i32
        i64.const -1
        i64.xor
        return
      end
      get_local 1
      i32.const 8388607
      i32.and
      i32.const 8388608
      i32.or
      set_local 1
      block  ;; label = @2
        get_local 2
        i32.const 22
        i32.gt_u
        br_if 0 (;@2;)
        get_local 1
        i32.const 150
        get_local 3
        i32.sub
        i32.const 31
        i32.and
        i32.shr_u
        i64.extend_u/i32
        return
      end
      get_local 1
      i64.extend_u/i32
      get_local 3
      i32.const 42
      i32.add
      i32.const 63
      i32.and
      i64.extend_u/i32
      i64.shl
      set_local 4
    end
    get_local 4)
  (func (;63;) (type 21) (param i32 f32)
    (local i32 i32 i32 i32 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 5
    i32.store offset=4
    i64.const 0
    set_local 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 1
          i32.reinterpret/f32
          tee_local 2
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          get_local 2
          i32.const 2139095040
          i32.and
          tee_local 4
          i32.const 1065353216
          i32.lt_u
          br_if 0 (;@3;)
          get_local 4
          i32.const 23
          i32.shr_u
          tee_local 4
          i32.const -127
          i32.add
          tee_local 3
          i32.const 127
          i32.le_u
          br_if 1 (;@2;)
          get_local 2
          i32.const 31
          i32.shr_s
          i64.extend_s/i32
          i64.const -1
          i64.xor
          tee_local 6
          set_local 7
          br 2 (;@1;)
        end
        i64.const 0
        set_local 6
        br 1 (;@1;)
      end
      get_local 2
      i32.const 8388607
      i32.and
      i32.const 8388608
      i32.or
      set_local 2
      block  ;; label = @2
        get_local 3
        i32.const 22
        i32.gt_u
        br_if 0 (;@2;)
        get_local 2
        i32.const 150
        get_local 4
        i32.sub
        i32.const 31
        i32.and
        i32.shr_u
        i64.extend_u/i32
        set_local 6
        br 1 (;@1;)
      end
      get_local 5
      get_local 2
      i64.extend_u/i32
      i64.const 0
      get_local 4
      i32.const 106
      i32.add
      i32.const 127
      i32.and
      call 40
      get_local 5
      i32.const 8
      i32.add
      i64.load
      set_local 7
      get_local 5
      i64.load
      set_local 6
    end
    get_local 0
    get_local 6
    i64.store
    get_local 0
    i32.const 8
    i32.add
    get_local 7
    i64.store
    i32.const 0
    get_local 5
    i32.const 16
    i32.add
    i32.store offset=4)
  (func (;64;) (type 22) (param f64) (result i32)
    (local i32 i32 i64 i64)
    i32.const 0
    set_local 2
    block  ;; label = @1
      get_local 0
      i64.reinterpret/f64
      tee_local 3
      i64.const 0
      i64.lt_s
      br_if 0 (;@1;)
      get_local 3
      i64.const 52
      i64.shr_u
      tee_local 4
      i32.wrap/i64
      i32.const 2047
      i32.and
      tee_local 1
      i32.const 1023
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        get_local 1
        i32.const -1023
        i32.add
        i32.const 31
        i32.le_u
        br_if 0 (;@2;)
        get_local 3
        i64.const 63
        i64.shr_s
        i32.wrap/i64
        i32.const -1
        i32.xor
        return
      end
      get_local 3
      i64.const 4503599627370495
      i64.and
      i64.const 4503599627370496
      i64.or
      i64.const 1075
      get_local 4
      i64.sub
      i64.const 63
      i64.and
      i64.shr_u
      i32.wrap/i64
      set_local 2
    end
    get_local 2)
  (func (;65;) (type 23) (param f64) (result i64)
    (local i32 i64 i64 i64)
    i64.const 0
    set_local 4
    block  ;; label = @1
      get_local 0
      i64.reinterpret/f64
      tee_local 2
      i64.const 0
      i64.lt_s
      br_if 0 (;@1;)
      get_local 2
      i64.const 52
      i64.shr_u
      tee_local 3
      i32.wrap/i64
      i32.const 2047
      i32.and
      tee_local 1
      i32.const 1023
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        get_local 1
        i32.const -1023
        i32.add
        tee_local 1
        i32.const 63
        i32.le_u
        br_if 0 (;@2;)
        get_local 2
        i64.const 63
        i64.shr_s
        i64.const -1
        i64.xor
        return
      end
      get_local 2
      i64.const 4503599627370495
      i64.and
      i64.const 4503599627370496
      i64.or
      set_local 2
      block  ;; label = @2
        get_local 1
        i32.const 51
        i32.gt_u
        br_if 0 (;@2;)
        get_local 2
        i64.const 1075
        get_local 3
        i64.sub
        i64.const 63
        i64.and
        i64.shr_u
        return
      end
      get_local 2
      get_local 3
      i64.const 13
      i64.add
      i64.const 63
      i64.and
      i64.shl
      set_local 4
    end
    get_local 4)
  (func (;66;) (type 24) (param i32 f64)
    (local i32 i32 i32 i64 i64 i64)
    i32.const 0
    i32.const 0
    i32.load offset=4
    i32.const 16
    i32.sub
    tee_local 4
    i32.store offset=4
    i64.const 0
    set_local 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          get_local 1
          i64.reinterpret/f64
          tee_local 5
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          get_local 5
          i64.const 52
          i64.shr_u
          tee_local 6
          i32.wrap/i64
          tee_local 2
          i32.const 2047
          i32.and
          tee_local 3
          i32.const 1023
          i32.lt_u
          br_if 0 (;@3;)
          get_local 3
          i32.const -1023
          i32.add
          tee_local 3
          i32.const 127
          i32.le_u
          br_if 1 (;@2;)
          get_local 5
          i64.const 63
          i64.shr_s
          i64.const -1
          i64.xor
          tee_local 5
          set_local 7
          br 2 (;@1;)
        end
        i64.const 0
        set_local 5
        br 1 (;@1;)
      end
      get_local 5
      i64.const 4503599627370495
      i64.and
      i64.const 4503599627370496
      i64.or
      set_local 5
      block  ;; label = @2
        get_local 3
        i32.const 51
        i32.gt_u
        br_if 0 (;@2;)
        get_local 5
        i64.const 1075
        get_local 6
        i64.sub
        i64.const 63
        i64.and
        i64.shr_u
        set_local 5
        br 1 (;@1;)
      end
      get_local 4
      get_local 5
      i64.const 0
      get_local 2
      i32.const 77
      i32.add
      i32.const 127
      i32.and
      call 40
      get_local 4
      i32.const 8
      i32.add
      i64.load
      set_local 7
      get_local 4
      i64.load
      set_local 5
    end
    get_local 0
    get_local 5
    i64.store
    get_local 0
    i32.const 8
    i32.add
    get_local 7
    i64.store
    i32.const 0
    get_local 4
    i32.const 16
    i32.add
    i32.store offset=4)
  (func (;67;) (type 0)
    call 0)
  (table (;0;) 0 anyfunc)
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "main" (func 0))
  (export "test" (func 1))
  (export "rust_eh_personality" (func 2))
  (export "memcpy" (func 3))
  (export "memmove" (func 4))
  (export "memset" (func 5))
  (export "memcmp" (func 6))
  (export "__subsf3" (func 7))
  (export "__subdf3" (func 8))
  (export "__udivsi3" (func 9))
  (export "__umodsi3" (func 10))
  (export "__udivmodsi4" (func 11))
  (export "__udivdi3" (func 12))
  (export "__udivmoddi4" (func 13))
  (export "__umoddi3" (func 14))
  (export "__udivti3" (func 15))
  (export "__udivmodti4" (func 16))
  (export "__umodti3" (func 17))
  (export "__addsf3" (func 18))
  (export "__adddf3" (func 19))
  (export "__muldi3" (func 20))
  (export "__multi3" (func 21))
  (export "__mulosi4" (func 22))
  (export "__mulodi4" (func 23))
  (export "__muloti4" (func 24))
  (export "__powisf2" (func 25))
  (export "__powidf2" (func 26))
  (export "__mulsf3" (func 27))
  (export "__muldf3" (func 28))
  (export "__divsi3" (func 29))
  (export "__divdi3" (func 30))
  (export "__divti3" (func 31))
  (export "__modsi3" (func 32))
  (export "__moddi3" (func 33))
  (export "__modti3" (func 34))
  (export "__divmodsi4" (func 35))
  (export "__divmoddi4" (func 36))
  (export "__divsf3" (func 37))
  (export "__divdf3" (func 38))
  (export "__ashldi3" (func 39))
  (export "__ashlti3" (func 40))
  (export "__ashrdi3" (func 41))
  (export "__ashrti3" (func 42))
  (export "__lshrdi3" (func 43))
  (export "__lshrti3" (func 44))
  (export "__floatsisf" (func 45))
  (export "__floatsidf" (func 46))
  (export "__floatdidf" (func 47))
  (export "__floattisf" (func 48))
  (export "__floattidf" (func 49))
  (export "__floatunsisf" (func 50))
  (export "__floatunsidf" (func 51))
  (export "__floatundidf" (func 52))
  (export "__floatuntisf" (func 53))
  (export "__floatuntidf" (func 54))
  (export "__fixsfsi" (func 55))
  (export "__fixsfdi" (func 56))
  (export "__fixsfti" (func 57))
  (export "__fixdfsi" (func 58))
  (export "__fixdfdi" (func 59))
  (export "__fixdfti" (func 60))
  (export "__fixunssfsi" (func 61))
  (export "__fixunssfdi" (func 62))
  (export "__fixunssfti" (func 63))
  (export "__fixunsdfsi" (func 64))
  (export "__fixunsdfdi" (func 65))
  (export "__fixunsdfti" (func 66))
  (start 67)
  (data (i32.const 4) "\10\00\10\00"))
