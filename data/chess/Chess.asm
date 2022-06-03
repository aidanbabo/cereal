		.DATA
pawn_sprite 		.FILL #0
		.FILL #896
		.FILL #1984
		.FILL #4064
		.FILL #4064
		.FILL #4064
		.FILL #1984
		.FILL #896
		.FILL #896
		.FILL #896
		.FILL #896
		.FILL #896
		.FILL #4064
		.FILL #8176
		.FILL #16376
		.DATA
rook_sprite 		.FILL #0
		.FILL #13208
		.FILL #13208
		.FILL #8176
		.FILL #8176
		.FILL #4064
		.FILL #1984
		.FILL #1984
		.FILL #1984
		.FILL #1984
		.FILL #1984
		.FILL #1984
		.FILL #4064
		.FILL #8176
		.FILL #16376
		.DATA
knight_sprite 		.FILL #0
		.FILL #2048
		.FILL #3840
		.FILL #7552
		.FILL #8128
		.FILL #16352
		.FILL #16352
		.FILL #15872
		.FILL #15872
		.FILL #16256
		.FILL #8128
		.FILL #4064
		.FILL #4064
		.FILL #16376
		.FILL #32764
		.DATA
bishop_sprite 		.FILL #0
		.FILL #256
		.FILL #896
		.FILL #256
		.FILL #896
		.FILL #1728
		.FILL #3168
		.FILL #3808
		.FILL #1984
		.FILL #896
		.FILL #896
		.FILL #896
		.FILL #1984
		.FILL #4064
		.FILL #8176
		.DATA
queen_sprite 		.FILL #256
		.FILL #896
		.FILL #4368
		.FILL #7088
		.FILL #8176
		.FILL #4064
		.FILL #1984
		.FILL #4064
		.FILL #896
		.FILL #896
		.FILL #896
		.FILL #896
		.FILL #4064
		.FILL #8176
		.FILL #16376
		.DATA
king_sprite 		.FILL #256
		.FILL #896
		.FILL #256
		.FILL #1984
		.FILL #4064
		.FILL #4064
		.FILL #1984
		.FILL #896
		.FILL #896
		.FILL #896
		.FILL #896
		.FILL #1984
		.FILL #4064
		.FILL #8176
		.FILL #16376
		.DATA
board_state 		.FILL #10
		.FILL #11
		.FILL #12
		.FILL #13
		.FILL #14
		.FILL #12
		.FILL #11
		.FILL #10
		.FILL #9
		.FILL #9
		.FILL #9
		.FILL #9
		.FILL #9
		.FILL #9
		.FILL #9
		.FILL #9
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #0
		.FILL #1
		.FILL #1
		.FILL #1
		.FILL #1
		.FILL #1
		.FILL #1
		.FILL #1
		.FILL #1
		.FILL #2
		.FILL #3
		.FILL #4
		.FILL #5
		.FILL #6
		.FILL #4
		.FILL #3
		.FILL #2
		.DATA
player_to_move 		.FILL #1
;;;;;;;;;;;;;;;;;;;;;;;;;;;;abs;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
abs
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-1	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #3
	CONST R3, #0
	CMP R7, R3
	BRn L4_Chess
	LDR R7, R5, #3
	STR R7, R5, #-1
	JMP L5_Chess
L4_Chess
	LDR R7, R5, #3
	NOT R7,R7
	ADD R7,R7,#1
	STR R7, R5, #-1
L5_Chess
	LDR R7, R5, #-1
L2_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;max;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
max
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-1	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #3
	LDR R3, R5, #4
	CMP R7, R3
	BRnz L8_Chess
	LDR R7, R5, #3
	STR R7, R5, #-1
	JMP L9_Chess
L8_Chess
	LDR R7, R5, #4
	STR R7, R5, #-1
L9_Chess
	LDR R7, R5, #-1
L6_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;printnum;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
printnum
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-13	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #3
	CONST R3, #0
	CMP R7, R3
	BRnp L11_Chess
	LEA R7, L13_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	JMP L10_Chess
L11_Chess
	LDR R7, R5, #3
	CONST R3, #0
	CMP R7, R3
	BRzp L15_Chess
	LDR R7, R5, #3
	NOT R7,R7
	ADD R7,R7,#1
	STR R7, R5, #-13
	JMP L16_Chess
L15_Chess
	LDR R7, R5, #3
	STR R7, R5, #-13
L16_Chess
	LDR R7, R5, #-13
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #0
	CMP R7, R3
	BRzp L17_Chess
	LEA R7, L19_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	JMP L10_Chess
L17_Chess
	ADD R7, R5, #-12
	ADD R7, R7, #10
	STR R7, R5, #-2
	LDR R7, R5, #-2
	ADD R7, R7, #-1
	STR R7, R5, #-2
	CONST R3, #0
	STR R3, R7, #0
	JMP L21_Chess
L20_Chess
	LDR R7, R5, #-2
	ADD R7, R7, #-1
	STR R7, R5, #-2
	LDR R3, R5, #-1
	CONST R2, #10
	MOD R3, R3, R2
	CONST R2, #48
	ADD R3, R3, R2
	STR R3, R7, #0
	LDR R7, R5, #-1
	CONST R3, #10
	DIV R7, R7, R3
	STR R7, R5, #-1
