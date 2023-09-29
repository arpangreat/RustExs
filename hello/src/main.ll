; ModuleID = 'main.f657fee0e41a2ec3-cgu.0'
source_filename = "main.f657fee0e41a2ec3-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } }

@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h18b0b307a53ad12dE", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2461ee5be47e7903E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h06019accf128c4fbE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h06019accf128c4fbE" }>, align 8
@alloc_513570631223a12912d85da2bec3b15a = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc_91c7fa63c3cfeaa3c795652d5cf060e4 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc_560206a49c61adca6f3f0639a12632eb = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_91c7fa63c3cfeaa3c795652d5cf060e4, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc_aa8e0fa65b6b16643e9b72475c35d238 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/8b35c0bb0f833c0077dc57006eb317edde2a2d1e/library/core/src/fmt/mod.rs" }>, align 1
@alloc_116b8908aee8082c07a71c3f909d9361 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_aa8e0fa65b6b16643e9b72475c35d238, [16 x i8] c"K\00\00\00\00\00\00\00+\01\00\00\0D\00\00\00" }>, align 8
@alloc_3213114faf700a46436312d7d5d956d1 = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c"Hello, world!\0A" }>, align 1
@alloc_004b71e11835e37789f0750656a25291 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_3213114faf700a46436312d7d5d956d1, [8 x i8] c"\0E\00\00\00\00\00\00\00" }>, align 8

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h0355a918e484c138E(ptr %f) unnamed_addr #0 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h4f377b0df43e8aa7E(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !3
  ret void
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17h1e5b1dae8ecdfbd4E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #1 {
start:
  %_8 = alloca ptr, align 8
  %_5 = alloca i64, align 8
  store ptr %main, ptr %_8, align 8
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17h2686d3c261ab96d7E(ptr align 1 %_8, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  store i64 %0, ptr %_5, align 8
  %1 = load i64, ptr %_5, align 8, !noundef !4
  ret i64 %1
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h06019accf128c4fbE"(ptr align 8 %_1) unnamed_addr #2 {
start:
  %self = alloca i8, align 1
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h0355a918e484c138E(ptr %_4)
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hf533851cd79f5069E"()
  store i8 %0, ptr %self, align 1
  %_6 = load i8, ptr %self, align 1, !noundef !4
  %1 = zext i8 %_6 to i32
  ret i32 %1
}

; core::fmt::Arguments::new_const
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments9new_const17h5f8bb7d6aec35eedE(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1) unnamed_addr #2 {
start:
  %_7 = alloca { ptr, i64 }, align 8
  %_5 = alloca %"core::fmt::Arguments<'_>", align 8
  %_2 = icmp ugt i64 %pieces.1, 1
  br i1 %_2, label %bb1, label %bb3

bb3:                                              ; preds = %start
  store ptr null, ptr %_7, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %pieces.0, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %pieces.1, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_7, i32 0, i32 0
  %4 = load ptr, ptr %3, align 8, !align !5, !noundef !4
  %5 = getelementptr inbounds { ptr, i64 }, ptr %_7, i32 0, i32 1
  %6 = load i64, ptr %5, align 8
  %7 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %8 = getelementptr inbounds { ptr, i64 }, ptr %7, i32 0, i32 0
  store ptr %4, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, i64 }, ptr %7, i32 0, i32 1
  store i64 %6, ptr %9, align 8
  %10 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %11 = getelementptr inbounds { ptr, i64 }, ptr %10, i32 0, i32 0
  store ptr @alloc_513570631223a12912d85da2bec3b15a, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %10, i32 0, i32 1
  store i64 0, ptr %12, align 8
  ret void

bb1:                                              ; preds = %start
; call core::fmt::Arguments::new_const
  call void @_ZN4core3fmt9Arguments9new_const17h5f8bb7d6aec35eedE(ptr sret(%"core::fmt::Arguments<'_>") %_5, ptr align 8 @alloc_560206a49c61adca6f3f0639a12632eb, i64 1)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h4b6c2a18441d476cE(ptr %_5, ptr align 8 @alloc_116b8908aee8082c07a71c3f909d9361) #5
  unreachable
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2461ee5be47e7903E"(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h0d286a0ae820bcfcE(ptr %0)
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h0d286a0ae820bcfcE(ptr %0) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h06019accf128c4fbE"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %3 = load ptr, ptr %1, align 8, !noundef !4
  %4 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %5 = load i32, ptr %4, align 8, !noundef !4
  %6 = insertvalue { ptr, i32 } poison, ptr %3, 0
  %7 = insertvalue { ptr, i32 } %6, i32 %5, 1
  resume { ptr, i32 } %7

cleanup:                                          ; preds = %start
  %8 = landingpad { ptr, i32 }
          cleanup
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
  %11 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %9, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %10, ptr %12, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %2
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h4f377b0df43e8aa7E(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h18b0b307a53ad12dE"(ptr align 8 %_1) unnamed_addr #2 {
start:
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hf533851cd79f5069E"() unnamed_addr #2 {
start:
  ret i8 0
}

; main::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN4main4main17h826b49218aaecea9E() unnamed_addr #1 {
start:
  %_2 = alloca %"core::fmt::Arguments<'_>", align 8
; call core::fmt::Arguments::new_const
  call void @_ZN4core3fmt9Arguments9new_const17h5f8bb7d6aec35eedE(ptr sret(%"core::fmt::Arguments<'_>") %_2, ptr align 8 @alloc_004b71e11835e37789f0750656a25291, i64 1)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h29eeb04801384e10E(ptr %_2)
  ret void
}

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h2686d3c261ab96d7E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17h4b6c2a18441d476cE(ptr, ptr align 8) unnamed_addr #3

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #1

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17h29eeb04801384e10E(ptr) unnamed_addr #1

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #4 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h1e5b1dae8ecdfbd4E(ptr @_ZN4main4main17h826b49218aaecea9E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { nonlazybind "target-cpu"="x86-64" }
attributes #5 = { noreturn }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{i32 1370293}
!4 = !{}
!5 = !{i64 8}
