target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"

%Object = type { i8 }

@typeId0 = external global i8
@typeId1 = external global i8
@typeId2 = external global i8
@typeId3 = external global i8
@tableOffset0 = external global i8
@biasedInstanceId = external global i8
@tableReferenceBias = external global i8
@unoptimizableOne = external global i8
@runtimeExceptionTypeInfo = external global i8
@functionDefMutableDatas0 = external global i8
@functionDefMutableDatas1 = external global i8
@functionDefMutableDatas2 = external global i8
@functionDefMutableDatas3 = external global i8
@functionDefMutableDatas4 = external global i8
@functionDefMutableDatas5 = external global i8

declare i32 @__gxx_personality_v0()

define fastcc { i8*, {} } @functionDef0(i8*) #0 prefix [4 x i64] [i64 1, i64 ptrtoint (i8* @functionDefMutableDatas0 to i64), i64 sub (i64 ptrtoint (i8* @biasedInstanceId to i64), i64 1), i64 ptrtoint (i8* @typeId2 to i64)] personality i32 ()* @__gxx_personality_v0 !dbg !3 {
entry:
  %context = alloca i8*
  store i8* %0, i8** %context
  %1 = load i8*, i8** %context
  %2 = ptrtoint i8* %1 to i64
  %3 = and i64 %2, -2147483648
  %4 = inttoptr i64 %3 to i8*
  %5 = load i8*, i8** %context, !dbg !5
  %6 = call fastcc { i8*, { i32 } } @functionDef4(i8* %5, i32 42, i32 7), !dbg !5
  %7 = extractvalue { i8*, { i32 } } %6, 0, !dbg !5
  store i8* %7, i8** %context, !dbg !5
  %8 = load i8*, i8** %context, !dbg !5
  %9 = ptrtoint i8* %8 to i64, !dbg !5
  %10 = and i64 %9, -2147483648, !dbg !5
  %11 = inttoptr i64 %10 to i8*, !dbg !5
  %12 = extractvalue { i8*, { i32 } } %6, 1, 0, !dbg !5
  %13 = load i8*, i8** %context, !dbg !6
  %14 = call fastcc { i8*, {} } @functionDef1(i8* %13, i32 %12), !dbg !6
  %15 = extractvalue { i8*, {} } %14, 0, !dbg !6
  store i8* %15, i8** %context, !dbg !6
  %16 = load i8*, i8** %context, !dbg !6
  %17 = ptrtoint i8* %16 to i64, !dbg !6
  %18 = and i64 %17, -2147483648, !dbg !6
  %19 = inttoptr i64 %18 to i8*, !dbg !6
  br label %return, !dbg !7

return:                                           ; preds = %entry
  %20 = load i8*, i8** %context, !dbg !7
  %21 = insertvalue { i8*, {} } zeroinitializer, i8* %20, 0, !dbg !7
  ret { i8*, {} } %21, !dbg !7
}

define fastcc { i8*, {} } @functionDef1(i8*, i32) #0 prefix [4 x i64] [i64 1, i64 ptrtoint (i8* @functionDefMutableDatas1 to i64), i64 sub (i64 ptrtoint (i8* @biasedInstanceId to i64), i64 1), i64 ptrtoint (i8* @typeId3 to i64)] personality i32 ()* @__gxx_personality_v0 !dbg !8 {
entry:
  %context = alloca i8*
  store i8* %0, i8** %context
  %2 = load i8*, i8** %context
  %3 = ptrtoint i8* %2 to i64
  %4 = and i64 %3, -2147483648
  %5 = inttoptr i64 %4 to i8*
  %6 = alloca i32
  store i32 %1, i32* %6
  %7 = load i32, i32* %6, !dbg !12
  %8 = load i32, i32* %6, !dbg !13
  %9 = load i8*, i8** %context, !dbg !14
  %10 = ptrtoint i8* %9 to i64, !dbg !14
  %11 = and i64 %10, -2147483648, !dbg !14
  %12 = inttoptr i64 %11 to i8*, !dbg !14
  %13 = getelementptr inbounds i8, i8* %12, i64 ptrtoint (i8* @tableOffset0 to i64), !dbg !14
  %14 = getelementptr inbounds i8, i8* %13, i64 0, !dbg !14
  %15 = bitcast i8* %14 to i64**, !dbg !14
  %16 = load i64*, i64** %15, align 8, !dbg !14
  %17 = getelementptr inbounds i8, i8* %13, i64 8, !dbg !14
  %18 = bitcast i8* %17 to i64*, !dbg !14
  %19 = load i64, i64* %18, align 8, !dbg !14
  %20 = icmp ult i64 1, %19, !dbg !14
  %21 = select i1 %20, i64 1, i64 %19, !dbg !14
  %22 = getelementptr inbounds i64, i64* %16, i64 %21, !dbg !14
  %23 = load atomic i64, i64* %22 acquire, align 8, !dbg !14
  %24 = add i64 %23, ptrtoint (i8* @tableReferenceBias to i64), !dbg !14
  %25 = inttoptr i64 %24 to i8*, !dbg !14
  %26 = getelementptr inbounds i8, i8* %25, i64 24, !dbg !14
  %27 = bitcast i8* %26 to i64*, !dbg !14
  %28 = load i64, i64* %27, align 8, !dbg !14
  %29 = icmp ne i64 ptrtoint (i8* @typeId0 to i64), %28, !dbg !14
  %30 = bitcast i8* %25 to %Object*, !dbg !14
  br i1 %29, label %callIndirectFailTrap, label %callIndirectFailSkip, !dbg !14, !prof !15