L21_Chess
	LDR R7, R5, #-1
	CONST R3, #0
	CMP R7, R3
	BRnp L20_Chess
	LDR R7, R5, #3
	CONST R3, #0
	CMP R7, R3
	BRzp L23_Chess
	LDR R7, R5, #-2
	ADD R7, R7, #-1
	STR R7, R5, #-2
	CONST R3, #45
	STR R3, R7, #0
L23_Chess
	LDR R7, R5, #-2
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
L10_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;endl;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
endl
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	;; function body
	LEA R7, L26_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
L25_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;get_ply;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
get_ply
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-5	;; allocate stack space for local variables
	;; function body
	LEA R7, ply
	CONST R3, #0
	STR R3, R7, #4
	JSR lc4_getc_echo
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #113
	CMP R7, R3
	BRnp L29_Chess
	CONST R7, #-2
	JMP L28_Chess
L29_Chess
	LDR R7, R5, #-1
	CONST R3, #120
	CMP R7, R3
	BRnp L31_Chess
	LEA R7, ply
	CONST R3, #1
	STR R3, R7, #4
	CONST R7, #1
	JMP L28_Chess
L31_Chess
	LDR R7, R5, #-1
	CONST R3, #121
	CMP R7, R3
	BRnp L33_Chess
	LEA R7, ply
	CONST R3, #2
	STR R3, R7, #4
	CONST R7, #1
	JMP L28_Chess
L33_Chess
	LDR R7, R5, #-1
	STR R7, R5, #-2
	CONST R3, #97
	CMP R7, R3
	BRn L37_Chess
	CONST R7, #104
	LDR R3, R5, #-2
	CMP R3, R7
	BRnz L35_Chess
L37_Chess
	CONST R7, #-1
	JMP L28_Chess
L35_Chess
	LEA R7, ply
	LDR R3, R5, #-1
	CONST R2, #97
	SUB R3, R3, R2
	STR R3, R7, #1
	JSR lc4_getc_echo
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #113
	CMP R7, R3
	BRnp L38_Chess
	CONST R7, #-2
	JMP L28_Chess
L38_Chess
	LDR R7, R5, #-1
	STR R7, R5, #-3
	CONST R3, #49
	CMP R7, R3
	BRn L42_Chess
	CONST R7, #56
	LDR R3, R5, #-3
	CMP R3, R7
	BRnz L40_Chess
L42_Chess
	CONST R7, #-1
	JMP L28_Chess
L40_Chess
	LEA R7, ply
	LDR R3, R5, #-1
	CONST R2, #49
	SUB R3, R3, R2
	STR R3, R7, #0
	JSR lc4_getc_echo
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #113
	CMP R7, R3
	BRnp L43_Chess
	CONST R7, #-2
	JMP L28_Chess
L43_Chess
	LDR R7, R5, #-1
	STR R7, R5, #-4
	CONST R3, #97
	CMP R7, R3
	BRn L47_Chess
	CONST R7, #104
	LDR R3, R5, #-4
	CMP R3, R7
	BRnz L45_Chess
L47_Chess
	CONST R7, #-1
	JMP L28_Chess
L45_Chess
	LEA R7, ply
	LDR R3, R5, #-1
	CONST R2, #97
	SUB R3, R3, R2
	STR R3, R7, #3
	JSR lc4_getc_echo
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #113
	CMP R7, R3
	BRnp L48_Chess
	CONST R7, #-2
	JMP L28_Chess
L48_Chess
	LDR R7, R5, #-1
	STR R7, R5, #-5
	CONST R3, #49
	CMP R7, R3
	BRn L52_Chess
	CONST R7, #56
	LDR R3, R5, #-5
	CMP R3, R7
	BRnz L50_Chess
L52_Chess
	CONST R7, #-1
	JMP L28_Chess
L50_Chess
	LEA R7, ply
	LDR R3, R5, #-1
	CONST R2, #49
	SUB R3, R3, R2
	STR R3, R7, #2
	CONST R7, #1
L28_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;draw_square;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
draw_square
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-9	;; allocate stack space for local variables
	;; function body
	CONST R7, #15
	LDR R3, R5, #3
	CONST R2, #7
	SUB R2, R2, R3
	MUL R2, R7, R2
	ADD R2, R2, #2
	STR R2, R5, #-3
	LDR R2, R5, #4
	MUL R7, R7, R2
	ADD R7, R7, #4
	STR R7, R5, #-4
	ADD R7, R3, R2
	AND R7, R7, #1
	CONST R3, #0
	CMP R7, R3
	BRz L55_Chess
	CONST R7, #8
	HICONST R7, #33
	STR R7, R5, #-8
	JMP L56_Chess
L55_Chess
	CONST R7, #0
	HICONST R7, #1
	STR R7, R5, #-8
L56_Chess
	LDR R7, R5, #-8
	STR R7, R5, #-5
	LDR R7, R5, #-5
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #15
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-3
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-4
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_draw_rect
	ADD R6, R6, #5	;; free space for arguments
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-2
	LDR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRz L57_Chess
	LDR R7, R5, #-2
	AND R3, R7, #7
	STR R3, R5, #-1
	AND R7, R7, #8
	CONST R3, #0
	CMP R7, R3
	BRz L60_Chess
	CONST R7, #255
	HICONST R7, #127
	STR R7, R5, #-9
	JMP L61_Chess
