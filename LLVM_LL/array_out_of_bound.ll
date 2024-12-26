; ModuleID = 'array_out_of_bound.c'
source_filename = "array_out_of_bound.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@.src = private unnamed_addr constant [21 x i8] c"array_out_of_bound.c\00", align 1
@0 = private unnamed_addr constant { i16, i16, [10 x i8] } { i16 -1, i16 0, [10 x i8] c"'int[10]'\00" }
@1 = private unnamed_addr constant { i16, i16, [6 x i8] } { i16 0, i16 11, [6 x i8] c"'int'\00" }
@2 = private unnamed_addr global { { [21 x i8]*, i32, i32 }, { i16, i16, [10 x i8] }*, { i16, i16, [6 x i8] }* } { { [21 x i8]*, i32, i32 } { [21 x i8]* @.src, i32 5, i32 5 }, { i16, i16, [10 x i8] }* @0, { i16, i16, [6 x i8] }* @1 }
@3 = private unnamed_addr global { { [21 x i8]*, i32, i32 } } { { [21 x i8]*, i32, i32 } { [21 x i8]* @.src, i32 5, i32 5 } }
@4 = private unnamed_addr global { { [21 x i8]*, i32, i32 }, { i16, i16, [6 x i8] }*, i8, i8 } { { [21 x i8]*, i32, i32 } { [21 x i8]* @.src, i32 5, i32 5 }, { i16, i16, [6 x i8] }* @1, i8 3, i8 1 }

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 !dbg !10 {
  %1 = alloca i32, align 4
  %2 = alloca [10 x i32], align 16
  store i32 0, i32* %1, align 4
  call void @llvm.dbg.declare(metadata [10 x i32]* %2, metadata !15, metadata !DIExpression()), !dbg !19
  br i1 false, label %4, label %3, !dbg !20, !prof !21, !nosanitize !14

3:                                                ; preds = %0
  call void @__ubsan_handle_out_of_bounds(i8* bitcast ({ { [21 x i8]*, i32, i32 }, { i16, i16, [10 x i8] }*, { i16, i16, [6 x i8] }* }* @2 to i8*), i64 10) #3, !dbg !20, !nosanitize !14
  br label %4, !dbg !20, !nosanitize !14

4:                                                ; preds = %3, %0
  %5 = getelementptr inbounds [10 x i32], [10 x i32]* %2, i64 0, i64 10, !dbg !20
  %6 = ptrtoint [10 x i32]* %2 to i64, !dbg !20, !nosanitize !14
  %7 = add i64 %6, 40, !dbg !20, !nosanitize !14
  %8 = icmp ne [10 x i32]* %2, null, !dbg !20, !nosanitize !14
  %9 = icmp ne i64 %7, 0, !dbg !20, !nosanitize !14
  %10 = and i1 %8, %9, !dbg !20, !nosanitize !14
  %11 = icmp uge i64 %7, %6, !dbg !20, !nosanitize !14
  %12 = icmp ult i64 %7, %6, !dbg !20, !nosanitize !14
  %13 = select i1 true, i1 %11, i1 %12, !dbg !20, !nosanitize !14
  %14 = and i1 %13, true, !dbg !20, !nosanitize !14
  %15 = and i1 %10, %14, !dbg !20, !nosanitize !14
  br i1 %15, label %17, label %16, !dbg !20, !prof !21, !nosanitize !14

16:                                               ; preds = %4
  call void @__ubsan_handle_pointer_overflow(i8* bitcast ({ { [21 x i8]*, i32, i32 } }* @3 to i8*), i64 %6, i64 %7) #3, !dbg !20, !nosanitize !14
  br label %17, !dbg !20, !nosanitize !14

17:                                               ; preds = %16, %4
  %18 = icmp ne i32* %5, null, !dbg !20, !nosanitize !14
  %19 = ptrtoint i32* %5 to i64, !dbg !20, !nosanitize !14
  %20 = and i64 %19, 7, !dbg !20, !nosanitize !14
  %21 = icmp eq i64 %20, 0, !dbg !20, !nosanitize !14
  %22 = and i1 %18, %21, !dbg !20, !nosanitize !14
  br i1 %22, label %24, label %23, !dbg !20, !prof !21, !nosanitize !14

23:                                               ; preds = %17
  call void @__ubsan_handle_type_mismatch_v1(i8* bitcast ({ { [21 x i8]*, i32, i32 }, { i16, i16, [6 x i8] }*, i8, i8 }* @4 to i8*), i64 %19) #3, !dbg !20, !nosanitize !14
  br label %24, !dbg !20, !nosanitize !14

24:                                               ; preds = %23, %17
  store i32 0, i32* %5, align 8, !dbg !20
  ret i32 0, !dbg !22
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: uwtable
declare void @__ubsan_handle_out_of_bounds(i8*, i64) #2

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.sadd.with.overflow.i64(i64, i64) #1

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.smul.with.overflow.i64(i64, i64) #1

; Function Attrs: uwtable
declare void @__ubsan_handle_pointer_overflow(i8*, i64, i64) #2

; Function Attrs: uwtable
declare void @__ubsan_handle_type_mismatch_v1(i8*, i64) #2

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { uwtable }
attributes #3 = { nounwind }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4, !5, !6, !7, !8}
!llvm.ident = !{!9}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "Ubuntu clang version 14.0.0-1ubuntu1.1", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "array_out_of_bound.c", directory: "/home/ubuntu/Desktop/workspace/graduate_1_first/Secure_Programming/overflow_detect", checksumkind: CSK_MD5, checksum: "586d825815d3b04c400f2f33aceee135")
!2 = !{i32 7, !"Dwarf Version", i32 5}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{i32 1, !"wchar_size", i32 4}
!5 = !{i32 7, !"PIC Level", i32 2}
!6 = !{i32 7, !"PIE Level", i32 2}
!7 = !{i32 7, !"uwtable", i32 1}
!8 = !{i32 7, !"frame-pointer", i32 2}
!9 = !{!"Ubuntu clang version 14.0.0-1ubuntu1.1"}
!10 = distinct !DISubprogram(name: "main", scope: !1, file: !1, line: 3, type: !11, scopeLine: 3, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !14)
!11 = !DISubroutineType(types: !12)
!12 = !{!13}
!13 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!14 = !{}
!15 = !DILocalVariable(name: "a", scope: !10, file: !1, line: 4, type: !16)
!16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 320, elements: !17)
!17 = !{!18}
!18 = !DISubrange(count: 10)
!19 = !DILocation(line: 4, column: 9, scope: !10)
!20 = !DILocation(line: 5, column: 11, scope: !10)
!21 = !{!"branch_weights", i32 1048575, i32 1}
!22 = !DILocation(line: 6, column: 5, scope: !10)