callIndirectFailTrap:                             ; preds = %entry
  %31 = load i8*, i8** %context, !dbg !14
  call void @callIndirectFail(i8* %31, i64 1, i64 udiv exact (i64 sub (i64 ptrtoint (i8* @tableOffset0 to i64), i64 6128), i64 16), %Object* %30, i64 ptrtoint (i8* @typeId0 to i64)), !dbg !14
  unreachable, !dbg !14

callIndirectFailSkip:                             ; preds = %entry
  %32 = getelementptr inbounds i8, i8* %25, i64 32, !dbg !14
  %33 = bitcast i8* %32 to { i8*, { i32 } } (i8*, i32, i32)*, !dbg !14
  %34 = load i8*, i8** %context, !dbg !14
  %35 = call fastcc { i8*, { i32 } } %33(i8* %34, i32 %7, i32 %8), !dbg !14
  %36 = extractvalue { i8*, { i32 } } %35, 0, !dbg !14
  store i8* %36, i8** %context, !dbg !14
  %37 = load i8*, i8** %context, !dbg !14
  %38 = ptrtoint i8* %37 to i64, !dbg !14
  %39 = and i64 %38, -2147483648, !dbg !14
  %40 = inttoptr i64 %39 to i8*, !dbg !14
  %41 = extractvalue { i8*, { i32 } } %35, 1, 0, !dbg !14
  %42 = load i32, i32* %6, !dbg !16
  %43 = load i8*, i8** %context, !dbg !17
  %44 = ptrtoint i8* %43 to i64, !dbg !17
  %45 = and i64 %44, -2147483648, !dbg !17
  %46 = inttoptr i64 %45 to i8*, !dbg !17
  %47 = getelementptr inbounds i8, i8* %46, i64 ptrtoint (i8* @tableOffset0 to i64), !dbg !17
  %48 = getelementptr inbounds i8, i8* %47, i64 0, !dbg !17
  %49 = bitcast i8* %48 to i64**, !dbg !17
  %50 = load i64*, i64** %49, align 8, !dbg !17
  %51 = getelementptr inbounds i8, i8* %47, i64 8, !dbg !17
  %52 = bitcast i8* %51 to i64*, !dbg !17
  %53 = load i64, i64* %52, align 8, !dbg !17
  %54 = icmp ult i64 0, %53, !dbg !17
  %55 = select i1 %54, i64 0, i64 %53, !dbg !17
  %56 = getelementptr inbounds i64, i64* %50, i64 %55, !dbg !17
  %57 = load atomic i64, i64* %56 acquire, align 8, !dbg !17
  %58 = add i64 %57, ptrtoint (i8* @tableReferenceBias to i64), !dbg !17
  %59 = inttoptr i64 %58 to i8*, !dbg !17
  %60 = getelementptr inbounds i8, i8* %59, i64 24, !dbg !17
  %61 = bitcast i8* %60 to i64*, !dbg !17
  %62 = load i64, i64* %61, align 8, !dbg !17
  %63 = icmp ne i64 ptrtoint (i8* @typeId1 to i64), %62, !dbg !17
  %64 = bitcast i8* %59 to %Object*, !dbg !17
  br i1 %63, label %callIndirectFailTrap1, label %callIndirectFailSkip2, !dbg !17, !prof !15

callIndirectFailTrap1:                            ; preds = %callIndirectFailSkip
  %65 = load i8*, i8** %context, !dbg !17
  call void @callIndirectFail(i8* %65, i64 0, i64 udiv exact (i64 sub (i64 ptrtoint (i8* @tableOffset0 to i64), i64 6128), i64 16), %Object* %64, i64 ptrtoint (i8* @typeId1 to i64)), !dbg !17
  unreachable, !dbg !17