L60_Chess
	CONST R7, #0
	STR R7, R5, #-9
L61_Chess
	LDR R7, R5, #-9
	STR R7, R5, #-7
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L62_Chess
	LEA R7, pawn_sprite
	STR R7, R5, #-6
L62_Chess
	LDR R7, R5, #-1
	CONST R3, #2
	CMP R7, R3
	BRnp L64_Chess
	LEA R7, rook_sprite
	STR R7, R5, #-6
L64_Chess
	LDR R7, R5, #-1
	CONST R3, #3
	CMP R7, R3
	BRnp L66_Chess
	LEA R7, knight_sprite
	STR R7, R5, #-6
L66_Chess
	LDR R7, R5, #-1
	CONST R3, #4
	CMP R7, R3
	BRnp L68_Chess
	LEA R7, bishop_sprite
	STR R7, R5, #-6
L68_Chess
	LDR R7, R5, #-1
	CONST R3, #5
	CMP R7, R3
	BRnp L70_Chess
	LEA R7, queen_sprite
	STR R7, R5, #-6
L70_Chess
	LDR R7, R5, #-1
	CONST R3, #6
	CMP R7, R3
	BRnp L72_Chess
	LEA R7, king_sprite
	STR R7, R5, #-6
L72_Chess
	LDR R7, R5, #-6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-7
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-3
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-4
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_draw_sprite
	ADD R6, R6, #4	;; free space for arguments
L57_Chess
L53_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;draw_chessboard;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
draw_chessboard
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-2	;; allocate stack space for local variables
	;; function body
	CONST R7, #0
	STR R7, R5, #-2
L75_Chess
	CONST R7, #0
	STR R7, R5, #-1
L79_Chess
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-2
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
L80_Chess
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #8
	CMP R7, R3
	BRn L79_Chess
L76_Chess
	LDR R7, R5, #-2
	ADD R7, R7, #1
	STR R7, R5, #-2
	LDR R7, R5, #-2
	CONST R3, #8
	CMP R7, R3
	BRn L75_Chess
L74_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_pawn_move;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_pawn_move
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-6	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #7
	CONST R3, #0
	CMP R7, R3
	BRz L85_Chess
	LDR R7, R5, #5
	LDR R3, R5, #3
	CMP R7, R3
	BRp L86_Chess
	CONST R7, #0
	JMP L84_Chess
L85_Chess
	LDR R7, R5, #5
	LDR R3, R5, #3
	CMP R7, R3
	BRn L89_Chess
	CONST R7, #0
	JMP L84_Chess
L89_Chess
L86_Chess
	LDR R7, R5, #5
	LDR R3, R5, #3
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-2
	LDR R7, R5, #6
	LDR R3, R5, #4
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnz L91_Chess
	CONST R7, #0
	JMP L84_Chess
L91_Chess
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L93_Chess
	LDR R7, R5, #-2
	CONST R3, #1
	CMP R7, R3
	BRz L95_Chess
	CONST R7, #0
	JMP L84_Chess
L95_Chess
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-3
	LDR R7, R5, #-3
	CONST R3, #0
	CMP R7, R3
	BRz L97_Chess
	LDR R7, R5, #-3
	AND R7, R7, #8
	LDR R3, R5, #7
	CMP R7, R3
	BRnp L94_Chess
	CONST R7, #0
	JMP L84_Chess
L97_Chess
	LEA R7, en_passant
	STR R7, R5, #-6
	CONST R3, #0
	LDR R2, R7, #0
	CMP R2, R3
	BRz L101_Chess
	LDR R7, R5, #-6
	LDR R7, R7, #1
	LDR R2, R5, #5
	CMP R7, R2
	BRnp L101_Chess
	LDR R7, R5, #-6
	LDR R7, R7, #2
	CMP R7, R3
	BRz L101_Chess
	CONST R7, #4
	JMP L84_Chess
L101_Chess
	CONST R7, #0
	JMP L84_Chess
L93_Chess
	LDR R7, R5, #-2
	CONST R3, #2
	CMP R7, R3
	BRnz L103_Chess
	CONST R7, #0
	JMP L84_Chess
L103_Chess
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L105_Chess
	CONST R7, #0
	JMP L84_Chess
L105_Chess
	LDR R7, R5, #-2
	CONST R3, #2
	CMP R7, R3
	BRnp L107_Chess
	LDR R7, R5, #3
	STR R7, R5, #-6
	CONST R3, #1
	CMP R7, R3
	BRz L111_Chess
	CONST R7, #6
	LDR R3, R5, #-6
	CMP R3, R7
	BRnp L109_Chess
L111_Chess
	CONST R7, #2
	LDR R3, R5, #5
	LDR R2, R5, #3
	ADD R3, R3, R2
	DIV R3, R3, R7
	STR R3, R5, #-4
	LDR R3, R5, #6
	LDR R2, R5, #4
	ADD R3, R3, R2
	DIV R7, R3, R7
	STR R7, R5, #-5
	LDR R7, R5, #-5
	LDR R3, R5, #-4
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L112_Chess
	CONST R7, #0
	JMP L84_Chess
