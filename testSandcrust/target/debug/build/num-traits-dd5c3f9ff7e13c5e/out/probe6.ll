; ModuleID = 'probe6.95503628b5f47325-cgu.0'
source_filename = "probe6.95503628b5f47325-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; core::f64::<impl f64>::is_subnormal
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h7f6ce02eeeadca2dE"(double %self) unnamed_addr #0 {
start:
  %_2 = alloca i8, align 1
; call core::f64::<impl f64>::classify
  %0 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17h98c63b0d4aae0744E"(double %self), !range !3
  store i8 %0, ptr %_2, align 1
  %1 = load i8, ptr %_2, align 1, !range !3, !noundef !4
  %_3 = zext i8 %1 to i64
  %_0 = icmp eq i64 %_3, 3
  ret i1 %_0
}

; probe6::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe65probe17h9481ca70e0812201E() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::is_subnormal
  %_1 = call zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h7f6ce02eeeadca2dE"(double 1.000000e+00)
  ret void
}

; core::f64::<impl f64>::classify
; Function Attrs: nonlazybind uwtable
declare i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17h98c63b0d4aae0744E"(double) unnamed_addr #1

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.75.0-nightly (c543b6f35 2023-10-14)"}
!3 = !{i8 0, i8 5}
!4 = !{}
