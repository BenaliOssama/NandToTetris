/*
for (i=0; i<n; i++){
	arr[i] = -1 
}
*/


// n = 10
@10
D=A
@n
M=D

// arr[r0..r9] -> addr, len, cap=10
@100
D=A
@arr
M=D

// i = 0 
@i
M=0

(loop)
@n
D=M

// if i < n jump to end
@i
D=D-M
@end
D;JLE
// else -1 
@arr
D=M

@i
D=D+M

A=D
M=-1

//increase i 
@i
//D=M
//D=D+1
M=M+1
@loop
0;JMP
(end)
@end
0;JMP
