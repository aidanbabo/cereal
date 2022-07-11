		.FILE "Chess.c"
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
		.LOC 76
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
		.LOC 77
		.LOC 78
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

		.LOC 81
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
		.LOC 82
		.LOC 83
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

		.LOC 89
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
		.LOC 89
		.LOC 94
	LDR R7, R5, #3
	CONST R3, #0
	CMP R7, R3
	BRnp L11_Chess
		.LOC 94
		.LOC 95
	LEA R7, L13_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 96
	JMP L10_Chess
L11_Chess
		.LOC 99
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
		.LOC 102
	LDR R7, R5, #-1
	CONST R3, #0
	CMP R7, R3
	BRzp L17_Chess
		.LOC 102
		.LOC 103
	LEA R7, L19_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 104
	JMP L10_Chess
L17_Chess
		.LOC 107
	ADD R7, R5, #-12
	ADD R7, R7, #10
	STR R7, R5, #-2
		.LOC 109
	LDR R7, R5, #-2
	ADD R7, R7, #-1
	STR R7, R5, #-2
	CONST R3, #0
	STR R3, R7, #0
	JMP L21_Chess
L20_Chess
		.LOC 111
		.LOC 112
	LDR R7, R5, #-2
	ADD R7, R7, #-1
	STR R7, R5, #-2
	LDR R3, R5, #-1
	CONST R2, #10
	MOD R3, R3, R2
	CONST R2, #48
	ADD R3, R3, R2
	STR R3, R7, #0
		.LOC 113
	LDR R7, R5, #-1
	CONST R3, #10
	DIV R7, R7, R3
	STR R7, R5, #-1
		.LOC 114
L21_Chess
		.LOC 111
	LDR R7, R5, #-1
	CONST R3, #0
	CMP R7, R3
	BRnp L20_Chess
		.LOC 117
	LDR R7, R5, #3
	CONST R3, #0
	CMP R7, R3
	BRzp L23_Chess
		.LOC 117
	LDR R7, R5, #-2
	ADD R7, R7, #-1
	STR R7, R5, #-2
	CONST R3, #45
	STR R3, R7, #0
L23_Chess
		.LOC 119
	LDR R7, R5, #-2
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 120
L10_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 122
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
		.LOC 122
		.LOC 123
	LEA R7, L26_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 124
L25_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 139
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
		.LOC 140
		.LOC 143
	LEA R7, ply
	CONST R3, #0
	STR R3, R7, #4
		.LOC 146
	JSR lc4_getc_echo
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
		.LOC 147
	LDR R7, R5, #-1
	CONST R3, #113
	CMP R7, R3
	BRnp L29_Chess
		.LOC 147
	CONST R7, #-2
	JMP L28_Chess
L29_Chess
		.LOC 149
	LDR R7, R5, #-1
	CONST R3, #120
	CMP R7, R3
	BRnp L31_Chess
		.LOC 149
		.LOC 150
	LEA R7, ply
	CONST R3, #1
	STR R3, R7, #4
		.LOC 151
	CONST R7, #1
	JMP L28_Chess
L31_Chess
		.LOC 154
	LDR R7, R5, #-1
	CONST R3, #121
	CMP R7, R3
	BRnp L33_Chess
		.LOC 154
		.LOC 155
	LEA R7, ply
	CONST R3, #2
	STR R3, R7, #4
		.LOC 156
	CONST R7, #1
	JMP L28_Chess
L33_Chess
		.LOC 159
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
		.LOC 159
	CONST R7, #-1
	JMP L28_Chess
L35_Chess
		.LOC 160
	LEA R7, ply
	LDR R3, R5, #-1
	CONST R2, #97
	SUB R3, R3, R2
	STR R3, R7, #1
		.LOC 163
	JSR lc4_getc_echo
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
		.LOC 164
	LDR R7, R5, #-1
	CONST R3, #113
	CMP R7, R3
	BRnp L38_Chess
		.LOC 164
	CONST R7, #-2
	JMP L28_Chess
L38_Chess
		.LOC 165
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
		.LOC 165
	CONST R7, #-1
	JMP L28_Chess
L40_Chess
		.LOC 166
	LEA R7, ply
	LDR R3, R5, #-1
	CONST R2, #49
	SUB R3, R3, R2
	STR R3, R7, #0
		.LOC 169
	JSR lc4_getc_echo
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
		.LOC 170
	LDR R7, R5, #-1
	CONST R3, #113
	CMP R7, R3
	BRnp L43_Chess
		.LOC 170
	CONST R7, #-2
	JMP L28_Chess
L43_Chess
		.LOC 171
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
		.LOC 171
	CONST R7, #-1
	JMP L28_Chess
