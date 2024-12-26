; ModuleID = 'overflow.c'
source_filename = "overflow.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@.str = private unnamed_addr constant [3 x i8] c"%d\00", align 1
@.str.1 = private unnamed_addr constant [10 x i8] c"sum = %d\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main(i32 noundef %0, i8** noundef %1) #0 !dbg !10 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i8**, align 8
  %6 = alloca i32, align 4
  %7 = alloca i32, align 4
  %8 = alloca i32, align 4
  store i32 0, i32* %3, align 4
  store i32 %0, i32* %4, align 4
  call void @llvm.dbg.declare(metadata i32* %4, metadata !18, metadata !DIExpression()), !dbg !19
  store i8** %1, i8*** %5, align 8
  call void @llvm.dbg.declare(metadata i8*** %5, metadata !20, metadata !DIExpression()), !dbg !21
  call void @llvm.dbg.declare(metadata i32* %6, metadata !22, metadata !DIExpression()), !dbg !24
  store i32 0, i32* %6, align 4, !dbg !24
  call void @llvm.dbg.declare(metadata i32* %7, metadata !25, metadata !DIExpression()), !dbg !26
  store i32 0, i32* %7, align 4, !dbg !26
  call void @llvm.dbg.declare(metadata i32* %8, metadata !27, metadata !DIExpression()), !dbg !28
  store i32 0, i32* %8, align 4, !dbg !28
  %9 = call i32 (i8*, ...) @__isoc99_scanf(i8* noundef getelementptr inbounds ([3 x i8], [3 x i8]* @.str, i64 0, i64 0), i32* noundef %7), !dbg !29
  %10 = call i32 (i8*, ...) @__isoc99_scanf(i8* noundef getelementptr inbounds ([3 x i8], [3 x i8]* @.str, i64 0, i64 0), i32* noundef %8), !dbg !30
  %11 = load i32, i32* %7, align 4, !dbg !31
  %12 = load i32, i32* %8, align 4, !dbg !32
  %13 = add i32 %11, %12, !dbg !33
  store i32 %13, i32* %6, align 4, !dbg !34
  %14 = load i32, i32* %6, align 4, !dbg !35
  %15 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([10 x i8], [10 x i8]* @.str.1, i64 0, i64 0), i32 noundef %14), !dbg !36
  ret i32 0, !dbg !37
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

declare i32 @__isoc99_scanf(i8* noundef, ...) #2

declare i32 @printf(i8* noundef, ...) #2

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4, !5, !6, !7, !8}
!llvm.ident = !{!9}

!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: "Ubuntu clang version 14.0.0-1ubuntu1.1", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "overflow.c", directory: "/home/ubuntu/Desktop/workspace/graduate_1_first/Secure_Programming/overflow_detect", checksumkind: CSK_MD5, checksum: "22cb0b5b0e4dfa6543a430c86d0aedf5")
!2 = !{i32 7, !"Dwarf Version", i32 5}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = !{i32 1, !"wchar_size", i32 4}
!5 = !{i32 7, !"PIC Level", i32 2}
!6 = !{i32 7, !"PIE Level", i32 2}
!7 = !{i32 7, !"uwtable", i32 1}
!8 = !{i32 7, !"frame-pointer", i32 2}
!9 = !{!"Ubuntu clang version 14.0.0-1ubuntu1.1"}
!10 = distinct !DISubprogram(name: "main", scope: !1, file: !1, line: 4, type: !11, scopeLine: 4, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !17)
!11 = !DISubroutineType(types: !12)
!12 = !{!13, !13, !14}
!13 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!14 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !15, size: 64)
!15 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !16, size: 64)
!16 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_signed_char)
!17 = !{}
!18 = !DILocalVariable(name: "argc", arg: 1, scope: !10, file: !1, line: 4, type: !13)
!19 = !DILocation(line: 4, column: 14, scope: !10)
!20 = !DILocalVariable(name: "argv", arg: 2, scope: !10, file: !1, line: 4, type: !14)
!21 = !DILocation(line: 4, column: 26, scope: !10)
!22 = !DILocalVariable(name: "sum", scope: !10, file: !1, line: 5, type: !23)
!23 = !DIBasicType(name: "unsigned int", size: 32, encoding: DW_ATE_unsigned)
!24 = !DILocation(line: 5, column: 18, scope: !10)
!25 = !DILocalVariable(name: "a", scope: !10, file: !1, line: 6, type: !23)
!26 = !DILocation(line: 6, column: 18, scope: !10)
!27 = !DILocalVariable(name: "b", scope: !10, file: !1, line: 7, type: !23)
!28 = !DILocation(line: 7, column: 18, scope: !10)
!29 = !DILocation(line: 8, column: 5, scope: !10)
!30 = !DILocation(line: 9, column: 5, scope: !10)
!31 = !DILocation(line: 10, column: 11, scope: !10)
!32 = !DILocation(line: 10, column: 15, scope: !10)
!33 = !DILocation(line: 10, column: 13, scope: !10)
!34 = !DILocation(line: 10, column: 9, scope: !10)
!35 = !DILocation(line: 11, column: 26, scope: !10)
!36 = !DILocation(line: 11, column: 5, scope: !10)
!37 = !DILocation(line: 12, column: 5, scope: !10)
