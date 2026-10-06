.set MB2_MAGIC,    0xE85250D6
.set MB2_ARCH,     0
.set MB2_LENGTH,   mb2_header_end - mb2_header_start
.set MB2_CHECKSUM, -(MB2_MAGIC + MB2_ARCH + MB2_LENGTH)

.section .multiboot, "a"
.align 8
mb2_header_start:
.long MB2_MAGIC
.long MB2_ARCH
.long MB2_LENGTH
.long MB2_CHECKSUM

# framebuffer tag
.align 8
.short 5    # type
.short 0    # flags (0 = required)
.long 20    # size
.long 1280  # width
.long 720   # height
.long 32    # depth

# module alignment tag (page-align modules)
.align 8
.short 6
.short 0
.long 8

# end tag
.align 8
.short 0
.short 0
.long 8
mb2_header_end:

.section .rodata
.align 8
gdt64:
	.quad 0                                             # null descriptor
	.quad (1<<43) | (1<<44) | (1<<47) | (1<<53)         # code: exec, code/data, present, 64-bit
gdt64_ptr:
	.word gdt64_ptr - gdt64 - 1
	.quad gdt64

.section .bss
.align 4096
pml4:
.skip 4096
pdpt:
.skip 4096
pd:
.skip 4096 * 4      # 4 page directories -> identity map first 4 GiB
stack_bottom:
.skip 16384
stack_top:

.section .text
.code32
.global _start
.type _start, @function
_start:
	cli
	mov $stack_top, %esp

	mov %eax, %edi
	mov %ebx, %esi

	mov $0x80000000, %eax
	cpuid
	cmp $0x80000001, %eax
	jb .Lhalt
	mov $0x80000001, %eax
	cpuid
	test $(1<<29), %edx
	jz .Lhalt

	# pml4[0] -> pdpt, pdpt[0..4] -> pd[0..4]
	mov $pdpt, %eax
	or $0b11, %eax
	mov %eax, pml4

	mov $pd, %eax
	or $0b11, %eax
	mov %eax, pdpt
	add $4096, %eax
	mov %eax, pdpt + 8
	add $4096, %eax
	mov %eax, pdpt + 16
	add $4096, %eax
	mov %eax, pdpt + 24

	# fill 2048 pd entries with 2 MiB huge pages (covers the framebuffer)
	xor %ecx, %ecx
.Lmap_pd:
	mov %ecx, %eax
	shl $21, %eax
	or $0b10000011, %eax
	mov %eax, pd(,%ecx,8)
	inc %ecx
	cmp $2048, %ecx
	jne .Lmap_pd

	# load page tables
	mov $pml4, %eax
	mov %eax, %cr3

	# enable PAE
	mov %cr4, %eax
	or $(1<<5), %eax
	mov %eax, %cr4

	# set EFER.LME
	mov $0xC0000080, %ecx
	rdmsr
	or $(1<<8), %eax
	wrmsr

	# enable paging
	mov %cr0, %eax
	or $0x80000000, %eax
	mov %eax, %cr0

	lgdt gdt64_ptr
	ljmp $0x08, $long_mode_start

.Lhalt:
	cli
	hlt
	jmp .Lhalt

.size _start, . - _start

.code64
long_mode_start:
	xor %ax, %ax
	mov %ax, %ss
	mov %ax, %ds
	mov %ax, %es
	mov %ax, %fs
	mov %ax, %gs

	# upper halves of registers are undefined after the switch
	mov $stack_top, %rsp
	mov %edi, %edi
	mov %esi, %esi
	call kernel_main

.Lhalt64:
	cli
	hlt
	jmp .Lhalt64