L45_Chess
		.LOC 172
	LEA R7, ply
	LDR R3, R5, #-1
	CONST R2, #97
	SUB R3, R3, R2
	STR R3, R7, #3
		.LOC 175
	JSR lc4_getc_echo
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
		.LOC 176
	LDR R7, R5, #-1
	CONST R3, #113
	CMP R7, R3
	BRnp L48_Chess
		.LOC 176
	CONST R7, #-2
	JMP L28_Chess
L48_Chess
		.LOC 177
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
		.LOC 177
	CONST R7, #-1
	JMP L28_Chess
L50_Chess
		.LOC 178
	LEA R7, ply
	LDR R3, R5, #-1
	CONST R2, #49
	SUB R3, R3, R2
	STR R3, R7, #2
		.LOC 180
	CONST R7, #1
L28_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 188
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
		.LOC 189
		.LOC 194
	CONST R7, #15
	CONST R3, #7
	LDR R2, R5, #3
	SUB R3, R3, R2
	MUL R7, R7, R3
	ADD R7, R7, #2
	STR R7, R5, #-3
		.LOC 195
	CONST R7, #15
	LDR R3, R5, #4
	MUL R7, R7, R3
	ADD R7, R7, #4
	STR R7, R5, #-4
		.LOC 198
	LDR R7, R5, #3
	LDR R3, R5, #4
	ADD R7, R7, R3
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
		.LOC 199
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
		.LOC 202
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-2
		.LOC 203
	LDR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRz L57_Chess
		.LOC 203
		.LOC 204
	LDR R7, R5, #-2
	AND R7, R7, #7
	STR R7, R5, #-1
		.LOC 205
	LDR R7, R5, #-2
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
		.LOC 207
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L62_Chess
		.LOC 207
	LEA R7, pawn_sprite
	STR R7, R5, #-6
L62_Chess
		.LOC 208
	LDR R7, R5, #-1
	CONST R3, #2
	CMP R7, R3
	BRnp L64_Chess
		.LOC 208
	LEA R7, rook_sprite
	STR R7, R5, #-6
L64_Chess
		.LOC 209
	LDR R7, R5, #-1
	CONST R3, #3
	CMP R7, R3
	BRnp L66_Chess
		.LOC 209
	LEA R7, knight_sprite
	STR R7, R5, #-6
L66_Chess
		.LOC 210
	LDR R7, R5, #-1
	CONST R3, #4
	CMP R7, R3
	BRnp L68_Chess
		.LOC 210
	LEA R7, bishop_sprite
	STR R7, R5, #-6
L68_Chess
		.LOC 211
	LDR R7, R5, #-1
	CONST R3, #5
	CMP R7, R3
	BRnp L70_Chess
		.LOC 211
	LEA R7, queen_sprite
	STR R7, R5, #-6
L70_Chess
		.LOC 212
	LDR R7, R5, #-1
	CONST R3, #6
	CMP R7, R3
	BRnp L72_Chess
		.LOC 212
	LEA R7, king_sprite
	STR R7, R5, #-6
L72_Chess
		.LOC 214
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
		.LOC 215
L57_Chess
		.LOC 216
L53_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 218
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
		.LOC 219
		.LOC 222
	CONST R7, #0
	STR R7, R5, #-2
L75_Chess
		.LOC 222
		.LOC 223
	CONST R7, #0
	STR R7, R5, #-1
L79_Chess
		.LOC 223
		.LOC 224
	LDR R7, R5, #-1
	ADD R6, R6, #-1
	STR R7, R6, #0
	LDR R7, R5, #-2
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 225
L80_Chess
		.LOC 223
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
		.LOC 223
	LDR R7, R5, #-1
	CONST R3, #8
	CMP R7, R3
	BRn L79_Chess
		.LOC 226
L76_Chess
		.LOC 222
	LDR R7, R5, #-2
	ADD R7, R7, #1
	STR R7, R5, #-2
		.LOC 222
	LDR R7, R5, #-2
	CONST R3, #8
	CMP R7, R3
	BRn L75_Chess
		.LOC 227
L74_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 250
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
		.LOC 251
		.LOC 255
	LDR R7, R5, #7
	CONST R3, #0
	CMP R7, R3
	BRz L85_Chess
		.LOC 255
		.LOC 257
	LDR R7, R5, #5
	LDR R3, R5, #3
	CMP R7, R3
	BRp L86_Chess
		.LOC 257
	CONST R7, #0
	JMP L84_Chess
		.LOC 258
L85_Chess
		.LOC 258
		.LOC 260
	LDR R7, R5, #5
	LDR R3, R5, #3
	CMP R7, R3
	BRn L89_Chess
		.LOC 260
	CONST R7, #0
	JMP L84_Chess
