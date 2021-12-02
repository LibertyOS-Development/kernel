.section .boot-stg1, "awx"
.global _start
.code16

_start:
	xor ax, ax
	mov dx, ax
	mov es, ax
	mov ss, ax
	mov fs, ax
	mov gs, ax
	cld
	mov sp, 0x7c00
	mov si, offset boot_start_msg
	call real_println
enable_a20:
	in al, 0x92
	test al, 2
	jnz enable_a20_after
	or al, 2
	and al, 0xFE
	out 0x92, al
enable_a20_after:

enter_protected:
	cli
	push ds
	push es
	lgdt [gdt32info]
	mov eax, cr0
	or al, 1
	mov cr0, eax
	jmp protected
protected:
	mov bx, 0x10
	mov ds, bx
	mov es, bx
	and al, 0xfe
	mov cr0, eax
unreal:
	pop es
	pop ds
	sti
	mov bx, 0x0f01
	mov eax, 0xb8f00
	mov word ptr ds:[eax], bx
check_int13h_ext:
	mov ah, 0x41
	mov bx, 0x55aa
	int 0x13
	jc no_int13h_ext
load_rem_bootloader:
	mov eax, offset _rem_bootloader_start_addr
	mov ecx, 0
load_disk:
	lea eax, _rem_bootloader_start_addr
	add eax, ecx
	mov ebx, eax
	shr ebx, 4
	mov [dap_buff_seg], bx
	shl ebx, 4
	sub eax, ebx
	mov [dap_buff_addr], ax
	mov eax, offset _rem_bootloader_start_addr
	add eax, ecx
	mov ebx, offset _rem_bootloader_end_addr
	sub ebx, eax
	jz end_load_disk
	shr ebx, 9
	cmp ebx, 127
	jle .cont_load_disk
	mov ebx, 127
.cont_load_disk:
	mov [dap_blk], bx
	shl ebx, 9
	add ecx, ebx
	mov ebx, offset _start
	sub eax, ebx
	shr eax, 9
	mov [dap_start_lba], eax
	mov si, offset dap
	mov ah, 0x42
	int 0x13
	jc rem_load_bootloader_failure
	jmp load_disk
load_disk_end:
	mov word ptr [dap_buff_seg], 0
to_stg2:
	mov eax, offset stg2
	jmp eax
spin:
	jmp spin
real_println:
	call real_print
	mov al, 13
	call real_printchar
	mov al, 10
	jmp real_printchar
real_print:
	cld
real_print_loop:
	lodsb al, BYTE PTR [si]
	test al, al
	jz real_print_end
	call real_printchar
	jmp real_print_loop
real_print_end:
	ret
real_printchar:
	mov ah, 0x0e
	int 0x10
	ret
real_printhex:
	mov cx, 4
.lp:
	mov al, bh
	shr al, 4
	cmp al, 0xA
	jb .below_0xA
	add al, 'A' - 0xA - '0'
.below_0xA:
	add al, '0'
	call real_printchar
	shl bx, 4
	loop .lp
	ret
real_err:
	call real_println
	jmp spin
no_int13h_ext:
	mov si, offset no_int13h_ext_msg
	jmp real_err