L112_Chess
	LEA R7, en_passant
	LDR R3, R5, #-4
	STR R3, R7, #1
	LEA R7, en_passant
	LDR R3, R5, #-5
	STR R3, R7, #2
	LEA R7, en_passant
	LDR R3, R5, #5
	STR R3, R7, #3
	LEA R7, en_passant
	LDR R3, R5, #6
	STR R3, R7, #4
	CONST R7, #2
	JMP L84_Chess
L109_Chess
	CONST R7, #0
	JMP L84_Chess
L107_Chess
L94_Chess
	LDR R7, R5, #5
	CONST R3, #0
	CMP R7, R3
	BRnp L114_Chess
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	CONST R3, #5
	STR R3, R7, #0
L114_Chess
	LDR R7, R5, #5
	CONST R3, #7
	CMP R7, R3
	BRnp L116_Chess
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	CONST R3, #13
	STR R3, R7, #0
L116_Chess
	CONST R7, #1
L84_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_clear;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_clear
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	;; function body
	LDR R7, R5, #4
	LDR R3, R5, #8
	ADD R7, R7, R3
	STR R7, R5, #4
	LDR R7, R5, #3
	LDR R3, R5, #7
	ADD R7, R7, R3
	STR R7, R5, #3
	JMP L122_Chess
L119_Chess
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L123_Chess
	CONST R7, #0
	JMP L118_Chess
L123_Chess
L120_Chess
	LDR R7, R5, #4
	LDR R3, R5, #8
	ADD R7, R7, R3
	STR R7, R5, #4
	LDR R7, R5, #3
	LDR R3, R5, #7
	ADD R7, R7, R3
	STR R7, R5, #3
L122_Chess
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRnp L119_Chess
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRnp L119_Chess
	CONST R7, #1
L118_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_rook_move;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_rook_move
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-2	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRnp L126_Chess
	CONST R7, #0
	STR R7, R5, #-1
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRp L128_Chess
	CONST R7, #1
	STR R7, R5, #-2
	JMP L127_Chess
L128_Chess
	CONST R7, #-1
	STR R7, R5, #-2
	JMP L127_Chess
L126_Chess
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRnp L130_Chess
	CONST R7, #0
	STR R7, R5, #-2
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRp L132_Chess
	CONST R7, #1
	STR R7, R5, #-1
	JMP L131_Chess
L132_Chess
	CONST R7, #-1
	STR R7, R5, #-1
	JMP L131_Chess
L130_Chess
	CONST R7, #0
	JMP L125_Chess
L131_Chess
L127_Chess
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-2
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_clear
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #6	;; free space for arguments
L125_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_bishop_move;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_bishop_move
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-3	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #3
	LDR R3, R5, #5
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-3
	LDR R3, R5, #4
	LDR R2, R5, #6
	SUB R3, R3, R2
	ADD R6, R6, #-1
	STR R3, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	LDR R3, R5, #-3
	CMP R3, R7
	BRz L135_Chess
	CONST R7, #0
	JMP L134_Chess
L135_Chess
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRzp L137_Chess
	CONST R7, #1
	STR R7, R5, #-2
	JMP L138_Chess
L137_Chess
	CONST R7, #-1
	STR R7, R5, #-2
L138_Chess
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRzp L139_Chess
	CONST R7, #1
	STR R7, R5, #-1
	JMP L140_Chess
L139_Chess
	CONST R7, #-1
	STR R7, R5, #-1
L140_Chess
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-2
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_clear
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #6	;; free space for arguments
L134_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_knight_move;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_knight_move
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-3	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #4
	LDR R3, R5, #6
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-1
	LDR R7, R5, #3
	LDR R3, R5, #5
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-2
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L146_Chess
	LDR R7, R5, #-2
	CONST R3, #2
	CMP R7, R3
	BRz L145_Chess
L146_Chess
	LDR R7, R5, #-1
	CONST R3, #2
	CMP R7, R3
	BRnp L143_Chess
	LDR R7, R5, #-2
	CONST R3, #1
	CMP R7, R3
	BRnp L143_Chess
L145_Chess
	CONST R7, #1
	STR R7, R5, #-3
	JMP L144_Chess
L143_Chess
	CONST R7, #0
	STR R7, R5, #-3
L144_Chess
	LDR R7, R5, #-3
L141_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_queen_move;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_queen_move
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-1	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_bishop_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRnp L151_Chess
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_rook_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L149_Chess
L151_Chess
	CONST R7, #1
	STR R7, R5, #-1
	JMP L150_Chess
L149_Chess
	CONST R7, #0
	STR R7, R5, #-1
L150_Chess
	LDR R7, R5, #-1
L147_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_king_move;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_king_move
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-4	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #4
	LDR R3, R5, #6
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-1
	LDR R7, R5, #3
	LDR R3, R5, #5
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-2
	CONST R7, #1
	STR R7, R5, #-4
	LDR R3, R5, #-1
	CMP R3, R7
	BRp L154_Chess
	LDR R7, R5, #-2
	LDR R3, R5, #-4
	CMP R7, R3
	BRp L154_Chess
	CONST R7, #1
	STR R7, R5, #-3
	JMP L155_Chess