L89_Chess
		.LOC 261
L86_Chess
		.LOC 263
	LDR R7, R5, #5
	LDR R3, R5, #3
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-2
		.LOC 264
	LDR R7, R5, #6
	LDR R3, R5, #4
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-1
		.LOC 266
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnz L91_Chess
		.LOC 266
		.LOC 267
	CONST R7, #0
	JMP L84_Chess
L91_Chess
		.LOC 268
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L93_Chess
		.LOC 268
		.LOC 271
	LDR R7, R5, #-2
	CONST R3, #1
	CMP R7, R3
	BRz L95_Chess
		.LOC 271
		.LOC 272
	CONST R7, #0
	JMP L84_Chess
L95_Chess
		.LOC 273
		.LOC 275
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-3
		.LOC 277
	LDR R7, R5, #-3
	CONST R3, #0
	CMP R7, R3
	BRz L97_Chess
		.LOC 277
		.LOC 279
	LDR R7, R5, #-3
	AND R7, R7, #8
	LDR R3, R5, #7
	CMP R7, R3
	BRnp L94_Chess
		.LOC 279
	CONST R7, #0
	JMP L84_Chess
		.LOC 280
L97_Chess
		.LOC 280
		.LOC 282
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
		.LOC 283
	CONST R7, #4
	JMP L84_Chess
L101_Chess
		.LOC 285
	CONST R7, #0
	JMP L84_Chess
		.LOC 287
		.LOC 289
L93_Chess
		.LOC 289
		.LOC 291
	LDR R7, R5, #-2
	CONST R3, #2
	CMP R7, R3
	BRnz L103_Chess
		.LOC 291
		.LOC 292
	CONST R7, #0
	JMP L84_Chess
L103_Chess
		.LOC 293
		.LOC 295
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
		.LOC 295
	CONST R7, #0
	JMP L84_Chess
L105_Chess
		.LOC 297
	LDR R7, R5, #-2
	CONST R3, #2
	CMP R7, R3
	BRnp L107_Chess
		.LOC 297
		.LOC 299
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
		.LOC 299
		.LOC 301
	LDR R7, R5, #5
	LDR R3, R5, #3
	ADD R7, R7, R3
	CONST R3, #2
	DIV R7, R7, R3
	STR R7, R5, #-4
		.LOC 302
	LDR R7, R5, #6
	LDR R3, R5, #4
	ADD R7, R7, R3
	CONST R3, #2
	DIV R7, R7, R3
	STR R7, R5, #-5
		.LOC 303
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
		.LOC 303
		.LOC 304
	CONST R7, #0
	JMP L84_Chess
L112_Chess
		.LOC 305
		.LOC 306
	LEA R7, en_passant
	LDR R3, R5, #-4
	STR R3, R7, #1
		.LOC 307
	LEA R7, en_passant
	LDR R3, R5, #-5
	STR R3, R7, #2
		.LOC 308
	LEA R7, en_passant
	LDR R3, R5, #5
	STR R3, R7, #3
		.LOC 309
	LEA R7, en_passant
	LDR R3, R5, #6
	STR R3, R7, #4
		.LOC 310
	CONST R7, #2
	JMP L84_Chess
L109_Chess
		.LOC 312
		.LOC 313
	CONST R7, #0
	JMP L84_Chess
L107_Chess
		.LOC 316
		.LOC 317
L94_Chess
		.LOC 320
	LDR R7, R5, #5
	CONST R3, #0
	CMP R7, R3
	BRnp L114_Chess
		.LOC 320
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	CONST R3, #5
	STR R3, R7, #0
L114_Chess
		.LOC 321
	LDR R7, R5, #5
	CONST R3, #7
	CMP R7, R3
	BRnp L116_Chess
		.LOC 321
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	CONST R3, #13
	STR R3, R7, #0
L116_Chess
		.LOC 323
	CONST R7, #1
L84_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 326
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
		.LOC 326
		.LOC 328
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
		.LOC 331
		.LOC 332
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
		.LOC 332
		.LOC 333
	CONST R7, #0
	JMP L118_Chess
L123_Chess
		.LOC 335
L120_Chess
		.LOC 330
	LDR R7, R5, #4
	LDR R3, R5, #8
	ADD R7, R7, R3
	STR R7, R5, #4
	LDR R7, R5, #3
	LDR R3, R5, #7
	ADD R7, R7, R3
	STR R7, R5, #3
L122_Chess
		.LOC 329
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRnp L119_Chess
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRnp L119_Chess
		.LOC 337
	CONST R7, #1
