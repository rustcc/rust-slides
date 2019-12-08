	.text
	.file	"a0.cgu-0.rs"
	.section	.text._ZN2a04main17h254ecee3350b52fdE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN2a04main17h254ecee3350b52fdE,@function
_ZN2a04main17h254ecee3350b52fdE:
	.cfi_startproc
	pushq	%rbp
.Lcfi0:
	.cfi_def_cfa_offset 16
	pushq	%r14
.Lcfi1:
	.cfi_def_cfa_offset 24
	pushq	%rbx
.Lcfi2:
	.cfi_def_cfa_offset 32
	subq	$80, %rsp
.Lcfi3:
	.cfi_def_cfa_offset 112
.Lcfi4:
	.cfi_offset %rbx, -32
.Lcfi5:
	.cfi_offset %r14, -24
.Lcfi6:
	.cfi_offset %rbp, -16
	leaq	str.0(%rip), %rbp
	movl	$8388639, %r14d
	leaq	str.0+13(%rip), %rbx
	.p2align	4, 0x90
.LBB0_1:
	leaq	1(%rbp), %rax
	movzbl	(%rbp), %edi
	testb	%dil, %dil
	js	.LBB0_3
	movq	%rax, %rbp
	jmp	.LBB0_18
	.p2align	4, 0x90
.LBB0_3:
	cmpq	%rbx, %rax
	je	.LBB0_4
	movzbl	1(%rbp), %eax
	addq	$2, %rbp
	andl	$63, %eax
	jmp	.LBB0_6
.LBB0_4:
	xorl	%eax, %eax
	movq	%rbx, %rbp
.LBB0_6:
	movl	%edi, %ecx
	andl	$31, %ecx
	cmpb	$-32, %dil
	jb	.LBB0_7
	cmpq	%rbx, %rbp
	je	.LBB0_9
	movzbl	(%rbp), %edx
	incq	%rbp
	andl	$63, %edx
	jmp	.LBB0_11
.LBB0_7:
	shll	$6, %ecx
	jmp	.LBB0_17
.LBB0_9:
	xorl	%edx, %edx
	movq	%rbx, %rbp
.LBB0_11:
	shll	$6, %eax
	orl	%edx, %eax
	cmpb	$-16, %dil
	jb	.LBB0_12
	cmpq	%rbx, %rbp
	je	.LBB0_14
	movzbl	(%rbp), %edx
	incq	%rbp
	andl	$63, %edx
	jmp	.LBB0_16
.LBB0_12:
	shll	$12, %ecx
	jmp	.LBB0_17
.LBB0_14:
	xorl	%edx, %edx
.LBB0_16:
	andl	$7, %ecx
	shll	$18, %ecx
	shll	$6, %eax
	orl	%edx, %eax
.LBB0_17:
	orl	%ecx, %eax
	movl	%eax, %edi
.LBB0_18:
	leal	-9(%rdi), %eax
	cmpl	$24, %eax
	jae	.LBB0_19
	btl	%eax, %r14d
	jb	.LBB0_21
	jmp	.LBB0_24
	.p2align	4, 0x90
.LBB0_19:
	cmpl	$127, %edi
	jbe	.LBB0_24
	callq	_ZN11std_unicode6tables8property11White_Space17hbee25c8f58171621E@PLT
	testb	%al, %al
	je	.LBB0_24
.LBB0_21:
	cmpq	%rbx, %rbp
	jne	.LBB0_1
	movb	$1, %al
	jmp	.LBB0_25
.LBB0_24:
	xorl	%eax, %eax
.LBB0_25:
	movb	%al, 15(%rsp)
	leaq	15(%rsp), %rax
	movq	%rax, 16(%rsp)
	movq	_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h5b2fa808d6dedb00E@GOTPCREL(%rip), %rax
	movq	%rax, 24(%rsp)
	leaq	ref.3(%rip), %rax
	movq	%rax, 32(%rsp)
	movq	$2, 40(%rsp)
	movq	$0, 48(%rsp)
	leaq	16(%rsp), %rax
	movq	%rax, 64(%rsp)
	movq	$1, 72(%rsp)
	leaq	32(%rsp), %rdi
	callq	_ZN3std2io5stdio6_print17hc2847a6726c4b4a3E@PLT
	addq	$80, %rsp
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
.Lfunc_end0:
	.size	_ZN2a04main17h254ecee3350b52fdE, .Lfunc_end0-_ZN2a04main17h254ecee3350b52fdE
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	%rsi, %rax
	movq	%rdi, %rcx
	leaq	_ZN2a04main17h254ecee3350b52fdE(%rip), %rdi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	jmp	_ZN3std2rt10lang_start17hdcba96cc6d0f043dE@PLT
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc

	.type	str.0,@object
	.section	.rodata.str.0,"a",@progbits
str.0:
	.zero	13,32
	.size	str.0, 13

	.type	str.1,@object
	.section	.rodata.str.1,"a",@progbits
str.1:
	.size	str.1, 0

	.type	str.2,@object
	.section	.rodata.str.2,"a",@progbits
str.2:
	.byte	10
	.size	str.2, 1

	.type	ref.3,@object
	.section	.data.rel.ro.ref.3,"aw",@progbits
	.p2align	3
ref.3:
	.quad	str.1
	.quad	0
	.quad	str.2
	.quad	1
	.size	ref.3, 32


	.section	".note.GNU-stack","",@progbits
