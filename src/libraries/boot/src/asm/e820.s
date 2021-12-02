.section .boot, "awx"
.code16

do_e820:
	xor ebx, ebx
	xor bp, bp
	mov edx, 0x0534D4150
	mov eax, 0xe820
	mov dword ptr es:[di + 20], 1
	mov ecx, 24
	int 0x15
	jc .failure
	mov edx, 0x0534D4150
	cmp eax, edx
	jne .failure
	test ebx, ebx
	je .failure
	jmp .jmpin
.e8201p:
	mov eax, 0xe820
	mov dword ptr es:[di + 20], 1
	mov ecx, 24
	int 0x15
	jc .e820f
	mov edx, 0x0534D4150
.jmpin:
	jcxz .skinentr
	cmp cl, 20
	jbe .notxt
	test byte ptr es:[di + 20], 1
	je .skipentr
.notxt:
	mov ecx, es:[di + 8]
	or ecx, es:[di + 12]
	jz .skipentr
	inc bp
	add di, 24
.skipentr:
	test ebx, ebx
	jne .e8201p
.e820f:
	mov [mmapentr], bp
	clc
	ret
.failure:
	stc
	ret
mmapentr:
	.word 0