L118_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 340
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
		.LOC 341
		.LOC 343
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRnp L126_Chess
		.LOC 343
		.LOC 344
	CONST R7, #0
	STR R7, R5, #-1
		.LOC 345
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRp L128_Chess
		.LOC 346
	CONST R7, #1
	STR R7, R5, #-2
	JMP L127_Chess
L128_Chess
		.LOC 348
	CONST R7, #-1
	STR R7, R5, #-2
		.LOC 350
	JMP L127_Chess
L126_Chess
		.LOC 350
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRnp L130_Chess
		.LOC 350
		.LOC 351
	CONST R7, #0
	STR R7, R5, #-2
		.LOC 353
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRp L132_Chess
		.LOC 354
	CONST R7, #1
	STR R7, R5, #-1
	JMP L131_Chess
L132_Chess
		.LOC 356
	CONST R7, #-1
	STR R7, R5, #-1
		.LOC 358
	JMP L131_Chess
L130_Chess
		.LOC 358
		.LOC 359
	CONST R7, #0
	JMP L125_Chess
L131_Chess
L127_Chess
		.LOC 362
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

		.LOC 365
;;;;;;;;;;;;;;;;;;;;;;;;;;;;is_legal_bishop_move;;;;;;;;;;;;;;;;;;;;;;;;;;;;
		.CODE
		.FALIGN
is_legal_bishop_move
	;; prologue
	STR R7, R6, #-2	;; save return address
	STR R5, R6, #-3	;; save base pointer
	ADD R6, R6, #-3
	ADD R5, R6, #0
	ADD R6, R6, #-5	;; allocate stack space for local variables
	;; function body
		.LOC 366
		.LOC 369
	LDR R7, R5, #3
	LDR R3, R5, #5
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-5
	LDR R3, R5, #4
	LDR R2, R5, #6
	SUB R3, R3, R2
	ADD R6, R6, #-1
	STR R3, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	LDR R3, R5, #-5
	CMP R3, R7
	BRz L135_Chess
		.LOC 369
	CONST R7, #0
	JMP L134_Chess
L135_Chess
		.LOC 371
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRzp L137_Chess
		.LOC 371
		.LOC 372
	CONST R7, #1
	STR R7, R5, #-2
		.LOC 373
	JMP L138_Chess
L137_Chess
		.LOC 373
		.LOC 374
	CONST R7, #-1
	STR R7, R5, #-2
		.LOC 375
L138_Chess
		.LOC 376
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRzp L139_Chess
		.LOC 376
		.LOC 377
	CONST R7, #1
	STR R7, R5, #-1
		.LOC 378
	JMP L140_Chess
L139_Chess
		.LOC 378
		.LOC 379
	CONST R7, #-1
	STR R7, R5, #-1
		.LOC 380
L140_Chess
		.LOC 382
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

		.LOC 385
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
		.LOC 386
		.LOC 388
	LDR R7, R5, #4
	LDR R3, R5, #6
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-1
		.LOC 389
	LDR R7, R5, #3
	LDR R3, R5, #5
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-2
		.LOC 391
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

		.LOC 394
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
		.LOC 395
		.LOC 396
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

		.LOC 399
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
		.LOC 400
		.LOC 402
	LDR R7, R5, #4
	LDR R3, R5, #6
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-1
		.LOC 403
	LDR R7, R5, #3
	LDR R3, R5, #5
	SUB R7, R7, R3
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR abs
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #1	;; free space for arguments
	STR R7, R5, #-2
		.LOC 405
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

		.LOC 408
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
		.LOC 409
		.LOC 415
	LDR R7, R5, #3
	LDR R3, R5, #5
	CMP R7, R3
	BRnp L157_Chess
	LDR R7, R5, #4
	LDR R3, R5, #6
	CMP R7, R3
	BRnp L157_Chess
		.LOC 415
	CONST R7, #0
	JMP L156_Chess
L157_Chess
		.LOC 417
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-2
		.LOC 418
	LDR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRnp L159_Chess
		.LOC 418
	CONST R7, #0
	JMP L156_Chess
L159_Chess
		.LOC 420
	LDR R7, R5, #-2
	AND R7, R7, #8
	STR R7, R5, #-4
		.LOC 422
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-3
		.LOC 425
	LDR R7, R5, #-3
	CONST R3, #0
	CMP R7, R3
	BRz L161_Chess
		.LOC 425
		.LOC 426
	LDR R7, R5, #-3
	AND R7, R7, #8
	STR R7, R5, #-5
		.LOC 427
	LDR R7, R5, #-4
	LDR R3, R5, #-5
	CMP R7, R3
	BRnp L163_Chess
		.LOC 427
	CONST R7, #0
	JMP L156_Chess
L163_Chess
		.LOC 428
