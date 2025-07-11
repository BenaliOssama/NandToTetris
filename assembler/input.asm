// Declaration
// R0 --> end 
@SCREEN
D=A
@8192
D=D+A
@R0
M=D

/////////////////////////////
// listen for key press
(KEY)
@KBD
D=M
@BLACK
D;JNE
@KEY
0;JMP

// blacken the screen
(BLACK)

// n --> screen
@SCREEN
D=A
@n
M=D

(BLOOP)
// listen for key unpressed
@KBD
D=M
@WHITE
D;JEQ
// if (n == R0) go to end 
@R0
D=M
@n
D=D-M
@KEY
D;JEQ

// n ++
@n
A=M
M=-1
@n
M=M+1
@BLOOP
0;JMP

(END)
@KEY
0;JMP
// end 

(WHITE)
// first make the screen white
// n --> screen
@SCREEN
D=A
@n
M=D

// if (n == R0) go to end 
(WLOOP)
@R0
D=M
@n
D=D-M
@KEY
D;JEQ
// colore white 
@n
A=M
M=0
// n ++
@n
M=M+1
@WLOOP
0;JMP
// end whitening the screen