callIndirectFailSkip2:                            ; preds = %callIndirectFailSkip
  %66 = getelementptr inbounds i8, i8* %59, i64 32, !dbg !17
  %67 = bitcast i8* %66 to { i8*, { i32 } } (i8*, i32)*, !dbg !17
  %68 = load i8*, i8** %context, !dbg !17
  %69 = call fastcc { i8*, { i32 } } %67(i8* %68, i32 %42), !dbg !17
  %70 = extractvalue { i8*, { i32 } } %69, 0, !dbg !17
  store i8* %70, i8** %context, !dbg !17
  %71 = load i8*, i8** %context, !dbg !17
  %72 = ptrtoint i8* %71 to i64, !dbg !17
  %73 = and i64 %72, -2147483648, !dbg !17
  %74 = inttoptr i64 %73 to i8*, !dbg !17
  %75 = extractvalue { i8*, { i32 } } %69, 1, 0, !dbg !17
  br label %return, !dbg !18

return:                                           ; preds = %callIndirectFailSkip2
  %76 = load i8*, i8** %context, !dbg !18
  %77 = insertvalue { i8*, {} } zeroinitializer, i8* %76, 0, !dbg !18
  ret { i8*, {} } %77, !dbg !18
}

define fastcc { i8*, { i32 } } @functionDef2(i8*, i32) #0 prefix [4 x i64] [i64 1, i64 ptrtoint (i8* @functionDefMutableDatas2 to i64), i64 sub (i64 ptrtoint (i8* @biasedInstanceId to i64), i64 1), i64 ptrtoint (i8* @typeId1 to i64)] personality i32 ()* @__gxx_personality_v0 !dbg !19 {
entry:
  %context = alloca i8*
  store i8* %0, i8** %context
  %2 = load i8*, i8** %context
  %3 = ptrtoint i8* %2 to i64
  %4 = and i64 %3, -2147483648
  %5 = inttoptr i64 %4 to i8*
  %6 = alloca i32
  store i32 %1, i32* %6
  %7 = load i32, i32* %6, !dbg !20
  %8 = add i32 1, %7, !dbg !21
  br label %return, !dbg !22

return:                                           ; preds = %entry
  %9 = phi i32 [ %8, %entry ]
  %10 = load i8*, i8** %context, !dbg !22
  %11 = insertvalue { i8*, { i32 } } zeroinitializer, i8* %10, 0, !dbg !22
  %12 = insertvalue { i8*, { i32 } } %11, i32 %9, 1, 0, !dbg !22
  ret { i8*, { i32 } } %12, !dbg !22
}

define fastcc { i8*, { i32 } } @functionDef3(i8*, i32, i32) #0 prefix [4 x i64] [i64 1, i64 ptrtoint (i8* @functionDefMutableDatas3 to i64), i64 sub (i64 ptrtoint (i8* @biasedInstanceId to i64), i64 1), i64 ptrtoint (i8* @typeId0 to i64)] personality i32 ()* @__gxx_personality_v0 !dbg !23 {
entry:
  %context = alloca i8*
  store i8* %0, i8** %context
  %3 = load i8*, i8** %context
  %4 = ptrtoint i8* %3 to i64
  %5 = and i64 %4, -2147483648
  %6 = inttoptr i64 %5 to i8*
  %7 = alloca i32
  store i32 %1, i32* %7
  %8 = alloca i32
  store i32 %2, i32* %8
  %9 = load i32, i32* %7, !dbg !26
  %10 = load i32, i32* %8, !dbg !27
  %11 = add i32 %9, %10, !dbg !28
  br label %return, !dbg !29

return:                                           ; preds = %entry
  %12 = phi i32 [ %11, %entry ]
  %13 = load i8*, i8** %context, !dbg !29
  %14 = insertvalue { i8*, { i32 } } zeroinitializer, i8* %13, 0, !dbg !29
  %15 = insertvalue { i8*, { i32 } } %14, i32 %12, 1, 0, !dbg !29
  ret { i8*, { i32 } } %15, !dbg !29
}

define fastcc { i8*, { i32 } } @functionDef4(i8*, i32, i32) #0 prefix [4 x i64] [i64 1, i64 ptrtoint (i8* @functionDefMutableDatas4 to i64), i64 sub (i64 ptrtoint (i8* @biasedInstanceId to i64), i64 1), i64 ptrtoint (i8* @typeId0 to i64)] personality i32 ()* @__gxx_personality_v0 !dbg !30 {
entry:
  %context = alloca i8*
  store i8* %0, i8** %context
  %3 = load i8*, i8** %context
  %4 = ptrtoint i8* %3 to i64
  %5 = and i64 %4, -2147483648
  %6 = inttoptr i64 %5 to i8*
  %7 = alloca i32
  store i32 %1, i32* %7
  %8 = alloca i32
  store i32 %2, i32* %8
  %9 = load i32, i32* %7, !dbg !31
  %10 = load i32, i32* %8, !dbg !32
  %11 = sub i32 %9, %10, !dbg !33
  br label %return, !dbg !34