L161_Chess
		.LOC 431
	LDR R7, R5, #-2
	AND R7, R7, #7
	STR R7, R5, #-1
		.LOC 433
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L165_Chess
		.LOC 434
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
		.LOC 436
	LDR R7, R5, #-1
	CONST R3, #2
	CMP R7, R3
	BRnp L167_Chess
		.LOC 437
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
		.LOC 439
	LDR R7, R5, #-1
	CONST R3, #3
	CMP R7, R3
	BRnp L169_Chess
		.LOC 440
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
		.LOC 442
	LDR R7, R5, #-1
	CONST R3, #4
	CMP R7, R3
	BRnp L171_Chess
		.LOC 443
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
		.LOC 445
	LDR R7, R5, #-1
	CONST R3, #5
	CMP R7, R3
	BRnp L173_Chess
		.LOC 446
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
		.LOC 448
	LDR R7, R5, #-1
	CONST R3, #6
	CMP R7, R3
	BRnp L175_Chess
		.LOC 449
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
		.LOC 451
	CONST R7, #1
L156_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 454
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
		.LOC 455
		.LOC 456
	CONST R7, #0
	STR R7, R5, #-3
		.LOC 459
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R7, R7, #0
	STR R7, R5, #-4
		.LOC 460
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
		.LOC 461
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R3, R5, #-4
	STR R3, R7, #0
		.LOC 464
	CONST R7, #0
	STR R7, R5, #-2
L178_Chess
		.LOC 464
		.LOC 465
	CONST R7, #0
	STR R7, R5, #-1
L182_Chess
		.LOC 465
		.LOC 466
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
		.LOC 466
		.LOC 467
	CONST R7, #1
	STR R7, R5, #-3
		.LOC 468
L186_Chess
		.LOC 469
L183_Chess
		.LOC 465
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
		.LOC 465
	LDR R7, R5, #-1
	CONST R3, #8
	CMP R7, R3
	BRn L182_Chess
		.LOC 470
L179_Chess
		.LOC 464
	LDR R7, R5, #-2
	ADD R7, R7, #1
	STR R7, R5, #-2
		.LOC 464
	LDR R7, R5, #-2
	CONST R3, #8
	CMP R7, R3
	BRn L178_Chess
		.LOC 473
	LDR R7, R5, #6
	LDR R3, R5, #5
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
		.LOC 474
	LDR R7, R5, #4
	LDR R3, R5, #3
	SLL R3, R3, #3
	LEA R2, board_state
	ADD R3, R3, R2
	ADD R7, R7, R3
	LDR R3, R5, #-4
	STR R3, R7, #0
		.LOC 476
	LDR R7, R5, #-3
L177_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 479
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
		.LOC 480
		.LOC 485
	LEA R7, ply
	LDR R3, R7, #1
	LDR R7, R7, #0
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R7, R7, #0
	STR R7, R5, #-6
		.LOC 486
	LEA R7, ply
	LDR R3, R7, #3
	LDR R7, R7, #2
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R7, R7, #0
	STR R7, R5, #-8
		.LOC 488
	LDR R7, R5, #-6
	CONST R3, #0
	CMP R7, R3
	BRnp L189_Chess
		.LOC 488
		.LOC 489
	LEA R7, L191_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 490
	CONST R7, #0
	JMP L188_Chess
L189_Chess
		.LOC 491
		.LOC 492
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
		.LOC 493
		.LOC 496
	LDR R7, R5, #-9
	LEA R3, player_to_move
	LDR R3, R3, #0
	CMP R7, R3
	BRz L195_Chess
		.LOC 496
		.LOC 497
	LEA R7, L197_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 498
	CONST R7, #0
	JMP L188_Chess
L195_Chess
		.LOC 501
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
		.LOC 503
	LDR R7, R5, #-7
	CONST R3, #0
	CMP R7, R3
	BRnp L198_Chess
		.LOC 503
	CONST R7, #0
	JMP L188_Chess
L198_Chess
		.LOC 505
	LEA R7, ply
	LDR R3, R7, #1
	LDR R7, R7, #0
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	CONST R3, #0
	STR R3, R7, #0
		.LOC 506
	LEA R7, ply
	LDR R3, R7, #3
	LDR R7, R7, #2
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R3, R5, #-6
	STR R3, R7, #0
		.LOC 509
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
		.LOC 510
	CONST R7, #0
	STR R7, R5, #-2
L203_Chess
		.LOC 510
		.LOC 511
	CONST R7, #0
	STR R7, R5, #-1
L207_Chess
		.LOC 511
		.LOC 512
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
		.LOC 512
		.LOC 513
	LDR R7, R5, #-2
	STR R7, R5, #-4
		.LOC 514
	LDR R7, R5, #-1
	STR R7, R5, #-5
		.LOC 515
