// var r0 = 10  // put value in r0 

// var r1 = 2  // put value in r1
    
// var sum  = 0
@sum
M=0

// i := 0 
@i
M=0

// if   i - r1  > 0  stop
(LOOP)
@i
D=M
@R1
D=D-M
@STOP
D;JGE

// i ++ 
@i
M=M+1
// sum = sum + r0
@sum
D=M
@R0
D=D+M
@sum
M=D
@LOOP
0;JMP
// stop fmt.Println(sum)
(STOP)
@sum
D=M
@R2
M=D
// end
(END)
@END
0;JMP