L154_Chess
	CONST R7, #0
	STR R7, R5, #-3
L155_Chess
	LDR R7, R5, #-3
L152_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_piece_move;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_piece_move
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-5	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRnp L157_Chess
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRnp L157_Chess
	CONST R7, #0
	JMP L156_Chess
L157_Chess
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-2
	LDR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRnp L159_Chess
	CONST R7, #0
	JMP L156_Chess
L159_Chess
	LDR R7, R5, #-2
	AND R7, R7, #8
	STR R7, R5, #-4
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-3
	LDR R7, R5, #-3
	CONST R3, #0
	CMP R7, R3
	BRz L161_Chess
	LDR R7, R5, #-3
	AND R7, R7, #8
	STR R7, R5, #-5
	LDR R7, R5, #-4
	LDR R3, R5, #-5
	CMP R7, R3
	BRnp L163_Chess
	CONST R7, #0
	JMP L156_Chess
L163_Chess
L161_Chess
	LDR R7, R5, #-2
	AND R7, R7, #7
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L165_Chess
	LDR R7, R5, #-4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_pawn_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #5	;; free space for arguments
	JMP L156_Chess
L165_Chess
	LDR R7, R5, #-1
	CONST R3, #2
	CMP R7, R3
	BRnp L167_Chess
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_rook_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	JMP L156_Chess
L167_Chess
	LDR R7, R5, #-1
	CONST R3, #3
	CMP R7, R3
	BRnp L169_Chess
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_knight_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	JMP L156_Chess
L169_Chess
	LDR R7, R5, #-1
	CONST R3, #4
	CMP R7, R3
	BRnp L171_Chess
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_bishop_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	JMP L156_Chess
L171_Chess
	LDR R7, R5, #-1
	CONST R3, #5
	CMP R7, R3
	BRnp L173_Chess
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_queen_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	JMP L156_Chess
L173_Chess
	LDR R7, R5, #-1
	CONST R3, #6
	CMP R7, R3
	BRnp L175_Chess
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_king_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	JMP L156_Chess
L175_Chess
	CONST R7, #1
L156_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_king_in_check;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_king_in_check
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-4	;; allocate stack space for local variables
	;; function body
	CONST R7, #0
	STR R7, R5, #-3
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R3, R7, #0
	STR R3, R5, #-4
	CONST R3, #0
	STR R3, R7, #0
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R3, R5, #-4
	STR R3, R7, #0
	CONST R7, #0
	STR R7, R5, #-2
L178_Chess
	CONST R7, #0
	STR R7, R5, #-1
L182_Chess
	LDR R7, R5, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-2
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_piece_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L186_Chess
	CONST R7, #1
	STR R7, R5, #-3
L186_Chess
L183_Chess
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #8
	CMP R7, R3
	BRn L182_Chess
L179_Chess
	LDR R7, R5, #-2
	ADD R7, R7, #1
	STR R7, R5, #-2
	LDR R7, R5, #-2
	CONST R3, #8
	CMP R7, R3
	BRn L178_Chess
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R3, R5, #-4
	STR R3, R7, #0
	LDR R7, R5, #-3
L177_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_ply;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_ply
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-11	;; allocate stack space for local variables
	;; function body
	LEA R7, ply
	LEA R3, board_state
	LDR R2, R7, #1
	LDR R1, R7, #0
	SLL R1, R1, #3
	ADD R1, R1, R3
	ADD R2, R2, R1
	LDR R2, R2, #0
	STR R2, R5, #-6
	LDR R2, R7, #3
	LDR R7, R7, #2
	SLL R7, R7, #3
	ADD R7, R7, R3
	ADD R7, R2, R7
	LDR R7, R7, #0
	STR R7, R5, #-8
	LDR R7, R5, #-6
	CONST R3, #0
	CMP R7, R3
	BRnp L189_Chess
	LEA R7, L191_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	CONST R7, #0
	JMP L188_Chess
L189_Chess
	LDR R7, R5, #-6
	AND R7, R7, #8
	CONST R3, #0
	CMP R7, R3
	BRz L193_Chess
	CONST R7, #1
	STR R7, R5, #-10
	JMP L194_Chess
L193_Chess
	CONST R7, #0
	STR R7, R5, #-10
L194_Chess
	LDR R7, R5, #-10
	STR R7, R5, #-9
	LDR R7, R5, #-9
	LEA R3, player_to_move
	LDR R3, R3, #0
	CMP R7, R3
	BRz L195_Chess
	LEA R7, L197_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	CONST R7, #0
	JMP L188_Chess
L195_Chess
	LEA R7, ply
	LDR R3, R7, #3
	ADD R6, R6, #-1
	STR R3, R6, #0
	LDR R3, R7, #2
	ADD R6, R6, #-1
	STR R3, R6, #0
	LDR R3, R7, #1
	ADD R6, R6, #-1
	STR R3, R6, #0
	LDR R7, R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_legal_piece_move
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	STR R7, R5, #-7
	LDR R7, R5, #-7
	CONST R3, #0
	CMP R7, R3
	BRnp L198_Chess
	CONST R7, #0
	JMP L188_Chess