L211_Chess
		.LOC 516
L208_Chess
		.LOC 511
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
		.LOC 511
	LDR R7, R5, #-1
	CONST R3, #8
	CMP R7, R3
	BRn L207_Chess
		.LOC 517
L204_Chess
		.LOC 510
	LDR R7, R5, #-2
	ADD R7, R7, #1
	STR R7, R5, #-2
		.LOC 510
	LDR R7, R5, #-2
	CONST R3, #8
	CMP R7, R3
	BRn L203_Chess
		.LOC 519
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
		.LOC 519
		.LOC 520
	LEA R7, L215_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 523
	LEA R7, ply
	LDR R3, R7, #1
	LDR R7, R7, #0
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R3, R5, #-6
	STR R3, R7, #0
		.LOC 524
	LEA R7, ply
	LDR R3, R7, #3
	LDR R7, R7, #2
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	LDR R3, R5, #-8
	STR R3, R7, #0
		.LOC 526
	CONST R7, #0
	JMP L188_Chess
L213_Chess
		.LOC 530
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
		.LOC 533
	LDR R7, R5, #-7
	CONST R3, #4
	CMP R7, R3
	BRnp L219_Chess
		.LOC 533
		.LOC 534
	LEA R7, en_passant
	LDR R3, R7, #4
	LDR R7, R7, #3
	SLL R7, R7, #3
	LEA R2, board_state
	ADD R7, R7, R2
	ADD R7, R3, R7
	CONST R3, #0
	STR R3, R7, #0
		.LOC 535
	CONST R7, #4
	JMP L188_Chess
L219_Chess
		.LOC 538
	CONST R7, #1
L188_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 541
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
		.LOC 542
		.LOC 543
	LEA R7, castle
	CONST R3, #1
	STR R3, R7, #0
		.LOC 544
	LEA R7, castle
	CONST R3, #1
	STR R3, R7, #1
		.LOC 545
	LEA R7, castle
	CONST R3, #1
	STR R3, R7, #2
		.LOC 546
	LEA R7, castle
	CONST R3, #1
	STR R3, R7, #3
		.LOC 547
L221_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 549
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
		.LOC 550
		.LOC 551
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
		.LOC 552
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #1
L223_Chess
		.LOC 554
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
		.LOC 555
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
L225_Chess
		.LOC 557
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
		.LOC 557
		.LOC 558
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
		.LOC 559
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #1
		.LOC 560
L227_Chess
		.LOC 562
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
		.LOC 563
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #1
L229_Chess
		.LOC 565
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
		.LOC 566
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
L231_Chess
		.LOC 568
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
		.LOC 568
		.LOC 569
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
		.LOC 570
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #1
		.LOC 571
L233_Chess
		.LOC 572
L222_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 574
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
		.LOC 575
		.LOC 578
	LDR R7, R5, #5
	STR R7, R5, #-1
	JMP L239_Chess
L236_Chess
		.LOC 578
		.LOC 579
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
		.LOC 580
	CONST R7, #0
	JMP L235_Chess
L240_Chess
		.LOC 581
L237_Chess
		.LOC 578
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
L239_Chess
		.LOC 578
	LDR R7, R5, #-1
	LDR R3, R5, #6
	CMP R7, R3
	BRnz L236_Chess
		.LOC 583
	CONST R7, #1
L235_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 586
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
		.LOC 587
		.LOC 590
	LDR R7, R5, #6
	LDR R3, R5, #4
	SUB R7, R7, R3
	STR R7, R5, #-2
		.LOC 591
	LDR R7, R5, #5
	LDR R3, R5, #3
	SUB R7, R7, R3
	STR R7, R5, #-3
		.LOC 593
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
		.LOC 595
	LDR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRzp L243_Chess
		.LOC 595
	CONST R7, #-1
	STR R7, R5, #-2
L243_Chess
		.LOC 596
	LDR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRnz L245_Chess
		.LOC 596
	CONST R7, #1
	STR R7, R5, #-2
L245_Chess
		.LOC 598
	LDR R7, R5, #-3
	CONST R3, #0
	CMP R7, R3
	BRzp L247_Chess
		.LOC 598
	CONST R7, #-1
	STR R7, R5, #-3
L247_Chess
		.LOC 599
	LDR R7, R5, #-3
	CONST R3, #0
	CMP R7, R3
	BRnz L249_Chess
		.LOC 599
	CONST R7, #1
	STR R7, R5, #-3
L249_Chess
		.LOC 602
	CONST R7, #1
	STR R7, R5, #-1
	JMP L254_Chess
