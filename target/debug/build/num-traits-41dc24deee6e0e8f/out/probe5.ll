; ModuleID = 'probe5.bdb889f61c2d1eff-cgu.0'
source_filename = "probe5.bdb889f61c2d1eff-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

; core::f64::<impl f64>::is_subnormal
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h944e654ec42a3eacE"(double %self) unnamed_addr #0 {
start:
  %_2 = alloca i8, align 1
  %_4 = fcmp une double %self, %self
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %b = bitcast double %self to i64
  %_6 = and i64 %b, 4503599627370495
  %_7 = and i64 %b, 9218868437227405312
  %0 = icmp eq i64 %_6, 0
  br i1 %0, label %bb5, label %bb6

bb1:                                              ; preds = %start
  store i8 0, ptr %_2, align 1
  br label %bb4

bb5:                                              ; preds = %bb2
  switch i64 %_7, label %bb6 [
    i64 9218868437227405312, label %bb8
    i64 0, label %bb9
  ]

bb6:                                              ; preds = %bb5, %bb2
  %1 = icmp eq i64 %_7, 0
  br i1 %1, label %bb10, label %bb7

bb8:                                              ; preds = %bb5
  store i8 1, ptr %_2, align 1
  br label %bb3

bb9:                                              ; preds = %bb5
  store i8 2, ptr %_2, align 1
  br label %bb3

bb3:                                              ; preds = %bb7, %bb10, %bb9, %bb8
  br label %bb4

bb10:                                             ; preds = %bb6
  store i8 3, ptr %_2, align 1
  br label %bb3

bb7:                                              ; preds = %bb6
  store i8 4, ptr %_2, align 1
  br label %bb3

bb4:                                              ; preds = %bb1, %bb3
  %2 = load i8, ptr %_2, align 1, !range !2, !noundef !3
  %_3 = zext i8 %2 to i64
  %_0 = icmp eq i64 %_3, 3
  ret i1 %_0
}

; probe5::probe
; Function Attrs: uwtable
define void @_ZN6probe55probe17h4a57e4314036a9a1E() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::is_subnormal
  %_1 = call zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h944e654ec42a3eacE"(double 1.000000e+00)
  ret void
}

attributes #0 = { inlinehint uwtable "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #1 = { uwtable "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.75.0 (82e1608df 2023-12-21)"}
!2 = !{i8 0, i8 5}
!3 = !{}