L198_Chess
	LEA R7, ply
	LDR R3, R7, #1
	LDR R7, R7, #0
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	CONST R3, #0
	STR R3, R7, #0
	LEA R7, ply
	LDR R3, R7, #3
	LDR R7, R7, #2
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R3, R5, #-6
	STR R3, R7, #0
	LEA R7, player_to_move
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L201_Chess
	CONST R7, #14
	STR R7, R5, #-10
	JMP L202_Chess
L201_Chess
	CONST R7, #6
	STR R7, R5, #-10
L202_Chess
	LDR R7, R5, #-10
	STR R7, R5, #-3
	CONST R7, #0
	STR R7, R5, #-2
L203_Chess
	CONST R7, #0
	STR R7, R5, #-1
L207_Chess
	LDR R7, R5, #-1
	LDR R3, R5, #-2
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	LDR R3, R5, #-3
	CMP R7, R3
	BRnp L211_Chess
	LDR R7, R5, #-2
	STR R7, R5, #-4
	LDR R7, R5, #-1
	STR R7, R5, #-5
L211_Chess
L208_Chess
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #8
	CMP R7, R3
	BRn L207_Chess
L204_Chess
	LDR R7, R5, #-2
	ADD R7, R7, #1
	STR R7, R5, #-2
	LDR R7, R5, #-2
	CONST R3, #8
	CMP R7, R3
	BRn L203_Chess
	LDR R7, R5, #-5
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R3, R5, #-4
	ADD R6, R6, #-1
	STR R3, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R3, R6, #0
	JSR is_king_in_check
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L213_Chess
	LEA R7, L215_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	LEA R7, ply
	LDR R3, R7, #1
	LDR R7, R7, #0
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R3, R5, #-6
	STR R3, R7, #0
	LEA R7, ply
	LDR R3, R7, #3
	LDR R7, R7, #2
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R3, R5, #-8
	STR R3, R7, #0
	CONST R7, #0
	JMP L188_Chess
L213_Chess
	LDR R7, R5, #-7
	CONST R3, #2
	CMP R7, R3
	BRnp L217_Chess
	CONST R7, #1
	STR R7, R5, #-11
	JMP L218_Chess
L217_Chess
	CONST R7, #0
	STR R7, R5, #-11
L218_Chess
	LEA R7, en_passant
	LDR R3, R5, #-11
	STR R3, R7, #0
	LDR R7, R5, #-7
	CONST R3, #4
	CMP R7, R3
	BRnp L219_Chess
	LEA R7, en_passant
	LDR R3, R7, #4
	LDR R7, R7, #3
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	CONST R3, #0
	STR R3, R7, #0
	CONST R7, #4
	JMP L188_Chess
L219_Chess
	CONST R7, #1
L188_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;init_castle_status;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
init_castle_status
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	;; function body
	LEA R7, castle
	CONST R3, #1
	STR R3, R7, #0
	STR R3, R7, #1
	LEA R7, castle
	CONST R3, #1
	STR R3, R7, #2
	LEA R7, castle
	CONST R3, #1
	STR R3, R7, #3
L221_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;update_castle_status;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
update_castle_status
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-6	;; allocate stack space for local variables
	;; function body
	LEA R7, ply
	STR R7, R5, #-1
	CONST R3, #0
	LDR R2, R7, #0
	CMP R2, R3
	BRnp L223_Chess
	LDR R7, R5, #-1
	LDR R7, R7, #1
	CMP R7, R3
	BRnp L223_Chess
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #1
L223_Chess
	LEA R7, ply
	STR R7, R5, #-2
	LDR R3, R7, #0
	CONST R2, #0
	CMP R3, R2
	BRnp L225_Chess
	LDR R7, R5, #-2
	LDR R7, R7, #1
	CONST R3, #7
	CMP R7, R3
	BRnp L225_Chess
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
L225_Chess
	LEA R7, ply
	STR R7, R5, #-3
	LDR R3, R7, #0
	CONST R2, #0
	CMP R3, R2
	BRnp L227_Chess
	LDR R7, R5, #-3
	LDR R7, R7, #1
	CONST R3, #4
	CMP R7, R3
	BRnp L227_Chess
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
	STR R3, R7, #1
L227_Chess
	LEA R7, ply
	STR R7, R5, #-4
	LDR R3, R7, #0
	CONST R2, #7
	CMP R3, R2
	BRnp L229_Chess
	LDR R7, R5, #-4
	LDR R7, R7, #1
	CONST R3, #0
	CMP R7, R3
	BRnp L229_Chess
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #1
L229_Chess
	LEA R7, ply
	STR R7, R5, #-5
	CONST R3, #7
	LDR R2, R7, #0
	CMP R2, R3
	BRnp L231_Chess
	LDR R7, R5, #-5
	LDR R7, R7, #1
	CMP R7, R3
	BRnp L231_Chess
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
L231_Chess
	LEA R7, ply
	STR R7, R5, #-6
	LDR R3, R7, #0
	CONST R2, #7
	CMP R3, R2
	BRnp L233_Chess
	LDR R7, R5, #-6
	LDR R7, R7, #1
	CONST R3, #4
	CMP R7, R3
	BRnp L233_Chess
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
	STR R3, R7, #1