L251_Chess
		.LOC 603
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
		.LOC 604
	CONST R7, #0
	JMP L242_Chess
L255_Chess
L252_Chess
		.LOC 602
	LDR R7, R5, #-1
	ADD R7, R7, #1
	STR R7, R5, #-1
L254_Chess
		.LOC 602
	LDR R7, R5, #-1
	LDR R3, R5, #-4
	CMP R7, R3
	BRn L251_Chess
		.LOC 606
	CONST R7, #1
L242_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 609
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
		.LOC 610
		.LOC 611
	LEA R7, player_to_move
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L258_Chess
		.LOC 611
		.LOC 613
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
		.LOC 613
		.LOC 614
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
		.LOC 614
		.LOC 615
	LEA R7, board_state
	CONST R3, #0
	STR R3, R7, #4
		.LOC 616
	LEA R7, board_state
	CONST R3, #10
	STR R3, R7, #5
		.LOC 617
	LEA R7, board_state
	CONST R3, #14
	STR R3, R7, #6
		.LOC 618
	LEA R7, board_state
	CONST R3, #0
	STR R3, R7, #7
		.LOC 620
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 621
	CONST R7, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 622
	CONST R7, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 623
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 625
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
		.LOC 626
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #1
		.LOC 628
	CONST R7, #1
	JMP L257_Chess
		.LOC 630
		.LOC 632
L258_Chess
		.LOC 632
		.LOC 634
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
		.LOC 634
		.LOC 635
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
		.LOC 635
		.LOC 636
	LEA R7, board_state
	CONST R3, #60
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
		.LOC 637
	LEA R7, board_state
	CONST R3, #61
	ADD R7, R7, R3
	CONST R3, #2
	STR R3, R7, #0
		.LOC 638
	LEA R7, board_state
	CONST R3, #62
	ADD R7, R7, R3
	CONST R3, #6
	STR R3, R7, #0
		.LOC 639
	LEA R7, board_state
	CONST R3, #63
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
		.LOC 641
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 642
	CONST R7, #5
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 643
	CONST R7, #6
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 644
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 646
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #2
		.LOC 647
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #3
		.LOC 649
	CONST R7, #1
	JMP L257_Chess
L266_Chess
		.LOC 651
L264_Chess
		.LOC 652
L259_Chess
		.LOC 654
	CONST R7, #0
L257_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 657
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
		.LOC 658
		.LOC 659
	LEA R7, player_to_move
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L269_Chess
		.LOC 659
		.LOC 661
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
		.LOC 661
		.LOC 662
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
		.LOC 662
		.LOC 663
	LEA R7, board_state
	CONST R3, #0
	STR R3, R7, #0
		.LOC 664
	LEA R7, board_state
	CONST R3, #14
	STR R3, R7, #2
		.LOC 665
	LEA R7, board_state
	CONST R3, #10
	STR R3, R7, #3
		.LOC 666
	LEA R7, board_state
	CONST R3, #0
	STR R3, R7, #4
		.LOC 668
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 669
	CONST R7, #2
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 670
	CONST R7, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 671
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 673
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #0
		.LOC 674
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #1
		.LOC 676
	CONST R7, #1
	JMP L268_Chess
		.LOC 678
		.LOC 679
L269_Chess
		.LOC 679
		.LOC 681
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
		.LOC 681
		.LOC 682
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
		.LOC 682
		.LOC 683
	LEA R7, board_state
	CONST R3, #56
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
		.LOC 684
	LEA R7, board_state
	CONST R3, #58
	ADD R7, R7, R3
	CONST R3, #6
	STR R3, R7, #0
		.LOC 685
	LEA R7, board_state
	CONST R3, #59
	ADD R7, R7, R3
	CONST R3, #2
	STR R3, R7, #0
		.LOC 686
	LEA R7, board_state
	CONST R3, #60
	ADD R7, R7, R3
	CONST R3, #0
	STR R3, R7, #0
		.LOC 688
	CONST R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 689
	CONST R7, #2
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 690
	CONST R7, #3
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 691
	CONST R7, #4
	ADD R6, R6, #-1
	STR R7, R6, #0
	CONST R7, #7
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 693
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #2
		.LOC 694
	LEA R7, castle
	CONST R3, #0
	STR R3, R7, #3
		.LOC 696
	CONST R7, #1
	JMP L268_Chess
L277_Chess
		.LOC 698
L275_Chess
		.LOC 699
L270_Chess
		.LOC 701
	CONST R7, #0
