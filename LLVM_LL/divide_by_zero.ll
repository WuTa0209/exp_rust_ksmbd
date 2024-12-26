; ModuleID = 'divide_by_zero.c'
source_filename = "divide_by_zero.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@.src = private unnamed_addr constant [17 x i8] c"divide_by_zero.c\00", align 1
@0 = private unnamed_addr constant { i16, i16, [6 x i8] } { i16 0, i16 11, [6 x i8] c"'int'\00" }
@1 = private unnamed_addr global { { [17 x i8]*, i32, i32 }, { i16, i16, [6 x i8] }* } { { [17 x i8]*, i32, i32 } { [17 x i8]* @.src, i32 6, i32 15 }, { i16, i16, [6 x i8] }* @0 }
@.str = private unnamed_addr constant [8 x i8] c"c = %d\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 !dbg !10 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 0, i32* %1, align 4
  call void @llvm.dbg.declare(metadata i32* %2, metadata !15, metadata !DIExpression()), !dbg !16
  store i32 1, i32* %2, align 4, !dbg !16
  call void @llvm.dbg.declare(metadata i32* %3, metadata !17, metadata !DIExpression()), !dbg !18
  store i32 0, i32* %3, align 4, !dbg !18
  call void @llvm.dbg.declare(metadata i32* %4, metadata !19, metadata !DIExpression()), !dbg !20
  %5 = load i32, i32* %2, align 4, !dbg !21
  %6 = load i32, i32* %3, align 4, !dbg !22
  %7 = icmp ne i32 %6, 0, !dbg !23, !nosanitize !14
  %8 = icmp ne i32 %5, -2147483648, !dbg !23, !nosanitize !14
  %9 = icmp ne i32 %6, -1, !dbg !23, !nosanitize !14
  %10 = or i1 %8, %9, !dbg !23, !nosanitize !14
  %11 = and i1 %7, %10, !dbg !23, !nosanitize !14
  br i1 %11, label %15, label %12, !dbg !23, !prof !24, !nosanitize !14

12:                                               ; preds = %0
  %13 = zext i32 %5 to i64, !dbg !23, !nosanitize !14
  %14 = zext i32 %6 to i64, !dbg !23, !nosanitize !14
  call void @__ubsan_handle_divrem_overflow(i8* bitcast ({ { [17 x i8]*, i32, i32 }, { i16, i16, [6 x i8] }* }* @1 to i8*), i64 %13, i64 %14) #4, !dbg !23, !nosanitize !14
  br label %15, !dbg !23, !nosanitize !14

15:                                               ; preds = %12, %0
  %16 = sdiv i32 %5, %6, !dbg !23
  store i32 %16, i32* %4, align 4, !dbg !20
  %17 = load i32, i32* %4, align 4, !dbg !25
  %18 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([8 x i8], [8 x i8]* @.str, i64 0, i64 0), i32 noundef %17), !dbg !26
  %19 = load i32, i32* %1, align 4, !dbg !27
  ret i32 %19, !dbg !27
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: uwtable
declare void @__ubsan_handle_divrem_overflow(i8*, i64, i64) #2

declare i32 @printf(i8* noundef, ...) #3

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { uwtable }
attributes #3 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { nounwind }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4, !5, !6, !7, !8}
!llvm.ident = !{!9}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "Ubuntu clang version 14.0.0-1ubuntu1.1", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "divide_by_zero.c", directory: "/home/ubuntu/Desktop/workspace/graduate_1_first/Secure_Programming/overflow_detect", checksumkind: CSK_MD5, checksum: "3723dd0019ac0acc6d2609838f0bd03a")
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
!15 = !DILocalVariable(name: "a", scope: !10, file: !1, line: 4, type: !13)
!16 = !DILocation(line: 4, column: 9, scope: !10)
!17 = !DILocalVariable(name: "b", scope: !10, file: !1, line: 5, type: !13)
!18 = !DILocation(line: 5, column: 9, scope: !10)
!19 = !DILocalVariable(name: "c", scope: !10, file: !1, line: 6, type: !13)
!20 = !DILocation(line: 6, column: 9, scope: !10)
!21 = !DILocation(line: 6, column: 13, scope: !10)
!22 = !DILocation(line: 6, column: 17, scope: !10)
!23 = !DILocation(line: 6, column: 15, scope: !10)
!24 = !{!"branch_weights", i32 1048575, i32 1}
!25 = !DILocation(line: 7, column: 24, scope: !10)
!26 = !DILocation(line: 7, column: 5, scope: !10)
!27 = !DILocation(line: 8, column: 1, scope: !10)