L233_Chess
L222_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;test_castle_checks;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
test_castle_checks
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-1	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #5
	STR R7, R5, #-1
	JMP L239_Chess
L236_Chess
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R3, R5, #4
	ADD R6, R6, #-1
	STR R3, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_king_in_check
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L240_Chess
	CONST R7, #0
	JMP L235_Chess
L240_Chess
L237_Chess
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
L239_Chess
	LDR R7, R5, #-1
	LDR R3, R5, #6
	CMP R7, R3
	BRnz L236_Chess
	CONST R7, #1
L235_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_clear_path;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_clear_path
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-5	;; allocate stack space for local variables
	;; function body
	LDR R7, R5, #6
	LDR R3, R5, #4
	SUB R7, R7, R3
	STR R7, R5, #-2
	LDR R7, R5, #5
	LDR R3, R5, #3
	SUB R7, R7, R3
	STR R7, R5, #-3
	LDR R7, R5, #-2
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-5
	LDR R3, R5, #-3
	ADD R6, R6, #-1
	STR R3, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-5
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR max
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #2	;; free space for arguments
	STR R7, R5, #-4
	LDR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRzp L243_Chess
	CONST R7, #-1
	STR R7, R5, #-2
L243_Chess
	LDR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRnz L245_Chess
	CONST R7, #1
	STR R7, R5, #-2
L245_Chess
	LDR R7, R5, #-3
	CONST R3, #0
	CMP R7, R3
	BRzp L247_Chess
	CONST R7, #-1
	STR R7, R5, #-3
L247_Chess
	LDR R7, R5, #-3
	CONST R3, #0
	CMP R7, R3
	BRnz L249_Chess
	CONST R7, #1
	STR R7, R5, #-3
L249_Chess
	CONST R7, #1
	STR R7, R5, #-1
	JMP L254_Chess
L251_Chess
	LDR R7, R5, #-1
	LDR R3, R5, #4
	LDR R2, R5, #-2
	MUL R2, R7, R2
	ADD R3, R3, R2
	LDR R2, R5, #3
	LDR R1, R5, #-3
	MUL R7, R7, R1
	ADD R7, R2, R7
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L255_Chess
	CONST R7, #0
	JMP L242_Chess
L255_Chess
L252_Chess
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
L254_Chess
	LDR R7, R5, #-1
	LDR R3, R5, #-4
	CMP R7, R3
	BRn L251_Chess
	CONST R7, #1
L242_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;handle_short_castle;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
handle_short_castle
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-1	;; allocate stack space for local variables
	;; function body
	LEA R7, player_to_move
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L258_Chess
	CONST R7, #0
	STR R7, R5, #-1
	LEA R3, castle
	LDR R3, R3, #0
	CMP R3, R7
	BRz L259_Chess
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_clear_path
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L259_Chess
	CONST R7, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR test_castle_checks
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L259_Chess
	LEA R7, board_state
	CONST R3, #0
	STR R3, R7, #4
	LEA R7, board_state
	CONST R3, #10
	STR R3, R7, #5
	LEA R7, board_state
	CONST R3, #14
	STR R3, R7, #6
	LEA R7, board_state
	CONST R3, #0
	STR R3, R7, #7
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
	STR R3, R7, #1
	CONST R7, #1
	JMP L257_Chess
L258_Chess
	LEA R7, castle
	LDR R7, R7, #2
	CONST R3, #0
	CMP R7, R3
	BRz L264_Chess
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R3, #4
	ADD R6, R6, #-1
	STR R3, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_clear_path
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L264_Chess
	CONST R7, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR test_castle_checks
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L266_Chess
	LEA R7, board_state
	CONST R3, #60
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
	LEA R7, board_state
	CONST R3, #61
	ADD R7, R7, R3
	CONST R3, #2
	STR R3, R7, #0
	LEA R7, board_state
	CONST R3, #62
	ADD R7, R7, R3
	CONST R3, #6
	STR R3, R7, #0
	LEA R7, board_state
	CONST R3, #63
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #2
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #3
	CONST R7, #1
	JMP L257_Chess
L266_Chess
L264_Chess
L259_Chess
	CONST R7, #0
L257_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;handle_long_castle;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
handle_long_castle
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-1	;; allocate stack space for local variables
	;; function body
	LEA R7, player_to_move
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L269_Chess
	CONST R7, #0
	STR R7, R5, #-1
	LEA R3, castle
	LDR R3, R3, #1
	CMP R3, R7
	BRz L270_Chess
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_clear_path
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L270_Chess
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R3, #2
	ADD R6, R6, #-1
	STR R3, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR test_castle_checks
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L270_Chess
	LEA R7, board_state
	CONST R3, #0
	STR R3, R7, #0
	CONST R3, #14
	STR R3, R7, #2
	LEA R7, board_state
	CONST R3, #10
	STR R3, R7, #3
	LEA R7, board_state
	CONST R3, #0
	STR R3, R7, #4
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #2
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
	STR R3, R7, #1
	CONST R7, #1
	JMP L268_Chess
