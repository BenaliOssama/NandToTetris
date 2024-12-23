// R0 --> end of screen
@SCREEN
D=A
@8192
D=D+A
@R0
M=D

// n --> screen
@SCREEN
D=A
@n
M=D


(LOOP)
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
@LOOP
0;JMP

(END)
@END
0;JMP