return:                                           ; preds = %entry
  %12 = phi i32 [ %11, %entry ]
  %13 = load i8*, i8** %context, !dbg !34
  %14 = insertvalue { i8*, { i32 } } zeroinitializer, i8* %13, 0, !dbg !34
  %15 = insertvalue { i8*, { i32 } } %14, i32 %12, 1, 0, !dbg !34
  ret { i8*, { i32 } } %15, !dbg !34
}

define fastcc { i8*, {} } @functionDef5(i8*) #0 prefix [4 x i64] [i64 1, i64 ptrtoint (i8* @functionDefMutableDatas5 to i64), i64 sub (i64 ptrtoint (i8* @biasedInstanceId to i64), i64 1), i64 ptrtoint (i8* @typeId2 to i64)] personality i32 ()* @__gxx_personality_v0 !dbg !35 {
entry:
  %context = alloca i8*
  store i8* %0, i8** %context
  %1 = load i8*, i8** %context
  %2 = ptrtoint i8* %1 to i64
  %3 = and i64 %2, -2147483648
  %4 = inttoptr i64 %3 to i8*
  br label %return, !dbg !36

return:                                           ; preds = %entry
  %5 = load i8*, i8** %context, !dbg !36
  %6 = insertvalue { i8*, {} } zeroinitializer, i8* %5, 0, !dbg !36
  ret { i8*, {} } %6, !dbg !36
}

declare void @callIndirectFail(i8*, i64, i64, %Object*, i64)

attributes #0 = { "no-frame-pointer-elim"="true" "probe-stack"="wavm_probe_stack" }

!llvm.dbg.cu = !{!0}

!0 = distinct !DICompileUnit(language: 65535, file: !1, producer: "WAVM", isOptimized: true, runtimeVersion: 0, emissionKind: LineTablesOnly, enums: !2, nameTableKind: None)
!1 = !DIFile(filename: "unknown", directory: "unknown")
!2 = !{}
!3 = distinct !DISubprogram(name: "functionDef0", linkageName: "functionDef0", scope: !1, file: !1, type: !4, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !2)
!4 = !DISubroutineType(types: !2)
!5 = !DILocation(line: 2, scope: !3)
!6 = !DILocation(line: 3, scope: !3)
!7 = !DILocation(line: 4, scope: !3)
!8 = distinct !DISubprogram(name: "functionDef1", linkageName: "functionDef1", scope: !1, file: !1, type: !9, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !2)
!9 = !DISubroutineType(types: !10)
!10 = !{!11}
!11 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!12 = !DILocation(line: 0, scope: !8)
!13 = !DILocation(line: 1, scope: !8)
!14 = !DILocation(line: 3, scope: !8)
!15 = distinct !{!"branch_weights", i32 0, i32 2147483647}
!16 = !DILocation(line: 5, scope: !8)
!17 = !DILocation(line: 7, scope: !8)
!18 = !DILocation(line: 9, scope: !8)
!19 = distinct !DISubprogram(name: "functionDef2", linkageName: "functionDef2", scope: !1, file: !1, type: !9, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !2)
!20 = !DILocation(line: 1, scope: !19)
!21 = !DILocation(line: 2, scope: !19)
!22 = !DILocation(line: 3, scope: !19)
!23 = distinct !DISubprogram(name: "functionDef3", linkageName: "functionDef3", scope: !1, file: !1, type: !24, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !2)
!24 = !DISubroutineType(types: !25)
!25 = !{!11, !11}
!26 = !DILocation(line: 0, scope: !23)
!27 = !DILocation(line: 1, scope: !23)
!28 = !DILocation(line: 2, scope: !23)
!29 = !DILocation(line: 3, scope: !23)
!30 = distinct !DISubprogram(name: "functionDef4", linkageName: "functionDef4", scope: !1, file: !1, type: !24, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !2)
!31 = !DILocation(line: 0, scope: !30)
!32 = !DILocation(line: 1, scope: !30)
!33 = !DILocation(line: 2, scope: !30)
!34 = !DILocation(line: 3, scope: !30)
!35 = distinct !DISubprogram(name: "functionDef5", linkageName: "functionDef5", scope: !1, file: !1, type: !4, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !2)
!36 = !DILocation(line: 1, scope: !35)