L269_Chess
	CONST R7, #0
	STR R7, R5, #-1
	LEA R3, castle
	LDR R3, R3, #3
	CMP R3, R7
	BRz L275_Chess
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R3, R5, #-1
	ADD R6, R6, #-1
	STR R3, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR is_clear_path
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L275_Chess
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R3, #2
	ADD R6, R6, #-1
	STR R3, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR test_castle_checks
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #4	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L277_Chess
	LEA R7, board_state
	CONST R3, #56
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
	LEA R7, board_state
	CONST R3, #58
	ADD R7, R7, R3
	CONST R3, #6
	STR R3, R7, #0
	LEA R7, board_state
	CONST R3, #59
	ADD R7, R7, R3
	CONST R3, #2
	STR R3, R7, #0
	LEA R7, board_state
	CONST R3, #60
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #2
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #2
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #3
	CONST R7, #1
	JMP L268_Chess
L277_Chess
L275_Chess
L270_Chess
	CONST R7, #0
L268_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;handle_castle;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
handle_castle
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	;; function body
	LEA R7, ply
	LDR R7, R7, #4
	CONST R3, #1
	CMP R7, R3
	BRnp L280_Chess
	JSR handle_short_castle
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	JMP L279_Chess
L280_Chess
	LEA R7, ply
	LDR R7, R7, #4
	CONST R3, #2
	CMP R7, R3
	BRnp L282_Chess
	JSR handle_long_castle
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	JMP L279_Chess
L282_Chess
	CONST R7, #0
L279_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

;;;;;;;;;;;;;;;;;;;;;;;;;;;;main;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
main
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-2	;; allocate stack space for local variables
	;; function body
	LEA R7, L285_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	JSR draw_chessboard
	ADD R6, R6, #0	;; free space for arguments
	JSR init_castle_status
	ADD R6, R6, #0	;; free space for arguments
	LEA R7, player_to_move
	CONST R3, #1
	STR R3, R7, #0
	JMP L287_Chess
L286_Chess
	LEA R7, player_to_move
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L289_Chess
	LEA R7, L291_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	JMP L290_Chess
L289_Chess
	LEA R7, L292_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
L290_Chess
	JSR get_ply
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
	LDR R7, R5, #-1
	CONST R3, #-1
	CMP R7, R3
	BRnp L293_Chess
	LEA R7, L295_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
L293_Chess
	LDR R7, R5, #-1
	CONST R3, #-2
	CMP R7, R3
	BRnp L296_Chess
	LEA R7, L298_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	JSR lc4_halt
	ADD R6, R6, #0	;; free space for arguments
L296_Chess
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L299_Chess
	LEA R7, ply
	LDR R7, R7, #4
	CONST R3, #0
	CMP R7, R3
	BRz L301_Chess
	JSR handle_castle
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L303_Chess
	LEA R7, player_to_move
	CONST R3, #1
	LDR R2, R7, #0
	SUB R3, R3, R2
	STR R3, R7, #0
	JMP L302_Chess
L303_Chess
	LEA R7, L305_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	JMP L302_Chess
L301_Chess
	JSR is_legal_ply
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRz L306_Chess
	LEA R7, L308_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	LEA R7, ply
	LDR R3, R7, #3
	ADD R6, R6, #-1
	STR R3, R6, #0
	LDR R7, R7, #2
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	LEA R7, ply
	LDR R3, R7, #1
	ADD R6, R6, #-1
	STR R3, R6, #0
	LDR R7, R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
	LDR R7, R5, #-2
	CONST R3, #4
	CMP R7, R3
	BRnp L309_Chess
	LEA R7, en_passant
	LDR R3, R7, #4
	ADD R6, R6, #-1
	STR R3, R6, #0
	LDR R7, R7, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
L309_Chess
	JSR update_castle_status
	ADD R6, R6, #0	;; free space for arguments
	LEA R7, player_to_move
	CONST R3, #1
	LDR R2, R7, #0
	SUB R3, R3, R2
	STR R3, R7, #0
	JMP L307_Chess
L306_Chess
	LEA R7, L311_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
L307_Chess
L302_Chess
L299_Chess
L287_Chess
	JMP L286_Chess
	CONST R7, #0
L284_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.DATA
en_passant 		.BLKW 5
		.DATA
ply 		.BLKW 5
		.DATA
castle 		.BLKW 4
		.DATA
L311_Chess 		.STRINGZ "\nIllegal Move!!\n"
		.DATA
L308_Chess 		.STRINGZ "\nLegal Move\n"
		.DATA
L305_Chess 		.STRINGZ "\n Illegal Castle! \n"
		.DATA
L298_Chess 		.STRINGZ "\nGame Ending\n"
		.DATA
L295_Chess 		.STRINGZ "\nBad PLY entered\n"
		.DATA
L292_Chess 		.STRINGZ "\n Black to move \n"
		.DATA
L291_Chess 		.STRINGZ "\n White to move \n"
		.DATA
L285_Chess 		.STRINGZ "!!! Welcome to LC4 Chess !!!\n"
		.DATA
L215_Chess 		.STRINGZ "\n Bad Move - King would be in check! \n"
		.DATA
L197_Chess 		.STRINGZ "\n Other side to move \n"
		.DATA
L191_Chess 		.STRINGZ "\n No piece selected \n"
		.DATA
L26_Chess 		.STRINGZ "\n"
		.DATA
L19_Chess 		.STRINGZ "-32768"
		.DATA
L13_Chess 		.STRINGZ "0"