L268_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 704
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
		.LOC 705
		.LOC 706
	LEA R7, ply
	LDR R7, R7, #4
	CONST R3, #1
	CMP R7, R3
	BRnp L280_Chess
		.LOC 707
	JSR handle_short_castle
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	JMP L279_Chess
L280_Chess
		.LOC 709
	LEA R7, ply
	LDR R7, R7, #4
	CONST R3, #2
	CMP R7, R3
	BRnp L282_Chess
		.LOC 710
	JSR handle_long_castle
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	JMP L279_Chess
L282_Chess
		.LOC 712
	CONST R7, #0
L279_Chess
	;; epilogue
	ADD R6, R5, #0	;; pop locals off stack
	ADD R6, R6, #3	;; free space for return address, base pointer, and return value
	STR R7, R6, #-1	;; store return value
	LDR R5, R6, #-3	;; restore base pointer
	LDR R7, R6, #-2	;; restore return address
	RET

		.LOC 719
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
		.LOC 720
		.LOC 723
	LEA R7, L285_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 725
	JSR draw_chessboard
	ADD R6, R6, #0	;; free space for arguments
		.LOC 727
	JSR init_castle_status
	ADD R6, R6, #0	;; free space for arguments
		.LOC 729
	LEA R7, player_to_move
	CONST R3, #1
	STR R3, R7, #0
	JMP L287_Chess
L286_Chess
		.LOC 731
		.LOC 733
	LEA R7, player_to_move
	LDR R7, R7, #0
	CONST R3, #0
	CMP R7, R3
	BRz L289_Chess
		.LOC 734
	LEA R7, L291_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
	JMP L290_Chess
L289_Chess
		.LOC 736
	LEA R7, L292_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
L290_Chess
		.LOC 739
	JSR get_ply
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-1
		.LOC 741
	LDR R7, R5, #-1
	CONST R3, #-1
	CMP R7, R3
	BRnp L293_Chess
		.LOC 741
		.LOC 742
	LEA R7, L295_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 743
L293_Chess
		.LOC 745
	LDR R7, R5, #-1
	CONST R3, #-2
	CMP R7, R3
	BRnp L296_Chess
		.LOC 745
		.LOC 746
	LEA R7, L298_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 747
	JSR lc4_halt
	ADD R6, R6, #0	;; free space for arguments
		.LOC 748
L296_Chess
		.LOC 750
	LDR R7, R5, #-1
	CONST R3, #1
	CMP R7, R3
	BRnp L299_Chess
		.LOC 750
		.LOC 751
	LEA R7, ply
	LDR R7, R7, #4
	CONST R3, #0
	CMP R7, R3
	BRz L301_Chess
		.LOC 751
		.LOC 752
	JSR handle_castle
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	CONST R3, #0
	CMP R7, R3
	BRz L303_Chess
		.LOC 752
		.LOC 754
	LEA R7, player_to_move
	CONST R3, #1
	LDR R2, R7, #0
	SUB R3, R3, R2
	STR R3, R7, #0
		.LOC 755
	JMP L302_Chess
L303_Chess
		.LOC 755
		.LOC 756
	LEA R7, L305_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 757
		.LOC 758
	JMP L302_Chess
L301_Chess
		.LOC 758
		.LOC 759
	JSR is_legal_ply
	LDR R7, R6, #-1	;; grab return value
	ADD R6, R6, #0	;; free space for arguments
	STR R7, R5, #-2
	CONST R3, #0
	CMP R7, R3
	BRz L306_Chess
		.LOC 759
		.LOC 760
	LEA R7, L308_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 762
	LEA R7, ply
	LDR R3, R7, #3
	ADD R6, R6, #-1
	STR R3, R6, #0
	LDR R7, R7, #2
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 763
	LEA R7, ply
	LDR R3, R7, #1
	ADD R6, R6, #-1
	STR R3, R6, #0
	LDR R7, R7, #0
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR draw_square
	ADD R6, R6, #2	;; free space for arguments
		.LOC 764
	LDR R7, R5, #-2
	CONST R3, #4
	CMP R7, R3
	BRnp L309_Chess
		.LOC 764
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
		.LOC 766
	JSR update_castle_status
	ADD R6, R6, #0	;; free space for arguments
		.LOC 769
	LEA R7, player_to_move
	CONST R3, #1
	LDR R2, R7, #0
	SUB R3, R3, R2
	STR R3, R7, #0
		.LOC 770
	JMP L307_Chess
L306_Chess
		.LOC 770
		.LOC 771
	LEA R7, L311_Chess
	ADD R6, R6, #-1
	STR R7, R6, #0
	JSR lc4_puts
	ADD R6, R6, #1	;; free space for arguments
		.LOC 772
L307_Chess
		.LOC 773
L302_Chess
		.LOC 774
L299_Chess
		.LOC 775
L287_Chess
		.LOC 731
	JMP L286_Chess
		.LOC 778
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
