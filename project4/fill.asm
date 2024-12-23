// Declaration
@SCREEN
D=A
@8192
D=D+A
@R0
M=D
// liste for key press
(LOOP)
@KBD
D=M
@START
D;JNE
@LOOP
0;JMP

// blacken the screen
(START)

// n --> screen
@SCREEN
D=A
@n
M=D

(BLOOP)
// listen for key unpressed
@KBD
D=M
@LOOP
D;JEQ
// if (n == R0) go to end 
@R0
D=M
@n
D=D-M
@END
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
@END
0;JMP
// end 

